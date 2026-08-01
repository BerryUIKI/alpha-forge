// Option chain repository — handles option_chains table operations.

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::error::AppError;
use domain::option::{DataSource, OptionChain};

/// Repository for accessing option chains.
pub struct OptionChainRepository {
    pool: SqlitePool,
}

impl OptionChainRepository {
    /// Creates a new option chain repository.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Creates a new option chain.
    pub async fn create(&self, chain: &OptionChain) -> Result<(), AppError> {
        sqlx::query(
            r#"
                INSERT INTO option_chains (
                    id, workspace_id, symbol, underlying_price,
                    as_of, data_source, created_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&chain.id)
        .bind(&chain.workspace_id)
        .bind(&chain.symbol)
        .bind(chain.underlying_price)
        .bind(chain.as_of)
        .bind(chain.data_source.to_string())
        .bind(chain.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create option chain: {}", e)))?;

        Ok(())
    }

    /// Gets an option chain by ID.
    pub async fn get(&self, id: &str) -> Result<Option<OptionChain>, AppError> {
        let row = sqlx::query_as::<_, OptionChainRow>(
            "SELECT id, workspace_id, symbol, underlying_price, as_of, data_source, created_at FROM option_chains WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to get option chain '{}': {}", id, e)))?;

        Ok(row.map(|r| r.into()))
    }

    /// Lists all option chains for a workspace.
    pub async fn list_by_workspace(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<OptionChain>, AppError> {
        let rows = sqlx::query_as::<_, OptionChainRow>(
            "SELECT id, workspace_id, symbol, underlying_price, as_of, data_source, created_at 
             FROM option_chains 
             WHERE workspace_id = ? 
             ORDER BY created_at DESC",
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list option chains: {}", e)))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    /// Gets the latest option chain for a symbol in a workspace.
    pub async fn get_latest(
        &self,
        workspace_id: &str,
        symbol: &str,
    ) -> Result<Option<OptionChain>, AppError> {
        let row = sqlx::query_as::<_, OptionChainRow>(
            "SELECT id, workspace_id, symbol, underlying_price, as_of, data_source, created_at 
             FROM option_chains 
             WHERE workspace_id = ? AND symbol = ? 
             ORDER BY as_of DESC 
             LIMIT 1",
        )
        .bind(workspace_id)
        .bind(symbol)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            AppError::Internal(format!(
                "Failed to get latest option chain for '{}': {}",
                symbol, e
            ))
        })?;

        Ok(row.map(|r| r.into()))
    }

    /// Deletes an option chain.
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        let rows_affected = sqlx::query("DELETE FROM option_chains WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to delete option chain: {}", e)))?
            .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!(
                "Option chain '{}' not found",
                id
            )));
        }

        Ok(())
    }
}

/// Database row representation of an option chain.
#[derive(Debug, sqlx::FromRow)]
struct OptionChainRow {
    id: String,
    workspace_id: String,
    symbol: String,
    underlying_price: f64,
    as_of: DateTime<Utc>,
    data_source: String,
    created_at: DateTime<Utc>,
}

impl From<OptionChainRow> for OptionChain {
    fn from(row: OptionChainRow) -> Self {
        OptionChain {
            id: row.id,
            workspace_id: row.workspace_id,
            symbol: row.symbol,
            underlying_price: row.underlying_price,
            as_of: row.as_of,
            data_source: match row.data_source.as_str() {
                "live" => DataSource::Live,
                "demo" => DataSource::Demo,
                "file" => DataSource::File,
                _ => DataSource::Demo, // Default fallback
            },
            created_at: row.created_at,
        }
    }
}
