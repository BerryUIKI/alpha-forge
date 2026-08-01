// Option contract repository — handles option_contracts table operations.

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::error::AppError;
use domain::option::{OptionContract, OptionType};

/// Repository for accessing option contracts.
pub struct OptionContractRepository {
    pool: SqlitePool,
}

impl OptionContractRepository {
    /// Creates a new option contract repository.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Creates a new option contract.
    pub async fn create(&self, contract: &OptionContract) -> Result<(), AppError> {
        sqlx::query(
            r#"
                INSERT INTO option_contracts (
                    id, workspace_id, chain_id, symbol, option_type,
                    strike, expiration, contract_multiplier, bid, ask,
                    last, volume, open_interest, implied_volatility,
                    created_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&contract.id)
        .bind(&contract.workspace_id)
        .bind(&contract.chain_id)
        .bind(&contract.symbol)
        .bind(contract.option_type.to_string())
        .bind(contract.strike)
        .bind(contract.expiration)
        .bind(contract.contract_multiplier)
        .bind(contract.bid)
        .bind(contract.ask)
        .bind(contract.last)
        .bind(contract.volume as i64)
        .bind(contract.open_interest as i64)
        .bind(contract.implied_volatility)
        .bind(contract.created_at)
        .bind(contract.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create option contract: {}", e)))?;

        Ok(())
    }

    /// Gets an option contract by ID.
    pub async fn get(&self, id: &str) -> Result<Option<OptionContract>, AppError> {
        let row = sqlx::query_as::<_, OptionContractRow>(
            r#"
                SELECT id, workspace_id, chain_id, symbol, option_type,
                       strike, expiration, contract_multiplier, bid, ask,
                       last, volume, open_interest, implied_volatility,
                       created_at, updated_at
                FROM option_contracts 
                WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            AppError::Internal(format!("Failed to get option contract '{}': {}", id, e))
        })?;

        Ok(row.map(|r| r.into()))
    }

    /// Lists all option contracts for a chain.
    pub async fn list_by_chain(&self, chain_id: &str) -> Result<Vec<OptionContract>, AppError> {
        let rows = sqlx::query_as::<_, OptionContractRow>(
            r#"
                SELECT id, workspace_id, chain_id, symbol, option_type,
                       strike, expiration, contract_multiplier, bid, ask,
                       last, volume, open_interest, implied_volatility,
                       created_at, updated_at
                FROM option_contracts 
                WHERE chain_id = ? 
                ORDER BY strike ASC, option_type ASC
            "#,
        )
        .bind(chain_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list option contracts: {}", e)))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    /// Updates an option contract.
    pub async fn update(&self, contract: &OptionContract) -> Result<(), AppError> {
        let rows_affected = sqlx::query(
            r#"
                UPDATE option_contracts 
                SET bid = ?, ask = ?, last = ?, volume = ?, 
                    open_interest = ?, implied_volatility = ?, updated_at = ?
                WHERE id = ?
            "#,
        )
        .bind(contract.bid)
        .bind(contract.ask)
        .bind(contract.last)
        .bind(contract.volume as i64)
        .bind(contract.open_interest as i64)
        .bind(contract.implied_volatility)
        .bind(Utc::now())
        .bind(&contract.id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update option contract: {}", e)))?
        .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!(
                "Option contract '{}' not found",
                contract.id
            )));
        }

        Ok(())
    }

    /// Deletes an option contract.
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        let rows_affected = sqlx::query("DELETE FROM option_contracts WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to delete option contract: {}", e)))?
            .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!(
                "Option contract '{}' not found",
                id
            )));
        }

        Ok(())
    }
}

/// Database row representation of an option contract.
#[derive(Debug, sqlx::FromRow)]
struct OptionContractRow {
    id: String,
    workspace_id: String,
    chain_id: String,
    symbol: String,
    option_type: String,
    strike: f64,
    expiration: DateTime<Utc>,
    contract_multiplier: i32,
    bid: f64,
    ask: f64,
    last: Option<f64>,
    volume: i64,
    open_interest: i64,
    implied_volatility: f64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<OptionContractRow> for OptionContract {
    fn from(row: OptionContractRow) -> Self {
        OptionContract {
            id: row.id,
            workspace_id: row.workspace_id,
            chain_id: row.chain_id,
            symbol: row.symbol,
            option_type: match row.option_type.as_str() {
                "call" => OptionType::Call,
                "put" => OptionType::Put,
                _ => OptionType::Call, // Default fallback
            },
            strike: row.strike,
            expiration: row.expiration,
            contract_multiplier: row.contract_multiplier as u32,
            bid: row.bid,
            ask: row.ask,
            last: row.last,
            volume: row.volume as u64,
            open_interest: row.open_interest as u64,
            implied_volatility: row.implied_volatility,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
