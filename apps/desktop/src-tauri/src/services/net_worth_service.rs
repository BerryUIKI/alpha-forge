// Net worth service — aggregate account values into a net-worth snapshot.
//
// Computes total assets, liabilities, and net worth across all accounts,
// converting values to a common base currency.

use std::sync::Arc;

use chrono::NaiveDate;
use domain::financial::{AccountType, NetWorthAccountEntry, NetWorthSnapshot};
use rust_decimal::Decimal;

use crate::database::repositories::account_repository::AccountRepository;
use crate::error::AppError;

use super::holdings_service::HoldingsService;

pub struct NetWorthService {
    account_repo: Arc<AccountRepository>,
    holdings_service: Arc<HoldingsService>,
}

impl NetWorthService {
    pub fn new(
        account_repo: Arc<AccountRepository>,
        holdings_service: Arc<HoldingsService>,
    ) -> Self {
        Self {
            account_repo,
            holdings_service,
        }
    }

    /// Compute net worth as of a given date.
    pub async fn compute_net_worth(
        &self,
        as_of_date: NaiveDate,
        base_currency: &str,
    ) -> Result<NetWorthSnapshot, AppError> {
        let accounts = self.account_repo.list().await?;
        let mut entries = Vec::new();
        let mut total_assets = Decimal::ZERO;
        let mut total_liabilities = Decimal::ZERO;

        for account in &accounts {
            if account.is_archived {
                continue;
            }

            let summary = match self
                .holdings_service
                .get_holdings(&account.id, as_of_date)
                .await
            {
                Ok(s) => s,
                Err(e) => {
                    tracing::warn!("skipping net worth for account {}: {}", account.id, e);
                    continue;
                }
            };

            let total_value = summary.total_market_value + summary.cash_balance;
            let total_value_base = summary.total_market_value_base + summary.cash_balance_base;

            let is_liability = account.account_type == AccountType::CreditCard;

            let entry = NetWorthAccountEntry {
                account_id: account.id.clone(),
                account_name: account.name.clone(),
                account_type: account.account_type,
                currency: account.currency.clone(),
                total_value,
                total_value_base,
                cash_balance: summary.cash_balance,
                investment_value: summary.total_market_value,
            };

            if is_liability {
                total_liabilities += total_value_base;
            } else {
                total_assets += total_value_base;
            }

            entries.push(entry);
        }

        let net_worth = total_assets - total_liabilities;

        Ok(NetWorthSnapshot {
            as_of_date,
            base_currency: base_currency.to_string(),
            total_assets,
            total_liabilities,
            net_worth,
            accounts: entries,
        })
    }
}
