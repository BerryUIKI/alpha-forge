// Agent service — handles agent task business logic and background execution.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{Mutex, Semaphore};
use tokio::time::timeout;

use crate::agent::events::EventPublisher;
use crate::agent::runtime::{AgentRuntime, CancellationToken, ExecutionResult, ProgressReporter};
use crate::database::repositories::agent_task_repository::AgentTaskRepository;
use crate::error::AppError;
use crate::services::artifact_service::ArtifactService;
use crate::text::required_text;
use domain::task::{AgentTask, AgentTaskEvent, CreateAgentTaskInput, TaskEventType, TaskStatus};

const DEFAULT_EXECUTION_TIMEOUT: Duration = Duration::from_secs(300);
const MAX_TITLE_CHARS: usize = 200;

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

    async fn load_task(&self, task_id: &str) -> Result<AgentTask, AppError> {
        self.repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Task '{}' not found", task_id)))
    }

    async fn record_event(
        &self,
        task_id: &str,
        event_type: TaskEventType,
        payload: Option<String>,
    ) -> Result<(), AppError> {
        let event = self.repo.create_event(task_id, event_type, payload).await?;
        self.publish(&event);
        Ok(())
    }

    /// Creates a new agent task.
    pub async fn create_task(&self, input: CreateAgentTaskInput) -> Result<AgentTask, AppError> {
        let CreateAgentTaskInput {
            workspace_id,
            title,
            description,
        } = input;
        let title = required_text(&title, "Task title", MAX_TITLE_CHARS)?;

        let task = self
            .repo
            .create(CreateAgentTaskInput {
                workspace_id,
                title,
                description,
            })
            .await?;
        self.record_event(&task.id, TaskEventType::TaskCreated, None)
            .await?;

        Ok(task)
    }

    /// Queues a task for execution.
    pub async fn queue_task(&self, task_id: &str) -> Result<AgentTask, AppError> {
        let task = self.load_task(task_id).await?;

        if task.status != TaskStatus::Created {
            return Err(AppError::Validation(format!(
                "Cannot queue task in '{}' state",
                task.status
            )));
        }

        let task = self.repo.update_status(task_id, TaskStatus::Queued).await?;
        self.record_event(task_id, TaskEventType::TaskQueued, None)
            .await?;

        Ok(task)
    }

    /// Starts task execution in the background.
    pub async fn start_task(&self, task_id: &str) -> Result<AgentTask, AppError> {
        let task = self.load_task(task_id).await?;

        if task.status != TaskStatus::Queued {
            return Err(AppError::Validation(format!(
                "Cannot start task in '{}' state",
                task.status
            )));
        }

        let running = self
            .repo
            .update_status(task_id, TaskStatus::Running)
            .await?;
        self.record_event(task_id, TaskEventType::TaskStarted, None)
            .await?;

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
                            ExecutionResult::failure(error.to_string()),
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

            let outcome = match permit {
                None => ExecutionResult::cancelled(),
                Some(_permit) => {
                    if cancellation.is_cancelled() {
                        ExecutionResult::cancelled()
                    } else {
                        match timeout(
                            timeout_duration,
                            runtime.execute(
                                task_for_run.clone(),
                                Arc::clone(&reporter),
                                cancellation,
                            ),
                        )
                        .await
                        {
                            Ok(Ok(result)) => result,
                            Ok(Err(error)) => ExecutionResult::failure(error.to_string()),
                            Err(_) => ExecutionResult::failure(format!(
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
                outcome,
            )
            .await;
            cancellations.lock().await.remove(&task_for_run.id);
        });

        Ok(running)
    }

    /// Completes a task.
    pub async fn complete_task(&self, task_id: &str) -> Result<AgentTask, AppError> {
        let task = self.load_task(task_id).await?;

        if task.status != TaskStatus::Running && task.status != TaskStatus::WaitingForInput {
            return Err(AppError::Validation(format!(
                "Cannot complete task in '{}' state",
                task.status
            )));
        }

        let task = self.repo.complete(task_id, None).await?;
        self.record_event(task_id, TaskEventType::TaskCompleted, None)
            .await?;

        Ok(task)
    }

    /// Cancels a task. Running tasks are cancelled cooperatively.
    pub async fn cancel_task(&self, task_id: &str) -> Result<AgentTask, AppError> {
        let task = self.load_task(task_id).await?;

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

        let task = self.repo.mark_cancelled(task_id).await?;
        self.record_event(task_id, TaskEventType::TaskCancelled, None)
            .await?;

        Ok(task)
    }

    /// Marks tasks left running by a previous process as failed.
    ///
    /// Called at startup; returns the number of recovered tasks.
    pub async fn recover_interrupted_tasks(&self) -> Result<usize, AppError> {
        let task_ids = self.repo.fail_running_tasks(INTERRUPTED_TASK_ERROR).await?;

        for task_id in &task_ids {
            if let Err(error) = self
                .record_event(
                    task_id,
                    TaskEventType::TaskFailed,
                    Some(INTERRUPTED_TASK_ERROR.to_string()),
                )
                .await
            {
                tracing::error!(
                    ?error,
                    task_id = %task_id,
                    "failed to record interrupted task event"
                );
            }
        }

        Ok(task_ids.len())
    }

    /// Returns a failed or cancelled task to the queue.
    pub async fn retry_task(&self, task_id: &str) -> Result<AgentTask, AppError> {
        let task = self.load_task(task_id).await?;

        if !matches!(task.status, TaskStatus::Failed | TaskStatus::Cancelled) {
            return Err(AppError::Validation(format!(
                "Cannot retry task in '{}' state",
                task.status
            )));
        }

        let task = self.repo.requeue(task_id).await?;
        self.record_event(task_id, TaskEventType::TaskQueued, None)
            .await?;

        Ok(task)
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
    outcome: ExecutionResult,
) {
    let (event_type, payload) = match &outcome {
        ExecutionResult::Completed { output, .. } => (TaskEventType::TaskCompleted, output.clone()),
        ExecutionResult::Failed { error } => (TaskEventType::TaskFailed, Some(error.clone())),
        ExecutionResult::Cancelled => (TaskEventType::TaskCancelled, None),
    };

    if let ExecutionResult::Completed {
        artifacts: drafts, ..
    } = &outcome
    {
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

    let update = match &outcome {
        ExecutionResult::Completed { output, .. } => {
            repo.complete(task_id, output.as_deref()).await
        }
        ExecutionResult::Failed { error } => repo.fail(task_id, error).await,
        ExecutionResult::Cancelled => repo.mark_cancelled(task_id).await,
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
