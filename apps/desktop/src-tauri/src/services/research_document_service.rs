// Research document service — handles document business logic.

use crate::database::repositories::research_document_repository::ResearchDocumentRepository;
use crate::error::AppError;
use domain::research::{CreateDocumentInput, ResearchDocument};

pub struct ResearchDocumentService { repo: ResearchDocumentRepository }

impl ResearchDocumentService {
    pub fn new(repo: ResearchDocumentRepository) -> Self { Self { repo } }

    pub async fn create_document(&self, input: CreateDocumentInput) -> Result<ResearchDocument, AppError> {
        if input.title.trim().is_empty() { return Err(AppError::Validation("Document title cannot be empty".to_string())); }
        self.repo.create(input).await
    }

    pub async fn get_document(&self, id: &str) -> Result<Option<ResearchDocument>, AppError> { self.repo.get(id).await }
    pub async fn list_documents(&self, project_id: &str) -> Result<Vec<ResearchDocument>, AppError> { self.repo.list_by_project(project_id).await }
    pub async fn delete_document(&self, id: &str) -> Result<(), AppError> { self.repo.delete(id).await }
}