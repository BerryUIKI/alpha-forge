// Agent service — handles agent task business logic and background execution.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{Mutex, Semaphore};
use tokio::time::timeout;

use crate::agent::events::EventPublisher;
use crate::agent::runtime::{AgentRuntime, CancellationToken, ProgressReporter};
use crate::artifacts::schema::ArtifactDraft;
use crate::database::repositories::agent_task_repository::AgentTaskRepository;
use crate::error::AppError;
use crate::services::artifact_service::ArtifactService;
use domain::task::{AgentTask, AgentTaskEvent, CreateAgentTaskInput, TaskEventType, TaskStatus};

const DEFAULT_EXECUTION_TIMEOUT: Duration = Duration::from_secs(300);

/// Default maximum number of agent tasks executing at the same time.
pub const DEFAULT_MAX_CONCURRENT_TASKS: usize = 2;

const INTERRUPTED_TASK_ERROR: &str = "Task interrupted by application restart";

/// Service for managing agent tasks and running them in the background.
pub struct AgentService {
    repo: AgentTaskRepository,
    runtime: Arc<dyn AgentRuntime>,
    publisher: Arc<dyn EventPublisher>,
    artifacts: ArtifactService,
    cancellations: Arc<Mutex<HashMap<String, CancellationToken>>>,
    semaphore: Arc<Semaphore>,
    execution_timeout: Duration,
}

impl AgentService {
    /// Creates a new agent service.
    pub fn new(
        repo: AgentTaskRepository,
        runtime: Arc<dyn AgentRuntime>,
        publisher: Arc<dyn EventPublisher>,
        artifacts: ArtifactService,
    ) -> Self {
        Self {
            repo,
            runtime,
            publisher,
            artifacts,
            cancellations: Arc::new(Mutex::new(HashMap::new())),
            semaphore: Arc::new(Semaphore::new(DEFAULT_MAX_CONCURRENT_TASKS)),
            execution_timeout: DEFAULT_EXECUTION_TIMEOUT,
        }
    }

    /// Overrides the execution timeout (primarily for tests).
    pub fn with_timeout(mut self, execution_timeout: Duration) -> Self {
        self.execution_timeout = execution_timeout;
        self
    }

    /// Overrides the maximum number of concurrently executing tasks.
    pub fn with_max_concurrency(mut self, max_concurrent: usize) -> Self {
        self.semaphore = Arc::new(Semaphore::new(max_concurrent.max(1)));
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
        let artifact_service = self.artifacts.clone();
        let cancellations = Arc::clone(&self.cancellations);
        let semaphore = Arc::clone(&self.semaphore);
        let timeout_duration = self.execution_timeout;
        let task_for_run = running.clone();

        tokio::spawn(async move {
            let reporter: Arc<dyn ProgressReporter> = Arc::new(TaskProgressReporter {
                repo: repo.clone(),
                publisher: Arc::clone(&publisher),
                task_id: task_for_run.id.clone(),
            });

            // Wait for an execution slot. A task cancelled while waiting is
            // finalized as cancelled without calling the runtime.
            let permit = match semaphore.clone().try_acquire_owned() {
                Ok(permit) => Some(permit),
                Err(_) => {
                    if let Err(error) = reporter
                        .report("Waiting for an execution slot...".to_string())
                        .await
                    {
                        finalize_task(
                            &repo,
                            &publisher,
                            &artifact_service,
                            &task_for_run.workspace_id,
                            &task_for_run.id,
                            FinalState::Failed(error.to_string()),
                        )
                        .await;
                        cancellations.lock().await.remove(&task_for_run.id);
                        return;
                    }

                    tokio::select! {
                        permit = semaphore.acquire_owned() => permit.ok(),
                        _ = cancellation.cancelled() => None,
                    }
                }
            };

            let final_state = match permit {
                None => FinalState::Cancelled,
                Some(_permit) => {
                    if cancellation.is_cancelled() {
                        FinalState::Cancelled
                    } else {
                        let outcome = timeout(
                            timeout_duration,
                            runtime.execute(
                                task_for_run.clone(),
                                Arc::clone(&reporter),
                                cancellation,
                            ),
                        )
                        .await;

                        match outcome {
                            Ok(Ok(result)) if result.cancelled => FinalState::Cancelled,
                            Ok(Ok(result)) if result.success => {
                                FinalState::Completed(result.output, result.artifacts)
                            }
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
                        }
                    }
                }
            };

            finalize_task(
                &repo,
                &publisher,
                &artifact_service,
                &task_for_run.workspace_id,
                &task_for_run.id,
                final_state,
            )
            .await;
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

    /// Marks tasks left running by a previous process as failed.
    ///
    /// Called at startup; returns the number of recovered tasks.
    pub async fn recover_interrupted_tasks(&self) -> Result<usize, AppError> {
        let task_ids = self.repo.fail_running_tasks(INTERRUPTED_TASK_ERROR).await?;

        for task_id in &task_ids {
            match self
                .repo
                .create_event(
                    task_id,
                    TaskEventType::TaskFailed,
                    Some(INTERRUPTED_TASK_ERROR.to_string()),
                )
                .await
            {
                Ok(event) => self.publish(&event),
                Err(error) => {
                    tracing::error!(
                        ?error,
                        task_id = %task_id,
                        "failed to record interrupted task event"
                    );
                }
            }
        }

        Ok(task_ids.len())
    }

    /// Returns a failed or cancelled task to the queue.
    pub async fn retry_task(&self, task_id: &str) -> Result<AgentTask, AppError> {
        let task = self
            .repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Task '{}' not found", task_id)))?;

        if !matches!(task.status, TaskStatus::Failed | TaskStatus::Cancelled) {
            return Err(AppError::Validation(format!(
                "Cannot retry task in '{}' state",
                task.status
            )));
        }

        self.repo.requeue(task_id).await?;

        let event = self
            .repo
            .create_event(task_id, TaskEventType::TaskQueued, None)
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

/// Persists the final task state and publishes the matching event.
async fn finalize_task(
    repo: &AgentTaskRepository,
    publisher: &Arc<dyn EventPublisher>,
    artifacts: &ArtifactService,
    workspace_id: &str,
    task_id: &str,
    final_state: FinalState,
) {
    let (event_type, payload) = match &final_state {
        FinalState::Completed(output, _) => (TaskEventType::TaskCompleted, output.clone()),
        FinalState::Failed(error) => (TaskEventType::TaskFailed, Some(error.clone())),
        FinalState::Cancelled => (TaskEventType::TaskCancelled, None),
    };

    if let FinalState::Completed(_, drafts) = &final_state {
        if !drafts.is_empty() {
            if let Err(error) = artifacts
                .persist_task_artifacts(workspace_id, task_id, drafts)
                .await
            {
                tracing::error!(
                    ?error,
                    task_id = %task_id,
                    "failed to persist task artifacts"
                );
            }
        }
    }

    let update = match &final_state {
        FinalState::Completed(output, _) => repo.complete(task_id, output.as_deref()).await,
        FinalState::Failed(error) => repo.fail(task_id, error).await,
        FinalState::Cancelled => repo.mark_cancelled(task_id).await,
    };
    if let Err(error) = update {
        tracing::error!(
            ?error,
            task_id = %task_id,
            "failed to persist agent task result"
        );
    }

    match repo.create_event(task_id, event_type, payload).await {
        Ok(event) => {
            if let Err(error) = publisher.publish(&event) {
                tracing::warn!(
                    ?error,
                    task_id = %task_id,
                    "failed to publish final agent task event"
                );
            }
        }
        Err(error) => {
            tracing::error!(
                ?error,
                task_id = %task_id,
                "failed to persist final agent task event"
            );
        }
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
    Completed(Option<String>, Vec<ArtifactDraft>),
    Failed(String),
    Cancelled,
}
