// Option strategy repository — handles option_strategies table operations.

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::error::AppError;
use domain::option::{OptionStrategy, StrategyLeg, StrategyType};

/// Repository for accessing option strategies.
pub struct OptionStrategyRepository {
    pool: SqlitePool,
}

impl OptionStrategyRepository {
    /// Creates a new option strategy repository.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Creates a new option strategy.
    pub async fn create(&self, strategy: &OptionStrategy) -> Result<(), AppError> {
        let break_even_json = serde_json::to_string(&strategy.break_even_points).map_err(|e| {
            AppError::Internal(format!("Failed to serialize break-even points: {}", e))
        })?;

        sqlx::query(
            r#"
                INSERT INTO option_strategies (
                    id, workspace_id, name, strategy_type, underlying,
                    total_cost, max_profit, max_loss, break_even_points,
                    created_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&strategy.id)
        .bind(&strategy.workspace_id)
        .bind(&strategy.name)
        .bind(strategy.strategy_type.to_string())
        .bind(&strategy.underlying)
        .bind(strategy.total_cost)
        .bind(strategy.max_profit)
        .bind(strategy.max_loss)
        .bind(break_even_json)
        .bind(strategy.created_at)
        .bind(strategy.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create option strategy: {}", e)))?;

        Ok(())
    }

    /// Persists a strategy and all of its legs in one transaction.
    pub async fn create_with_legs(
        &self,
        strategy: &OptionStrategy,
        legs: &[StrategyLeg],
    ) -> Result<(), AppError> {
        if legs.iter().any(|leg| leg.strategy_id != strategy.id) {
            return Err(AppError::Validation(
                "Strategy leg does not belong to the strategy".to_string(),
            ));
        }

        let break_even_json = serde_json::to_string(&strategy.break_even_points).map_err(|e| {
            AppError::Internal(format!("Failed to serialize break-even points: {}", e))
        })?;
        let mut transaction = self.pool.begin().await.map_err(|e| {
            AppError::Internal(format!("Failed to start strategy transaction: {}", e))
        })?;

        sqlx::query(
            r#"
                INSERT INTO option_strategies (
                    id, workspace_id, name, strategy_type, underlying,
                    total_cost, max_profit, max_loss, break_even_points,
                    created_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&strategy.id)
        .bind(&strategy.workspace_id)
        .bind(&strategy.name)
        .bind(strategy.strategy_type.to_string())
        .bind(&strategy.underlying)
        .bind(strategy.total_cost)
        .bind(strategy.max_profit)
        .bind(strategy.max_loss)
        .bind(break_even_json)
        .bind(strategy.created_at)
        .bind(strategy.updated_at)
        .execute(&mut *transaction)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create option strategy: {}", e)))?;

        for leg in legs {
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
            .execute(&mut *transaction)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to create strategy leg: {}", e)))?;
        }

        transaction.commit().await.map_err(|e| {
            AppError::Internal(format!("Failed to commit strategy transaction: {}", e))
        })?;

        Ok(())
    }

    /// Gets an option strategy by ID.
    pub async fn get(&self, id: &str) -> Result<Option<OptionStrategy>, AppError> {
        let row = sqlx::query_as::<_, OptionStrategyRow>(
            r#"
                SELECT id, workspace_id, name, strategy_type, underlying,
                       total_cost, max_profit, max_loss, break_even_points,
                       created_at, updated_at
                FROM option_strategies 
                WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            AppError::Internal(format!("Failed to get option strategy '{}': {}", id, e))
        })?;

        Ok(row.map(|r| r.into()))
    }

    /// Lists all option strategies for a workspace.
    pub async fn list_by_workspace(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<OptionStrategy>, AppError> {
        let rows = sqlx::query_as::<_, OptionStrategyRow>(
            r#"
                SELECT id, workspace_id, name, strategy_type, underlying,
                       total_cost, max_profit, max_loss, break_even_points,
                       created_at, updated_at
                FROM option_strategies 
                WHERE workspace_id = ? 
                ORDER BY created_at DESC
            "#,
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list option strategies: {}", e)))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    /// Updates an option strategy.
    pub async fn update(&self, strategy: &OptionStrategy) -> Result<(), AppError> {
        let break_even_json = serde_json::to_string(&strategy.break_even_points).map_err(|e| {
            AppError::Internal(format!("Failed to serialize break-even points: {}", e))
        })?;

        let rows_affected = sqlx::query(
            r#"
                UPDATE option_strategies 
                SET name = ?, total_cost = ?, max_profit = ?, max_loss = ?,
                    break_even_points = ?, updated_at = ?
                WHERE id = ?
            "#,
        )
        .bind(&strategy.name)
        .bind(strategy.total_cost)
        .bind(strategy.max_profit)
        .bind(strategy.max_loss)
        .bind(break_even_json)
        .bind(Utc::now())
        .bind(&strategy.id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update option strategy: {}", e)))?
        .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!(
                "Option strategy '{}' not found",
                strategy.id
            )));
        }

        Ok(())
    }

    /// Deletes an option strategy.
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        let rows_affected = sqlx::query("DELETE FROM option_strategies WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to delete option strategy: {}", e)))?
            .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!(
                "Option strategy '{}' not found",
                id
            )));
        }

        Ok(())
    }
}

/// Database row representation of an option strategy.
#[derive(Debug, sqlx::FromRow)]
struct OptionStrategyRow {
    id: String,
    workspace_id: String,
    name: String,
    strategy_type: String,
    underlying: String,
    total_cost: f64,
    max_profit: Option<f64>,
    max_loss: Option<f64>,
    break_even_points: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<OptionStrategyRow> for OptionStrategy {
    fn from(row: OptionStrategyRow) -> Self {
        let break_even_points: Vec<f64> =
            serde_json::from_str(&row.break_even_points).unwrap_or_else(|_| Vec::new());

        OptionStrategy {
            id: row.id,
            workspace_id: row.workspace_id,
            name: row.name,
            strategy_type: match row.strategy_type.as_str() {
                "long_call" => StrategyType::LongCall,
                "long_put" => StrategyType::LongPut,
                "covered_call" => StrategyType::CoveredCall,
                "protective_put" => StrategyType::ProtectivePut,
                "bull_call_spread" => StrategyType::BullCallSpread,
                "bear_put_spread" => StrategyType::BearPutSpread,
                "straddle" => StrategyType::Straddle,
                "strangle" => StrategyType::Strangle,
                "iron_condor" => StrategyType::IronCondor,
                "butterfly" => StrategyType::Butterfly,
                _ => StrategyType::Custom,
            },
            underlying: row.underlying,
            total_cost: row.total_cost,
            max_profit: row.max_profit,
            max_loss: row.max_loss,
            break_even_points,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
