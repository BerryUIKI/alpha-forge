// Agent runtime interface.
//
// Defines the trait for agent execution and provides a demo runtime
// that simulates task execution without external AI calls.

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::database::repositories::agent_task_repository::AgentTaskRepository;
use crate::error::AppError;
use domain::task::{AgentTask, TaskEventType};

/// Result of agent execution
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// Whether execution was successful
    pub success: bool,
    /// Optional output data (JSON)
    pub output: Option<String>,
    /// Optional error message
    pub error: Option<String>,
}

/// Trait for agent runtime implementations.
#[async_trait::async_trait]
pub trait AgentRuntime: Send + Sync {
    /// Execute an agent task.
    async fn execute(&self, task: AgentTask) -> Result<ExecutionResult, AppError>;
}

/// Local demo runtime that simulates agent execution.
/// Used for testing the runtime infrastructure without external AI calls.
pub struct LocalDemoRuntime {
    repo: Arc<Mutex<AgentTaskRepository>>,
}

impl LocalDemoRuntime {
    /// Creates a new local demo runtime.
    pub fn new(repo: AgentTaskRepository) -> Self {
        Self {
            repo: Arc::new(Mutex::new(repo)),
        }
    }

    /// Simulates task execution with progress events.
    async fn simulate_execution(&self, task_id: String) -> Result<ExecutionResult, AppError> {
        // Simulate progress updates
        let repo = self.repo.lock().await;

        // Step 1: Progress - Analyzing
        repo.create_event(
            &task_id,
            TaskEventType::TaskProgress,
            Some("Analyzing request...".to_string()),
        )
        .await?;
        sleep(Duration::from_secs(2)).await;

        // Step 2: Progress - Processing
        repo.create_event(
            &task_id,
            TaskEventType::TaskProgress,
            Some("Processing data...".to_string()),
        )
        .await?;
        sleep(Duration::from_secs(2)).await;

        // Step 3: Progress - Generating output
        repo.create_event(
            &task_id,
            TaskEventType::TaskProgress,
            Some("Generating output...".to_string()),
        )
        .await?;
        sleep(Duration::from_secs(1)).await;

        // Return success
        Ok(ExecutionResult {
            success: true,
            output: Some(r#"{"summary": "Demo task completed successfully"}"#.to_string()),
            error: None,
        })
    }
}

#[async_trait::async_trait]
impl AgentRuntime for LocalDemoRuntime {
    async fn execute(&self, task: AgentTask) -> Result<ExecutionResult, AppError> {
        // Simulate async execution
        self.simulate_execution(task.id.clone()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_result_success() {
        let result = ExecutionResult {
            success: true,
            output: Some("test".to_string()),
            error: None,
        };
        assert!(result.success);
    }

    #[test]
    fn test_execution_result_failure() {
        let result = ExecutionResult {
            success: false,
            output: None,
            error: Some("Something went wrong".to_string()),
        };
        assert!(!result.success);
    }
}
