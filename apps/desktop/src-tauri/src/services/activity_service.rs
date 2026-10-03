// Activity service — atomic and idempotent financial activity management.
//
// Manages the canonical transaction ledger (`activities`) and coordinates
// downstream tax lots (`lots`) and disposals (`lot_disposals`) within
// a single atomic SQLx database transaction.
//
// Guarantees:
// 1. Atomicity: manual buy creates activity + open lot together; manual sell
//    disposes lots and creates activity together. Any validation failure
//    (e.g., selling more shares than held) aborts the transaction cleanly
//    without creating an orphan activity or partial ledger record.
// 2. Idempotency: submissions with the same `idempotency_key` are deduplicated
//    and return the existing activity without creating duplicate lots/disposals.
// 3. Exact decimal precision: all financial quantities and cost-basis arithmetic
//    are performed using `rust_decimal::Decimal`.

use std::sync::Arc;

use chrono::Utc;
use domain::financial::{Activity, ActivityType, CostBasisMethod, CreateActivityInput};
use rust_decimal::Decimal;
use sqlx::SqlitePool;

use crate::database::repositories::account_repository::AccountRepository;
use crate::database::repositories::activity_repository::ActivityRepository;
use crate::database::repositories::asset_repository::AssetRepository;
use crate::database::repositories::lot_repository::{LotDisposalRepository, LotRepository};
use crate::error::AppError;

fn storage_money(value: Decimal) -> Decimal {
    value.round_dp(2)
}

fn json_or_null(value: &Option<serde_json::Value>) -> Option<String> {
    value.as_ref().map(|v| v.to_string())
}

pub struct ActivityService {
    pool: SqlitePool,
    activity_repo: Arc<ActivityRepository>,
    account_repo: Arc<AccountRepository>,
    asset_repo: Arc<AssetRepository>,
    #[allow(dead_code)]
    lot_repo: Arc<LotRepository>,
    #[allow(dead_code)]
    disposal_repo: Arc<LotDisposalRepository>,
}

impl ActivityService {
    pub fn new(
        pool: SqlitePool,
        activity_repo: Arc<ActivityRepository>,
        account_repo: Arc<AccountRepository>,
        asset_repo: Arc<AssetRepository>,
        lot_repo: Arc<LotRepository>,
        disposal_repo: Arc<LotDisposalRepository>,
    ) -> Self {
        Self {
            pool,
            activity_repo,
            account_repo,
            asset_repo,
            lot_repo,
            disposal_repo,
        }
    }

    /// Record a financial activity atomically and idempotently.
    ///
    /// For Buy activities, automatically generates a corresponding FIFO tax lot.
    /// For Sell activities, verifies open lot inventory and generates FIFO lot disposals.
    /// If validation or execution fails, the entire transaction is rolled back.
    pub async fn create_activity(&self, input: CreateActivityInput) -> Result<Activity, AppError> {
        // 1. Idempotency check: if key already exists, return the previously created activity.
        if let Some(ref key) = input.idempotency_key {
            let trimmed = key.trim();
            if !trimmed.is_empty() {
                if let Some(existing) = self.activity_repo.find_by_idempotency_key(trimmed).await? {
                    return Ok(existing);
                }
            }
        }

        // 2. Validate account exists.
        let account = self
            .account_repo
            .get(&input.account_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("account {} not found", input.account_id)))?;

        // 3. Validate currency.
        let currency = input.currency.trim().to_uppercase();
        if currency.is_empty() {
            return Err(AppError::Validation("currency cannot be empty".to_string()));
        }

        // 4. Validate asset & quantities by activity type.
        let fee = input.fee.unwrap_or(Decimal::ZERO);
        if fee < Decimal::ZERO {
            return Err(AppError::Validation("fee cannot be negative".to_string()));
        }

        match input.activity_type {
            ActivityType::Buy | ActivityType::Sell | ActivityType::Split => {
                let asset_id = input.asset_id.as_deref().ok_or_else(|| {
                    AppError::Validation(format!(
                        "asset_id is required for {:?} activity",
                        input.activity_type
                    ))
                })?;
                self.asset_repo
                    .get(asset_id)
                    .await?
                    .ok_or_else(|| AppError::NotFound(format!("asset {asset_id} not found")))?;
            }
            _ => {}
        }

        if input.activity_type == ActivityType::Buy {
            let qty = input.quantity.ok_or_else(|| {
                AppError::Validation("quantity is required for buy activity".to_string())
            })?;
            if qty <= Decimal::ZERO {
                return Err(AppError::Validation(
                    "quantity must be positive for buy activity".to_string(),
                ));
            }
            let price = input.unit_price.ok_or_else(|| {
                AppError::Validation("unit_price is required for buy activity".to_string())
            })?;
            if price < Decimal::ZERO {
                return Err(AppError::Validation(
                    "unit_price cannot be negative for buy activity".to_string(),
                ));
            }
        } else if input.activity_type == ActivityType::Sell {
            let qty = input.quantity.ok_or_else(|| {
                AppError::Validation("quantity is required for sell activity".to_string())
            })?;
            if qty <= Decimal::ZERO {
                return Err(AppError::Validation(
                    "quantity must be positive for sell activity".to_string(),
                ));
            }
        }

        // 5. Compute gross/net amount using Decimal without floating-point errors.
        let calculated_amount = match input.amount {
            Some(a) => a,
            None => match input.activity_type {
                ActivityType::Buy => {
                    let qty = input.quantity.unwrap_or(Decimal::ZERO);
                    let price = input.unit_price.unwrap_or(Decimal::ZERO);
                    storage_money(qty * price + fee)
                }
                ActivityType::Sell => {
                    let qty = input.quantity.unwrap_or(Decimal::ZERO);
                    let price = input.unit_price.unwrap_or(Decimal::ZERO);
                    let gross = qty * price;
                    if gross >= fee {
                        storage_money(gross - fee)
                    } else {
                        storage_money(gross)
                    }
                }
                _ => Decimal::ZERO,
            },
        };

        let fx_rate_to_base = input.fx_rate.unwrap_or(Decimal::ONE);
        let activity_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        // 6. Execute atomic transaction.
        let mut tx = self.pool.begin().await.map_err(|e| {
            AppError::Internal(format!("failed to start activity transaction: {e}"))
        })?;

        // 6a. Insert activity.
        let insert_activity_result = sqlx::query(
            "INSERT INTO activities
                (id, account_id, asset_id, activity_type, activity_type_override, source_type,
                 subtype, status, activity_date, settlement_date, quantity, unit_price, amount,
                 fee, tax, currency, fx_rate, notes, metadata, source_system, source_record_id,
                 source_group_id, idempotency_key, import_run_id, is_user_modified, needs_review,
                 created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 0, ?, ?)",
        )
        .bind(&activity_id)
        .bind(&input.account_id)
        .bind(&input.asset_id)
        .bind(input.activity_type.to_string())
        .bind(&input.activity_type_override)
        .bind(&input.source_type)
        .bind(&input.subtype)
        .bind(input.status.to_string())
        .bind(input.activity_date.to_string())
        .bind(input.settlement_date.map(|d| d.to_string()))
        .bind(input.quantity.map(|d| d.to_string()))
        .bind(input.unit_price.map(|d| d.to_string()))
        .bind(calculated_amount.to_string())
        .bind(input.fee.map(|d| d.to_string()))
        .bind(input.tax.map(|d| d.to_string()))
        .bind(&currency)
        .bind(input.fx_rate.map(|d| d.to_string()))
        .bind(&input.notes)
        .bind(json_or_null(&input.metadata))
        .bind(&input.source_system)
        .bind(&input.source_record_id)
        .bind(&input.source_group_id)
        .bind(&input.idempotency_key)
        .bind(&input.import_run_id)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&mut *tx)
        .await;

        if let Err(e) = insert_activity_result {
            // Check for concurrent idempotency collision.
            if let Some(ref key) = input.idempotency_key {
                if let Ok(Some(existing)) = self.activity_repo.find_by_idempotency_key(key).await {
                    return Ok(existing);
                }
            }
            return Err(AppError::Internal(format!(
                "failed to insert activity: {e}"
            )));
        }

        // 6b. Handle Buy -> Create Lot.
        if input.activity_type == ActivityType::Buy {
            let asset_id_str = input.asset_id.as_ref().unwrap();
            let qty = input.quantity.unwrap();
            let price = input.unit_price.unwrap();
            let original_cost_basis = storage_money(qty * price + fee);
            let lot_id = uuid::Uuid::new_v4().to_string();

            sqlx::query(
                "INSERT INTO lots
                    (id, account_id, asset_id, open_date, open_activity_id, original_quantity,
                     cost_per_unit, original_cost_basis, remaining_cost_basis, fee_allocated,
                     tax_allocated, currency, base_currency, fx_rate_to_base, fx_rate_to_account,
                     account_currency, cost_basis_method, remaining_quantity, split_ratio, is_closed,
                     created_at, updated_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'FIFO', ?, '1', 0, ?, ?)",
            )
            .bind(&lot_id)
            .bind(&input.account_id)
            .bind(asset_id_str)
            .bind(input.activity_date.to_string())
            .bind(&activity_id)
            .bind(qty.to_string())
            .bind(price.to_string())
            .bind(original_cost_basis.to_string())
            .bind(original_cost_basis.to_string())
            .bind(fee.to_string())
            .bind(Decimal::ZERO.to_string())
            .bind(&currency)
            .bind(&account.currency)
            .bind(fx_rate_to_base.to_string())
            .bind(None::<String>)
            .bind(Some(&account.currency))
            .bind(qty.to_string())
            .bind(now.to_rfc3339())
            .bind(now.to_rfc3339())
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(format!("failed to create buy lot: {e}")))?;
        }

        // 6c. Handle Sell -> FIFO Lot Consumption & Disposal.
        if input.activity_type == ActivityType::Sell {
            let asset_id_str = input.asset_id.as_ref().unwrap();
            let sell_quantity = input.quantity.unwrap();
            let total_proceeds = calculated_amount;

            #[derive(sqlx::FromRow)]
            struct OpenLotRow {
                id: String,
                remaining_quantity: String,
                remaining_cost_basis: String,
                currency: String,
                base_currency: String,
                fx_rate_to_base: String,
                cost_basis_method: String,
            }

            let open_lot_rows = sqlx::query_as::<_, OpenLotRow>(
                "SELECT id, remaining_quantity, remaining_cost_basis, currency, base_currency,
                        fx_rate_to_base, cost_basis_method
                 FROM lots
                 WHERE account_id = ? AND asset_id = ? AND is_closed = 0
                 ORDER BY open_date ASC, created_at ASC",
            )
            .bind(&input.account_id)
            .bind(asset_id_str)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(format!("failed to fetch open lots for sell: {e}")))?;

            if open_lot_rows.is_empty() {
                return Err(AppError::Validation(format!(
                    "no open lots to sell for account {} asset {}",
                    input.account_id, asset_id_str
                )));
            }

            let mut parsed_lots = Vec::new();
            let mut total_available = Decimal::ZERO;
            for row in open_lot_rows {
                let rem_qty = Decimal::from_str_exact(&row.remaining_quantity)
                    .map_err(|e| AppError::Internal(format!("invalid lot quantity: {e}")))?;
                let rem_cost = Decimal::from_str_exact(&row.remaining_cost_basis)
                    .map_err(|e| AppError::Internal(format!("invalid lot cost basis: {e}")))?;
                let fx = Decimal::from_str_exact(&row.fx_rate_to_base)
                    .map_err(|e| AppError::Internal(format!("invalid lot fx rate: {e}")))?;
                let method =
                    CostBasisMethod::parse(&row.cost_basis_method).unwrap_or(CostBasisMethod::Fifo);
                total_available += rem_qty;
                parsed_lots.push((
                    row.id,
                    rem_qty,
                    rem_cost,
                    row.currency,
                    row.base_currency,
                    fx,
                    method,
                ));
            }

            if sell_quantity > total_available {
                return Err(AppError::Validation(format!(
                    "sell quantity {sell_quantity} exceeds available {total_available}"
                )));
            }

            let mut remaining_to_sell = sell_quantity;
            for (lot_id, rem_qty, rem_cost, lot_currency, base_currency, fx_rate, method) in
                parsed_lots
            {
                if remaining_to_sell.is_zero() {
                    break;
                }

                let effective_quantity = if rem_qty <= remaining_to_sell {
                    rem_qty
                } else {
                    remaining_to_sell
                };

                let proceeds = if total_proceeds.is_zero() || sell_quantity.is_zero() {
                    Decimal::ZERO
                } else {
                    storage_money(total_proceeds * effective_quantity / sell_quantity)
                };
                let cost_basis = storage_money(rem_cost * effective_quantity / rem_qty);
                let realized_pnl = storage_money(proceeds - cost_basis);
                let proceeds_base = storage_money(fx_rate * proceeds);
                let cost_basis_base = storage_money(fx_rate * cost_basis);
                let realized_pnl_base = storage_money(proceeds_base - cost_basis_base);

                let disposal_id = uuid::Uuid::new_v4().to_string();
                sqlx::query(
                    "INSERT INTO lot_disposals
                        (id, lot_id, account_id, asset_id, disposal_activity_id, disposal_date,
                         quantity, proceeds, cost_basis, realized_pnl, proceeds_base, cost_basis_base,
                         realized_pnl_base, currency, base_currency, fx_rate_to_base, cost_basis_method,
                         created_at)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                )
                .bind(&disposal_id)
                .bind(&lot_id)
                .bind(&input.account_id)
                .bind(asset_id_str)
                .bind(&activity_id)
                .bind(input.activity_date.to_string())
                .bind(effective_quantity.to_string())
                .bind(proceeds.to_string())
                .bind(cost_basis.to_string())
                .bind(realized_pnl.to_string())
                .bind(proceeds_base.to_string())
                .bind(cost_basis_base.to_string())
                .bind(realized_pnl_base.to_string())
                .bind(&lot_currency)
                .bind(&base_currency)
                .bind(fx_rate.to_string())
                .bind(method.to_string())
                .bind(now.to_rfc3339())
                .execute(&mut *tx)
                .await
                .map_err(|e| AppError::Internal(format!("failed to record lot disposal: {e}")))?;

                let new_rem_qty = rem_qty - effective_quantity;
                let new_rem_cost = rem_cost - cost_basis;
                let is_closed = new_rem_qty.is_zero();

                sqlx::query(
                    "UPDATE lots
                     SET remaining_quantity = ?, remaining_cost_basis = ?, is_closed = ?,
                         close_date = ?, close_activity_id = ?, updated_at = ?
                     WHERE id = ?",
                )
                .bind(new_rem_qty.to_string())
                .bind(new_rem_cost.to_string())
                .bind(if is_closed { 1 } else { 0 })
                .bind(if is_closed {
                    Some(input.activity_date.to_string())
                } else {
                    None
                })
                .bind(if is_closed { Some(&activity_id) } else { None })
                .bind(now.to_rfc3339())
                .bind(&lot_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| AppError::Internal(format!("failed to update lot state: {e}")))?;

                remaining_to_sell -= effective_quantity;
            }
        }

        // 6d. Commit transaction atomically.
        tx.commit().await.map_err(|e| {
            AppError::Internal(format!("failed to commit activity transaction: {e}"))
        })?;

        // 7. Return created activity.
        self.activity_repo
            .get(&activity_id)
            .await?
            .ok_or_else(|| AppError::Internal("failed to fetch created activity".to_string()))
    }
}
