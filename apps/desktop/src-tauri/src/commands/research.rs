// Research Tauri commands — Phase 1 placeholder.
// Will be connected to the research system in later phases.

use crate::error::AppError;

#[tauri::command]
pub async fn list_research_documents() -> Result<Vec<String>, AppError> {
    Ok(Vec::new())
}
