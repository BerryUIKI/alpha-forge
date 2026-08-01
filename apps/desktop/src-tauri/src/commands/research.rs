// Research project Tauri commands — M4 Research Workspace.

use tauri::State;
use crate::app::state::AppState;
use crate::error::AppError;
use domain::research::{CreateProjectInput, ResearchProject};

#[tauri::command]
pub async fn create_research_project(workspace_id: String, title: String, description: Option<String>, state: State<'_, AppState>) -> Result<ResearchProject, AppError> {
    state.research_project_service.create_project(CreateProjectInput { workspace_id, title, description }).await
}

#[tauri::command]
pub async fn get_research_project(id: String, state: State<'_, AppState>) -> Result<Option<ResearchProject>, AppError> {
    state.research_project_service.get_project(&id).await
}

#[tauri::command]
pub async fn list_research_projects(workspace_id: String, state: State<'_, AppState>) -> Result<Vec<ResearchProject>, AppError> {
    state.research_project_service.list_projects(&workspace_id).await
}

#[tauri::command]
pub async fn archive_research_project(id: String, state: State<'_, AppState>) -> Result<ResearchProject, AppError> {
    state.research_project_service.archive_project(&id).await
}

#[tauri::command]
pub async fn complete_research_project(id: String, state: State<'_, AppState>) -> Result<ResearchProject, AppError> {
    state.research_project_service.complete_project(&id).await
}

#[tauri::command]
pub async fn delete_research_project(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    state.research_project_service.delete_project(&id).await
}

// Legacy placeholder
#[tauri::command]
pub async fn list_research_documents() -> Result<Vec<String>, AppError> {
    Ok(Vec::new())
}
