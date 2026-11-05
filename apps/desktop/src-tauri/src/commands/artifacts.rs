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
