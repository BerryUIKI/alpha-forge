// Read-only research document tools.

use serde::Deserialize;
use serde_json::{json, Value};

use crate::agent::tools::AgentTool;
use crate::database::repositories::research_document_repository::ResearchDocumentRepository;
use crate::error::AppError;

const DEFAULT_SEARCH_LIMIT: i64 = 5;
const MAX_SEARCH_LIMIT: i64 = 10;
const MAX_EXCERPT_CHARS: usize = 1_500;
const MAX_DOCUMENT_CHARS: usize = 8_000;

/// Searches research documents in a workspace.
#[derive(Clone)]
pub struct SearchResearchTool {
    repo: ResearchDocumentRepository,
}

impl SearchResearchTool {
    pub fn new(repo: ResearchDocumentRepository) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Deserialize)]
struct SearchResearchArgs {
    workspace_id: String,
    query: String,
    limit: Option<i64>,
}

#[async_trait::async_trait]
impl AgentTool for SearchResearchTool {
    fn name(&self) -> &'static str {
        "search_research"
    }

    fn description(&self) -> &'static str {
        "Search research documents in a workspace by title or content. Returns ids, titles, and excerpts."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "workspace_id": {
                    "type": "string",
                    "description": "Workspace ID"
                },
                "query": {
                    "type": "string",
                    "description": "Text to search for in document titles and content"
                },
                "limit": {
                    "type": "integer",
                    "minimum": 1,
                    "maximum": MAX_SEARCH_LIMIT,
                    "description": "Maximum number of documents to return"
                }
            },
            "required": ["workspace_id", "query"],
            "additionalProperties": false
        })
    }

    async fn execute(&self, arguments: Value) -> Result<Value, AppError> {
        let args: SearchResearchArgs = serde_json::from_value(arguments).map_err(|error| {
            AppError::Validation(format!("invalid search_research arguments: {error}"))
        })?;

        let workspace_id = require_text(&args.workspace_id, "workspace id")?;
        let query = require_text(&args.query, "search query")?;
        let limit = args
            .limit
            .unwrap_or(DEFAULT_SEARCH_LIMIT)
            .clamp(1, MAX_SEARCH_LIMIT);

        let documents = self.repo.search(&workspace_id, &query, limit).await?;
        let items = documents
            .iter()
            .map(|document| {
                json!({
                    "id": document.id,
                    "title": document.title,
                    "kind": document.kind.as_str(),
                    "excerpt": document.content.as_deref().map(|content| truncate(content, MAX_EXCERPT_CHARS)),
                })
            })
            .collect::<Vec<_>>();

        Ok(json!({ "documents": items }))
    }
}

/// Reads a research document by ID.
#[derive(Clone)]
pub struct GetResearchDocumentTool {
    repo: ResearchDocumentRepository,
}

impl GetResearchDocumentTool {
    pub fn new(repo: ResearchDocumentRepository) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Deserialize)]
struct GetResearchDocumentArgs {
    document_id: String,
}

#[async_trait::async_trait]
impl AgentTool for GetResearchDocumentTool {
    fn name(&self) -> &'static str {
        "get_research_document"
    }

    fn description(&self) -> &'static str {
        "Read a research document's title and content by document ID."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "document_id": {
                    "type": "string",
                    "description": "Document ID"
                }
            },
            "required": ["document_id"],
            "additionalProperties": false
        })
    }

    async fn execute(&self, arguments: Value) -> Result<Value, AppError> {
        let args: GetResearchDocumentArgs = serde_json::from_value(arguments).map_err(|error| {
            AppError::Validation(format!("invalid get_research_document arguments: {error}"))
        })?;

        let document_id = require_text(&args.document_id, "document id")?;
        let document = self.repo.get(&document_id).await?.ok_or_else(|| {
            AppError::NotFound(format!("Research document '{}' not found", document_id))
        })?;

        Ok(json!({
            "id": document.id,
            "title": document.title,
            "kind": document.kind.as_str(),
            "format": document.format.as_str(),
            "summary": document.summary,
            "content": document.content.as_deref().map(|content| truncate(content, MAX_DOCUMENT_CHARS)),
            "updated_at": document.updated_at.to_rfc3339(),
        }))
    }
}

fn require_text(value: &str, field: &str) -> Result<String, AppError> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err(AppError::Validation(format!("{} cannot be empty", field)));
    }

    Ok(trimmed.to_string())
}

fn truncate(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }

    let truncated: String = value.chars().take(max_chars).collect();
    format!("{truncated}… (truncated)")
}

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use super::*;
    use crate::database::repositories::research_document_repository::NewResearchDocument;
    use domain::research::{ResearchDocumentFormat, ResearchDocumentKind};

    async fn setup_repo() -> ResearchDocumentRepository {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("Failed to create test database");

        crate::database::migrations::run(&pool)
            .await
            .expect("Failed to run migrations");

        sqlx::query("INSERT INTO workspaces (id, name) VALUES ('w1', 'Workspace')")
            .execute(&pool)
            .await
            .expect("Failed to insert workspace");

        let repo = ResearchDocumentRepository::new(pool);
        repo.create(NewResearchDocument {
            id: "d1",
            workspace_id: "w1",
            title: "Tesla notes",
            content: Some("Q4 earnings and demand"),
            kind: ResearchDocumentKind::Note,
            format: ResearchDocumentFormat::Markdown,
            summary: Some("Tesla research"),
        })
        .await
        .expect("document should be created");
        repo.create(NewResearchDocument {
            id: "d2",
            workspace_id: "w1",
            title: "Nvidia notes",
            content: Some("Data center"),
            kind: ResearchDocumentKind::Note,
            format: ResearchDocumentFormat::Markdown,
            summary: None,
        })
        .await
        .expect("document should be created");

        repo
    }

    #[tokio::test]
    async fn test_search_research_returns_excerpts() {
        let tool = SearchResearchTool::new(setup_repo().await);

        let result = tool
            .execute(json!({"workspace_id": "w1", "query": "Tesla"}))
            .await
            .expect("tool should execute");

        let documents = result["documents"].as_array().expect("documents array");
        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0]["id"], "d1");
        assert_eq!(documents[0]["excerpt"], "Q4 earnings and demand");
    }

    #[tokio::test]
    async fn test_search_research_rejects_empty_query() {
        let tool = SearchResearchTool::new(setup_repo().await);

        let result = tool
            .execute(json!({"workspace_id": "w1", "query": "   "}))
            .await;

        assert!(matches!(result, Err(AppError::Validation(_))));
    }

    #[tokio::test]
    async fn test_get_research_document_returns_content() {
        let tool = GetResearchDocumentTool::new(setup_repo().await);

        let result = tool
            .execute(json!({"document_id": "d1"}))
            .await
            .expect("tool should execute");

        assert_eq!(result["title"], "Tesla notes");
        assert_eq!(result["kind"], "note");
        assert_eq!(result["summary"], "Tesla research");
        assert_eq!(result["content"], "Q4 earnings and demand");
    }

    #[tokio::test]
    async fn test_get_research_document_rejects_missing_document() {
        let tool = GetResearchDocumentTool::new(setup_repo().await);

        let result = tool.execute(json!({"document_id": "missing"})).await;

        assert!(matches!(result, Err(AppError::NotFound(_))));
    }
}
