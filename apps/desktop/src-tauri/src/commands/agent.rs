// Agent Tauri commands — Phase 1 placeholder.
// Will be connected to the agent runtime in later phases.

use crate::agent::task::{AgentTask, TaskStatus};
use crate::error::AppError;

#[tauri::command]
pub async fn create_task(input: String) -> Result<AgentTask, AppError> {
    // Phase 1: return a stub task that is always "completed"
    let task_id = uuid::Uuid::new_v4().to_string();

    Ok(AgentTask {
        id: task_id,
        status: TaskStatus::Completed,
        input,
    })
}

#[tauri::command]
pub async fn list_tasks() -> Result<Vec<AgentTask>, AppError> {
    // Phase 1: return empty list
    Ok(Vec::new())
}
