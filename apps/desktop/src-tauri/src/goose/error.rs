//! Goose integration errors

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum GooseError {
    #[error("Goose binary not found at {path}")]
    BinaryNotFound { path: PathBuf },

    #[error("Goose binary integrity check failed: expected {expected}, got {actual}")]
    IntegrityCheckFailed { expected: String, actual: String },

    #[error("Failed to spawn Goose process: {source}")]
    SpawnFailed { source: std::io::Error },

    #[error("Goose process timed out after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },

    #[error("Goose process was cancelled")]
    Cancelled,

    #[error("Goose process exited with non-zero status: {code}")]
    ExitCode { code: i32 },

    #[error("Failed to parse Goose output: {source}")]
    OutputParseError { source: serde_json::Error },

    #[error("Goose output validation failed: {reason}")]
    OutputValidationFailed { reason: String },

    #[error("Recipe validation failed: {reason}")]
    RecipeValidationFailed { reason: String },

    #[error("Recipe file not found: {path}")]
    RecipeNotFound { path: PathBuf },

    #[error("Output exceeded byte limit: {size} > {limit}")]
    OutputSizeExceeded { size: usize, limit: usize },

    #[error("Goose execution budget exceeded: {budget_type}")]
    BudgetExceeded { budget_type: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl GooseError {
    pub fn code(&self) -> &'static str {
        match self {
            GooseError::BinaryNotFound { .. } => "GOOSE_BINARY_NOT_FOUND",
            GooseError::IntegrityCheckFailed { .. } => "GOOSE_INTEGRITY_FAILED",
            GooseError::SpawnFailed { .. } => "GOOSE_SPAWN_FAILED",
            GooseError::Timeout { .. } => "GOOSE_TIMEOUT",
            GooseError::Cancelled => "GOOSE_CANCELLED",
            GooseError::ExitCode { .. } => "GOOSE_EXIT_CODE",
            GooseError::OutputParseError { .. } => "GOOSE_OUTPUT_PARSE_ERROR",
            GooseError::OutputValidationFailed { .. } => "GOOSE_OUTPUT_VALIDATION_FAILED",
            GooseError::RecipeValidationFailed { .. } => "GOOSE_RECIPE_VALIDATION_FAILED",
            GooseError::RecipeNotFound { .. } => "GOOSE_RECIPE_NOT_FOUND",
            GooseError::OutputSizeExceeded { .. } => "GOOSE_OUTPUT_SIZE_EXCEEDED",
            GooseError::BudgetExceeded { .. } => "GOOSE_BUDGET_EXCEEDED",
            GooseError::Io(_) => "GOOSE_IO_ERROR",
            GooseError::Internal(_) => "GOOSE_INTERNAL_ERROR",
        }
    }

    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            GooseError::Timeout { .. }
                | GooseError::Cancelled
                | GooseError::OutputSizeExceeded { .. }
                | GooseError::BudgetExceeded { .. }
        )
    }
}