use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use domain::knowledge_graph::{CreateKnowledgeEntityInput, CreateKnowledgeRelationshipInput, KnowledgeEntity, KnowledgeEntityType, KnowledgeRelationship, ThesisEntityLink};

#[tauri::command]
pub async fn create_knowledge_entity(workspace_id: String, entity_type: String, name: String, description: Option<String>, state: State<'_, AppState>) -> Result<KnowledgeEntity, AppError> {
    let entity_type = match entity_type.as_str() { "company" => KnowledgeEntityType::Company, "industry" => KnowledgeEntityType::Industry, "technology" => KnowledgeEntityType::Technology, "macro_theme" => KnowledgeEntityType::MacroTheme, _ => return Err(AppError::Validation("Unknown knowledge entity type".to_string())) };
    state.knowledge_graph_service.create_entity(CreateKnowledgeEntityInput { workspace_id, entity_type, name, description }).await
}

#[tauri::command]
pub async fn list_knowledge_entities(workspace_id: String, state: State<'_, AppState>) -> Result<Vec<KnowledgeEntity>, AppError> { state.knowledge_graph_service.list_entities(&workspace_id).await }

#[tauri::command]
pub async fn create_knowledge_relationship(source_entity_id: String, target_entity_id: String, relationship_type: String, state: State<'_, AppState>) -> Result<KnowledgeRelationship, AppError> { state.knowledge_graph_service.create_relationship(CreateKnowledgeRelationshipInput { source_entity_id, target_entity_id, relationship_type }).await }

#[tauri::command]
pub async fn list_knowledge_relationships(workspace_id: String, state: State<'_, AppState>) -> Result<Vec<KnowledgeRelationship>, AppError> { state.knowledge_graph_service.list_relationships(&workspace_id).await }

#[tauri::command]
pub async fn link_thesis_knowledge_entity(thesis_id: String, entity_id: String, state: State<'_, AppState>) -> Result<ThesisEntityLink, AppError> { state.knowledge_graph_service.link_thesis(&thesis_id, &entity_id).await }

#[tauri::command]
pub async fn list_thesis_knowledge_links(thesis_id: String, state: State<'_, AppState>) -> Result<Vec<ThesisEntityLink>, AppError> { state.knowledge_graph_service.list_thesis_links(&thesis_id).await }
