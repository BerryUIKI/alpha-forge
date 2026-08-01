// Artifacts Tauri commands — M3 Artifact Intelligence System.

use tauri::State;

use crate::app::state::AppState;
use crate::artifacts::manager::ArtifactWindowConfig;
use crate::error::AppError;
use domain::artifact::{Artifact, ArtifactType, CreateArtifactInput};

#[tauri::command]
pub async fn create_artifact(
    workspace_id: String,
    task_id: Option<String>,
    artifact_type: String,
    input: serde_json::Value,
    state: State<'_, AppState>,
) -> Result<Artifact, AppError> {
    // Parse artifact type
    let artifact_type = match artifact_type.as_str() {
        "comparison_table" => ArtifactType::ComparisonTable,
        "timeline" => ArtifactType::Timeline,
        "industry_map" => ArtifactType::IndustryMap,
        "valuation_model" => ArtifactType::ValuationModel,
        "risk_dashboard" => ArtifactType::RiskDashboard,
        "earnings_analysis" => ArtifactType::EarningsAnalysis,
        "macro_dashboard" => ArtifactType::MacroDashboard,
        other => ArtifactType::Custom(other.to_string()),
    };

    let input = CreateArtifactInput {
        workspace_id,
        task_id,
        artifact_type,
        input,
    };

    state.artifact_service.create_artifact(input).await
}

#[tauri::command]
pub async fn get_artifact(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<Artifact>, AppError> {
    state.artifact_service.get_artifact(&id).await
}

#[tauri::command]
pub async fn list_artifacts(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Artifact>, AppError> {
    state.artifact_service.list_artifacts(&workspace_id).await
}

#[tauri::command]
pub async fn list_task_artifacts(
    task_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Artifact>, AppError> {
    state.artifact_service.list_task_artifacts(&task_id).await
}

#[tauri::command]
pub async fn start_artifact_generation(
    id: String,
    state: State<'_, AppState>,
) -> Result<Artifact, AppError> {
    state.artifact_service.start_generation(&id).await
}

#[tauri::command]
pub async fn complete_artifact_generation(
    id: String,
    output: serde_json::Value,
    state: State<'_, AppState>,
) -> Result<Artifact, AppError> {
    state.artifact_service.complete_generation(&id, output).await
}

#[tauri::command]
pub async fn fail_artifact_generation(
    id: String,
    error: String,
    state: State<'_, AppState>,
) -> Result<Artifact, AppError> {
    state.artifact_service.fail_generation(&id, &error).await
}

#[tauri::command]
pub async fn start_viewing_artifact(
    id: String,
    state: State<'_, AppState>,
) -> Result<Artifact, AppError> {
    // Get artifact first
    let artifact = state
        .artifact_service
        .get_artifact(&id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Artifact '{}' not found", id)))?;

    // Open artifact window
    let window_config = ArtifactWindowConfig {
        artifact_id: id.clone(),
        artifact_type: artifact.artifact_type.to_string(),
        title: format!("Artifact: {}", artifact.artifact_type),
        width: 1024.0,
        height: 768.0,
    };

    state
        .artifact_manager
        .open_artifact(window_config)
        .await?;

    // Update status to viewing
    state.artifact_service.start_viewing(&id).await
}

#[tauri::command]
pub async fn close_artifact(
    id: String,
    state: State<'_, AppState>,
) -> Result<Artifact, AppError> {
    // Close artifact window
    state.artifact_manager.close_artifact(&id).await?;

    // Update status
    state.artifact_service.close_artifact(&id).await
}

#[tauri::command]
pub async fn delete_artifact(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.artifact_service.delete_artifact(&id).await
}

#[tauri::command]
pub async fn list_open_artifacts(
    state: State<'_, AppState>,
) -> Result<Vec<String>, AppError> {
    Ok(state.artifact_manager.list_open_artifacts().await)
}
