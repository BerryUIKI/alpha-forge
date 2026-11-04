// AI provider configuration Tauri commands.

use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use crate::services::provider_service::ProviderStatus;

#[tauri::command]
pub fn get_provider_status(state: State<'_, AppState>) -> Result<ProviderStatus, AppError> {
    state.provider_service.status()
}

#[tauri::command]
pub fn set_provider_api_key(api_key: String, state: State<'_, AppState>) -> Result<(), AppError> {
    state.provider_service.set_api_key(&api_key)
}

#[tauri::command]
pub fn clear_provider_api_key(state: State<'_, AppState>) -> Result<(), AppError> {
    state.provider_service.clear_api_key()
}
