// Journal Tauri commands — Phase 1 placeholder.
// Note: Thesis commands have been moved to commands::thesis module.

use crate::error::AppError;

#[tauri::command]
pub async fn list_journal_entries() -> Result<Vec<String>, AppError> {
    Ok(Vec::new())
}
