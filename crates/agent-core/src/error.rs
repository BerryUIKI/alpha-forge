use thiserror::Error;

#[derive(Debug, Error)]
pub enum SupervisorError {
    #[error("Worker binary not found for manifest '{0}'")]
    BinaryNotFound(String),

    #[error("Worker binary integrity check failed: expected SHA-256 {expected}, got {actual}")]
    IntegrityMismatch { expected: String, actual: String },

    #[error("Worker binary path is invalid or attempts path traversal: {0}")]
    InvalidBinaryPath(String),

    #[error("Failed to spawn worker subprocess: {0}")]
    SpawnFailed(#[from] std::io::Error),

    #[error("Worker handshake timed out after {timeout_ms} ms")]
    HandshakeTimeout { timeout_ms: u64 },

    #[error("Worker protocol violation: {0}")]
    Protocol(#[from] agent_protocol::ProtocolError),

    #[error("Worker exited unexpectedly with status: {0}")]
    ProcessExited(String),

    #[error("Worker process was terminated due to {reason}")]
    Terminated { reason: String },

    #[error("Active worker concurrency limit reached: maximum {max_concurrent}")]
    ConcurrencyLimitReached { max_concurrent: usize },

    #[error("Run ID '{0}' not found in active supervisor registry")]
    RunNotFound(String),

    #[error("Internal supervisor error: {0}")]
    Internal(String),
}

pub type SupervisorResult<T> = Result<T, SupervisorError>;
