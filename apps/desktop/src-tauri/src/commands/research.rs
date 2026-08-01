// Research project Tauri commands — M4 Research Workspace.

use tauri::State;
use crate::app::state::AppState;
use crate::error::AppError;
use domain::research::{CreateDocumentInput, CreateProjectInput, DocumentType, ResearchDocument, ResearchProject};

// Project commands
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

// Document commands
#[tauri::command]
pub async fn create_research_document(project_id: String, document_type: String, title: String, content: Option<String>, source_url: Option<String>, file_path: Option<String>, state: State<'_, AppState>) -> Result<ResearchDocument, AppError> {
    let doc_type = match document_type.as_str() { "pdf" => DocumentType::Pdf, "web_page" => DocumentType::WebPage, "note" => DocumentType::Note, "report" => DocumentType::Report, _ => DocumentType::Note };
    state.research_document_service.create_document(CreateDocumentInput { project_id, document_type: doc_type, title, content, source_url, file_path }).await
}

#[tauri::command]
pub async fn get_research_document(id: String, state: State<'_, AppState>) -> Result<Option<ResearchDocument>, AppError> {
    state.research_document_service.get_document(&id).await
}

#[tauri::command]
pub async fn list_research_documents(project_id: String, state: State<'_, AppState>) -> Result<Vec<ResearchDocument>, AppError> {
    state.research_document_service.list_documents(&project_id).await
}

#[tauri::command]
pub async fn delete_research_document(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    state.research_document_service.delete_document(&id).await
}
