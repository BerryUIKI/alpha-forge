use crate::database::repositories::knowledge_graph_repository::KnowledgeGraphRepository;
use crate::database::repositories::thesis_repository::ThesisRepository;
use crate::error::AppError;
use domain::knowledge_graph::{
    CreateKnowledgeEntityInput, CreateKnowledgeRelationshipInput, KnowledgeEntity,
    KnowledgeRelationship, ThesisEntityLink,
};

pub struct KnowledgeGraphService {
    repo: KnowledgeGraphRepository,
    thesis_repo: ThesisRepository,
}

impl KnowledgeGraphService {
    pub fn new(repo: KnowledgeGraphRepository, thesis_repo: ThesisRepository) -> Self {
        Self { repo, thesis_repo }
    }

    pub async fn create_entity(
        &self,
        input: CreateKnowledgeEntityInput,
    ) -> Result<KnowledgeEntity, AppError> {
        if input.name.trim().is_empty() {
            return Err(AppError::Validation(
                "Knowledge entity name cannot be empty".to_string(),
            ));
        }
        if input.name.len() > 200 {
            return Err(AppError::Validation(
                "Knowledge entity name cannot exceed 200 characters".to_string(),
            ));
        }
        self.repo.create_entity(input).await
    }

    pub async fn list_entities(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<KnowledgeEntity>, AppError> {
        self.repo.list_entities(workspace_id).await
    }

    pub async fn create_relationship(
        &self,
        input: CreateKnowledgeRelationshipInput,
    ) -> Result<KnowledgeRelationship, AppError> {
        if input.source_entity_id == input.target_entity_id {
            return Err(AppError::Validation(
                "A knowledge entity cannot relate to itself".to_string(),
            ));
        }
        if input.relationship_type.trim().is_empty() {
            return Err(AppError::Validation(
                "Relationship type cannot be empty".to_string(),
            ));
        }
        let source = self
            .repo
            .get_entity(&input.source_entity_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Source knowledge entity not found".to_string()))?;
        let target = self
            .repo
            .get_entity(&input.target_entity_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Target knowledge entity not found".to_string()))?;
        if source.workspace_id != target.workspace_id {
            return Err(AppError::Validation(
                "Knowledge relationships must remain within one workspace".to_string(),
            ));
        }
        self.repo.create_relationship(input).await
    }

    pub async fn list_relationships(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<KnowledgeRelationship>, AppError> {
        self.repo.list_relationships(workspace_id).await
    }

    pub async fn link_thesis(
        &self,
        thesis_id: &str,
        entity_id: &str,
    ) -> Result<ThesisEntityLink, AppError> {
        let thesis = self
            .thesis_repo
            .get_thesis(thesis_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Thesis not found".to_string()))?;
        let entity = self
            .repo
            .get_entity(entity_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Knowledge entity not found".to_string()))?;
        if thesis.workspace_id != entity.workspace_id {
            return Err(AppError::Validation(
                "Theses can only link to entities in the same workspace".to_string(),
            ));
        }
        self.repo.link_thesis(thesis_id, entity_id).await
    }

    pub async fn list_thesis_links(
        &self,
        thesis_id: &str,
    ) -> Result<Vec<ThesisEntityLink>, AppError> {
        self.repo.list_thesis_links(thesis_id).await
    }
}
