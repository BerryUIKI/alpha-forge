// Agent runtime interface and simulated local runtime.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use tokio::time::sleep;

use crate::artifacts::schema::ArtifactDraft;
use crate::error::AppError;
use domain::task::AgentTask;

/// Result of agent execution.
#[derive(Debug, Clone)]
pub enum ExecutionResult {
    /// Execution finished and produced an optional structured output.
    Completed {
        output: Option<String>,
        artifacts: Vec<ArtifactDraft>,
    },
    /// Execution failed with a user-facing error.
    Failed { error: String },
    /// Execution was cancelled cooperatively.
    Cancelled,
}

impl ExecutionResult {
    pub fn success(output: Option<String>) -> Self {
        Self::success_with_artifacts(output, Vec::new())
    }

    pub fn success_with_artifacts(output: Option<String>, artifacts: Vec<ArtifactDraft>) -> Self {
        Self::Completed { output, artifacts }
    }

    pub fn failure(error: impl Into<String>) -> Self {
        Self::Failed {
            error: error.into(),
        }
    }

    pub fn cancelled() -> Self {
        Self::Cancelled
    }
}

/// Cooperative cancellation token shared between the service and a runtime.
#[derive(Debug, Clone, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    /// Resolves once the token has been cancelled.
    pub async fn cancelled(&self) {
        while !self.is_cancelled() {
            sleep(Duration::from_millis(100)).await;
        }
    }
}

/// Callback used by runtimes to publish incremental progress.
#[async_trait::async_trait]
pub trait ProgressReporter: Send + Sync {
    async fn report(&self, message: String) -> Result<(), AppError>;
}

/// Trait for agent runtime implementations.
#[async_trait::async_trait]
pub trait AgentRuntime: Send + Sync {
    /// Execute an agent task.
    async fn execute(
        &self,
        task: AgentTask,
        progress: Arc<dyn ProgressReporter>,
        cancellation: CancellationToken,
    ) -> Result<ExecutionResult, AppError>;
}

/// Local demo runtime that simulates execution without external AI calls.
pub struct LocalDemoRuntime;

impl LocalDemoRuntime {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LocalDemoRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl AgentRuntime for LocalDemoRuntime {
    async fn execute(
        &self,
        _task: AgentTask,
        progress: Arc<dyn ProgressReporter>,
        cancellation: CancellationToken,
    ) -> Result<ExecutionResult, AppError> {
        progress.report("Analyzing request...".to_string()).await?;
        sleep(Duration::from_secs(2)).await;
        if cancellation.is_cancelled() {
            return Ok(ExecutionResult::cancelled());
        }

        progress.report("Processing data...".to_string()).await?;
        sleep(Duration::from_secs(2)).await;
        if cancellation.is_cancelled() {
            return Ok(ExecutionResult::cancelled());
        }

        progress.report("Generating output...".to_string()).await?;
        sleep(Duration::from_secs(1)).await;
        if cancellation.is_cancelled() {
            return Ok(ExecutionResult::cancelled());
        }

        Ok(ExecutionResult::success(Some(
            r#"{"summary":"Demo task completed successfully"}"#.to_string(),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_result_success() {
        let result = ExecutionResult::success(Some("test".to_string()));

        match result {
            ExecutionResult::Completed { output, artifacts } => {
                assert_eq!(output.as_deref(), Some("test"));
                assert!(artifacts.is_empty());
            }
            other => panic!("expected completed result, got {other:?}"),
        }
    }

    #[test]
    fn test_execution_result_failure() {
        let result = ExecutionResult::failure("Something went wrong");

        match result {
            ExecutionResult::Failed { error } => {
                assert_eq!(error, "Something went wrong");
            }
            other => panic!("expected failed result, got {other:?}"),
        }
    }

    #[test]
    fn test_execution_result_cancelled() {
        assert!(matches!(
            ExecutionResult::cancelled(),
            ExecutionResult::Cancelled
        ));
    }

    #[test]
    fn test_cancellation_token() {
        let token = CancellationToken::new();
        assert!(!token.is_cancelled());
        token.cancel();
        assert!(token.is_cancelled());
    }
}
