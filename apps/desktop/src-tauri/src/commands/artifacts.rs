// Artifacts Tauri commands — Phase 1 placeholder.

use crate::error::AppError;

#[tauri::command]
pub async fn list_artifacts() -> Result<Vec<String>, AppError> {
    Ok(Vec::new())
}
