// Research service — validation and business rules for research documents.

use uuid::Uuid;

use crate::database::repositories::research_document_repository::{
    NewResearchDocument, ResearchDocumentRepository,
};
use crate::error::AppError;
use domain::research::{
    CreateResearchDocumentInput, ResearchDocument, ResearchDocumentFormat, ResearchDocumentKind,
    UpdateResearchDocumentInput,
};

const MAX_WORKSPACE_ID_CHARS: usize = 100;
const MAX_TITLE_CHARS: usize = 200;
const MAX_CONTENT_CHARS: usize = 1_000_000;
const MAX_SUMMARY_CHARS: usize = 1_000;
const MAX_SEARCH_RESULTS: i64 = 50;

/// Service for managing research documents.
pub struct ResearchService {
    repo: ResearchDocumentRepository,
}

impl ResearchService {
    pub fn new(repo: ResearchDocumentRepository) -> Self {
        Self { repo }
    }

    /// Creates a document in a workspace.
    pub async fn create(
        &self,
        input: CreateResearchDocumentInput,
    ) -> Result<ResearchDocument, AppError> {
        let workspace_id =
            required_text(&input.workspace_id, "workspace id", MAX_WORKSPACE_ID_CHARS)?;
        let title = required_text(&input.title, "document title", MAX_TITLE_CHARS)?;
        let content = normalize_content(input.content)?;
        let summary = normalize_summary(input.summary)?;
        let kind = input.kind.unwrap_or(ResearchDocumentKind::Note);
        let format = input.format.unwrap_or(ResearchDocumentFormat::Markdown);
        let id = Uuid::new_v4().to_string();

        self.repo
            .create(NewResearchDocument {
                id: &id,
                workspace_id: &workspace_id,
                title: &title,
                content: content.as_deref(),
                kind,
                format,
                summary: summary.as_deref(),
            })
            .await
    }

    /// Lists documents in a workspace.
    pub async fn list(&self, workspace_id: &str) -> Result<Vec<ResearchDocument>, AppError> {
        let workspace_id = required_text(workspace_id, "workspace id", MAX_WORKSPACE_ID_CHARS)?;

        self.repo.list_by_workspace(&workspace_id).await
    }

    /// Gets a document by ID.
    pub async fn get(&self, id: &str) -> Result<Option<ResearchDocument>, AppError> {
        let id = required_text(id, "document id", MAX_WORKSPACE_ID_CHARS)?;
        self.repo.get(&id).await
    }

    /// Updates a document's title and content.
    pub async fn update(
        &self,
        id: &str,
        input: UpdateResearchDocumentInput,
    ) -> Result<ResearchDocument, AppError> {
        let id = required_text(id, "document id", MAX_WORKSPACE_ID_CHARS)?;
        let title = required_text(&input.title, "document title", MAX_TITLE_CHARS)?;
        let content = normalize_content(input.content)?;

        self.repo.update(&id, &title, content.as_deref()).await
    }

    /// Archives a document (soft delete).
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        let id = required_text(id, "document id", MAX_WORKSPACE_ID_CHARS)?;
        self.repo.archive(&id).await
    }

    /// Permanently deletes an archived document and its sources/notes.
    pub async fn purge(&self, id: &str) -> Result<(), AppError> {
        let id = required_text(id, "document id", MAX_WORKSPACE_ID_CHARS)?;
        let document =
            self.repo.get(&id).await?.ok_or_else(|| {
                AppError::NotFound(format!("Research document '{}' not found", id))
            })?;

        if document.archived_at.is_none() {
            return Err(AppError::Validation(
                "Archive the document before purging it".to_string(),
            ));
        }

        self.repo.purge(&id).await
    }

    /// Searches documents in a workspace.
    pub async fn search(
        &self,
        workspace_id: &str,
        query: &str,
        limit: i64,
    ) -> Result<Vec<ResearchDocument>, AppError> {
        let workspace_id = required_text(workspace_id, "workspace id", MAX_WORKSPACE_ID_CHARS)?;
        let query = required_text(query, "search query", MAX_TITLE_CHARS)?;
        let limit = limit.clamp(1, MAX_SEARCH_RESULTS);

        self.repo.search(&workspace_id, &query, limit).await
    }
}

fn required_text(value: &str, field: &str, max_chars: usize) -> Result<String, AppError> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err(AppError::Validation(format!("{} cannot be empty", field)));
    }
    if trimmed.chars().count() > max_chars {
        return Err(AppError::Validation(format!(
            "{} cannot exceed {} characters",
            field, max_chars
        )));
    }

    Ok(trimmed.to_string())
}

fn normalize_content(content: Option<String>) -> Result<Option<String>, AppError> {
    let content = content
        .as_deref()
        .map(str::trim)
        .filter(|content| !content.is_empty())
        .map(str::to_string);

    if let Some(content) = &content {
        if content.chars().count() > MAX_CONTENT_CHARS {
            return Err(AppError::Validation(
                "document content is too large".to_string(),
            ));
        }
    }

    Ok(content)
}

fn normalize_summary(summary: Option<String>) -> Result<Option<String>, AppError> {
    let summary = summary
        .as_deref()
        .map(str::trim)
        .filter(|summary| !summary.is_empty())
        .map(str::to_string);

    if let Some(summary) = &summary {
        if summary.chars().count() > MAX_SUMMARY_CHARS {
            return Err(AppError::Validation(
                "document summary is too long".to_string(),
            ));
        }
    }

    Ok(summary)
}
