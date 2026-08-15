// Snapshot service — capture point-in-time holdings as snapshots.
//
// A snapshot freezes the current holdings of an account into a set of
// positions that can be compared over time or used for reporting.

use std::sync::Arc;

use chrono::NaiveDate;
use domain::financial::{
    CreateSnapshotInput, HoldingSnapshot, HoldingSnapshotSource, SnapshotPositionInput,
};
use rust_decimal::Decimal;

use crate::database::repositories::account_repository::AccountRepository;
use crate::database::repositories::snapshot_repository::SnapshotRepository;
use crate::error::AppError;

use super::holdings_service::HoldingsService;

pub struct SnapshotService {
    snapshot_repo: Arc<SnapshotRepository>,
    account_repo: Arc<AccountRepository>,
    holdings_service: Arc<HoldingsService>,
}

impl SnapshotService {
    pub fn new(
        snapshot_repo: Arc<SnapshotRepository>,
        account_repo: Arc<AccountRepository>,
        holdings_service: Arc<HoldingsService>,
    ) -> Self {
        Self {
            snapshot_repo,
            account_repo,
            holdings_service,
        }
    }

    /// Create a snapshot from the current holdings of an account.
    pub async fn create_snapshot(
        &self,
        account_id: &str,
        snapshot_date: NaiveDate,
        _label: Option<&str>,
    ) -> Result<HoldingSnapshot, AppError> {
        let account = self
            .account_repo
            .get(account_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("account {account_id} not found")))?;

        let holdings = self
            .holdings_service
            .get_holdings(account_id, snapshot_date)
            .await?;

        let total_market_value = holdings.total_market_value + holdings.cash_balance;

        let positions: Vec<SnapshotPositionInput> = holdings
            .holdings
            .iter()
            .map(|h| {
                let average_cost = if !h.quantity.is_zero() {
                    h.cost_basis / h.quantity
                } else {
                    Decimal::ZERO
                };

                SnapshotPositionInput {
                    asset_id: h.asset_id.clone(),
                    quantity: h.quantity,
                    average_cost,
                    total_cost_basis: h.cost_basis,
                    currency: h.currency.clone(),
                    contract_multiplier: Decimal::ONE,
                    inception_date: snapshot_date,
                    is_alternative: false,
                    cost_basis_base: Some(h.cost_basis_base),
                    cost_basis_account: None,
                }
            })
            .collect();

        let cash_balance = holdings.cash_balance;
        let cash_balance_base = holdings.cash_balance_base;

        let input = CreateSnapshotInput {
            account_id: account_id.to_string(),
            snapshot_date,
            currency: account.currency.clone(),
            positions,
            cash_balances: serde_json::json!({
                "total": cash_balance.to_string(),
                "total_base": cash_balance_base.to_string(),
                "currency": account.currency,
            }),
            cost_basis: holdings.total_cost_basis,
            net_contribution: total_market_value,
            net_contribution_base: total_market_value, // simplified
            cash_total_account_currency: cash_balance,
            cash_total_base_currency: cash_balance_base,
            source: HoldingSnapshotSource::Calculated,
        };

        self.snapshot_repo.create(input).await
    }

    /// Get a snapshot by ID.
    pub async fn get_snapshot(&self, id: &str) -> Result<Option<HoldingSnapshot>, AppError> {
        self.snapshot_repo.get(id).await
    }

    /// List snapshots for an account.
    pub async fn list_snapshots(&self, account_id: &str) -> Result<Vec<HoldingSnapshot>, AppError> {
        self.snapshot_repo.list_by_account(account_id).await
    }

    /// Delete a snapshot.
    pub async fn delete_snapshot(&self, id: &str) -> Result<(), AppError> {
        self.snapshot_repo.delete(id).await
    }
}
