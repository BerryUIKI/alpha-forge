// Financial repositories — tax lots + lot disposals.
//
// SQLx persistence for `lots` and `lot_disposals` (migration 0018). Lots are
// the FIFO cost-basis inventory; disposals record realized PnL per sell.

use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use sqlx::SqlitePool;

use crate::database::repositories::financial_support::{
    parse_date, parse_decimal, parse_optional_date, parse_optional_decimal, parse_timestamp,
};
use crate::error::AppError;
use domain::financial::{
    CostBasisMethod, CreateLotDisposalInput, CreateLotInput, Lot, LotDisposal,
};

pub struct LotRepository {
    pool: SqlitePool,
}

impl LotRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateLotInput) -> Result<Lot, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let original_cost_basis = input.original_cost_basis;

        sqlx::query(
            "INSERT INTO lots
                (id, account_id, asset_id, open_date, open_activity_id, original_quantity,
                 cost_per_unit, original_cost_basis, remaining_cost_basis, fee_allocated,
                 tax_allocated, currency, base_currency, fx_rate_to_base, fx_rate_to_account,
                 account_currency, cost_basis_method, remaining_quantity, split_ratio, is_closed,
                 created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, '1', 0, ?, ?)",
        )
        .bind(&id)
        .bind(&input.account_id)
        .bind(&input.asset_id)
        .bind(input.open_date.to_string())
        .bind(&input.open_activity_id)
        .bind(input.original_quantity.to_string())
        .bind(input.cost_per_unit.to_string())
        .bind(original_cost_basis.to_string())
        .bind(original_cost_basis.to_string())
        .bind(input.fee_allocated.to_string())
        .bind(Decimal::ZERO.to_string())
        .bind(&input.currency)
        .bind(&input.base_currency)
        .bind(input.fx_rate_to_base.to_string())
        .bind(input.fx_rate_to_account.map(|d| d.to_string()))
        .bind(&input.account_currency)
        .bind(input.cost_basis_method.to_string())
        .bind(input.original_quantity.to_string())
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to create lot: {e}")))?;

        let row = sqlx::query_as::<_, LotRow>(
            "SELECT id, account_id, asset_id, open_date, open_activity_id, original_quantity,
                    cost_per_unit, original_cost_basis, remaining_cost_basis, fee_allocated,
                    tax_allocated, currency, base_currency, fx_rate_to_base, fx_rate_to_account,
                    account_currency, cost_basis_method, remaining_quantity, split_ratio, is_closed,
                    close_date, close_activity_id, created_at, updated_at
             FROM lots WHERE id = ?",
        )
        .bind(&id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to read created lot: {e}")))?;

        row.try_into()
    }

    pub async fn get(&self, id: &str) -> Result<Option<Lot>, AppError> {
        let row = sqlx::query_as::<_, LotRow>(
            "SELECT id, account_id, asset_id, open_date, open_activity_id, original_quantity,
                    cost_per_unit, original_cost_basis, remaining_cost_basis, fee_allocated,
                    tax_allocated, currency, base_currency, fx_rate_to_base, fx_rate_to_account,
                    account_currency, cost_basis_method, remaining_quantity, split_ratio, is_closed,
                    close_date, close_activity_id, created_at, updated_at
             FROM lots WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to get lot: {e}")))?;

        row.map(TryInto::try_into).transpose()
    }

    pub async fn list_open_by_account_asset(
        &self,
        account_id: &str,
        asset_id: &str,
    ) -> Result<Vec<Lot>, AppError> {
        let rows = sqlx::query_as::<_, LotRow>(
            "SELECT id, account_id, asset_id, open_date, open_activity_id, original_quantity,
                    cost_per_unit, original_cost_basis, remaining_cost_basis, fee_allocated,
                    tax_allocated, currency, base_currency, fx_rate_to_base, fx_rate_to_account,
                    account_currency, cost_basis_method, remaining_quantity, split_ratio, is_closed,
                    close_date, close_activity_id, created_at, updated_at
             FROM lots
             WHERE account_id = ? AND asset_id = ? AND is_closed = 0
             ORDER BY open_date, created_at",
        )
        .bind(account_id)
        .bind(asset_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to list open lots: {e}")))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    pub async fn list_open_by_account(&self, account_id: &str) -> Result<Vec<Lot>, AppError> {
        let rows = sqlx::query_as::<_, LotRow>(
            "SELECT id, account_id, asset_id, open_date, open_activity_id, original_quantity,
                    cost_per_unit, original_cost_basis, remaining_cost_basis, fee_allocated,
                    tax_allocated, currency, base_currency, fx_rate_to_base, fx_rate_to_account,
                    account_currency, cost_basis_method, remaining_quantity, split_ratio, is_closed,
                    close_date, close_activity_id, created_at, updated_at
             FROM lots WHERE account_id = ? AND is_closed = 0 ORDER BY open_date, created_at",
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to list account open lots: {e}")))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    /// Update the remaining quantity and cost basis of a lot after a partial sale.
    /// Marks the lot as closed when `remaining_quantity` reaches zero.
    pub async fn update_lot_state(
        &self,
        lot_id: &str,
        remaining_quantity: Decimal,
        remaining_cost_basis: Decimal,
        is_closed: bool,
        close_date: Option<NaiveDate>,
        close_activity_id: Option<String>,
    ) -> Result<(), AppError> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE lots
             SET remaining_quantity = ?, remaining_cost_basis = ?, is_closed = ?,
                 close_date = ?, close_activity_id = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(remaining_quantity.to_string())
        .bind(remaining_cost_basis.to_string())
        .bind(is_closed)
        .bind(close_date.map(|d| d.to_string()))
        .bind(&close_activity_id)
        .bind(now.to_rfc3339())
        .bind(lot_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to update lot state: {e}")))?;
        Ok(())
    }
}

pub struct LotDisposalRepository {
    pool: SqlitePool,
}

impl LotDisposalRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateLotDisposalInput) -> Result<LotDisposal, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO lot_disposals
                (id, lot_id, account_id, asset_id, disposal_activity_id, disposal_date, quantity,
                 proceeds, cost_basis, realized_pnl, proceeds_base, cost_basis_base,
                 realized_pnl_base, currency, base_currency, fx_rate_to_base, cost_basis_method,
                 created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&input.lot_id)
        .bind(&input.account_id)
        .bind(&input.asset_id)
        .bind(&input.disposal_activity_id)
        .bind(input.disposal_date.to_string())
        .bind(input.quantity.to_string())
        .bind(input.proceeds.to_string())
        .bind(input.cost_basis.to_string())
        .bind(input.realized_pnl.to_string())
        .bind(input.proceeds_base.to_string())
        .bind(input.cost_basis_base.to_string())
        .bind(input.realized_pnl_base.to_string())
        .bind(&input.currency)
        .bind(&input.base_currency)
        .bind(input.fx_rate_to_base.to_string())
        .bind(input.cost_basis_method.to_string())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to create lot disposal: {e}")))?;

        let row = sqlx::query_as::<_, LotDisposalRow>(
            "SELECT id, lot_id, account_id, asset_id, disposal_activity_id, disposal_date, quantity,
                    proceeds, cost_basis, realized_pnl, proceeds_base, cost_basis_base,
                    realized_pnl_base, currency, base_currency, fx_rate_to_base, cost_basis_method,
                    created_at
             FROM lot_disposals WHERE id = ?",
        )
        .bind(&id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to read created disposal: {e}")))?;

        row.try_into()
    }

    pub async fn list_by_account(&self, account_id: &str) -> Result<Vec<LotDisposal>, AppError> {
        let rows = sqlx::query_as::<_, LotDisposalRow>(
            "SELECT id, lot_id, account_id, asset_id, disposal_activity_id, disposal_date, quantity,
                    proceeds, cost_basis, realized_pnl, proceeds_base, cost_basis_base,
                    realized_pnl_base, currency, base_currency, fx_rate_to_base, cost_basis_method,
                    created_at
             FROM lot_disposals WHERE account_id = ? ORDER BY disposal_date, created_at",
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to list lot disposals: {e}")))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }
}

#[derive(sqlx::FromRow)]
struct LotRow {
    id: String,
    account_id: String,
    asset_id: String,
    open_date: String,
    open_activity_id: Option<String>,
    original_quantity: String,
    cost_per_unit: String,
    original_cost_basis: String,
    remaining_cost_basis: String,
    fee_allocated: String,
    tax_allocated: String,
    currency: String,
    base_currency: String,
    fx_rate_to_base: String,
    fx_rate_to_account: Option<String>,
    account_currency: Option<String>,
    cost_basis_method: String,
    remaining_quantity: String,
    split_ratio: String,
    is_closed: bool,
    close_date: Option<String>,
    close_activity_id: Option<String>,
    created_at: String,
    updated_at: String,
}

impl TryFrom<LotRow> for Lot {
    type Error = AppError;

    fn try_from(row: LotRow) -> Result<Self, Self::Error> {
        let cost_basis_method =
            CostBasisMethod::parse(&row.cost_basis_method).ok_or_else(|| {
                AppError::Internal(format!(
                    "invalid cost_basis_method in database: {}",
                    row.cost_basis_method
                ))
            })?;

        Ok(Self {
            id: row.id,
            account_id: row.account_id,
            asset_id: row.asset_id,
            open_date: parse_date(&row.open_date, "lot open date")?,
            open_activity_id: row.open_activity_id,
            original_quantity: parse_decimal(&row.original_quantity, "lot original quantity")?,
            cost_per_unit: parse_decimal(&row.cost_per_unit, "lot cost per unit")?,
            original_cost_basis: parse_decimal(
                &row.original_cost_basis,
                "lot original cost basis",
            )?,
            remaining_cost_basis: parse_decimal(
                &row.remaining_cost_basis,
                "lot remaining cost basis",
            )?,
            fee_allocated: parse_decimal(&row.fee_allocated, "lot fee allocated")?,
            tax_allocated: parse_decimal(&row.tax_allocated, "lot tax allocated")?,
            currency: row.currency,
            base_currency: row.base_currency,
            fx_rate_to_base: parse_decimal(&row.fx_rate_to_base, "lot fx rate to base")?,
            fx_rate_to_account: parse_optional_decimal(
                row.fx_rate_to_account,
                "lot fx rate to account",
            )?,
            account_currency: row.account_currency,
            cost_basis_method,
            remaining_quantity: parse_decimal(&row.remaining_quantity, "lot remaining quantity")?,
            split_ratio: parse_decimal(&row.split_ratio, "lot split ratio")?,
            is_closed: row.is_closed,
            close_date: parse_optional_date(row.close_date, "lot close date")?,
            close_activity_id: row.close_activity_id,
            created_at: parse_timestamp(&row.created_at, "lot creation")?,
            updated_at: parse_timestamp(&row.updated_at, "lot update")?,
        })
    }
}

#[derive(sqlx::FromRow)]
struct LotDisposalRow {
    id: String,
    lot_id: String,
    account_id: String,
    asset_id: String,
    disposal_activity_id: String,
    disposal_date: String,
    quantity: String,
    proceeds: String,
    cost_basis: String,
    realized_pnl: String,
    proceeds_base: String,
    cost_basis_base: String,
    realized_pnl_base: String,
    currency: String,
    base_currency: String,
    fx_rate_to_base: String,
    cost_basis_method: String,
    created_at: String,
}

impl TryFrom<LotDisposalRow> for LotDisposal {
    type Error = AppError;

    fn try_from(row: LotDisposalRow) -> Result<Self, Self::Error> {
        let cost_basis_method =
            CostBasisMethod::parse(&row.cost_basis_method).ok_or_else(|| {
                AppError::Internal(format!(
                    "invalid cost_basis_method in database: {}",
                    row.cost_basis_method
                ))
            })?;

        Ok(Self {
            id: row.id,
            lot_id: row.lot_id,
            account_id: row.account_id,
            asset_id: row.asset_id,
            disposal_activity_id: row.disposal_activity_id,
            disposal_date: parse_date(&row.disposal_date, "disposal date")?,
            quantity: parse_decimal(&row.quantity, "disposal quantity")?,
            proceeds: parse_decimal(&row.proceeds, "disposal proceeds")?,
            cost_basis: parse_decimal(&row.cost_basis, "disposal cost basis")?,
            realized_pnl: parse_decimal(&row.realized_pnl, "disposal realized pnl")?,
            proceeds_base: parse_decimal(&row.proceeds_base, "disposal proceeds base")?,
            cost_basis_base: parse_decimal(&row.cost_basis_base, "disposal cost basis base")?,
            realized_pnl_base: parse_decimal(&row.realized_pnl_base, "disposal realized pnl base")?,
            currency: row.currency,
            base_currency: row.base_currency,
            fx_rate_to_base: parse_decimal(&row.fx_rate_to_base, "disposal fx rate to base")?,
            cost_basis_method,
            created_at: parse_timestamp(&row.created_at, "disposal creation")?,
        })
    }
}
