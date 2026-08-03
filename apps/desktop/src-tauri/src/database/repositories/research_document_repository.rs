// Research document repository — handles document persistence.

use crate::error::AppError;
use chrono::Utc;
use domain::research::{CreateDocumentInput, DocumentType, ResearchDocument};
use sqlx::SqlitePool;

pub struct ResearchDocumentRepository {
    pool: SqlitePool,
}

impl ResearchDocumentRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateDocumentInput) -> Result<ResearchDocument, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let doc_type = input.document_type.to_string();
        sqlx::query("INSERT INTO research_documents (id, project_id, document_type, title, content, source_url, file_path, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(&id).bind(&input.project_id).bind(&doc_type).bind(&input.title)
            .bind(&input.content).bind(&input.source_url).bind(&input.file_path)
            .bind(now.to_rfc3339()).bind(now.to_rfc3339())
            .execute(&self.pool).await.map_err(|e| AppError::Internal(format!("Failed to create document: {}", e)))?;
        Ok(ResearchDocument {
            id,
            project_id: input.project_id,
            document_type: input.document_type,
            title: input.title,
            content: input.content,
            source_url: input.source_url,
            file_path: input.file_path,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get(&self, id: &str) -> Result<Option<ResearchDocument>, AppError> {
        sqlx::query_as::<_, DocumentRow>("SELECT id, project_id, document_type, title, content, source_url, file_path, created_at, updated_at FROM research_documents WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await.map_err(|e| AppError::Internal(format!("Failed to get document: {}", e))).map(|r| r.map(|r| r.into()))
    }

    pub async fn list_by_project(
        &self,
        project_id: &str,
    ) -> Result<Vec<ResearchDocument>, AppError> {
        sqlx::query_as::<_, DocumentRow>("SELECT id, project_id, document_type, title, content, source_url, file_path, created_at, updated_at FROM research_documents WHERE project_id = ? ORDER BY created_at DESC")
            .bind(project_id).fetch_all(&self.pool).await.map_err(|e| AppError::Internal(format!("Failed to list documents: {}", e))).map(|rows| rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM research_documents WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to delete document: {}", e)))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct DocumentRow {
    id: String,
    project_id: String,
    document_type: String,
    title: String,
    content: Option<String>,
    source_url: Option<String>,
    file_path: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<DocumentRow> for ResearchDocument {
    fn from(row: DocumentRow) -> Self {
        let document_type = match row.document_type.as_str() {
            "pdf" => DocumentType::Pdf,
            "web_page" => DocumentType::WebPage,
            "note" => DocumentType::Note,
            "report" => DocumentType::Report,
            _ => DocumentType::Note,
        };
        ResearchDocument {
            id: row.id,
            project_id: row.project_id,
            document_type,
            title: row.title,
            content: row.content,
            source_url: row.source_url,
            file_path: row.file_path,
            created_at: row.created_at.parse().unwrap_or_else(|_| Utc::now()),
            updated_at: row.updated_at.parse().unwrap_or_else(|_| Utc::now()),
        }
    }
}
