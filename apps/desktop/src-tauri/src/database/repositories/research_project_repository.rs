// Research project repository — handles persistence for research projects.

use chrono::Utc;
use sqlx::SqlitePool;

use crate::error::AppError;
use domain::research::{CreateProjectInput, ProjectStatus, ResearchProject};

pub struct ResearchProjectRepository {
    pool: SqlitePool,
}

impl ResearchProjectRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateProjectInput) -> Result<ResearchProject, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO research_projects (id, workspace_id, title, description, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&input.workspace_id)
        .bind(&input.title)
        .bind(&input.description)
        .bind("active")
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create research project: {}", e)))?;

        Ok(ResearchProject {
            id,
            workspace_id: input.workspace_id,
            title: input.title,
            description: input.description,
            status: ProjectStatus::Active,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get(&self, id: &str) -> Result<Option<ResearchProject>, AppError> {
        let row = sqlx::query_as::<_, ResearchProjectRow>(
            "SELECT id, workspace_id, title, description, status, created_at, updated_at FROM research_projects WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to get research project: {}", e)))?;

        Ok(row.map(|r| r.into()))
    }

    pub async fn list_by_workspace(&self, workspace_id: &str) -> Result<Vec<ResearchProject>, AppError> {
        let rows = sqlx::query_as::<_, ResearchProjectRow>(
            "SELECT id, workspace_id, title, description, status, created_at, updated_at FROM research_projects WHERE workspace_id = ? ORDER BY created_at DESC"
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list research projects: {}", e)))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn update_status(&self, id: &str, status: ProjectStatus) -> Result<(), AppError> {
        let status_str = status.to_string();
        sqlx::query("UPDATE research_projects SET status = ? WHERE id = ?")
            .bind(&status_str)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to update project status: {}", e)))?;
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM research_projects WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to delete research project: {}", e)))?;
        Ok(())
    }
}

#[derive(Debug, sqlx::FromRow)]
struct ResearchProjectRow {
    id: String,
    workspace_id: String,
    title: String,
    description: Option<String>,
    status: String,
    created_at: String,
    updated_at: String,
}

impl From<ResearchProjectRow> for ResearchProject {
    fn from(row: ResearchProjectRow) -> Self {
        let status = match row.status.as_str() {
            "active" => ProjectStatus::Active,
            "archived" => ProjectStatus::Archived,
            "completed" => ProjectStatus::Completed,
            _ => ProjectStatus::Active,
        };

        ResearchProject {
            id: row.id,
            workspace_id: row.workspace_id,
            title: row.title,
            description: row.description,
            status,
            created_at: row.created_at.parse().unwrap_or_else(|_| Utc::now()),
            updated_at: row.updated_at.parse().unwrap_or_else(|_| Utc::now()),
        }
    }
}