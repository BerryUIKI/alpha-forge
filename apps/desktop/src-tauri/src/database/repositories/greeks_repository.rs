// Greeks repository — handles greeks table operations.

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::error::AppError;
use domain::option::{Greeks, PricingModel};

/// Repository for accessing Greeks calculations.
pub struct GreeksRepository {
    pool: SqlitePool,
}

impl GreeksRepository {
    /// Creates a new Greeks repository.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Creates new Greeks for an option contract.
    pub async fn create(&self, greeks: &Greeks) -> Result<(), AppError> {
        sqlx::query(
            r#"
                INSERT INTO greeks (
                    id, option_contract_id, delta, gamma, theta,
                    vega, rho, iv, calculated_at, calculation_model
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&greeks.id)
        .bind(&greeks.option_contract_id)
        .bind(greeks.delta)
        .bind(greeks.gamma)
        .bind(greeks.theta)
        .bind(greeks.vega)
        .bind(greeks.rho)
        .bind(greeks.iv)
        .bind(greeks.calculated_at)
        .bind(greeks.calculation_model.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create Greeks: {}", e)))?;

        Ok(())
    }

    /// Gets the latest Greeks for an option contract.
    pub async fn get_latest(&self, option_contract_id: &str) -> Result<Option<Greeks>, AppError> {
        let row = sqlx::query_as::<_, GreeksRow>(
            r#"
                SELECT id, option_contract_id, delta, gamma, theta,
                       vega, rho, iv, calculated_at, calculation_model
                FROM greeks 
                WHERE option_contract_id = ? 
                ORDER BY calculated_at DESC 
                LIMIT 1
            "#,
        )
        .bind(option_contract_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to get Greeks: {}", e)))?;

        Ok(row.map(|r| r.into()))
    }

    /// Lists all Greeks for an option contract (historical).
    pub async fn list_by_contract(
        &self,
        option_contract_id: &str,
    ) -> Result<Vec<Greeks>, AppError> {
        let rows = sqlx::query_as::<_, GreeksRow>(
            r#"
                SELECT id, option_contract_id, delta, gamma, theta,
                       vega, rho, iv, calculated_at, calculation_model
                FROM greeks 
                WHERE option_contract_id = ? 
                ORDER BY calculated_at DESC
            "#,
        )
        .bind(option_contract_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list Greeks: {}", e)))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    /// Deletes all Greeks for an option contract.
    pub async fn delete_by_contract(&self, option_contract_id: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM greeks WHERE option_contract_id = ?")
            .bind(option_contract_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to delete Greeks: {}", e)))?;

        Ok(())
    }
}

/// Database row representation of Greeks.
#[derive(Debug, sqlx::FromRow)]
struct GreeksRow {
    id: String,
    option_contract_id: String,
    delta: f64,
    gamma: f64,
    theta: f64,
    vega: f64,
    rho: f64,
    iv: f64,
    calculated_at: DateTime<Utc>,
    calculation_model: String,
}

impl From<GreeksRow> for Greeks {
    fn from(row: GreeksRow) -> Self {
        Greeks {
            id: row.id,
            option_contract_id: row.option_contract_id,
            delta: row.delta,
            gamma: row.gamma,
            theta: row.theta,
            vega: row.vega,
            rho: row.rho,
            iv: row.iv,
            calculated_at: row.calculated_at,
            calculation_model: match row.calculation_model.as_str() {
                "black_scholes" => PricingModel::BlackScholes,
                "binomial" => PricingModel::Binomial,
                "finite_difference" => PricingModel::FiniteDifference,
                _ => PricingModel::BlackScholes,
            },
        }
    }
}
