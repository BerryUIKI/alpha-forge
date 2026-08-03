// Research source repository — handles source persistence.

use crate::error::AppError;
use chrono::Utc;
use domain::research::ResearchSource;
use sqlx::SqlitePool;

pub struct ResearchSourceRepository {
    pool: SqlitePool,
}

impl ResearchSourceRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        document_id: String,
        url: Option<String>,
        title: Option<String>,
    ) -> Result<ResearchSource, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        sqlx::query("INSERT INTO research_sources (id, document_id, url, title, retrieved_at, created_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&id).bind(&document_id).bind(&url).bind(&title).bind(now.to_rfc3339()).bind(now.to_rfc3339())
            .execute(&self.pool).await.map_err(|e| AppError::Internal(format!("Failed to create source: {}", e)))?;
        Ok(ResearchSource {
            id,
            document_id,
            url,
            title,
            retrieved_at: Some(now),
            created_at: now,
        })
    }

    pub async fn list_by_document(
        &self,
        document_id: &str,
    ) -> Result<Vec<ResearchSource>, AppError> {
        let rows = sqlx::query_as::<_, SourceRow>("SELECT id, document_id, url, title, retrieved_at, created_at FROM research_sources WHERE document_id = ? ORDER BY created_at DESC")
            .bind(document_id).fetch_all(&self.pool).await.map_err(|e| AppError::Internal(format!("Failed to list sources: {}", e)))?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct SourceRow {
    id: String,
    document_id: String,
    url: Option<String>,
    title: Option<String>,
    retrieved_at: Option<String>,
    created_at: String,
}

impl From<SourceRow> for ResearchSource {
    fn from(row: SourceRow) -> Self {
        ResearchSource {
            id: row.id,
            document_id: row.document_id,
            url: row.url,
            title: row.title,
            retrieved_at: row.retrieved_at.and_then(|s| s.parse().ok()),
            created_at: row.created_at.parse().unwrap_or_else(|_| Utc::now()),
        }
    }
}
