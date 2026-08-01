// Workspace repository — handles workspaces table operations.

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::error::AppError;
use domain::workspace::Workspace;

/// Repository for accessing workspaces.
pub struct WorkspaceRepository {
    pool: SqlitePool,
}

impl WorkspaceRepository {
    /// Creates a new workspace repository.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Creates a new workspace.
    pub async fn create(&self, id: &str, name: &str) -> Result<Workspace, AppError> {
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO workspaces (id, name, created_at, updated_at) VALUES (?, ?, ?, ?)",
        )
        .bind(id)
        .bind(name)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create workspace: {}", e)))?;

        Ok(Workspace {
            id: id.to_string(),
            name: name.to_string(),
            created_at: now,
            updated_at: now,
        })
    }

    /// Lists all workspaces.
    pub async fn list(&self) -> Result<Vec<Workspace>, AppError> {
        let rows = sqlx::query_as::<_, WorkspaceRow>(
            "SELECT id, name, created_at, updated_at FROM workspaces ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list workspaces: {}", e)))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    /// Gets a workspace by ID.
    pub async fn get(&self, id: &str) -> Result<Option<Workspace>, AppError> {
        let row = sqlx::query_as::<_, WorkspaceRow>(
            "SELECT id, name, created_at, updated_at FROM workspaces WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to get workspace '{}': {}", id, e)))?;

        Ok(row.map(|r| r.into()))
    }

    /// Updates a workspace.
    pub async fn update(&self, id: &str, name: &str) -> Result<Workspace, AppError> {
        let now = Utc::now();

        let rows_affected =
            sqlx::query("UPDATE workspaces SET name = ?, updated_at = ? WHERE id = ?")
                .bind(name)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::Internal(format!("Failed to update workspace: {}", e)))?
                .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Workspace '{}' not found", id)));
        }

        // Fetch the updated workspace
        self.get(id)
            .await?
            .ok_or_else(|| AppError::Internal("Workspace updated but not found".to_string()))
    }

    /// Deletes a workspace.
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        let rows_affected = sqlx::query("DELETE FROM workspaces WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to delete workspace: {}", e)))?
            .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Workspace '{}' not found", id)));
        }

        Ok(())
    }
}

/// Database row representation of a workspace.
#[derive(Debug, sqlx::FromRow)]
struct WorkspaceRow {
    id: String,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<WorkspaceRow> for Workspace {
    fn from(row: WorkspaceRow) -> Self {
        Workspace {
            id: row.id,
            name: row.name,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
