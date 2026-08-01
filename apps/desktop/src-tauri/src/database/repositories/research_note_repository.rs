// Research note repository — handles note persistence.

use chrono::Utc;
use sqlx::SqlitePool;
use crate::error::AppError;
use domain::research::{CreateNoteInput, ResearchNote};

pub struct ResearchNoteRepository { pool: SqlitePool }

impl ResearchNoteRepository {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, input: CreateNoteInput) -> Result<ResearchNote, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        sqlx::query("INSERT INTO research_notes (id, document_id, content, created_at, updated_at) VALUES (?, ?, ?, ?, ?)")
            .bind(&id).bind(&input.document_id).bind(&input.content).bind(now.to_rfc3339()).bind(now.to_rfc3339())
            .execute(&self.pool).await.map_err(|e| AppError::Internal(format!("Failed to create note: {}", e)))?;
        Ok(ResearchNote { id, document_id: input.document_id, content: input.content, created_at: now, updated_at: now })
    }

    pub async fn list_by_document(&self, document_id: &str) -> Result<Vec<ResearchNote>, AppError> {
        let rows = sqlx::query_as::<_, NoteRow>("SELECT id, document_id, content, created_at, updated_at FROM research_notes WHERE document_id = ? ORDER BY created_at DESC")
            .bind(document_id).fetch_all(&self.pool).await.map_err(|e| AppError::Internal(format!("Failed to list notes: {}", e)))?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM research_notes WHERE id = ?").bind(id).execute(&self.pool).await
            .map_err(|e| AppError::Internal(format!("Failed to delete note: {}", e)))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct NoteRow { id: String, document_id: String, content: String, created_at: String, updated_at: String }

impl From<NoteRow> for ResearchNote {
    fn from(row: NoteRow) -> Self {
        ResearchNote {
            id: row.id,
            document_id: row.document_id,
            content: row.content,
            created_at: row.created_at.parse().unwrap_or_else(|_| Utc::now()),
            updated_at: row.updated_at.parse().unwrap_or_else(|_| Utc::now()),
        }
    }
}