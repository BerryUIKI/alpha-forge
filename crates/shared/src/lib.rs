// Shared utilities and common types.

use serde::{Deserialize, Serialize};

/// Standard error codes for the application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorCode {
    pub code: String,
    pub message: String,
    pub recoverable: bool,
}

impl ErrorCode {
    pub fn internal(msg: impl Into<String>) -> Self {
        Self {
            code: "INTERNAL".into(),
            message: msg.into(),
            recoverable: false,
        }
    }

    pub fn validation(msg: impl Into<String>) -> Self {
        Self {
            code: "VALIDATION".into(),
            message: msg.into(),
            recoverable: true,
        }
    }
}
