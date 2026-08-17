// Lot service — FIFO cost-basis tracking and realized-gain computation.
//
// When a sell activity is recorded, this service:
// 1. Loads open lots for (account, asset) in FIFO order.
// 2. Consumes lots from oldest to newest until the sell quantity is met.
// 3. Creates LotDisposal records with pro-rata proceeds and realized PnL.
// 4. Updates or closes the consumed lots.

use std::sync::Arc;

use domain::financial::{ActivityType, CreateLotDisposalInput, FifoReductionResult, Lot};
use rust_decimal::Decimal;

use crate::database::repositories::activity_repository::ActivityRepository;
use crate::database::repositories::lot_repository::{LotDisposalRepository, LotRepository};
use crate::error::AppError;

/// Helper: round a monetary amount for storage (2 decimal places = cents).
/// Matches Wealthfolio's `storage_money` convention.
fn storage_money(value: Decimal) -> Decimal {
    value.round_dp(2)
}

pub struct LotService {
    lot_repo: Arc<LotRepository>,
    disposal_repo: Arc<LotDisposalRepository>,
    activity_repo: Arc<ActivityRepository>,
}

impl LotService {
    pub fn new(
        lot_repo: Arc<LotRepository>,
        disposal_repo: Arc<LotDisposalRepository>,
        activity_repo: Arc<ActivityRepository>,
    ) -> Self {
        Self {
            lot_repo,
            disposal_repo,
            activity_repo,
        }
    }

    /// Record a sell activity against the FIFO lot inventory and return the
    /// realized P&L breakdown.
    pub async fn record_sell(
        &self,
        account_id: &str,
        asset_id: &str,
        activity_id: &str,
    ) -> Result<FifoReductionResult, AppError> {
        let activity = self
            .activity_repo
            .get(activity_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("activity {activity_id} not found")))?;

        if activity.activity_type != ActivityType::Sell {
            return Err(AppError::Validation(format!(
                "activity {activity_id} is not a sell (type: {:?})",
                activity.activity_type
            )));
        }

        let sell_quantity = activity
            .quantity
            .ok_or_else(|| AppError::Validation("sell activity has no quantity".to_string()))?;

        let total_proceeds = activity
            .amount
            .ok_or_else(|| AppError::Validation("sell activity has no amount".to_string()))?;

        let open_lots = self
            .lot_repo
            .list_open_by_account_asset(account_id, asset_id)
            .await?;

        if open_lots.is_empty() {
            return Err(AppError::Validation(format!(
                "no open lots to sell for account {account_id} asset {asset_id}"
            )));
        }

        let total_available: Decimal = open_lots.iter().map(|l| l.remaining_quantity).sum();
        if sell_quantity > total_available {
            return Err(AppError::Validation(format!(
                "sell quantity {sell_quantity} exceeds available {total_available}"
            )));
        }

        let mut remaining_to_sell = sell_quantity;
        let mut lots_consumed = 0u32;
        let mut lots_partially_consumed = 0u32;
        let mut total_cost_basis = Decimal::ZERO;
        let mut total_cost_basis_base = Decimal::ZERO;

        for lot in &open_lots {
            if remaining_to_sell.is_zero() {
                break;
            }

            let effective_quantity = if lot.remaining_quantity <= remaining_to_sell {
                lot.remaining_quantity
            } else {
                remaining_to_sell
            };

            let proceeds = if total_proceeds.is_zero() || sell_quantity.is_zero() {
                Decimal::ZERO
            } else {
                storage_money(total_proceeds * effective_quantity / sell_quantity)
            };
            let cost_basis = storage_money(
                lot.remaining_cost_basis * effective_quantity / lot.remaining_quantity,
            );
            let realized_pnl = storage_money(proceeds - cost_basis);
            let cost_basis_base = storage_money(lot.fx_rate_to_base * cost_basis);
            let proceeds_base = storage_money(lot.fx_rate_to_base * proceeds);
            let realized_pnl_base = storage_money(proceeds_base - cost_basis_base);

            total_cost_basis += cost_basis;
            total_cost_basis_base += cost_basis_base;

            // Record the disposal
            self.disposal_repo
                .create(CreateLotDisposalInput {
                    lot_id: lot.id.clone(),
                    account_id: account_id.to_string(),
                    asset_id: asset_id.to_string(),
                    disposal_activity_id: activity_id.to_string(),
                    disposal_date: activity.activity_date,
                    quantity: effective_quantity,
                    proceeds,
                    cost_basis,
                    realized_pnl,
                    proceeds_base,
                    cost_basis_base,
                    realized_pnl_base,
                    currency: lot.currency.clone(),
                    base_currency: lot.base_currency.clone(),
                    fx_rate_to_base: lot.fx_rate_to_base,
                    cost_basis_method: lot.cost_basis_method,
                })
                .await?;

            // Update lot remaining
            let new_remaining_qty = lot.remaining_quantity - effective_quantity;
            let new_remaining_cost_basis = lot.remaining_cost_basis - cost_basis;
            let is_closed = new_remaining_qty.is_zero();

            self.lot_repo
                .update_lot_state(
                    &lot.id,
                    new_remaining_qty,
                    new_remaining_cost_basis,
                    is_closed,
                    if is_closed {
                        Some(activity.activity_date)
                    } else {
                        None
                    },
                    if is_closed {
                        Some(activity_id.to_string())
                    } else {
                        None
                    },
                )
                .await?;

            if is_closed {
                lots_consumed += 1;
            } else {
                lots_partially_consumed += 1;
            }

            remaining_to_sell -= effective_quantity;
        }

        let total_proceeds_rounded = storage_money(total_proceeds);

        Ok(FifoReductionResult {
            account_id: account_id.to_string(),
            asset_id: asset_id.to_string(),
            disposal_date: activity.activity_date,
            total_quantity: sell_quantity,
            total_proceeds: total_proceeds_rounded,
            total_cost_basis: storage_money(total_cost_basis),
            total_realized_pnl: storage_money(total_proceeds_rounded - total_cost_basis),
            total_proceeds_base: total_proceeds_rounded, // simplified
            total_cost_basis_base: storage_money(total_cost_basis_base),
            total_realized_pnl_base: storage_money(total_proceeds_rounded - total_cost_basis_base),
            lots_consumed,
            lots_partially_consumed,
        })
    }

    /// Get open lots for an account+asset combination (FIFO order).
    pub async fn get_open_lots(
        &self,
        account_id: &str,
        asset_id: &str,
    ) -> Result<Vec<Lot>, AppError> {
        self.lot_repo
            .list_open_by_account_asset(account_id, asset_id)
            .await
    }

    /// Get all open lots for an account.
    pub async fn get_open_lots_for_account(&self, account_id: &str) -> Result<Vec<Lot>, AppError> {
        self.lot_repo.list_open_by_account(account_id).await
    }
}
