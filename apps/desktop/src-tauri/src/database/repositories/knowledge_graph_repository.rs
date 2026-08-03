use chrono::Utc;
use sqlx::SqlitePool;

use crate::error::AppError;
use domain::knowledge_graph::{
    CreateKnowledgeEntityInput, CreateKnowledgeRelationshipInput, KnowledgeEntity,
    KnowledgeEntityType, KnowledgeRelationship, ThesisEntityLink,
};

pub struct KnowledgeGraphRepository {
    pool: SqlitePool,
}

impl KnowledgeGraphRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_entity(
        &self,
        input: CreateKnowledgeEntityInput,
    ) -> Result<KnowledgeEntity, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        sqlx::query("INSERT INTO knowledge_entities (id, workspace_id, entity_type, name, description, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(&id).bind(&input.workspace_id).bind(input.entity_type.to_string()).bind(&input.name).bind(&input.description).bind(now.to_rfc3339()).bind(now.to_rfc3339()).execute(&self.pool).await
            .map_err(|e| AppError::Internal(format!("Failed to create knowledge entity: {e}")))?;
        Ok(KnowledgeEntity {
            id,
            workspace_id: input.workspace_id,
            entity_type: input.entity_type,
            name: input.name,
            description: input.description,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn list_entities(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<KnowledgeEntity>, AppError> {
        let rows = sqlx::query_as::<_, EntityRow>("SELECT id, workspace_id, entity_type, name, description, created_at, updated_at FROM knowledge_entities WHERE workspace_id = ? ORDER BY name")
            .bind(workspace_id).fetch_all(&self.pool).await.map_err(|e| AppError::Internal(format!("Failed to list knowledge entities: {e}")))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn get_entity(&self, id: &str) -> Result<Option<KnowledgeEntity>, AppError> {
        let row = sqlx::query_as::<_, EntityRow>("SELECT id, workspace_id, entity_type, name, description, created_at, updated_at FROM knowledge_entities WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await.map_err(|e| AppError::Internal(format!("Failed to get knowledge entity: {e}")))?;
        Ok(row.map(Into::into))
    }

    pub async fn create_relationship(
        &self,
        input: CreateKnowledgeRelationshipInput,
    ) -> Result<KnowledgeRelationship, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        sqlx::query("INSERT INTO knowledge_relationships (id, source_entity_id, target_entity_id, relationship_type, created_at) VALUES (?, ?, ?, ?, ?)")
            .bind(&id).bind(&input.source_entity_id).bind(&input.target_entity_id).bind(&input.relationship_type).bind(now.to_rfc3339()).execute(&self.pool).await
            .map_err(|e| AppError::Internal(format!("Failed to create knowledge relationship: {e}")))?;
        Ok(KnowledgeRelationship {
            id,
            source_entity_id: input.source_entity_id,
            target_entity_id: input.target_entity_id,
            relationship_type: input.relationship_type,
            created_at: now,
        })
    }

    pub async fn list_relationships(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<KnowledgeRelationship>, AppError> {
        let rows = sqlx::query_as::<_, RelationshipRow>("SELECT r.id, r.source_entity_id, r.target_entity_id, r.relationship_type, r.created_at FROM knowledge_relationships r JOIN knowledge_entities e ON e.id = r.source_entity_id WHERE e.workspace_id = ? ORDER BY r.created_at DESC")
            .bind(workspace_id).fetch_all(&self.pool).await.map_err(|e| AppError::Internal(format!("Failed to list knowledge relationships: {e}")))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn link_thesis(
        &self,
        thesis_id: &str,
        entity_id: &str,
    ) -> Result<ThesisEntityLink, AppError> {
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO thesis_entity_links (thesis_id, entity_id, created_at) VALUES (?, ?, ?)",
        )
        .bind(thesis_id)
        .bind(entity_id)
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to link thesis entity: {e}")))?;
        Ok(ThesisEntityLink {
            thesis_id: thesis_id.to_string(),
            entity_id: entity_id.to_string(),
            created_at: now,
        })
    }

    pub async fn list_thesis_links(
        &self,
        thesis_id: &str,
    ) -> Result<Vec<ThesisEntityLink>, AppError> {
        let rows = sqlx::query_as::<_, ThesisLinkRow>("SELECT thesis_id, entity_id, created_at FROM thesis_entity_links WHERE thesis_id = ? ORDER BY created_at DESC").bind(thesis_id).fetch_all(&self.pool).await
            .map_err(|e| AppError::Internal(format!("Failed to list thesis entity links: {e}")))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
}

#[derive(sqlx::FromRow)]
struct EntityRow {
    id: String,
    workspace_id: String,
    entity_type: String,
    name: String,
    description: Option<String>,
    created_at: String,
    updated_at: String,
}
impl From<EntityRow> for KnowledgeEntity {
    fn from(row: EntityRow) -> Self {
        let entity_type = match row.entity_type.as_str() {
            "company" => KnowledgeEntityType::Company,
            "industry" => KnowledgeEntityType::Industry,
            "technology" => KnowledgeEntityType::Technology,
            "macro_theme" => KnowledgeEntityType::MacroTheme,
            _ => KnowledgeEntityType::Company,
        };
        Self {
            id: row.id,
            workspace_id: row.workspace_id,
            entity_type,
            name: row.name,
            description: row.description,
            created_at: row.created_at.parse().unwrap_or_else(|_| Utc::now()),
            updated_at: row.updated_at.parse().unwrap_or_else(|_| Utc::now()),
        }
    }
}
#[derive(sqlx::FromRow)]
struct RelationshipRow {
    id: String,
    source_entity_id: String,
    target_entity_id: String,
    relationship_type: String,
    created_at: String,
}
impl From<RelationshipRow> for KnowledgeRelationship {
    fn from(row: RelationshipRow) -> Self {
        Self {
            id: row.id,
            source_entity_id: row.source_entity_id,
            target_entity_id: row.target_entity_id,
            relationship_type: row.relationship_type,
            created_at: row.created_at.parse().unwrap_or_else(|_| Utc::now()),
        }
    }
}
#[derive(sqlx::FromRow)]
struct ThesisLinkRow {
    thesis_id: String,
    entity_id: String,
    created_at: String,
}
impl From<ThesisLinkRow> for ThesisEntityLink {
    fn from(row: ThesisLinkRow) -> Self {
        Self {
            thesis_id: row.thesis_id,
            entity_id: row.entity_id,
            created_at: row.created_at.parse().unwrap_or_else(|_| Utc::now()),
        }
    }
}
