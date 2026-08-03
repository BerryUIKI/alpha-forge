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
    pub fn code(&self) -> &'static str {
        match self {
            AppError::Internal(_) => "INTERNAL",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Validation(_) => "VALIDATION",
            AppError::PermissionDenied(_) => "PERMISSION_DENIED",
            AppError::Timeout(_) => "TIMEOUT",
        }
    }

    pub fn to_response(&self) -> AppErrorResponse {
        match self {
            AppError::Internal(msg) => AppErrorResponse {
                code: self.code().into(),
                message: msg.clone(),
                recoverable: false,
            },
            AppError::NotFound(msg) => AppErrorResponse {
                code: self.code().into(),
                message: msg.clone(),
                recoverable: true,
            },
            AppError::Validation(msg) => AppErrorResponse {
                code: self.code().into(),
                message: msg.clone(),
                recoverable: true,
            },
            AppError::PermissionDenied(msg) => AppErrorResponse {
                code: self.code().into(),
                message: msg.clone(),
                recoverable: true,
            },
            AppError::Timeout(msg) => AppErrorResponse {
                code: self.code().into(),
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

#[cfg(test)]
mod tests {
    use super::AppError;

    #[test]
    fn exposes_stable_error_codes() {
        assert_eq!(AppError::Internal("failure".to_string()).code(), "INTERNAL");
        assert_eq!(
            AppError::PermissionDenied("denied".to_string()).code(),
            "PERMISSION_DENIED"
        );
    }
}
