// Agent runtime interface and simulated local runtime.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use tokio::time::sleep;

use crate::error::AppError;
use domain::task::AgentTask;

/// Result of agent execution.
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// Whether execution completed successfully.
    pub success: bool,
    /// Whether the task was cancelled cooperatively.
    pub cancelled: bool,
    /// Optional structured output (JSON).
    pub output: Option<String>,
    /// Optional error message.
    pub error: Option<String>,
}

impl ExecutionResult {
    pub fn success(output: Option<String>) -> Self {
        Self {
            success: true,
            cancelled: false,
            output,
            error: None,
        }
    }

    pub fn failure(error: impl Into<String>) -> Self {
        Self {
            success: false,
            cancelled: false,
            output: None,
            error: Some(error.into()),
        }
    }

    pub fn cancelled() -> Self {
        Self {
            success: false,
            cancelled: true,
            output: None,
            error: None,
        }
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
}

/// Callback used by runtimes to publish incremental progress.
#[allow(unknown_lints, clippy::double_must_use)]
#[async_trait::async_trait]
pub trait ProgressReporter: Send + Sync {
    async fn report(&self, message: String) -> Result<(), AppError>;
}

/// Trait for agent runtime implementations.
///
/// `async_trait` adds `#[must_use]` to the boxed future; newer Clippy versions
/// flag that as redundant because the future returns `Result`, which is already
/// `#[must_use]`.
#[allow(unknown_lints, clippy::double_must_use)]
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
        assert!(result.success);
        assert!(!result.cancelled);
        assert_eq!(result.output.as_deref(), Some("test"));
    }

    #[test]
    fn test_execution_result_failure() {
        let result = ExecutionResult::failure("Something went wrong");
        assert!(!result.success);
        assert!(!result.cancelled);
        assert_eq!(result.error.as_deref(), Some("Something went wrong"));
    }

    #[test]
    fn test_execution_result_cancelled() {
        let result = ExecutionResult::cancelled();
        assert!(!result.success);
        assert!(result.cancelled);
    }

    #[test]
    fn test_cancellation_token() {
        let token = CancellationToken::new();
        assert!(!token.is_cancelled());
        token.cancel();
        assert!(token.is_cancelled());
    }
}
