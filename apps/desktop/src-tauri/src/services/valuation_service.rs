// Valuation service — daily portfolio valuation time-series.
//
// Calculates the total value of an account on a given date by summing
// holdings (market value of open lots + cash) and persists the result
// as a `DailyAccountValuation` row. The valuation series powers the
// performance (XIRR, TWR) calculations.

use std::sync::Arc;

use chrono::NaiveDate;
use domain::financial::{
    BasisStatus, DailyAccountValuation, ExternalFlowSource, UpsertValuationInput, ValuationStatus,
};
use rust_decimal::Decimal;

use crate::database::repositories::account_repository::AccountRepository;
use crate::database::repositories::valuation_repository::ValuationRepository;
use crate::error::AppError;

use super::holdings_service::HoldingsService;

pub struct ValuationService {
    valuation_repo: Arc<ValuationRepository>,
    account_repo: Arc<AccountRepository>,
    holdings_service: Arc<HoldingsService>,
}

impl ValuationService {
    pub fn new(
        valuation_repo: Arc<ValuationRepository>,
        account_repo: Arc<AccountRepository>,
        holdings_service: Arc<HoldingsService>,
    ) -> Self {
        Self {
            valuation_repo,
            account_repo,
            holdings_service,
        }
    }

    /// Calculate and persist one day's valuation for an account.
    pub async fn calculate_day(
        &self,
        account_id: &str,
        date: NaiveDate,
    ) -> Result<DailyAccountValuation, AppError> {
        let account = self
            .account_repo
            .get(account_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("account {account_id} not found")))?;

        let base_currency = "USD".to_string(); // TODO: make configurable
        let fx_rate_to_base = if account.currency == base_currency {
            Decimal::ONE
        } else {
            // TODO: fetch FX rate from rates table
            Decimal::ONE
        };

        let holdings = self.holdings_service.get_holdings(account_id, date).await?;
        let cash_balance = holdings.cash_balance;
        let investment_market_value = holdings.total_market_value;
        let total_value = cash_balance + investment_market_value;
        let cost_basis = holdings.total_cost_basis;
        let net_contribution = total_value; // simplified: first calculation

        let cash_balance_base = cash_balance * fx_rate_to_base;
        let investment_market_value_base = investment_market_value * fx_rate_to_base;
        let total_value_base = total_value * fx_rate_to_base;
        let cost_basis_base = cost_basis * fx_rate_to_base;
        let net_contribution_base = net_contribution * fx_rate_to_base;

        // Determine valuation quality
        let (value_status, basis_status) =
            self.determine_status(&holdings, investment_market_value, cost_basis);

        self.valuation_repo
            .upsert(UpsertValuationInput {
                account_id: account_id.to_string(),
                valuation_date: date,
                account_currency: account.currency.clone(),
                base_currency: base_currency.clone(),
                fx_rate_to_base,
                cash_balance,
                investment_market_value,
                total_value,
                cost_basis,
                net_contribution,
                cash_balance_base,
                investment_market_value_base,
                total_value_base,
                cost_basis_base,
                net_contribution_base,
                external_inflow_base: Decimal::ZERO,
                external_outflow_base: Decimal::ZERO,
                performance_eligible_value_base: total_value_base,
                external_flow_source: ExternalFlowSource::NoFlow,
                value_status,
                basis_status,
            })
            .await
    }

    /// Get a single valuation row.
    pub async fn get_valuation(
        &self,
        account_id: &str,
        date: &str,
    ) -> Result<Option<DailyAccountValuation>, AppError> {
        self.valuation_repo.get(account_id, date).await
    }

    /// Get the full valuation series for an account.
    pub async fn get_valuation_series(
        &self,
        account_id: &str,
    ) -> Result<Vec<DailyAccountValuation>, AppError> {
        self.valuation_repo.list_by_account(account_id).await
    }

    /// Calculate and persist valuations for all active accounts on a date.
    pub async fn calculate_all(
        &self,
        date: NaiveDate,
    ) -> Result<Vec<DailyAccountValuation>, AppError> {
        let accounts = self.account_repo.list().await?;
        let mut results = Vec::new();
        for account in &accounts {
            if account.is_archived {
                continue;
            }
            match self.calculate_day(&account.id, date).await {
                Ok(v) => results.push(v),
                Err(e) => {
                    tracing::warn!("skipping valuation for account {}: {}", account.id, e);
                }
            }
        }
        Ok(results)
    }

    fn determine_status(
        &self,
        holdings: &domain::financial::HoldingsSummary,
        _investment_market_value: Decimal,
        _cost_basis: Decimal,
    ) -> (ValuationStatus, BasisStatus) {
        let all_priced = holdings
            .holdings
            .iter()
            .all(|h| h.market_value > Decimal::ZERO);
        let all_basis_known = holdings
            .holdings
            .iter()
            .all(|h| h.cost_basis > Decimal::ZERO);

        // Empty holdings must be checked before the vacuous `.all()` results:
        // an empty slice makes `all_priced`/`all_basis_known` true, which
        // would otherwise mislabel empty accounts as Complete.
        let value_status = if holdings.holdings.is_empty() {
            ValuationStatus::Unavailable
        } else if all_priced {
            ValuationStatus::Complete
        } else {
            ValuationStatus::PartialUnpriced
        };

        let basis_status = if holdings.holdings.is_empty() {
            BasisStatus::NotApplicable
        } else if all_basis_known {
            BasisStatus::Complete
        } else {
            BasisStatus::PartialUnknown
        };

        (value_status, basis_status)
    }
}
