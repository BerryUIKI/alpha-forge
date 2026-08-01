// Option position repository — handles option_positions table operations.

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::error::AppError;
use domain::option::OptionPosition;

/// Repository for accessing option positions.
pub struct OptionPositionRepository {
    pool: SqlitePool,
}

impl OptionPositionRepository {
    /// Creates a new option position repository.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Creates a new option position.
    pub async fn create(&self, position: &OptionPosition) -> Result<(), AppError> {
        sqlx::query(
            r#"
                INSERT INTO option_positions (
                    id, workspace_id, account_id, option_contract_id,
                    quantity, cost_basis, opened_at, closed_at, notes
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&position.id)
        .bind(&position.workspace_id)
        .bind(&position.account_id)
        .bind(&position.option_contract_id)
        .bind(position.quantity)
        .bind(position.cost_basis)
        .bind(position.opened_at)
        .bind(position.closed_at)
        .bind(&position.notes)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create option position: {}", e)))?;

        Ok(())
    }

    /// Gets an option position by ID.
    pub async fn get(&self, id: &str) -> Result<Option<OptionPosition>, AppError> {
        let row = sqlx::query_as::<_, OptionPositionRow>(
            r#"
                SELECT id, workspace_id, account_id, option_contract_id,
                       quantity, cost_basis, opened_at, closed_at, notes
                FROM option_positions 
                WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            AppError::Internal(format!("Failed to get option position '{}': {}", id, e))
        })?;

        Ok(row.map(|r| r.into()))
    }

    /// Lists all option positions for a workspace.
    pub async fn list_by_workspace(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<OptionPosition>, AppError> {
        let rows = sqlx::query_as::<_, OptionPositionRow>(
            r#"
                SELECT id, workspace_id, account_id, option_contract_id,
                       quantity, cost_basis, opened_at, closed_at, notes
                FROM option_positions 
                WHERE workspace_id = ? 
                ORDER BY opened_at DESC
            "#,
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list option positions: {}", e)))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    /// Lists open option positions for a workspace.
    pub async fn list_open(&self, workspace_id: &str) -> Result<Vec<OptionPosition>, AppError> {
        let rows = sqlx::query_as::<_, OptionPositionRow>(
            r#"
                SELECT id, workspace_id, account_id, option_contract_id,
                       quantity, cost_basis, opened_at, closed_at, notes
                FROM option_positions 
                WHERE workspace_id = ? AND closed_at IS NULL
                ORDER BY opened_at DESC
            "#,
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list open option positions: {}", e)))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    /// Updates an option position.
    pub async fn update(&self, position: &OptionPosition) -> Result<(), AppError> {
        let rows_affected = sqlx::query(
            r#"
                UPDATE option_positions 
                SET quantity = ?, cost_basis = ?, closed_at = ?, notes = ?
                WHERE id = ?
            "#,
        )
        .bind(position.quantity)
        .bind(position.cost_basis)
        .bind(position.closed_at)
        .bind(&position.notes)
        .bind(&position.id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update option position: {}", e)))?
        .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!(
                "Option position '{}' not found",
                position.id
            )));
        }

        Ok(())
    }

    /// Closes an option position.
    pub async fn close(&self, id: &str, closed_at: DateTime<Utc>) -> Result<(), AppError> {
        let rows_affected = sqlx::query("UPDATE option_positions SET closed_at = ? WHERE id = ?")
            .bind(closed_at)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to close option position: {}", e)))?
            .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!(
                "Option position '{}' not found",
                id
            )));
        }

        Ok(())
    }

    /// Deletes an option position.
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        let rows_affected = sqlx::query("DELETE FROM option_positions WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to delete option position: {}", e)))?
            .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!(
                "Option position '{}' not found",
                id
            )));
        }

        Ok(())
    }
}

/// Database row representation of an option position.
#[derive(Debug, sqlx::FromRow)]
struct OptionPositionRow {
    id: String,
    workspace_id: String,
    account_id: Option<String>,
    option_contract_id: String,
    quantity: i32,
    cost_basis: f64,
    opened_at: DateTime<Utc>,
    closed_at: Option<DateTime<Utc>>,
    notes: Option<String>,
}

impl From<OptionPositionRow> for OptionPosition {
    fn from(row: OptionPositionRow) -> Self {
        OptionPosition {
            id: row.id,
            workspace_id: row.workspace_id,
            account_id: row.account_id,
            option_contract_id: row.option_contract_id,
            quantity: row.quantity,
            cost_basis: row.cost_basis,
            opened_at: row.opened_at,
            closed_at: row.closed_at,
            notes: row.notes,
        }
    }
}
