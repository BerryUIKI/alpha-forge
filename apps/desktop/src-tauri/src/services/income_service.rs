// Income service — dividend/interest income aggregation.
//
// Ported from Wealthfolio's income_service.rs. Simplified: FX conversion
// uses Decimal::ONE (placeholder), matching the existing pattern in other
// Phase 2 services.

use chrono::{Datelike, NaiveDate};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::sync::Arc;

use crate::database::repositories::account_repository::AccountRepository;
use crate::database::repositories::activity_repository::ActivityRepository;
use crate::error::AppError;
use domain::financial::{IncomeByAccount, IncomeByAsset, IncomeSummary};

const DISPLAY_DECIMAL_PRECISION: u32 = 2;

pub struct IncomeService {
    activity_repository: Arc<ActivityRepository>,
    _account_repository: Arc<AccountRepository>,
}

impl IncomeService {
    pub fn new(
        activity_repository: Arc<ActivityRepository>,
        account_repository: Arc<AccountRepository>,
    ) -> Self {
        Self {
            activity_repository,
            _account_repository: account_repository,
        }
    }

    /// Get income summary for the given accounts (or all accounts if None).
    pub async fn get_income_summary(
        &self,
        account_ids: Option<&[String]>,
    ) -> Result<Vec<IncomeSummary>, AppError> {
        // Get income activities from the repository
        let activities = self
            .activity_repository
            .get_income_activities(account_ids)
            .await?;

        if activities.is_empty() {
            return Ok(Vec::new());
        }

        let base_currency = "USD".to_string();
        let today = chrono::Utc::now().date_naive();
        let current_year = today.year();
        let last_year = current_year - 1;
        let two_years_ago = current_year - 2;
        let current_month = today.month();

        // Get oldest activity date for monthly average calculation
        let oldest_date = if let Some(ids) = account_ids.filter(|ids| !ids.is_empty()) {
            self.activity_repository
                .get_first_activity_date(Some(ids))
                .await?
                .unwrap_or(today)
        } else {
            self.activity_repository
                .get_first_activity_date_overall()
                .await?
                .unwrap_or(today)
        };

        let months_since_first = ((current_year - oldest_date.year()) * 12 + current_month as i32
            - oldest_date.month() as i32)
            .max(1) as u32;

        let months_in_last_year = if oldest_date.year() >= current_year - 1 {
            (13 - oldest_date.month() as i32).max(1) as u32
        } else {
            12
        };

        let months_two_years_ago = if oldest_date.year() >= current_year - 2 {
            (13 - oldest_date.month() as i32).max(1) as u32
        } else {
            12
        };

        // Build period summaries
        let mut total = IncomeSummary {
            period: "ALL".to_string(),
            by_month: HashMap::new(),
            by_type: HashMap::new(),
            by_asset: HashMap::new(),
            by_currency: HashMap::new(),
            by_account: HashMap::new(),
            total_income: Decimal::ZERO,
            currency: base_currency.clone(),
            monthly_average: Decimal::ZERO,
            yoy_growth: None,
        };
        let mut ytd = IncomeSummary {
            period: "YTD".to_string(),
            ..total.clone()
        };
        let mut last_year_summary = IncomeSummary {
            period: "LAST_YEAR".to_string(),
            ..total.clone()
        };
        let mut two_years_ago_summary = IncomeSummary {
            period: "TWO_YEARS_AGO".to_string(),
            ..total.clone()
        };

        for activity in &activities {
            let date = match NaiveDate::parse_from_str(
                &format!("{}-01", activity.month_key),
                "%Y-%m-%d",
            ) {
                Ok(d) => d,
                Err(_) => continue,
            };

            // FX conversion placeholder (simplified)
            let converted_amount = activity.amount;

            add_to_summary(&mut total, activity, converted_amount);
            if date.year() == current_year {
                add_to_summary(&mut ytd, activity, converted_amount);
            } else if date.year() == last_year {
                add_to_summary(&mut last_year_summary, activity, converted_amount);
            } else if date.year() == two_years_ago {
                add_to_summary(&mut two_years_ago_summary, activity, converted_amount);
            }
        }

        total.monthly_average = if months_since_first > 0 {
            total.total_income / Decimal::from(months_since_first)
        } else {
            Decimal::ZERO
        };
        ytd.monthly_average = if current_month > 0 {
            ytd.total_income / Decimal::from(current_month)
        } else {
            Decimal::ZERO
        };
        last_year_summary.monthly_average = if months_in_last_year > 0 {
            last_year_summary.total_income / Decimal::from(months_in_last_year)
        } else {
            Decimal::ZERO
        };
        two_years_ago_summary.monthly_average = if months_two_years_ago > 0 {
            two_years_ago_summary.total_income / Decimal::from(months_two_years_ago)
        } else {
            Decimal::ZERO
        };

        // YoY growth
        if ytd.total_income > Decimal::ZERO && last_year_summary.total_income > Decimal::ZERO {
            ytd.yoy_growth = Some(
                (ytd.total_income - last_year_summary.total_income)
                    / last_year_summary.total_income,
            );
        }
        if last_year_summary.total_income > Decimal::ZERO
            && two_years_ago_summary.total_income > Decimal::ZERO
        {
            last_year_summary.yoy_growth = Some(
                (last_year_summary.total_income - two_years_ago_summary.total_income)
                    / two_years_ago_summary.total_income,
            );
        }

        let mut results = vec![
            round_summary(total),
            round_summary(ytd),
            round_summary(last_year_summary),
            round_summary(two_years_ago_summary),
        ];
        results.retain(|s| s.total_income > Decimal::ZERO);

        Ok(results)
    }
}

fn add_to_summary(summary: &mut IncomeSummary, activity: &IncomeActivityRow, amount: Decimal) {
    *summary
        .by_month
        .entry(activity.month_key.clone())
        .or_insert(Decimal::ZERO) += amount;
    *summary
        .by_type
        .entry(activity.income_type.clone())
        .or_insert(Decimal::ZERO) += amount;
    summary
        .by_asset
        .entry(activity.asset_id.clone())
        .and_modify(|e| e.income += amount)
        .or_insert_with(|| IncomeByAsset {
            asset_id: activity.asset_id.clone(),
            kind: activity.asset_kind.clone(),
            symbol: activity.symbol.clone(),
            name: activity.symbol_name.clone(),
            income: amount,
        });
    *summary
        .by_currency
        .entry(activity.currency.clone())
        .or_insert(Decimal::ZERO) += activity.amount;
    summary
        .by_account
        .entry(activity.account_id.clone())
        .and_modify(|e| {
            *e.by_month
                .entry(activity.month_key.clone())
                .or_insert(Decimal::ZERO) += amount;
            e.total += amount;
        })
        .or_insert_with(|| {
            let mut by_month = HashMap::new();
            by_month.insert(activity.month_key.clone(), amount);
            IncomeByAccount {
                account_id: activity.account_id.clone(),
                account_name: activity.account_name.clone(),
                by_month,
                total: amount,
            }
        });
    summary.total_income += amount;
}

fn round_summary(mut s: IncomeSummary) -> IncomeSummary {
    s.total_income = s.total_income.round_dp(DISPLAY_DECIMAL_PRECISION);
    s.monthly_average = s.monthly_average.round_dp(DISPLAY_DECIMAL_PRECISION);
    s.yoy_growth = s.yoy_growth.map(|g| g.round_dp(DISPLAY_DECIMAL_PRECISION));
    for v in s.by_month.values_mut() {
        *v = v.round_dp(DISPLAY_DECIMAL_PRECISION);
    }
    for v in s.by_type.values_mut() {
        *v = v.round_dp(DISPLAY_DECIMAL_PRECISION);
    }
    for v in s.by_asset.values_mut() {
        v.income = v.income.round_dp(DISPLAY_DECIMAL_PRECISION);
    }
    for v in s.by_currency.values_mut() {
        *v = v.round_dp(DISPLAY_DECIMAL_PRECISION);
    }
    for v in s.by_account.values_mut() {
        v.total = v.total.round_dp(DISPLAY_DECIMAL_PRECISION);
        for m in v.by_month.values_mut() {
            *m = m.round_dp(DISPLAY_DECIMAL_PRECISION);
        }
    }
    s
}

/// Row returned by the activity repository for income aggregation.
pub struct IncomeActivityRow {
    pub month_key: String,
    pub income_type: String,
    pub asset_id: String,
    pub asset_kind: String,
    pub symbol: String,
    pub symbol_name: String,
    pub currency: String,
    pub amount: Decimal,
    pub account_id: String,
    pub account_name: String,
}
