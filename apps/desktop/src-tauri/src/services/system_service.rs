// System service — handles system-level operations.

use tauri::Manager;
use crate::error::AppError;
use serde::{Deserialize, Serialize};

/// System information structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub app_name: String,
    pub app_version: String,
    pub platform: String,
    pub architecture: String,
}

/// Service for system-level operations.
pub struct SystemService {
    app_handle: tauri::AppHandle,
}

impl SystemService {
    /// Creates a new system service.
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self { app_handle }
    }

    /// Gets system information.
    pub fn get_info(&self) -> Result<SystemInfo, AppError> {
        let config = self.app_handle.config();

        Ok(SystemInfo {
            app_name: "Investment OS".to_string(),
            app_version: config
                .version
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
            platform: std::env::consts::OS.to_string(),
            architecture: std::env::consts::ARCH.to_string(),
        })
    }

    /// Performs health check.
    pub fn health_check(&self) -> Result<String, AppError> {
        // In a real application, this would check:
        // - Database connectivity
        // - Required services status
        // - Critical resources availability

        Ok("ok".to_string())
    }

    /// Gets application configuration directory.
    pub fn get_config_dir(&self) -> Result<std::path::PathBuf, AppError> {
        self.app_handle
            .path()
            .app_config_dir()
            .map_err(|e| AppError::Internal(format!("Failed to get config directory: {}", e)))
    }

    /// Gets application data directory.
    pub fn get_data_dir(&self) -> Result<std::path::PathBuf, AppError> {
        self.app_handle
            .path()
            .app_data_dir()
            .map_err(|e| AppError::Internal(format!("Failed to get data directory: {}", e)))
    }
}
