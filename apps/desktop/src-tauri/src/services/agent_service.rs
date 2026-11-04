// Agent service — handles agent task business logic and background execution.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use tokio::time::timeout;

use crate::agent::events::EventPublisher;
use crate::agent::runtime::{AgentRuntime, CancellationToken, ProgressReporter};
use crate::database::repositories::agent_task_repository::AgentTaskRepository;
use crate::error::AppError;
use domain::task::{AgentTask, AgentTaskEvent, CreateAgentTaskInput, TaskEventType, TaskStatus};

const DEFAULT_EXECUTION_TIMEOUT: Duration = Duration::from_secs(300);

/// Service for managing agent tasks and running them in the background.
pub struct AgentService {
    repo: AgentTaskRepository,
    runtime: Arc<dyn AgentRuntime>,
    publisher: Arc<dyn EventPublisher>,
    cancellations: Arc<Mutex<HashMap<String, CancellationToken>>>,
    execution_timeout: Duration,
}

impl AgentService {
    /// Creates a new agent service.
    pub fn new(
        repo: AgentTaskRepository,
        runtime: Arc<dyn AgentRuntime>,
        publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            repo,
            runtime,
            publisher,
            cancellations: Arc::new(Mutex::new(HashMap::new())),
            execution_timeout: DEFAULT_EXECUTION_TIMEOUT,
        }
    }

    /// Overrides the execution timeout (primarily for tests).
    pub fn with_timeout(mut self, execution_timeout: Duration) -> Self {
        self.execution_timeout = execution_timeout;
        self
    }

    fn publish(&self, event: &AgentTaskEvent) {
        if let Err(error) = self.publisher.publish(event) {
            tracing::warn!(
                ?error,
                task_id = %event.task_id,
                "failed to publish agent task event"
            );
        }
    }

    /// Creates a new agent task.
    pub async fn create_task(&self, input: CreateAgentTaskInput) -> Result<AgentTask, AppError> {
        // Validate input
        if input.title.trim().is_empty() {
            return Err(AppError::Validation(
                "Task title cannot be empty".to_string(),
            ));
        }

        if input.title.len() > 200 {
            return Err(AppError::Validation(
                "Task title cannot exceed 200 characters".to_string(),
            ));
        }

        // Create task via repository
        let task = self.repo.create(input).await?;

        // The repository records a TaskCreated event; forward the latest one.
        if let Ok(events) = self.repo.list_events(&task.id).await {
            if let Some(event) = events.last() {
                self.publish(event);
            }
        }

        Ok(task)
    }

    /// Queues a task for execution.
    pub async fn queue_task(&self, task_id: &str) -> Result<AgentTask, AppError> {
        let task = self
            .repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Task '{}' not found", task_id)))?;

        // Validate state transition
        if task.status != TaskStatus::Created {
            return Err(AppError::Validation(format!(
                "Cannot queue task in '{}' state",
                task.status
            )));
        }

        // Update status
        self.repo.update_status(task_id, TaskStatus::Queued).await?;

        // Create event
        let event = self
            .repo
            .create_event(task_id, TaskEventType::TaskQueued, None)
            .await?;
        self.publish(&event);

        // Fetch updated task
        self.repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::Internal("Task disappeared after update".to_string()))
    }

    /// Starts task execution in the background.
    pub async fn start_task(&self, task_id: &str) -> Result<AgentTask, AppError> {
        let task = self
            .repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Task '{}' not found", task_id)))?;

        // Validate state transition
        if task.status != TaskStatus::Queued {
            return Err(AppError::Validation(format!(
                "Cannot start task in '{}' state",
                task.status
            )));
        }

        self.repo
            .update_status(task_id, TaskStatus::Running)
            .await?;

        let started = self
            .repo
            .create_event(task_id, TaskEventType::TaskStarted, None)
            .await?;
        self.publish(&started);

        let running = self
            .repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::Internal("Task disappeared after update".to_string()))?;

        let cancellation = CancellationToken::new();
        self.cancellations
            .lock()
            .await
            .insert(task_id.to_string(), cancellation.clone());

        let runtime = Arc::clone(&self.runtime);
        let repo = self.repo.clone();
        let publisher = Arc::clone(&self.publisher);
        let cancellations = Arc::clone(&self.cancellations);
        let timeout_duration = self.execution_timeout;
        let task_for_run = running.clone();

        tokio::spawn(async move {
            let reporter: Arc<dyn ProgressReporter> = Arc::new(TaskProgressReporter {
                repo: repo.clone(),
                publisher: Arc::clone(&publisher),
                task_id: task_for_run.id.clone(),
            });

            let outcome = timeout(
                timeout_duration,
                runtime.execute(task_for_run.clone(), reporter, cancellation),
            )
            .await;

            let final_state = match outcome {
                Ok(Ok(result)) if result.cancelled => FinalState::Cancelled,
                Ok(Ok(result)) if result.success => FinalState::Completed(result.output),
                Ok(Ok(result)) => FinalState::Failed(
                    result
                        .error
                        .unwrap_or_else(|| "Agent task failed".to_string()),
                ),
                Ok(Err(error)) => FinalState::Failed(error.to_string()),
                Err(_) => FinalState::Failed(format!(
                    "Agent task timed out after {} ms",
                    timeout_duration.as_millis()
                )),
            };

            let (event_type, payload) = match &final_state {
                FinalState::Completed(output) => (TaskEventType::TaskCompleted, output.clone()),
                FinalState::Failed(error) => (TaskEventType::TaskFailed, Some(error.clone())),
                FinalState::Cancelled => (TaskEventType::TaskCancelled, None),
            };

            let update = match &final_state {
                FinalState::Completed(output) => {
                    repo.complete(&task_for_run.id, output.as_deref()).await
                }
                FinalState::Failed(error) => repo.fail(&task_for_run.id, error).await,
                FinalState::Cancelled => repo.mark_cancelled(&task_for_run.id).await,
            };
            if let Err(error) = update {
                tracing::error!(
                    ?error,
                    task_id = %task_for_run.id,
                    "failed to persist agent task result"
                );
            }

            match repo
                .create_event(&task_for_run.id, event_type, payload)
                .await
            {
                Ok(event) => {
                    if let Err(error) = publisher.publish(&event) {
                        tracing::warn!(
                            ?error,
                            task_id = %task_for_run.id,
                            "failed to publish final agent task event"
                        );
                    }
                }
                Err(error) => {
                    tracing::error!(
                        ?error,
                        task_id = %task_for_run.id,
                        "failed to persist final agent task event"
                    );
                }
            }

            cancellations.lock().await.remove(&task_for_run.id);
        });

        Ok(running)
    }

    /// Completes a task.
    pub async fn complete_task(&self, task_id: &str) -> Result<AgentTask, AppError> {
        let task = self
            .repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Task '{}' not found", task_id)))?;

        if task.status != TaskStatus::Running && task.status != TaskStatus::WaitingForInput {
            return Err(AppError::Validation(format!(
                "Cannot complete task in '{}' state",
                task.status
            )));
        }

        self.repo.complete(task_id, None).await?;

        let event = self
            .repo
            .create_event(task_id, TaskEventType::TaskCompleted, None)
            .await?;
        self.publish(&event);

        self.repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::Internal("Task disappeared after update".to_string()))
    }

    /// Fails a task with an error message.
    pub async fn fail_task(&self, task_id: &str, error: String) -> Result<AgentTask, AppError> {
        self.repo.fail(task_id, &error).await?;

        let event = self
            .repo
            .create_event(task_id, TaskEventType::TaskFailed, Some(error))
            .await?;
        self.publish(&event);

        self.repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::Internal("Task disappeared after update".to_string()))
    }

    /// Cancels a task. Running tasks are cancelled cooperatively.
    pub async fn cancel_task(&self, task_id: &str) -> Result<AgentTask, AppError> {
        let task = self
            .repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Task '{}' not found", task_id)))?;

        if !task.status.is_cancellable() {
            return Err(AppError::Validation(format!(
                "Cannot cancel task in '{}' state",
                task.status
            )));
        }

        if task.status == TaskStatus::Running {
            let token = self.cancellations.lock().await.get(task_id).cloned();
            if let Some(token) = token {
                token.cancel();
                // The background task persists the final cancelled state.
                return Ok(task);
            }
        }

        self.repo.mark_cancelled(task_id).await?;

        let event = self
            .repo
            .create_event(task_id, TaskEventType::TaskCancelled, None)
            .await?;
        self.publish(&event);

        self.repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::Internal("Task disappeared after update".to_string()))
    }

    /// Gets a task by ID.
    pub async fn get_task(&self, task_id: &str) -> Result<Option<AgentTask>, AppError> {
        self.repo.get(task_id).await
    }

    /// Lists tasks for a workspace.
    pub async fn list_tasks(&self, workspace_id: &str) -> Result<Vec<AgentTask>, AppError> {
        self.repo.list_by_workspace(workspace_id).await
    }

    /// Gets task events.
    pub async fn get_task_events(&self, task_id: &str) -> Result<Vec<AgentTaskEvent>, AppError> {
        self.repo.list_events(task_id).await
    }
}

/// Progress reporter backed by the repository and Tauri event publisher.
struct TaskProgressReporter {
    repo: AgentTaskRepository,
    publisher: Arc<dyn EventPublisher>,
    task_id: String,
}

#[async_trait::async_trait]
impl ProgressReporter for TaskProgressReporter {
    async fn report(&self, message: String) -> Result<(), AppError> {
        let event = self
            .repo
            .create_event(&self.task_id, TaskEventType::TaskProgress, Some(message))
            .await?;

        if let Err(error) = self.publisher.publish(&event) {
            tracing::warn!(
                ?error,
                task_id = %self.task_id,
                "failed to publish agent progress event"
            );
        }

        Ok(())
    }
}

/// Final outcome of a background execution.
enum FinalState {
    Completed(Option<String>),
    Failed(String),
    Cancelled,
}
