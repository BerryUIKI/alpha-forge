// Strategy leg repository — handles strategy_legs table operations.

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::error::AppError;
use domain::option::{OptionType, PositionType, StrategyLeg};

/// Repository for accessing strategy legs.
pub struct StrategyLegRepository {
    pool: SqlitePool,
}

impl StrategyLegRepository {
    /// Creates a new strategy leg repository.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Creates a new strategy leg.
    pub async fn create(&self, leg: &StrategyLeg) -> Result<(), AppError> {
        sqlx::query(
            r#"
                INSERT INTO strategy_legs (
                    id, strategy_id, option_contract_id, quantity,
                    position_type, premium, strike, expiration, option_type
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&leg.id)
        .bind(&leg.strategy_id)
        .bind(&leg.option_contract_id)
        .bind(leg.quantity)
        .bind(leg.position_type.to_string())
        .bind(leg.premium)
        .bind(leg.strike)
        .bind(leg.expiration)
        .bind(leg.option_type.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create strategy leg: {}", e)))?;

        Ok(())
    }

    /// Gets a strategy leg by ID.
    pub async fn get(&self, id: &str) -> Result<Option<StrategyLeg>, AppError> {
        let row = sqlx::query_as::<_, StrategyLegRow>(
            "SELECT id, strategy_id, option_contract_id, quantity, position_type, premium, strike, expiration, option_type FROM strategy_legs WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to get strategy leg '{}': {}", id, e)))?;

        Ok(row.map(|r| r.into()))
    }

    /// Lists all legs for a strategy.
    pub async fn list_by_strategy(&self, strategy_id: &str) -> Result<Vec<StrategyLeg>, AppError> {
        let rows = sqlx::query_as::<_, StrategyLegRow>(
            r#"
                SELECT id, strategy_id, option_contract_id, quantity,
                       position_type, premium, strike, expiration, option_type
                FROM strategy_legs 
                WHERE strategy_id = ?
                ORDER BY strike, expiration
            "#,
        )
        .bind(strategy_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list strategy legs: {}", e)))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    /// Lists all strategy legs for a workspace by joining through option_strategies.
    pub async fn list_by_workspace(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<StrategyLeg>, AppError> {
        let rows = sqlx::query_as::<_, StrategyLegRow>(
            r#"
                SELECT sl.id, sl.strategy_id, sl.option_contract_id, sl.quantity,
                       sl.position_type, sl.premium, sl.strike, sl.expiration, sl.option_type
                FROM strategy_legs sl
                INNER JOIN option_strategies os ON sl.strategy_id = os.id
                WHERE os.workspace_id = ?
                ORDER BY os.created_at DESC, sl.strike, sl.expiration
            "#,
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list strategy legs: {}", e)))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    /// Updates a strategy leg.
    pub async fn update(&self, leg: &StrategyLeg) -> Result<(), AppError> {
        let rows_affected = sqlx::query(
            r#"
                UPDATE strategy_legs 
                SET quantity = ?, position_type = ?, premium = ?, 
                    strike = ?, expiration = ?, option_type = ?
                WHERE id = ?
            "#,
        )
        .bind(leg.quantity)
        .bind(leg.position_type.to_string())
        .bind(leg.premium)
        .bind(leg.strike)
        .bind(leg.expiration)
        .bind(leg.option_type.to_string())
        .bind(&leg.id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update strategy leg: {}", e)))?
        .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!(
                "Strategy leg '{}' not found",
                leg.id
            )));
        }

        Ok(())
    }

    /// Deletes a strategy leg.
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        let rows_affected = sqlx::query("DELETE FROM strategy_legs WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to delete strategy leg: {}", e)))?
            .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!(
                "Strategy leg '{}' not found",
                id
            )));
        }

        Ok(())
    }

    /// Deletes all legs for a strategy.
    pub async fn delete_by_strategy(&self, strategy_id: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM strategy_legs WHERE strategy_id = ?")
            .bind(strategy_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to delete strategy legs: {}", e)))?;

        Ok(())
    }
}

/// Database row representation of a strategy leg.
#[derive(Debug, sqlx::FromRow)]
struct StrategyLegRow {
    id: String,
    strategy_id: String,
    option_contract_id: String,
    quantity: i32,
    position_type: String,
    premium: f64,
    strike: f64,
    expiration: DateTime<Utc>,
    option_type: String,
}

impl From<StrategyLegRow> for StrategyLeg {
    fn from(row: StrategyLegRow) -> Self {
        StrategyLeg {
            id: row.id,
            strategy_id: row.strategy_id,
            option_contract_id: row.option_contract_id,
            quantity: row.quantity,
            position_type: match row.position_type.as_str() {
                "long" => PositionType::Long,
                "short" => PositionType::Short,
                _ => PositionType::Long, // Default fallback
            },
            premium: row.premium,
            strike: row.strike,
            expiration: row.expiration,
            option_type: match row.option_type.as_str() {
                "call" => OptionType::Call,
                "put" => OptionType::Put,
                _ => OptionType::Call, // Default fallback
            },
        }
    }
}
