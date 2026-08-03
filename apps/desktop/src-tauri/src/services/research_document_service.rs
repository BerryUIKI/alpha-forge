// Research document service — handles document business logic.

use crate::database::repositories::research_document_repository::ResearchDocumentRepository;
use crate::documents::chunker::chunk_text;
use crate::documents::indexer::{rank_chunks, semantic_rank_chunks};
use crate::documents::parser::{extract_text, ContentFormat};
use crate::error::AppError;
use crate::security::url_policy::normalize_optional_research_url;
use domain::research::{CreateDocumentInput, DocumentType, ResearchDocument, ResearchSearchMatch};

pub struct ResearchDocumentService {
    repo: ResearchDocumentRepository,
}

impl ResearchDocumentService {
    pub fn new(repo: ResearchDocumentRepository) -> Self {
        Self { repo }
    }

    pub async fn create_document(
        &self,
        mut input: CreateDocumentInput,
    ) -> Result<ResearchDocument, AppError> {
        if input.title.trim().is_empty() {
            return Err(AppError::Validation(
                "Document title cannot be empty".to_string(),
            ));
        }
        input.source_url = normalize_optional_research_url(input.source_url)?;
        self.repo.create(input).await
    }

    pub async fn get_document(&self, id: &str) -> Result<Option<ResearchDocument>, AppError> {
        self.repo.get(id).await
    }
    pub async fn list_documents(
        &self,
        project_id: &str,
    ) -> Result<Vec<ResearchDocument>, AppError> {
        self.repo.list_by_project(project_id).await
    }
    pub async fn delete_document(&self, id: &str) -> Result<(), AppError> {
        self.repo.delete(id).await
    }

    pub async fn search_document(
        &self,
        id: &str,
        query: &str,
    ) -> Result<Vec<ResearchSearchMatch>, AppError> {
        self.search_document_with(id, query, false).await
    }

    pub async fn semantic_search_document(
        &self,
        id: &str,
        query: &str,
    ) -> Result<Vec<ResearchSearchMatch>, AppError> {
        self.search_document_with(id, query, true).await
    }

    async fn search_document_with(
        &self,
        id: &str,
        query: &str,
        semantic: bool,
    ) -> Result<Vec<ResearchSearchMatch>, AppError> {
        let normalized_query = query.trim();
        if normalized_query.is_empty() {
            return Err(AppError::Validation(
                "Search query cannot be empty".to_string(),
            ));
        }
        if normalized_query.len() > 200 {
            return Err(AppError::Validation("Search query is too long".to_string()));
        }
        let document = self
            .repo
            .get(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Research document not found".to_string()))?;
        let content = document.content.as_deref().unwrap_or_default();
        let format = match document.document_type {
            DocumentType::WebPage => ContentFormat::Html,
            _ => ContentFormat::PlainText,
        };
        let text = extract_text(content, format)?;
        let chunks = chunk_text(&text, 800);
        let matches = if semantic {
            semantic_rank_chunks(&chunks, normalized_query, 20)
        } else {
            rank_chunks(&chunks, normalized_query, 20)
        };
        Ok(matches
            .into_iter()
            .map(|item| ResearchSearchMatch {
                ordinal: item.ordinal,
                content: item.content,
                score: item.score,
            })
            .collect())
    }
}
