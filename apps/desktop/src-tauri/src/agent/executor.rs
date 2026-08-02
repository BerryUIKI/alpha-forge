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
use domain::task::{AgentTask, TaskEventType, TaskStatus};
use provider_core::{ProviderError, ResearchCompletionRequest, ResearchProvider};

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
    _handle: JoinHandle<Result<(), AppError>>,
    cancel_token: CancellationToken,
}

/// Agent task executor.
pub struct TaskExecutor {
    repo: Arc<Mutex<AgentTaskRepository>>,
    app: AppHandle,
    config: ExecutorConfig,
    provider: Arc<dyn ResearchProvider>,
    running_tasks: Arc<RwLock<HashMap<String, RunningTask>>>,
}

impl TaskExecutor {
    /// Creates a new task executor.
    pub fn new(
        repo: AgentTaskRepository,
        app: AppHandle,
        config: ExecutorConfig,
        provider: Arc<dyn ResearchProvider>,
    ) -> Self {
        Self {
            repo: Arc::new(Mutex::new(repo)),
            app,
            config,
            provider,
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
        let provider = self.provider.clone();
        let timeout = Duration::from_secs(self.config.default_timeout_secs);

        // Start background task
        let cancel_token_clone = cancel_token.clone();
        let task_clone = task.clone();
        let handle = tokio::spawn(async move {
            let result = Self::execute_task(
                task_clone,
                repo.clone(),
                app,
                provider,
                timeout,
                cancel_token_clone,
            )
            .await;

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
                    _handle: handle,
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
            events::emit_cancellation(&self.app, task_id);
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
        provider: Arc<dyn ResearchProvider>,
        timeout: Duration,
        cancel_token: CancellationToken,
    ) -> Result<(), AppError> {
        let task_id = task.id.clone();

        Self::record_progress(
            &repo,
            &app,
            &task_id,
            "Preparing structured research request...",
        )
        .await?;

        let request = research_request(&task);
        let completion = tokio::select! {
            _ = cancel_token.cancelled() => return Ok(()),
            result = tokio::time::timeout(timeout, provider.complete_research(request)) => match result {
                Ok(Ok(completion)) => completion,
                Ok(Err(error)) => return Self::record_failure(&repo, &app, &task_id, provider_failure_message(error)).await,
                Err(_) => return Self::record_failure(&repo, &app, &task_id, "The research task timed out before a result was returned.").await,
            },
        };

        if cancel_token.is_cancelled() {
            return Ok(());
        }

        let output = serde_json::to_string(&completion).map_err(|_| {
            AppError::Internal("could not serialize structured research output".to_string())
        })?;
        {
            let repo_guard = repo.lock().await;
            repo_guard
                .update_status(&task_id, TaskStatus::Completed)
                .await?;
            repo_guard
                .create_event(&task_id, TaskEventType::TaskCompleted, Some(output.clone()))
                .await?;
        }
        events::emit_completion(&app, &task_id, Some(&output));

        Ok(())
    }

    async fn record_progress(
        repo: &Arc<Mutex<AgentTaskRepository>>,
        app: &AppHandle,
        task_id: &str,
        message: &str,
    ) -> Result<(), AppError> {
        let repo_guard = repo.lock().await;
        repo_guard
            .create_event(
                task_id,
                TaskEventType::TaskProgress,
                Some(message.to_string()),
            )
            .await?;
        events::emit_progress(app, task_id, message);
        Ok(())
    }

    async fn record_failure(
        repo: &Arc<Mutex<AgentTaskRepository>>,
        app: &AppHandle,
        task_id: &str,
        message: &str,
    ) -> Result<(), AppError> {
        let repo_guard = repo.lock().await;
        repo_guard
            .update_status(task_id, TaskStatus::Failed)
            .await?;
        repo_guard
            .create_event(
                task_id,
                TaskEventType::TaskFailed,
                Some(message.to_string()),
            )
            .await?;
        events::emit_failure(app, task_id, message);
        Ok(())
    }

    /// Gets the number of currently running tasks.
    pub async fn running_count(&self) -> usize {
        let running = self.running_tasks.read().await;
        running.len()
    }
}

fn research_request(task: &AgentTask) -> ResearchCompletionRequest {
    ResearchCompletionRequest {
        system_prompt: "You are an investment research assistant. Provide factual, evidence-aware research only. Do not give buy, sell, or trade instructions, and do not claim certainty where evidence is incomplete.".to_string(),
        user_prompt: format!(
            "Research task title: {}\n\nTask details:\n{}",
            task.title,
            task.description.as_deref().unwrap_or("No additional task details were provided.")
        ),
        max_output_tokens: 2_048,
    }
}

fn provider_failure_message(error: ProviderError) -> &'static str {
    match error {
        ProviderError::CredentialsUnavailable => {
            "OpenAI credentials are unavailable. Add the openai.api_key credential in the OS keychain and retry."
        }
        ProviderError::RequestFailed => "The research provider request failed. Retry the task later.",
        ProviderError::InvalidResponse => "The research provider returned an invalid structured result. Retry the task later.",
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::{provider_failure_message, research_request};
    use domain::task::{AgentTask, TaskStatus};
    use provider_core::ProviderError;

    fn task(description: Option<&str>) -> AgentTask {
        AgentTask {
            id: "task-1".to_string(),
            workspace_id: "workspace-1".to_string(),
            title: "Assess demand".to_string(),
            description: description.map(str::to_string),
            status: TaskStatus::Running,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn builds_a_bounded_research_request_from_task_details() {
        let request = research_request(&task(Some("Compare demand drivers.")));

        assert_eq!(request.max_output_tokens, 2_048);
        assert!(request
            .system_prompt
            .contains("Do not give buy, sell, or trade instructions"));
        assert!(request.user_prompt.contains("Assess demand"));
        assert!(request.user_prompt.contains("Compare demand drivers."));
    }

    #[test]
    fn omits_sensitive_provider_error_details_from_task_events() {
        assert_eq!(
            provider_failure_message(ProviderError::RequestFailed),
            "The research provider request failed. Retry the task later."
        );
        assert!(
            provider_failure_message(ProviderError::CredentialsUnavailable)
                .contains("openai.api_key")
        );
    }
}
