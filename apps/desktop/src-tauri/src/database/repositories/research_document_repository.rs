// Research document repository.

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::error::AppError;
use domain::research::ResearchDocument;

/// Repository for research documents.
#[derive(Clone)]
pub struct ResearchDocumentRepository {
    pool: SqlitePool,
}

impl ResearchDocumentRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Creates a research document.
    pub async fn create(
        &self,
        id: &str,
        workspace_id: &str,
        title: &str,
        content: Option<&str>,
    ) -> Result<ResearchDocument, AppError> {
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO research_documents (id, workspace_id, title, content, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id)
        .bind(workspace_id)
        .bind(title)
        .bind(content)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create research document: {}", e)))?;

        Ok(ResearchDocument {
            id: id.to_string(),
            workspace_id: workspace_id.to_string(),
            title: title.to_string(),
            content: content.map(str::to_string),
            created_at: now,
            updated_at: now,
        })
    }

    /// Lists documents in a workspace, newest first.
    pub async fn list_by_workspace(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<ResearchDocument>, AppError> {
        let rows = sqlx::query_as::<_, ResearchDocumentRow>(
            r#"
            SELECT id, workspace_id, title, content, created_at, updated_at
            FROM research_documents
            WHERE workspace_id = ?
            ORDER BY updated_at DESC, rowid DESC
            "#,
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list research documents: {}", e)))?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }

    /// Gets a document by ID.
    pub async fn get(&self, id: &str) -> Result<Option<ResearchDocument>, AppError> {
        let row = sqlx::query_as::<_, ResearchDocumentRow>(
            r#"
            SELECT id, workspace_id, title, content, created_at, updated_at
            FROM research_documents
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to get research document: {}", e)))?;

        Ok(row.map(|row| row.into()))
    }

    /// Updates a document's title and content.
    pub async fn update(
        &self,
        id: &str,
        title: &str,
        content: Option<&str>,
    ) -> Result<ResearchDocument, AppError> {
        let rows_affected = sqlx::query(
            r#"
            UPDATE research_documents
            SET title = ?, content = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(title)
        .bind(content)
        .bind(Utc::now())
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update research document: {}", e)))?
        .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!(
                "Research document '{}' not found",
                id
            )));
        }

        self.get(id)
            .await?
            .ok_or_else(|| AppError::Internal("Document updated but not found".to_string()))
    }

    /// Deletes a document.
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        let rows_affected = sqlx::query("DELETE FROM research_documents WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to delete research document: {}", e)))?
            .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!(
                "Research document '{}' not found",
                id
            )));
        }

        Ok(())
    }

    /// Searches document titles and content in a workspace.
    pub async fn search(
        &self,
        workspace_id: &str,
        query: &str,
        limit: i64,
    ) -> Result<Vec<ResearchDocument>, AppError> {
        let pattern = format!("%{}%", query);

        let rows = sqlx::query_as::<_, ResearchDocumentRow>(
            r#"
            SELECT id, workspace_id, title, content, created_at, updated_at
            FROM research_documents
            WHERE workspace_id = ? AND (title LIKE ? OR content LIKE ?)
            ORDER BY updated_at DESC, rowid DESC
            LIMIT ?
            "#,
        )
        .bind(workspace_id)
        .bind(&pattern)
        .bind(&pattern)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to search research documents: {}", e)))?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }
}

/// Database row representation of a research document.
#[derive(Debug, sqlx::FromRow)]
struct ResearchDocumentRow {
    id: String,
    workspace_id: String,
    title: String,
    content: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<ResearchDocumentRow> for ResearchDocument {
    fn from(row: ResearchDocumentRow) -> Self {
        ResearchDocument {
            id: row.id,
            workspace_id: row.workspace_id,
            title: row.title,
            content: row.content,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
