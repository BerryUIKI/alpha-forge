// Research Tauri commands.

use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use domain::research::{
    CreateResearchDocumentInput, ResearchDocument, UpdateResearchDocumentInput,
};

#[tauri::command]
pub async fn list_research_documents(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ResearchDocument>, AppError> {
    state.research_service.list(&workspace_id).await
}

#[tauri::command]
pub async fn create_research_document(
    workspace_id: String,
    title: String,
    content: Option<String>,
    state: State<'_, AppState>,
) -> Result<ResearchDocument, AppError> {
    state
        .research_service
        .create(CreateResearchDocumentInput {
            workspace_id,
            title,
            content,
            kind: None,
            format: None,
            summary: None,
        })
        .await
}

#[tauri::command]
pub async fn get_research_document(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<ResearchDocument>, AppError> {
    state.research_service.get(&id).await
}

#[tauri::command]
pub async fn update_research_document(
    id: String,
    title: String,
    content: Option<String>,
    state: State<'_, AppState>,
) -> Result<ResearchDocument, AppError> {
    state
        .research_service
        .update(&id, UpdateResearchDocumentInput { title, content })
        .await
}

#[tauri::command]
pub async fn delete_research_document(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.research_service.delete(&id).await
}

#[tauri::command]
pub async fn purge_research_document(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.research_service.purge(&id).await
}
