// Agent task commands.
//
// `task start` reuses the desktop task executor through the headless
// `TaskEventEmitter` boundary. The CLI process hosts the execution, persists the
// same task events, and waits for the terminal state so a human sees the result.

use std::io::Write;
use std::process::ExitCode;
use std::sync::Arc;
use std::time::{Duration, Instant};

use alpha_forge_lib::agent::executor::{ExecutorConfig, TaskExecutor};
use alpha_forge_lib::database::repositories::agent_task_repository::AgentTaskRepository;
use alpha_forge_lib::error::AppError;
use domain::task::{AgentTask, AgentTaskEvent, CreateAgentTaskInput, TaskEventType, TaskStatus};
use serde::Serialize;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::cli::TaskCommand;
use crate::context::Services;
use crate::error::{CliError, EXIT_TASK_CANCELLED, EXIT_TASK_FAILED};
use crate::events::ChannelTaskEventEmitter;
use crate::output::{self, OutputFormat};

/// How often the runner re-reads the persisted task state.
const POLL_INTERVAL: Duration = Duration::from_millis(200);

/// Maximum time to wait for the terminal event after a terminal status.
///
/// The executor persists the terminal status just before the terminal event; a
/// short grace period closes that gap without hanging the CLI.
const TERMINAL_EVENT_GRACE: Duration = Duration::from_secs(2);
const TERMINAL_EVENT_POLL: Duration = Duration::from_millis(50);

/// JSON shape returned by `task start`.
#[derive(Debug, Serialize)]
struct TaskRunOutput<'a> {
    task: &'a AgentTask,
    events: &'a [AgentTaskEvent],
}

pub async fn run(
    services: &Services,
    command: TaskCommand,
    format: OutputFormat,
    out: &mut dyn Write,
) -> Result<ExitCode, CliError> {
    match command {
        TaskCommand::List { workspace } => {
            let tasks = services.agent.list_tasks(&workspace).await?;
            print_tasks(out, &tasks, format)?;
            Ok(ExitCode::SUCCESS)
        }
        TaskCommand::Create {
            workspace,
            title,
            description,
        } => {
            ensure_workspace_exists(services, &workspace).await?;

            let task = services
                .agent
                .create_task(CreateAgentTaskInput {
                    workspace_id: workspace,
                    title,
                    description,
                })
                .await?;

            print_task(out, &task, format)?;
            Ok(ExitCode::SUCCESS)
        }
        TaskCommand::Start { id, timeout } => start(services, &id, timeout, format, out).await,
        TaskCommand::Cancel { id } => {
            let task = services.agent.cancel_task(&id).await?;
            print_task(out, &task, format)?;
            Ok(ExitCode::SUCCESS)
        }
    }
}

async fn ensure_workspace_exists(services: &Services, workspace_id: &str) -> Result<(), CliError> {
    if services.workspaces.get(workspace_id).await?.is_none() {
        return Err(AppError::Validation(format!("Workspace '{}' not found", workspace_id)).into());
    }

    Ok(())
}

async fn start(
    services: &Services,
    task_id: &str,
    timeout_secs: Option<u64>,
    format: OutputFormat,
    out: &mut dyn Write,
) -> Result<ExitCode, CliError> {
    let task = get_task(services, task_id).await?;

    match task.status {
        TaskStatus::Created => {
            services.agent.queue_task(task_id).await?;
        }
        TaskStatus::Queued => {}
        other => {
            return Err(
                AppError::Validation(format!("Cannot start task in '{}' state", other)).into(),
            );
        }
    }

    let (emitter, mut events) = ChannelTaskEventEmitter::new();
    let executor = Arc::new(TaskExecutor::new(
        AgentTaskRepository::new(services.database_pool()),
        Arc::new(emitter),
        ExecutorConfig::default(),
        services.provider.clone(),
    ));

    let running = services.agent.start_task(task_id).await?;
    if let Err(error) = executor.start_task(running).await {
        // Preserve the queued state when the executor cannot admit the task.
        let _ = services.agent.requeue_running_task(task_id).await;
        return Err(error.into());
    }

    if !format.is_json() {
        writeln!(out, "Task started.").map_err(CliError::Output)?;
    }

    let ctrl_c = spawn_cancel_on_interrupt(services, executor.clone(), task_id);

    let deadline = timeout_secs.map(|secs| Instant::now() + Duration::from_secs(secs));
    let mut timeout_requested = false;

    loop {
        drain_progress(&mut events, format, out)?;

        let current = get_task(services, task_id).await?;

        if current.status == TaskStatus::Cancelled {
            // A cancellation may have been persisted by another process (the
            // desktop app or a second CLI invocation). Stop the request that is
            // still running in this process so it cannot overwrite the state.
            let _ = executor.cancel_task(task_id).await;
        }

        if current.status == TaskStatus::WaitingForInput {
            ctrl_c.abort();
            return Err(AppError::Validation(
                "Task is waiting for input and cannot continue from the CLI".to_string(),
            )
            .into());
        }

        if current.status.is_terminal() {
            ctrl_c.abort();
            drain_progress(&mut events, format, out)?;

            let persisted = settle_terminal_events(services, task_id, &current.status).await?;
            return finish(current, &persisted, format, out);
        }

        if let Some(deadline) = deadline {
            if !timeout_requested && Instant::now() >= deadline {
                timeout_requested = true;
                if !format.is_json() {
                    writeln!(out, "Timeout reached; cancelling task...")
                        .map_err(CliError::Output)?;
                }
                let _ = executor.cancel_task(task_id).await;
                // A concurrent completion can win this race; that is not an error.
                let _ = services.agent.cancel_task(task_id).await;
            }
        }

        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

fn spawn_cancel_on_interrupt(
    services: &Services,
    executor: Arc<TaskExecutor>,
    task_id: &str,
) -> tokio::task::JoinHandle<()> {
    let agent = services.agent.clone();
    let task_id = task_id.to_string();

    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            let _ = executor.cancel_task(&task_id).await;
            let _ = agent.cancel_task(&task_id).await;
        }
    })
}

async fn get_task(services: &Services, task_id: &str) -> Result<AgentTask, CliError> {
    services
        .agent
        .get_task(task_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Task '{}' not found", task_id)).into())
}

/// Waits until the persisted timeline contains the event for the terminal
/// status, then returns the complete timeline.
async fn settle_terminal_events(
    services: &Services,
    task_id: &str,
    status: &TaskStatus,
) -> Result<Vec<AgentTaskEvent>, CliError> {
    let expected = match status {
        TaskStatus::Completed => Some(TaskEventType::TaskCompleted),
        TaskStatus::Failed => Some(TaskEventType::TaskFailed),
        TaskStatus::Cancelled => Some(TaskEventType::TaskCancelled),
        _ => None,
    };
    let deadline = Instant::now() + TERMINAL_EVENT_GRACE;

    loop {
        let events = services.agent.get_task_events(task_id).await?;
        let settled = expected
            .as_ref()
            .is_none_or(|expected| events.iter().any(|event| &event.event_type == expected));

        if settled || Instant::now() >= deadline {
            return Ok(events);
        }

        tokio::time::sleep(TERMINAL_EVENT_POLL).await;
    }
}

fn finish(
    task: AgentTask,
    events: &[AgentTaskEvent],
    format: OutputFormat,
    out: &mut dyn Write,
) -> Result<ExitCode, CliError> {
    let exit_code = match task.status {
        TaskStatus::Failed => ExitCode::from(EXIT_TASK_FAILED),
        TaskStatus::Cancelled => ExitCode::from(EXIT_TASK_CANCELLED),
        _ => ExitCode::SUCCESS,
    };

    match format {
        OutputFormat::Json => output::print_json(
            out,
            &TaskRunOutput {
                task: &task,
                events,
            },
        )?,
        OutputFormat::Text => {
            writeln!(out, "{}", output::render_task_result(&task, events))
                .map_err(CliError::Output)?;
        }
    }

    Ok(exit_code)
}

fn drain_progress(
    events: &mut UnboundedReceiver<AgentTaskEvent>,
    format: OutputFormat,
    out: &mut dyn Write,
) -> Result<(), CliError> {
    while let Ok(event) = events.try_recv() {
        if format.is_json() {
            continue;
        }

        match event.event_type {
            TaskEventType::TaskStarted => {
                writeln!(out, "Task started.").map_err(CliError::Output)?;
            }
            TaskEventType::TaskProgress => {
                if let Some(payload) = &event.payload {
                    writeln!(out, "{payload}").map_err(CliError::Output)?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn print_tasks(
    out: &mut dyn Write,
    tasks: &[AgentTask],
    format: OutputFormat,
) -> Result<(), CliError> {
    match format {
        OutputFormat::Json => output::print_json(out, tasks),
        OutputFormat::Text => {
            writeln!(out, "{}", output::render_tasks(tasks)).map_err(CliError::Output)
        }
    }
}

fn print_task(out: &mut dyn Write, task: &AgentTask, format: OutputFormat) -> Result<(), CliError> {
    match format {
        OutputFormat::Json => output::print_json(out, task),
        OutputFormat::Text => {
            writeln!(out, "{}", output::render_task(task)).map_err(CliError::Output)
        }
    }
}
