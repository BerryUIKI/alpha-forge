// Journal Tauri commands — Phase 1 placeholder.
// Note: Thesis commands have been moved to commands::thesis module.

use crate::error::AppError;

#[tauri::command]
pub async fn list_journal_entries(workspace_id: String) -> Result<Vec<String>, AppError> {
    // TODO: Implement journal entries retrieval from database
    // Currently returns empty array as placeholder
    let _ = workspace_id; // Acknowledge parameter until implementation
    Ok(Vec::new())
}
