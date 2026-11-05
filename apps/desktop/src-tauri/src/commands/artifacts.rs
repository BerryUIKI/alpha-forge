// Artifacts Tauri commands.

use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use domain::artifact::Artifact;

#[tauri::command]
pub async fn list_artifacts(
    workspace_id: Option<String>,
    task_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<Artifact>, AppError> {
    state
        .artifact_service
        .list(workspace_id.as_deref(), task_id.as_deref())
        .await
}

#[tauri::command]
pub async fn open_artifact_window(
    artifact_id: String,
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let artifact = state
        .artifact_service
        .get(&artifact_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Artifact '{}' not found", artifact_id)))?;

    crate::windows::artifact_window::open_artifact_window(&app, &artifact)
}
