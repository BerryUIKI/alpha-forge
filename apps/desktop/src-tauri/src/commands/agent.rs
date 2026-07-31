// Agent Tauri commands — Phase 2.

use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use domain::task::{AgentTask, AgentTaskEvent, CreateAgentTaskInput};

#[tauri::command]
pub async fn create_agent_task(
    workspace_id: String,
    title: String,
    description: Option<String>,
    state: State<'_, AppState>,
) -> Result<AgentTask, AppError> {
    let input = CreateAgentTaskInput {
        workspace_id,
        title,
        description,
    };

    state.agent_service.create_task(input).await
}

#[tauri::command]
pub async fn get_agent_task(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<AgentTask>, AppError> {
    state.agent_service.get_task(&id).await
}

#[tauri::command]
pub async fn list_agent_tasks(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<AgentTask>, AppError> {
    state.agent_service.list_tasks(&workspace_id).await
}

#[tauri::command]
pub async fn get_task_events(
    task_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<AgentTaskEvent>, AppError> {
    state.agent_service.get_task_events(&task_id).await
}

#[tauri::command]
pub async fn queue_agent_task(
    task_id: String,
    state: State<'_, AppState>,
) -> Result<AgentTask, AppError> {
    state.agent_service.queue_task(&task_id).await
}

#[tauri::command]
pub async fn start_agent_task(
    task_id: String,
    state: State<'_, AppState>,
) -> Result<AgentTask, AppError> {
    state.agent_service.start_task(&task_id).await
}

#[tauri::command]
pub async fn cancel_agent_task(
    task_id: String,
    state: State<'_, AppState>,
) -> Result<AgentTask, AppError> {
    state.agent_service.cancel_task(&task_id).await
}