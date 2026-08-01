// Artifact repository — handles persistence for research artifacts.

use chrono::Utc;
use sqlx::SqlitePool;

use crate::error::AppError;
use domain::artifact::{Artifact, ArtifactStatus, ArtifactType, CreateArtifactInput};

/// Repository for managing artifacts.
pub struct ArtifactRepository {
    pool: SqlitePool,
}

/// Database row representation of an artifact.
#[derive(Debug, sqlx::FromRow)]
struct ArtifactRow {
    id: String,
    workspace_id: String,
    task_id: Option<String>,
    artifact_type: String,
    status: String,
    input: String,
    output: Option<String>,
    error: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<ArtifactRow> for Artifact {
    fn from(row: ArtifactRow) -> Self {
        let artifact_type = match row.artifact_type.as_str() {
            "comparison_table" => ArtifactType::ComparisonTable,
            "timeline" => ArtifactType::Timeline,
            "industry_map" => ArtifactType::IndustryMap,
            "valuation_model" => ArtifactType::ValuationModel,
            "risk_dashboard" => ArtifactType::RiskDashboard,
            "earnings_analysis" => ArtifactType::EarningsAnalysis,
            "macro_dashboard" => ArtifactType::MacroDashboard,
            other => ArtifactType::Custom(other.to_string()),
        };

        let status = match row.status.as_str() {
            "pending" => ArtifactStatus::Pending,
            "generating" => ArtifactStatus::Generating,
            "completed" => ArtifactStatus::Completed,
            "viewing" => ArtifactStatus::Viewing,
            "closed" => ArtifactStatus::Closed,
            "failed" => ArtifactStatus::Failed,
            _ => ArtifactStatus::Pending,
        };

        let input = serde_json::from_str(&row.input).unwrap_or(serde_json::Value::Null);
        let output = row.output.and_then(|s| serde_json::from_str(&s).ok());

        Artifact {
            id: row.id,
            task_id: row.task_id,
            workspace_id: row.workspace_id,
            artifact_type,
            status,
            input,
            output,
            error: row.error,
            created_at: row.created_at.parse().unwrap_or(Utc::now()),
            updated_at: row.updated_at.parse().unwrap_or(Utc::now()),
        }
    }
}

impl ArtifactRepository {
    /// Creates a new artifact repository.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Creates a new artifact.
    pub async fn create(&self, input: CreateArtifactInput) -> Result<Artifact, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        let artifact_type_str = match input.artifact_type {
            ArtifactType::ComparisonTable => "comparison_table",
            ArtifactType::Timeline => "timeline",
            ArtifactType::IndustryMap => "industry_map",
            ArtifactType::ValuationModel => "valuation_model",
            ArtifactType::RiskDashboard => "risk_dashboard",
            ArtifactType::EarningsAnalysis => "earnings_analysis",
            ArtifactType::MacroDashboard => "macro_dashboard",
            ArtifactType::Custom(ref s) => s,
        };

        let input_json = serde_json::to_string(&input.input)
            .map_err(|e| AppError::Internal(format!("Failed to serialize artifact input: {}", e)))?;

        sqlx::query(
            r#"
            INSERT INTO artifacts (id, workspace_id, task_id, artifact_type, status, input, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&input.workspace_id)
        .bind(&input.task_id)
        .bind(artifact_type_str)
        .bind("pending")
        .bind(&input_json)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create artifact: {}", e)))?;

        Ok(Artifact {
            id,
            task_id: input.task_id,
            workspace_id: input.workspace_id,
            artifact_type: input.artifact_type,
            status: ArtifactStatus::Pending,
            input: input.input,
            output: None,
            error: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Lists all artifacts for a workspace.
    pub async fn list_by_workspace(&self, workspace_id: &str) -> Result<Vec<Artifact>, AppError> {
        let rows = sqlx::query_as::<_, ArtifactRow>(
            r#"
            SELECT id, workspace_id, task_id, artifact_type, status, input, output, error, created_at, updated_at
            FROM artifacts
            WHERE workspace_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list artifacts: {}", e)))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    /// Gets an artifact by ID.
    pub async fn get(&self, id: &str) -> Result<Option<Artifact>, AppError> {
        let row = sqlx::query_as::<_, ArtifactRow>(
            r#"
            SELECT id, workspace_id, task_id, artifact_type, status, input, output, error, created_at, updated_at
            FROM artifacts
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to get artifact: {}", e)))?;

        Ok(row.map(|r| r.into()))
    }

    /// Updates artifact status.
    pub async fn update_status(&self, id: &str, status: ArtifactStatus) -> Result<(), AppError> {
        let status_str = match status {
            ArtifactStatus::Pending => "pending",
            ArtifactStatus::Generating => "generating",
            ArtifactStatus::Completed => "completed",
            ArtifactStatus::Viewing => "viewing",
            ArtifactStatus::Closed => "closed",
            ArtifactStatus::Failed => "failed",
        };

        sqlx::query(
            r#"
            UPDATE artifacts
            SET status = ?
            WHERE id = ?
            "#,
        )
        .bind(status_str)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update artifact status: {}", e)))?;

        Ok(())
    }

    /// Updates artifact output.
    pub async fn update_output(&self, id: &str, output: serde_json::Value) -> Result<(), AppError> {
        let output_json = serde_json::to_string(&output)
            .map_err(|e| AppError::Internal(format!("Failed to serialize artifact output: {}", e)))?;

        sqlx::query(
            r#"
            UPDATE artifacts
            SET output = ?, status = 'completed'
            WHERE id = ?
            "#,
        )
        .bind(&output_json)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update artifact output: {}", e)))?;

        Ok(())
    }

    /// Sets artifact error.
    pub async fn set_error(&self, id: &str, error: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE artifacts
            SET error = ?, status = 'failed'
            WHERE id = ?
            "#,
        )
        .bind(error)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to set artifact error: {}", e)))?;

        Ok(())
    }

    /// Deletes an artifact.
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM artifacts
            WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to delete artifact: {}", e)))?;

        Ok(())
    }

    /// Lists artifacts by task ID.
    pub async fn list_by_task(&self, task_id: &str) -> Result<Vec<Artifact>, AppError> {
        let rows = sqlx::query_as::<_, ArtifactRow>(
            r#"
            SELECT id, workspace_id, task_id, artifact_type, status, input, output, error, created_at, updated_at
            FROM artifacts
            WHERE task_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(task_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list artifacts by task: {}", e)))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}
