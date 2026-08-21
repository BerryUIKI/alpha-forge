use serde::{Deserialize, Serialize};
use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use domain::knowledge_graph::{
    CreateKnowledgeEntityInput, CreateKnowledgeRelationshipInput, KnowledgeEntity,
    KnowledgeEntityType, KnowledgeRelationship, ThesisEntityLink,
};

/// DTO for KnowledgeEntity with camelCase serialization for the IPC boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeEntityDto {
    pub id: String,
    pub workspace_id: String,
    pub entity_type: KnowledgeEntityType,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<KnowledgeEntity> for KnowledgeEntityDto {
    fn from(entity: KnowledgeEntity) -> Self {
        Self {
            id: entity.id,
            workspace_id: entity.workspace_id,
            entity_type: entity.entity_type,
            name: entity.name,
            description: entity.description,
            created_at: entity.created_at.to_rfc3339(),
            updated_at: entity.updated_at.to_rfc3339(),
        }
    }
}

/// DTO for KnowledgeRelationship with camelCase serialization for the IPC boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeRelationshipDto {
    pub id: String,
    pub source_entity_id: String,
    pub target_entity_id: String,
    pub relationship_type: String,
    pub created_at: String,
}

impl From<KnowledgeRelationship> for KnowledgeRelationshipDto {
    fn from(rel: KnowledgeRelationship) -> Self {
        Self {
            id: rel.id,
            source_entity_id: rel.source_entity_id,
            target_entity_id: rel.target_entity_id,
            relationship_type: rel.relationship_type,
            created_at: rel.created_at.to_rfc3339(),
        }
    }
}

/// DTO for ThesisEntityLink with camelCase serialization for the IPC boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThesisEntityLinkDto {
    pub thesis_id: String,
    pub entity_id: String,
    pub created_at: String,
}

impl From<ThesisEntityLink> for ThesisEntityLinkDto {
    fn from(link: ThesisEntityLink) -> Self {
        Self {
            thesis_id: link.thesis_id,
            entity_id: link.entity_id,
            created_at: link.created_at.to_rfc3339(),
        }
    }
}

#[tauri::command]
pub async fn create_knowledge_entity(
    workspace_id: String,
    entity_type: String,
    name: String,
    description: Option<String>,
    state: State<'_, AppState>,
) -> Result<KnowledgeEntityDto, AppError> {
    let entity_type = match entity_type.as_str() {
        "company" => KnowledgeEntityType::Company,
        "industry" => KnowledgeEntityType::Industry,
        "technology" => KnowledgeEntityType::Technology,
        "macro_theme" => KnowledgeEntityType::MacroTheme,
        _ => {
            return Err(AppError::Validation(
                "Unknown knowledge entity type".to_string(),
            ))
        }
    };
    state
        .knowledge_graph_service
        .create_entity(CreateKnowledgeEntityInput {
            workspace_id,
            entity_type,
            name,
            description,
        })
        .await
        .map(KnowledgeEntityDto::from)
}

#[tauri::command]
pub async fn list_knowledge_entities(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<KnowledgeEntityDto>, AppError> {
    state
        .knowledge_graph_service
        .list_entities(&workspace_id)
        .await
        .map(|entities| entities.into_iter().map(KnowledgeEntityDto::from).collect())
}

#[tauri::command]
pub async fn create_knowledge_relationship(
    source_entity_id: String,
    target_entity_id: String,
    relationship_type: String,
    state: State<'_, AppState>,
) -> Result<KnowledgeRelationshipDto, AppError> {
    state
        .knowledge_graph_service
        .create_relationship(CreateKnowledgeRelationshipInput {
            source_entity_id,
            target_entity_id,
            relationship_type,
        })
        .await
        .map(KnowledgeRelationshipDto::from)
}

#[tauri::command]
pub async fn list_knowledge_relationships(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<KnowledgeRelationshipDto>, AppError> {
    state
        .knowledge_graph_service
        .list_relationships(&workspace_id)
        .await
        .map(|rels| {
            rels.into_iter()
                .map(KnowledgeRelationshipDto::from)
                .collect()
        })
}

#[tauri::command]
pub async fn link_thesis_knowledge_entity(
    thesis_id: String,
    entity_id: String,
    state: State<'_, AppState>,
) -> Result<ThesisEntityLinkDto, AppError> {
    state
        .knowledge_graph_service
        .link_thesis(&thesis_id, &entity_id)
        .await
        .map(ThesisEntityLinkDto::from)
}

#[tauri::command]
pub async fn list_thesis_knowledge_links(
    thesis_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ThesisEntityLinkDto>, AppError> {
    state
        .knowledge_graph_service
        .list_thesis_links(&thesis_id)
        .await
        .map(|links| links.into_iter().map(ThesisEntityLinkDto::from).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_knowledge_graph_dtos_camel_case_serialization() {
        let now = Utc::now();
        let entity = KnowledgeEntity {
            id: "ent-1".to_string(),
            workspace_id: "ws-1".to_string(),
            entity_type: KnowledgeEntityType::Company,
            name: "NVIDIA".to_string(),
            description: Some("GPU maker".to_string()),
            created_at: now,
            updated_at: now,
        };
        let entity_dto = KnowledgeEntityDto::from(entity);
        let entity_json = serde_json::to_string(&entity_dto).expect("entity serialization");
        assert!(entity_json.contains("\"workspaceId\":\"ws-1\""));
        assert!(entity_json.contains("\"entityType\":\"company\""));
        assert!(entity_json.contains("\"createdAt\":"));
        assert!(!entity_json.contains("\"workspace_id\":"));
        assert!(!entity_json.contains("\"entity_type\":"));

        let rel = KnowledgeRelationship {
            id: "rel-1".to_string(),
            source_entity_id: "ent-1".to_string(),
            target_entity_id: "ent-2".to_string(),
            relationship_type: "supplies".to_string(),
            created_at: now,
        };
        let rel_dto = KnowledgeRelationshipDto::from(rel);
        let rel_json = serde_json::to_string(&rel_dto).expect("rel serialization");
        assert!(rel_json.contains("\"sourceEntityId\":\"ent-1\""));
        assert!(rel_json.contains("\"targetEntityId\":\"ent-2\""));
        assert!(rel_json.contains("\"relationshipType\":\"supplies\""));
        assert!(!rel_json.contains("\"source_entity_id\":"));
        assert!(!rel_json.contains("\"target_entity_id\":"));
        assert!(!rel_json.contains("\"relationship_type\":"));

        let link = ThesisEntityLink {
            thesis_id: "th-1".to_string(),
            entity_id: "ent-1".to_string(),
            created_at: now,
        };
        let link_dto = ThesisEntityLinkDto::from(link);
        let link_json = serde_json::to_string(&link_dto).expect("link serialization");
        assert!(link_json.contains("\"thesisId\":\"th-1\""));
        assert!(link_json.contains("\"entityId\":\"ent-1\""));
        assert!(!link_json.contains("\"thesis_id\":"));
        assert!(!link_json.contains("\"entity_id\":"));
    }
}
