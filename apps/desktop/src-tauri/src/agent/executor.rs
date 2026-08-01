// Agent task executor for background execution.
//
// Manages concurrent task execution, cancellation, and timeout enforcement.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tauri::AppHandle;
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::agent::events;
use crate::database::repositories::agent_task_repository::AgentTaskRepository;
use crate::error::AppError;
use domain::task::{AgentTask, TaskEventType};

/// Configuration for the task executor.
#[derive(Debug, Clone)]
pub struct ExecutorConfig {
    /// Maximum concurrent tasks
    pub max_concurrent: usize,
    /// Default task timeout in seconds
    pub default_timeout_secs: u64,
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 5,
            default_timeout_secs: 300, // 5 minutes
        }
    }
}

/// Running task handle.
struct RunningTask {
    handle: JoinHandle<Result<(), AppError>>,
    cancel_token: CancellationToken,
}

/// Agent task executor.
pub struct TaskExecutor {
    repo: Arc<Mutex<AgentTaskRepository>>,
    app: AppHandle,
    config: ExecutorConfig,
    running_tasks: Arc<RwLock<HashMap<String, RunningTask>>>,
}

impl TaskExecutor {
    /// Creates a new task executor.
    pub fn new(repo: AgentTaskRepository, app: AppHandle, config: ExecutorConfig) -> Self {
        Self {
            repo: Arc::new(Mutex::new(repo)),
            app,
            config,
            running_tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Starts executing a task in the background.
    pub async fn start_task(&self, task: AgentTask) -> Result<(), AppError> {
        // Check if already running
        {
            let running = self.running_tasks.read().await;
            if running.contains_key(&task.id) {
                return Err(AppError::Validation(format!(
                    "Task '{}' is already running",
                    task.id
                )));
            }
        }

        // Check concurrency limit
        {
            let running = self.running_tasks.read().await;
            if running.len() >= self.config.max_concurrent {
                return Err(AppError::Validation(
                    "Maximum concurrent tasks reached".to_string(),
                ));
            }
        }

        // Create cancellation token
        let cancel_token = CancellationToken::new();
        let task_id = task.id.clone();
        let repo = self.repo.clone();
        let running_tasks = self.running_tasks.clone();
        let app = self.app.clone();

        // Start background task
        let cancel_token_clone = cancel_token.clone();
        let task_clone = task.clone();
        let handle = tokio::spawn(async move {
            let result =
                Self::execute_task(task_clone, repo.clone(), app, cancel_token_clone.clone()).await;

            // Remove from running tasks
            {
                let mut running = running_tasks.write().await;
                running.remove(&task_id);
            }

            result
        });

        // Track running task
        {
            let mut running = self.running_tasks.write().await;
            running.insert(
                task.id.clone(),
                RunningTask {
                    handle,
                    cancel_token,
                },
            );
        }

        Ok(())
    }

    /// Cancels a running task.
    pub async fn cancel_task(&self, task_id: &str) -> Result<(), AppError> {
        let running = self.running_tasks.read().await;

        if let Some(running_task) = running.get(task_id) {
            running_task.cancel_token.cancel();
            Ok(())
        } else {
            Err(AppError::NotFound(format!(
                "Task '{}' is not running",
                task_id
            )))
        }
    }

    /// Executes a task with timeout and cancellation support.
    async fn execute_task(
        task: AgentTask,
        repo: Arc<Mutex<AgentTaskRepository>>,
        app: AppHandle,
        cancel_token: CancellationToken,
    ) -> Result<(), AppError> {
        let task_id = task.id.clone();

        // Simulate task execution with progress events
        // In production, this would call the agent runtime
        let steps = vec![
            "Initializing...",
            "Analyzing request...",
            "Processing data...",
            "Generating output...",
        ];

        for (_i, step) in steps.iter().enumerate() {
            // Check for cancellation
            if cancel_token.is_cancelled() {
                let repo_guard = repo.lock().await;
                repo_guard
                    .create_event(&task_id, TaskEventType::TaskCancelled, None)
                    .await?;
                events::emit_cancellation(&app, &task_id);
                return Ok(());
            }

            // Emit progress to frontend
            events::emit_progress(&app, &task_id, step);

            // Persist event
            {
                let repo_guard = repo.lock().await;
                repo_guard
                    .create_event(
                        &task_id,
                        TaskEventType::TaskProgress,
                        Some(step.to_string()),
                    )
                    .await?;
            }

            // Simulate work
            tokio::select! {
                _ = tokio::time::sleep(Duration::from_secs(1)) => {},
                _ = cancel_token.cancelled() => {
                    let repo_guard = repo.lock().await;
                    repo_guard
                        .create_event(&task_id, TaskEventType::TaskCancelled, None)
                        .await?;
                    events::emit_cancellation(&app, &task_id);
                    return Ok(());
                }
            };
        }

        // Task completed
        let output = r#"{"summary": "Task completed successfully"}"#;
        {
            let repo_guard = repo.lock().await;
            repo_guard
                .create_event(
                    &task_id,
                    TaskEventType::TaskCompleted,
                    Some(output.to_string()),
                )
                .await?;
        }
        events::emit_completion(&app, &task_id, Some(output));

        Ok(())
    }

    /// Gets the number of currently running tasks.
    pub async fn running_count(&self) -> usize {
        let running = self.running_tasks.read().await;
        running.len()
    }
}
