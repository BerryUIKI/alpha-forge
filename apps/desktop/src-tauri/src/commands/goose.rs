//! Tauri commands for Goose integration

use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use crate::services::goose_service::{
    GooseHealthStatus, ShadowAnalysisResult, StartShadowAnalysisInput,
};

/// Start a Goose shadow analysis
#[tauri::command]
pub async fn start_goose_shadow_analysis(
    input: StartShadowAnalysisInput,
    state: State<'_, AppState>,
) -> Result<ShadowAnalysisResult, AppError> {
    let goose_service = state
        .goose_service
        .as_ref()
        .ok_or_else(|| AppError::Internal("Goose service not initialized".to_string()))?;

    goose_service.start_shadow_analysis(input).await
}

/// Cancel a running Goose analysis
#[tauri::command]
pub async fn cancel_goose_analysis(
    run_id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let goose_service = state
        .goose_service
        .as_ref()
        .ok_or_else(|| AppError::Internal("Goose service not initialized".to_string()))?;

    goose_service.cancel_analysis(&run_id).await
}

/// Check Goose service health
#[tauri::command]
pub async fn check_goose_health(state: State<'_, AppState>) -> Result<GooseHealthStatus, AppError> {
    let goose_service = state
        .goose_service
        .as_ref()
        .ok_or_else(|| AppError::Internal("Goose service not initialized".to_string()))?;

    goose_service.health_check().await
}
