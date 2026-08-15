// Holdings service — aggregate open lots into current holdings with market
// values, cost basis, and gains. This is the primary read model for the
// portfolio dashboard.
//
// A holding = the sum of open lots for one asset in one account, enriched
// with the latest quoted price to compute market value and unrealized gains.

use std::collections::HashMap;
use std::sync::Arc;

use chrono::NaiveDate;
use domain::financial::{AssetKind, Holding, HoldingsSummary};
use rust_decimal::Decimal;

use crate::database::repositories::account_repository::AccountRepository;
use crate::database::repositories::asset_repository::{AssetRepository, QuoteRepository};
use crate::database::repositories::lot_repository::{LotDisposalRepository, LotRepository};
use crate::error::AppError;

pub struct HoldingsService {
    account_repo: Arc<AccountRepository>,
    asset_repo: Arc<AssetRepository>,
    quote_repo: Arc<QuoteRepository>,
    lot_repo: Arc<LotRepository>,
    disposal_repo: Arc<LotDisposalRepository>,
}

impl HoldingsService {
    pub fn new(
        account_repo: Arc<AccountRepository>,
        asset_repo: Arc<AssetRepository>,
        quote_repo: Arc<QuoteRepository>,
        lot_repo: Arc<LotRepository>,
        disposal_repo: Arc<LotDisposalRepository>,
    ) -> Self {
        Self {
            account_repo,
            asset_repo,
            quote_repo,
            lot_repo,
            disposal_repo,
        }
    }

    /// Compute current holdings for a single account as of `date`.
    pub async fn get_holdings(
        &self,
        account_id: &str,
        as_of_date: NaiveDate,
    ) -> Result<HoldingsSummary, AppError> {
        let account = self
            .account_repo
            .get(account_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("account {account_id} not found")))?;

        let account_currency = &account.currency;
        let lots = self.lot_repo.list_open_by_account(account_id).await?;

        // Group lots by asset_id, fetching disposals once.
        let disposals = self.disposal_repo.list_by_account(account_id).await?;
        let mut realized_by_asset: HashMap<String, Decimal> = HashMap::new();
        for d in &disposals {
            *realized_by_asset.entry(d.asset_id.clone()).or_default() += d.realized_pnl;
        }

        // Collect unique asset IDs
        let mut asset_ids: Vec<&str> = lots.iter().map(|l| l.asset_id.as_str()).collect();
        asset_ids.sort_unstable();
        asset_ids.dedup();

        // Fetch assets and latest quotes
        let mut asset_map = HashMap::new();
        let mut quote_map = HashMap::new();
        for asset_id in &asset_ids {
            if let Some(asset) = self.asset_repo.get(asset_id).await? {
                let quote = self
                    .quote_repo
                    .get_for_day(asset_id, &as_of_date, "market")
                    .await
                    .ok()
                    .flatten();
                asset_map.insert(asset_id.to_string(), asset);
                quote_map.insert(asset_id.to_string(), quote);
            }
        }

        // Build holdings
        let mut cash_balance = Decimal::ZERO;
        let mut holdings = Vec::new();

        for asset_id in &asset_ids {
            let asset = match asset_map.get(*asset_id) {
                Some(a) => a,
                None => continue,
            };

            let asset_lots: Vec<_> = lots.iter().filter(|l| l.asset_id == **asset_id).collect();

            let total_quantity: Decimal = asset_lots.iter().map(|l| l.remaining_quantity).sum();
            if total_quantity.is_zero() {
                continue;
            }

            // Cash accounts: the quantity IS the cash balance
            if asset.kind == AssetKind::Investment && asset.quote_mode.to_string() == "MANUAL"
                || asset.kind == AssetKind::Other
            {
                cash_balance += total_quantity;
                continue;
            }

            // Total cost basis = sum of remaining_cost_basis across open lots
            let cost_basis: Decimal = asset_lots.iter().map(|l| l.remaining_cost_basis).sum();
            let cost_basis_base: Decimal = asset_lots
                .iter()
                .map(|l| l.remaining_cost_basis * l.fx_rate_to_base)
                .sum();

            let quote_currency = &asset.quote_ccy;
            let fx_rate = if quote_currency == account_currency {
                Decimal::ONE
            } else {
                // TODO: fetch FX rate from a rates table when available
                Decimal::ONE
            };

            // Market value from latest quote
            let (market_value, market_value_base) = if let Some(price) = quote_map
                .get(*asset_id)
                .and_then(|q| q.as_ref().map(|q| q.close))
            {
                let mv = total_quantity * price;
                let mv_base = mv * fx_rate;
                (mv, mv_base)
            } else {
                // No quote — use cost basis as fallback
                (cost_basis, cost_basis_base)
            };

            let unrealized_gain = market_value - cost_basis;
            let unrealized_gain_base = market_value_base - cost_basis_base;
            let unrealized_gain_pct = if !cost_basis.is_zero() {
                Some((unrealized_gain * Decimal::ONE_HUNDRED) / cost_basis)
            } else {
                None
            };

            let realized_gain = realized_by_asset
                .get(*asset_id)
                .copied()
                .unwrap_or_default();
            let realized_gain_base = realized_gain; // simplified: same currency for now
            let total_gain = unrealized_gain + realized_gain;
            let total_gain_pct = if !cost_basis.is_zero() {
                Some((total_gain * Decimal::ONE_HUNDRED) / cost_basis)
            } else {
                None
            };

            holdings.push(Holding {
                account_id: account_id.to_string(),
                asset_id: asset_id.to_string(),
                asset_name: asset.name.clone(),
                asset_symbol: asset.display_code.clone(),
                asset_kind: asset.kind,
                currency: account_currency.clone(),
                quantity: total_quantity,
                cost_basis,
                market_value,
                unrealized_gain,
                unrealized_gain_pct,
                realized_gain,
                total_gain,
                total_gain_pct,
                fx_rate,
                cost_basis_base,
                market_value_base,
                unrealized_gain_base,
                realized_gain_base,
                weight_pct: Decimal::ZERO, // computed after total
                open_lot_count: asset_lots.len() as u32,
            });
        }

        // Compute weights
        let total_market_value: Decimal = holdings.iter().map(|h| h.market_value).sum();
        let total_market_value_base: Decimal = holdings.iter().map(|h| h.market_value_base).sum();
        for h in &mut holdings {
            h.weight_pct = if !total_market_value.is_zero() {
                (h.market_value * Decimal::ONE_HUNDRED) / total_market_value
            } else {
                Decimal::ZERO
            };
        }

        let total_cost_basis: Decimal = holdings.iter().map(|h| h.cost_basis).sum();
        let total_unrealized_gain: Decimal = holdings.iter().map(|h| h.unrealized_gain).sum();
        let total_realized_gain: Decimal = holdings.iter().map(|h| h.realized_gain).sum();
        let total_cost_basis_base: Decimal = holdings.iter().map(|h| h.cost_basis_base).sum();
        let total_unrealized_gain_base: Decimal =
            holdings.iter().map(|h| h.unrealized_gain_base).sum();
        let total_realized_gain_base: Decimal = holdings.iter().map(|h| h.realized_gain_base).sum();

        // Cash is all open lots where the asset is a cash-like entry
        // (already handled above via the cash_balance accumulator)

        Ok(HoldingsSummary {
            account_id: account_id.to_string(),
            as_of_date,
            total_market_value,
            total_cost_basis,
            total_unrealized_gain,
            total_realized_gain,
            total_market_value_base,
            total_cost_basis_base,
            total_unrealized_gain_base,
            total_realized_gain_base,
            holdings,
            cash_balance,
            cash_balance_base: cash_balance, // simplified
        })
    }

    /// Compute holdings for all non-archived accounts.
    pub async fn get_all_holdings(
        &self,
        as_of_date: NaiveDate,
    ) -> Result<Vec<HoldingsSummary>, AppError> {
        let accounts = self.account_repo.list().await?;
        let mut results = Vec::new();
        for account in &accounts {
            if account.is_archived {
                continue;
            }
            match self.get_holdings(&account.id, as_of_date).await {
                Ok(summary) => results.push(summary),
                Err(e) => {
                    tracing::warn!("skipping holdings for account {}: {}", account.id, e);
                }
            }
        }
        Ok(results)
    }
}
