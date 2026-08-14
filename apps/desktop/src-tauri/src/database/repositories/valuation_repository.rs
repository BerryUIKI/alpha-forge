// Financial repositories — daily account valuation.
//
// SQLx persistence for `daily_account_valuation` (migration 0019 + 0021). The
// derived daily valuation series is upserted by (account_id, valuation_date)
// so re-running a performance calculation never produces duplicate rows.

use chrono::Utc;
use sqlx::SqlitePool;

use crate::database::repositories::financial_support::{
    parse_date, parse_decimal, parse_timestamp,
};
use crate::error::AppError;
use domain::financial::{
    BasisStatus, DailyAccountValuation, ExternalFlowSource, UpsertValuationInput, ValuationStatus,
};

pub struct ValuationRepository {
    pool: SqlitePool,
}

impl ValuationRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn upsert(
        &self,
        input: UpsertValuationInput,
    ) -> Result<DailyAccountValuation, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO daily_account_valuation
                (id, account_id, valuation_date, account_currency, base_currency, fx_rate_to_base,
                 cash_balance, investment_market_value, total_value, cost_basis, net_contribution,
                 cash_balance_base, investment_market_value_base, total_value_base, cost_basis_base,
                 net_contribution_base, external_inflow_base, external_outflow_base,
                 performance_eligible_value_base, external_flow_source, value_status, basis_status,
                 calculated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT (account_id, valuation_date) DO UPDATE SET
                 account_currency = excluded.account_currency,
                 base_currency = excluded.base_currency,
                 fx_rate_to_base = excluded.fx_rate_to_base,
                 cash_balance = excluded.cash_balance,
                 investment_market_value = excluded.investment_market_value,
                 total_value = excluded.total_value,
                 cost_basis = excluded.cost_basis,
                 net_contribution = excluded.net_contribution,
                 cash_balance_base = excluded.cash_balance_base,
                 investment_market_value_base = excluded.investment_market_value_base,
                 total_value_base = excluded.total_value_base,
                 cost_basis_base = excluded.cost_basis_base,
                 net_contribution_base = excluded.net_contribution_base,
                 external_inflow_base = excluded.external_inflow_base,
                 external_outflow_base = excluded.external_outflow_base,
                 performance_eligible_value_base = excluded.performance_eligible_value_base,
                 external_flow_source = excluded.external_flow_source,
                 value_status = excluded.value_status,
                 basis_status = excluded.basis_status,
                 calculated_at = excluded.calculated_at",
        )
        .bind(&id)
        .bind(&input.account_id)
        .bind(input.valuation_date.to_string())
        .bind(&input.account_currency)
        .bind(&input.base_currency)
        .bind(input.fx_rate_to_base.to_string())
        .bind(input.cash_balance.to_string())
        .bind(input.investment_market_value.to_string())
        .bind(input.total_value.to_string())
        .bind(input.cost_basis.to_string())
        .bind(input.net_contribution.to_string())
        .bind(input.cash_balance_base.to_string())
        .bind(input.investment_market_value_base.to_string())
        .bind(input.total_value_base.to_string())
        .bind(input.cost_basis_base.to_string())
        .bind(input.net_contribution_base.to_string())
        .bind(input.external_inflow_base.to_string())
        .bind(input.external_outflow_base.to_string())
        .bind(input.performance_eligible_value_base.to_string())
        .bind(input.external_flow_source.to_string())
        .bind(input.value_status.to_string())
        .bind(input.basis_status.to_string())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to upsert daily valuation: {e}")))?;

        let row = sqlx::query_as::<_, ValuationRow>(
            "SELECT id, account_id, valuation_date, account_currency, base_currency, fx_rate_to_base,
                    cash_balance, investment_market_value, total_value, cost_basis, net_contribution,
                    cash_balance_base, investment_market_value_base, total_value_base, cost_basis_base,
                    net_contribution_base, external_inflow_base, external_outflow_base,
                    performance_eligible_value_base, external_flow_source, value_status, basis_status,
                    calculated_at
             FROM daily_account_valuation WHERE account_id = ? AND valuation_date = ?",
        )
        .bind(&input.account_id)
        .bind(input.valuation_date.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to read upserted valuation: {e}")))?;

        row.try_into()
    }

    pub async fn get(
        &self,
        account_id: &str,
        date: &str,
    ) -> Result<Option<DailyAccountValuation>, AppError> {
        let row = sqlx::query_as::<_, ValuationRow>(
            "SELECT id, account_id, valuation_date, account_currency, base_currency, fx_rate_to_base,
                    cash_balance, investment_market_value, total_value, cost_basis, net_contribution,
                    cash_balance_base, investment_market_value_base, total_value_base, cost_basis_base,
                    net_contribution_base, external_inflow_base, external_outflow_base,
                    performance_eligible_value_base, external_flow_source, value_status, basis_status,
                    calculated_at
             FROM daily_account_valuation WHERE account_id = ? AND valuation_date = ?",
        )
        .bind(account_id)
        .bind(date)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to get daily valuation: {e}")))?;

        row.map(TryInto::try_into).transpose()
    }

    pub async fn list_by_account(
        &self,
        account_id: &str,
    ) -> Result<Vec<DailyAccountValuation>, AppError> {
        let rows = sqlx::query_as::<_, ValuationRow>(
            "SELECT id, account_id, valuation_date, account_currency, base_currency, fx_rate_to_base,
                    cash_balance, investment_market_value, total_value, cost_basis, net_contribution,
                    cash_balance_base, investment_market_value_base, total_value_base, cost_basis_base,
                    net_contribution_base, external_inflow_base, external_outflow_base,
                    performance_eligible_value_base, external_flow_source, value_status, basis_status,
                    calculated_at
             FROM daily_account_valuation WHERE account_id = ? ORDER BY valuation_date",
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to list daily valuations: {e}")))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    pub async fn delete_for_date(&self, account_id: &str, date: &str) -> Result<(), AppError> {
        sqlx::query(
            "DELETE FROM daily_account_valuation WHERE account_id = ? AND valuation_date = ?",
        )
        .bind(account_id)
        .bind(date)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to delete daily valuation: {e}")))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct ValuationRow {
    id: String,
    account_id: String,
    valuation_date: String,
    account_currency: String,
    base_currency: String,
    fx_rate_to_base: String,
    cash_balance: String,
    investment_market_value: String,
    total_value: String,
    cost_basis: String,
    net_contribution: String,
    cash_balance_base: String,
    investment_market_value_base: String,
    total_value_base: String,
    cost_basis_base: String,
    net_contribution_base: String,
    external_inflow_base: String,
    external_outflow_base: String,
    performance_eligible_value_base: String,
    external_flow_source: String,
    value_status: String,
    basis_status: String,
    calculated_at: String,
}

impl TryFrom<ValuationRow> for DailyAccountValuation {
    type Error = AppError;

    fn try_from(row: ValuationRow) -> Result<Self, Self::Error> {
        let external_flow_source = ExternalFlowSource::parse(&row.external_flow_source)
            .ok_or_else(|| {
                AppError::Internal(format!(
                    "invalid external_flow_source in database: {}",
                    row.external_flow_source
                ))
            })?;
        let value_status = ValuationStatus::parse(&row.value_status).ok_or_else(|| {
            AppError::Internal(format!(
                "invalid value_status in database: {}",
                row.value_status
            ))
        })?;
        let basis_status = BasisStatus::parse(&row.basis_status).ok_or_else(|| {
            AppError::Internal(format!(
                "invalid basis_status in database: {}",
                row.basis_status
            ))
        })?;

        Ok(Self {
            id: row.id,
            account_id: row.account_id,
            valuation_date: parse_date(&row.valuation_date, "valuation date")?,
            account_currency: row.account_currency,
            base_currency: row.base_currency,
            fx_rate_to_base: parse_decimal(&row.fx_rate_to_base, "valuation fx rate to base")?,
            cash_balance: parse_decimal(&row.cash_balance, "valuation cash balance")?,
            investment_market_value: parse_decimal(
                &row.investment_market_value,
                "valuation investment market value",
            )?,
            total_value: parse_decimal(&row.total_value, "valuation total value")?,
            cost_basis: parse_decimal(&row.cost_basis, "valuation cost basis")?,
            net_contribution: parse_decimal(&row.net_contribution, "valuation net contribution")?,
            cash_balance_base: parse_decimal(
                &row.cash_balance_base,
                "valuation cash balance base",
            )?,
            investment_market_value_base: parse_decimal(
                &row.investment_market_value_base,
                "valuation investment market value base",
            )?,
            total_value_base: parse_decimal(&row.total_value_base, "valuation total value base")?,
            cost_basis_base: parse_decimal(&row.cost_basis_base, "valuation cost basis base")?,
            net_contribution_base: parse_decimal(
                &row.net_contribution_base,
                "valuation net contribution base",
            )?,
            external_inflow_base: parse_decimal(
                &row.external_inflow_base,
                "valuation external inflow base",
            )?,
            external_outflow_base: parse_decimal(
                &row.external_outflow_base,
                "valuation external outflow base",
            )?,
            performance_eligible_value_base: parse_decimal(
                &row.performance_eligible_value_base,
                "valuation performance eligible value base",
            )?,
            external_flow_source,
            value_status,
            basis_status,
            calculated_at: parse_timestamp(&row.calculated_at, "valuation calculation")?,
        })
    }
}
