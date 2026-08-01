// System commands for system-level operations.

use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use crate::services::system_service::SystemInfo;

/// Gets system information.
#[tauri::command]
pub async fn get_system_info(state: State<'_, AppState>) -> Result<SystemInfo, AppError> {
    state.system_service.get_info()
}

/// Gets application configuration directory.
#[tauri::command]
pub async fn get_config_dir(state: State<'_, AppState>) -> Result<String, AppError> {
    let path = state.system_service.get_config_dir()?;
    Ok(path.to_string_lossy().to_string())
}

/// Gets application data directory.
#[tauri::command]
pub async fn get_data_dir(state: State<'_, AppState>) -> Result<String, AppError> {
    let path = state.system_service.get_data_dir()?;
    Ok(path.to_string_lossy().to_string())
}

/// Performs health check.
#[tauri::command]
pub async fn check_database_health(state: State<'_, AppState>) -> Result<String, AppError> {
    // Simple health check - try to ping the database
    sqlx::query("SELECT 1")
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(format!("Database health check failed: {}", e)))?;

    Ok("healthy".to_string())
}
