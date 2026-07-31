// Agent event types — Phase 1 placeholder.
// Tauri event streaming will be wired in later phases.

use crate::agent::task::TaskEvent;

/// Emit a task event to the frontend via Tauri events.
/// Phase 1: stub — does not actually emit yet.
pub fn emit_task_event(_app: &tauri::AppHandle, _event: TaskEvent) {
    // Will be implemented in Phase 7 (Agent Runtime)
}
