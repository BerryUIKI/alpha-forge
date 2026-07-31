// Settings Tauri commands — Phase 1.5 refactored to use services.

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub identifier: String,
}

#[tauri::command]
pub fn health_check() -> Result<String, AppError> {
    Ok("ok".to_string())
}

#[tauri::command]
pub fn get_app_info(app_handle: tauri::AppHandle) -> Result<AppInfo, AppError> {
    let config = app_handle.config();

    Ok(AppInfo {
        name: "Investment OS".to_string(),
        version: config.version.clone().unwrap_or_default(),
        identifier: config.identifier.clone(),
    })
}

#[tauri::command]
pub async fn get_setting(
    key: String,
    state: State<'_, AppState>,
) -> Result<Option<String>, AppError> {
    state.settings_service.get(&key).await
}

#[tauri::command]
pub async fn set_setting(
    key: String,
    value: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.settings_service.set(&key, &value).await
}