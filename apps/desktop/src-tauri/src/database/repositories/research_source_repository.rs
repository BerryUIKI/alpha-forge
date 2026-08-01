// Research source repository — handles source persistence.

use chrono::Utc;
use sqlx::SqlitePool;
use crate::error::AppError;
use domain::research::ResearchSource;

pub struct ResearchSourceRepository { pool: SqlitePool }

impl ResearchSourceRepository {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, document_id: String, url: Option<String>, title: Option<String>) -> Result<ResearchSource, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        sqlx::query("INSERT INTO research_sources (id, document_id, url, title, retrieved_at, created_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&id).bind(&document_id).bind(&url).bind(&title).bind(now.to_rfc3339()).bind(now.to_rfc3339())
            .execute(&self.pool).await.map_err(|e| AppError::Internal(format!("Failed to create source: {}", e)))?;
        Ok(ResearchSource { id, document_id, url, title, retrieved_at: Some(now), created_at: now })
    }

    pub async fn list_by_document(&self, document_id: &str) -> Result<Vec<ResearchSource>, AppError> {
        sqlx::query_as("SELECT id, document_id, url, title, retrieved_at, created_at FROM research_sources WHERE document_id = ? ORDER BY created_at DESC")
            .bind(document_id).fetch_all(&self.pool).await.map_err(|e| AppError::Internal(format!("Failed to list sources: {}", e)))
    }
}