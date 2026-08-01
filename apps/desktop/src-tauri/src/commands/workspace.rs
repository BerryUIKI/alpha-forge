// Workspace Tauri commands — Phase 1.5.

use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use domain::workspace::{CreateWorkspaceInput, UpdateWorkspaceInput, Workspace};

#[tauri::command]
pub async fn create_workspace(
    name: String,
    state: State<'_, AppState>,
) -> Result<Workspace, AppError> {
    let input = CreateWorkspaceInput { name };
    state.workspace_service.create(input).await
}

#[tauri::command]
pub async fn list_workspaces(state: State<'_, AppState>) -> Result<Vec<Workspace>, AppError> {
    state.workspace_service.list().await
}

#[tauri::command]
pub async fn get_workspace(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<Workspace>, AppError> {
    state.workspace_service.get(&id).await
}

#[tauri::command]
pub async fn update_workspace(
    id: String,
    name: String,
    state: State<'_, AppState>,
) -> Result<Workspace, AppError> {
    let input = UpdateWorkspaceInput { name: Some(name) };
    state.workspace_service.update(&id, input).await
}

#[tauri::command]
pub async fn delete_workspace(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    state.workspace_service.delete(&id).await
}
