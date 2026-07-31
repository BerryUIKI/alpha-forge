use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("internal error: {0}")]
    Internal(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("permission denied: {0}")]
    PermissionDenied(String),

    #[error("timeout: {0}")]
    Timeout(String),
}

#[derive(Debug, Serialize)]
pub struct AppErrorResponse {
    pub code: String,
    pub message: String,
    pub recoverable: bool,
}

impl AppError {
    pub fn to_response(&self) -> AppErrorResponse {
        match self {
            AppError::Internal(msg) => AppErrorResponse {
                code: "INTERNAL".into(),
                message: msg.clone(),
                recoverable: false,
            },
            AppError::NotFound(msg) => AppErrorResponse {
                code: "NOT_FOUND".into(),
                message: msg.clone(),
                recoverable: true,
            },
            AppError::Validation(msg) => AppErrorResponse {
                code: "VALIDATION".into(),
                message: msg.clone(),
                recoverable: true,
            },
            AppError::PermissionDenied(msg) => AppErrorResponse {
                code: "PERMISSION_DENIED".into(),
                message: msg.clone(),
                recoverable: true,
            },
            AppError::Timeout(msg) => AppErrorResponse {
                code: "TIMEOUT".into(),
                message: msg.clone(),
                recoverable: true,
            },
        }
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_response().serialize(serializer)
    }
}
