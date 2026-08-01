// Research report repository — handles report persistence.

use chrono::Utc;
use sqlx::SqlitePool;
use crate::error::AppError;
use domain::research::{CreateReportInput, ReportType, ResearchReport};

pub struct ResearchReportRepository { pool: SqlitePool }

impl ResearchReportRepository {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, input: CreateReportInput) -> Result<ResearchReport, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let report_type = input.report_type.to_string();
        sqlx::query("INSERT INTO research_reports (id, project_id, title, content, report_type, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(&id).bind(&input.project_id).bind(&input.title).bind(&input.content).bind(&report_type)
            .bind(now.to_rfc3339()).bind(now.to_rfc3339())
            .execute(&self.pool).await.map_err(|e| AppError::Internal(format!("Failed to create report: {}", e)))?;
        Ok(ResearchReport { id, project_id: input.project_id, title: input.title, content: input.content, report_type: input.report_type, created_at: now, updated_at: now })
    }

    pub async fn get(&self, id: &str) -> Result<Option<ResearchReport>, AppError> {
        sqlx::query_as::<_, ReportRow>("SELECT id, project_id, title, content, report_type, created_at, updated_at FROM research_reports WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await.map_err(|e| AppError::Internal(format!("Failed to get report: {}", e))).map(|r| r.map(|r| r.into()))
    }

    pub async fn list_by_project(&self, project_id: &str) -> Result<Vec<ResearchReport>, AppError> {
        sqlx::query_as::<_, ReportRow>("SELECT id, project_id, title, content, report_type, created_at, updated_at FROM research_reports WHERE project_id = ? ORDER BY created_at DESC")
            .bind(project_id).fetch_all(&self.pool).await.map_err(|e| AppError::Internal(format!("Failed to list reports: {}", e))).map(|rows| rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM research_reports WHERE id = ?").bind(id).execute(&self.pool).await
            .map_err(|e| AppError::Internal(format!("Failed to delete report: {}", e)))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct ReportRow { id: String, project_id: String, title: String, content: String, report_type: String, created_at: String, updated_at: String }

impl From<ReportRow> for ResearchReport {
    fn from(row: ReportRow) -> Self {
        let report_type = match row.report_type.as_str() { "analysis" => ReportType::Analysis, "summary" => ReportType::Summary, "thesis" => ReportType::Thesis, "recommendation" => ReportType::Recommendation, _ => ReportType::Analysis };
        ResearchReport { id: row.id, project_id: row.project_id, title: row.title, content: row.content, report_type, created_at: row.created_at.parse().unwrap_or_else(|_| Utc::now()), updated_at: row.updated_at.parse().unwrap_or_else(|_| Utc::now()) }
    }
}