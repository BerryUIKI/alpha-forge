// Agent task commands.

use std::collections::HashSet;
use std::process::ExitCode;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{anyhow, bail, Result};
use domain::task::{AgentTask, CreateAgentTaskInput, TaskEventType, TaskStatus};

use crate::cli::TaskCommand;
use crate::context::Services;
use crate::output::{self, OutputFormat};

pub async fn run(
    services: &Services,
    command: TaskCommand,
    format: OutputFormat,
) -> Result<ExitCode> {
    match command {
        TaskCommand::List { workspace } => {
            let tasks = services.agent.list_tasks(&workspace).await?;
            output::print_tasks(&tasks, format)?;
            Ok(ExitCode::SUCCESS)
        }
        TaskCommand::Create {
            workspace,
            title,
            description,
        } => {
            let task = services
                .agent
                .create_task(CreateAgentTaskInput {
                    workspace_id: workspace,
                    title,
                    description,
                })
                .await?;
            output::print_task(&task, format)?;
            Ok(ExitCode::SUCCESS)
        }
        TaskCommand::Show { id, events } => {
            let task = get_task(services, &id).await?;

            if events {
                let events = services.agent.get_task_events(&id).await?;
                match format {
                    OutputFormat::Json => {
                        output::print_json(&serde_json::json!({ "task": task, "events": events }))?;
                    }
                    OutputFormat::Text => {
                        output::print_task(&task, format)?;
                        output::print_events(&events, format)?;
                    }
                }
            } else {
                output::print_task(&task, format)?;
            }

            Ok(ExitCode::SUCCESS)
        }
        TaskCommand::Run { id, timeout } => {
            let task = get_task(services, &id).await?;
            match task.status {
                TaskStatus::Created => {
                    services.agent.queue_task(&id).await?;
                }
                TaskStatus::Queued => {}
                other => bail!(
                    "cannot run a task in '{}' state; use `task retry` for failed or cancelled tasks",
                    other
                ),
            }

            services.agent.start_task(&id).await?;
            wait_for_terminal(services, &id, timeout.map(Duration::from_secs), format).await
        }
        TaskCommand::Cancel { id } => {
            let task = services.agent.cancel_task(&id).await?;
            if task.status.is_terminal() {
                output::print_task(&task, format)?;
                return Ok(ExitCode::SUCCESS);
            }

            wait_for_terminal(services, &id, None, format).await
        }
        TaskCommand::Retry { id } => {
            services.agent.retry_task(&id).await?;
            services.agent.start_task(&id).await?;
            wait_for_terminal(services, &id, None, format).await
        }
    }
}

async fn get_task(services: &Services, id: &str) -> Result<AgentTask> {
    services
        .agent
        .get_task(id)
        .await?
        .ok_or_else(|| anyhow!("Task '{}' not found", id))
}

async fn wait_for_terminal(
    services: &Services,
    task_id: &str,
    timeout: Option<Duration>,
    format: OutputFormat,
) -> Result<ExitCode> {
    let started = Instant::now();
    let mut seen_events: HashSet<String> = HashSet::new();
    let mut timeout_requested = false;

    let cancel_handle = Arc::clone(&services.agent);
    let cancel_id = task_id.to_string();
    let ctrl_c = tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            let _ = cancel_handle.cancel_task(&cancel_id).await;
        }
    });

    loop {
        if format == OutputFormat::Text {
            for event in services.agent.get_task_events(task_id).await? {
                if !seen_events.insert(event.id.clone()) {
                    continue;
                }

                match event.event_type {
                    TaskEventType::TaskStarted => println!("Task started."),
                    TaskEventType::TaskProgress => {
                        if let Some(payload) = &event.payload {
                            println!("{payload}");
                        }
                    }
                    _ => {}
                }
            }
        }

        let task = get_task(services, task_id).await?;
        if task.status.is_terminal() {
            ctrl_c.abort();
            output::print_task(&task, format)?;
            return Ok(match task.status {
                TaskStatus::Failed => ExitCode::from(2),
                TaskStatus::Cancelled => ExitCode::from(3),
                _ => ExitCode::SUCCESS,
            });
        }

        if let Some(limit) = timeout {
            if !timeout_requested && started.elapsed() >= limit {
                timeout_requested = true;
                if format == OutputFormat::Text {
                    println!("Timeout reached; cancelling task...");
                }
                let _ = services.agent.cancel_task(task_id).await;
            }
        }

        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}
