// Artifact repository — persistence for renderable agent artifacts.

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::error::AppError;
use domain::artifact::{Artifact, ArtifactStatus};

/// Repository for managing artifacts.
#[derive(Clone)]
pub struct ArtifactRepository {
    pool: SqlitePool,
}

impl ArtifactRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Exposes the pool to sibling test modules.
    #[cfg(test)]
    pub(crate) fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// Stores a validated artifact and marks it completed.
    pub async fn create(
        &self,
        task_id: Option<&str>,
        workspace_id: Option<&str>,
        artifact_type: &str,
        title: &str,
        data: &str,
    ) -> Result<Artifact, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO artifacts
                (id, task_id, workspace_id, artifact_type, title, input, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, 'completed', ?, ?)
            "#,
        )
        .bind(&id)
        .bind(task_id)
        .bind(workspace_id)
        .bind(artifact_type)
        .bind(title)
        .bind(data)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create artifact: {}", e)))?;

        Ok(Artifact {
            id,
            task_id: task_id.map(str::to_string),
            workspace_id: workspace_id.map(str::to_string),
            artifact_type: artifact_type.to_string(),
            title: title.to_string(),
            data: data.to_string(),
            status: ArtifactStatus::Completed,
            created_at: now,
            updated_at: now,
        })
    }

    /// Gets an artifact by ID.
    pub async fn get(&self, id: &str) -> Result<Option<Artifact>, AppError> {
        let row = sqlx::query_as::<_, ArtifactRow>(
            r#"
            SELECT id, task_id, workspace_id, artifact_type, title, input AS data,
                   status, created_at, updated_at
            FROM artifacts
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to get artifact: {}", e)))?;

        row.map(Artifact::try_from).transpose()
    }

    /// Lists artifacts, optionally filtered by workspace and task.
    pub async fn list(
        &self,
        workspace_id: Option<&str>,
        task_id: Option<&str>,
    ) -> Result<Vec<Artifact>, AppError> {
        let rows = sqlx::query_as::<_, ArtifactRow>(
            r#"
            SELECT id, task_id, workspace_id, artifact_type, title, input AS data,
                   status, created_at, updated_at
            FROM artifacts
            WHERE (? IS NULL OR workspace_id = ?)
              AND (? IS NULL OR task_id = ?)
            ORDER BY created_at DESC, rowid DESC
            "#,
        )
        .bind(workspace_id)
        .bind(workspace_id)
        .bind(task_id)
        .bind(task_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list artifacts: {}", e)))?;

        rows.into_iter().map(Artifact::try_from).collect()
    }
}

/// Database row representation of an artifact.
#[derive(Debug, sqlx::FromRow)]
struct ArtifactRow {
    id: String,
    task_id: Option<String>,
    workspace_id: Option<String>,
    artifact_type: String,
    title: String,
    data: String,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<ArtifactRow> for Artifact {
    type Error = AppError;

    fn try_from(row: ArtifactRow) -> Result<Self, Self::Error> {
        let status = ArtifactStatus::from_db(&row.status).ok_or_else(|| {
            AppError::Internal(format!("unknown artifact status '{}'", row.status))
        })?;

        Ok(Artifact {
            id: row.id,
            task_id: row.task_id,
            workspace_id: row.workspace_id,
            artifact_type: row.artifact_type,
            title: row.title,
            data: row.data,
            status,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}
