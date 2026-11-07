// Research document repository.

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::error::AppError;
use domain::research::{ResearchDocument, ResearchDocumentFormat, ResearchDocumentKind};

/// Repository for research documents.
#[derive(Clone)]
pub struct ResearchDocumentRepository {
    pool: SqlitePool,
}

/// Values required to create a research document.
pub struct NewResearchDocument<'a> {
    pub id: &'a str,
    pub workspace_id: &'a str,
    pub title: &'a str,
    pub content: Option<&'a str>,
    pub kind: ResearchDocumentKind,
    pub format: ResearchDocumentFormat,
    pub summary: Option<&'a str>,
}

impl ResearchDocumentRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Exposes the pool to sibling test modules.
    #[cfg(test)]
    pub(crate) fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// Creates a research document.
    pub async fn create(
        &self,
        document: NewResearchDocument<'_>,
    ) -> Result<ResearchDocument, AppError> {
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO research_documents
                (id, workspace_id, title, content, kind, format, summary, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(document.id)
        .bind(document.workspace_id)
        .bind(document.title)
        .bind(document.content)
        .bind(document.kind.as_str())
        .bind(document.format.as_str())
        .bind(document.summary)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create research document: {}", e)))?;

        Ok(ResearchDocument {
            id: document.id.to_string(),
            workspace_id: document.workspace_id.to_string(),
            title: document.title.to_string(),
            content: document.content.map(str::to_string),
            kind: document.kind,
            format: document.format,
            summary: document.summary.map(str::to_string),
            archived_at: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Lists active documents in a workspace, newest first.
    pub async fn list_by_workspace(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<ResearchDocument>, AppError> {
        let rows = sqlx::query_as::<_, ResearchDocumentRow>(
            r#"
            SELECT id, workspace_id, title, content, kind, format, summary,
                   archived_at, created_at, updated_at
            FROM research_documents
            WHERE workspace_id = ? AND archived_at IS NULL
            ORDER BY updated_at DESC, rowid DESC
            "#,
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list research documents: {}", e)))?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }

    /// Gets a document by ID, including archived documents.
    pub async fn get(&self, id: &str) -> Result<Option<ResearchDocument>, AppError> {
        let row = sqlx::query_as::<_, ResearchDocumentRow>(
            r#"
            SELECT id, workspace_id, title, content, kind, format, summary,
                   archived_at, created_at, updated_at
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
            WHERE id = ? AND archived_at IS NULL
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

    /// Archives a document (soft delete).
    pub async fn archive(&self, id: &str) -> Result<(), AppError> {
        let now = Utc::now();

        let rows_affected = sqlx::query(
            r#"
            UPDATE research_documents
            SET archived_at = ?, updated_at = ?
            WHERE id = ? AND archived_at IS NULL
            "#,
        )
        .bind(now)
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to archive research document: {}", e)))?
        .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!(
                "Research document '{}' not found",
                id
            )));
        }

        Ok(())
    }

    /// Permanently deletes a document and its sources/notes.
    pub async fn purge(&self, id: &str) -> Result<(), AppError> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to start transaction: {}", e)))?;

        for table in ["research_sources", "research_notes"] {
            sqlx::query(&format!("DELETE FROM {} WHERE document_id = ?", table))
                .bind(id)
                .execute(&mut *transaction)
                .await
                .map_err(|e| {
                    AppError::Internal(format!("Failed to delete document children: {}", e))
                })?;
        }

        let rows_affected = sqlx::query("DELETE FROM research_documents WHERE id = ?")
            .bind(id)
            .execute(&mut *transaction)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to purge research document: {}", e)))?
            .rows_affected();

        transaction
            .commit()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to commit purge: {}", e)))?;

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!(
                "Research document '{}' not found",
                id
            )));
        }

        Ok(())
    }

    /// Searches active document titles and content in a workspace.
    pub async fn search(
        &self,
        workspace_id: &str,
        query: &str,
        limit: i64,
    ) -> Result<Vec<ResearchDocument>, AppError> {
        let pattern = format!("%{}%", query);

        let rows = sqlx::query_as::<_, ResearchDocumentRow>(
            r#"
            SELECT id, workspace_id, title, content, kind, format, summary,
                   archived_at, created_at, updated_at
            FROM research_documents
            WHERE workspace_id = ? AND archived_at IS NULL
              AND (title LIKE ? OR content LIKE ?)
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
    kind: String,
    format: String,
    summary: Option<String>,
    archived_at: Option<DateTime<Utc>>,
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
            kind: ResearchDocumentKind::from_db(&row.kind),
            format: ResearchDocumentFormat::from_db(&row.format),
            summary: row.summary,
            archived_at: row.archived_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
