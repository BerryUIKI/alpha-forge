// Performance service — XIRR and time-weighted return calculations.
//
// XIRR (Extended Internal Rate of Return) solves for the discount rate that
// makes the net present value of cash flows + ending value equal to zero.
// Time-weighted return (TWR) links daily sub-period returns to remove the
// effect of external cash flows.

use std::sync::Arc;

use chrono::NaiveDate;
use domain::financial::{
    ConsolidatedPerformance, ConsolidatedPerformancePoint, DailyAccountValuation, PerformancePoint,
    PerformanceSummary,
};
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;

use crate::database::repositories::account_repository::AccountRepository;
use crate::database::repositories::valuation_repository::ValuationRepository;
use crate::error::AppError;

pub struct PerformanceService {
    valuation_repo: Arc<ValuationRepository>,
    account_repo: Arc<AccountRepository>,
}

impl PerformanceService {
    pub fn new(
        valuation_repo: Arc<ValuationRepository>,
        account_repo: Arc<AccountRepository>,
    ) -> Self {
        Self {
            valuation_repo,
            account_repo,
        }
    }

    /// Compute performance summary for an account between two dates.
    pub async fn compute_summary(
        &self,
        account_id: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<PerformanceSummary, AppError> {
        let _account = self
            .account_repo
            .get(account_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("account {account_id} not found")))?;

        let series = self.valuation_repo.list_by_account(account_id).await?;

        // Filter to date range
        let filtered: Vec<&DailyAccountValuation> = series
            .iter()
            .filter(|v| v.valuation_date >= start_date && v.valuation_date <= end_date)
            .collect();

        if filtered.len() < 2 {
            return Ok(PerformanceSummary {
                account_id: account_id.to_string(),
                start_date,
                end_date,
                total_return_pct: None,
                xirr_pct: None,
                twr_pct: None,
                start_value: filtered.first().map(|v| v.total_value).unwrap_or_default(),
                end_value: filtered.last().map(|v| v.total_value).unwrap_or_default(),
                net_contribution: Decimal::ZERO,
                total_gain: Decimal::ZERO,
                total_gain_base: Decimal::ZERO,
                data_quality: "insufficient_data".to_string(),
            });
        }

        let (start_value, end_value) = match (filtered.first(), filtered.last()) {
            (Some(first), Some(last)) => (first.total_value, last.total_value),
            _ => (Decimal::ZERO, Decimal::ZERO),
        };

        // Net contribution = sum of external flows
        let net_contribution: Decimal = filtered.iter().map(|v| v.net_contribution).sum();

        // Total gain = end_value - start_value - net_contribution
        let total_gain = end_value - start_value - net_contribution;
        let total_gain_base = total_gain; // simplified

        // Simple return = (end_value - start_value - net_contribution) / start_value
        let total_return_pct = if !start_value.is_zero() {
            Some((total_gain * Decimal::ONE_HUNDRED) / start_value)
        } else {
            None
        };

        // XIRR: solve using Newton's method on the valuation series
        let xirr_pct = self.calculate_xirr(&filtered);

        // TWR: chain-link daily returns
        let twr_pct = self.calculate_twr(&filtered);

        let data_quality = if xirr_pct.is_some() {
            "complete".to_string()
        } else {
            "partial".to_string()
        };

        Ok(PerformanceSummary {
            account_id: account_id.to_string(),
            start_date,
            end_date,
            total_return_pct,
            xirr_pct,
            twr_pct,
            start_value,
            end_value,
            net_contribution,
            total_gain,
            total_gain_base,
            data_quality,
        })
    }

    /// Compute consolidated performance for several accounts that share one
    /// base currency.
    ///
    /// The series is built from portfolio-level base-currency valuations:
    /// each date sums the last known value of every covered account
    /// (forward-filled), and external flows are summed per date. Returns,
    /// XIRR, and TWR are therefore computed on portfolio cash flows rather
    /// than by averaging per-account metrics.
    pub async fn compute_consolidated(
        &self,
        account_ids: &[String],
        start_date: NaiveDate,
        end_date: NaiveDate,
        requested_base_currency: Option<&str>,
    ) -> Result<ConsolidatedPerformance, AppError> {
        let mut loaded: Vec<Vec<DailyAccountValuation>> = Vec::with_capacity(account_ids.len());
        for account_id in account_ids {
            self.account_repo
                .get(account_id)
                .await?
                .ok_or_else(|| AppError::NotFound(format!("account {account_id} not found")))?;
            loaded.push(self.valuation_repo.list_by_account(account_id).await?);
        }

        // One explicit base currency: an explicit request wins, otherwise the
        // first known valuation currency. Accounts in other currencies are
        // skipped and reported instead of being silently summed.
        let base_currency = requested_base_currency
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| value.to_ascii_uppercase())
            .or_else(|| {
                loaded
                    .iter()
                    .flat_map(|series| series.iter())
                    .map(|valuation| valuation.base_currency.clone())
                    .find(|currency| !currency.is_empty())
            })
            .unwrap_or_default();

        let mut included: Vec<&[DailyAccountValuation]> = Vec::new();
        let mut skipped_accounts = 0usize;
        for series in &loaded {
            match series.first() {
                Some(valuation) if valuation.base_currency != base_currency => {
                    skipped_accounts += 1;
                }
                _ => included.push(series.as_slice()),
            }
        }

        let mut dates: Vec<NaiveDate> = included
            .iter()
            .flat_map(|series| series.iter())
            .filter(|valuation| {
                valuation.valuation_date >= start_date && valuation.valuation_date <= end_date
            })
            .map(|valuation| valuation.valuation_date)
            .collect();
        dates.sort_unstable();
        dates.dedup();

        if dates.len() < 2 {
            return Ok(ConsolidatedPerformance {
                base_currency,
                points: Vec::new(),
                total_return_pct: None,
                xirr_pct: None,
                twr_pct: None,
                start_value: Decimal::ZERO,
                end_value: Decimal::ZERO,
                net_contribution: Decimal::ZERO,
                total_gain: Decimal::ZERO,
                account_count: included.len(),
                covered_account_count: 0,
                skipped_accounts,
                data_quality: "insufficient_data".to_string(),
            });
        }

        let mut cursors = vec![0usize; included.len()];
        let mut points: Vec<ConsolidatedPerformancePoint> = Vec::with_capacity(dates.len());
        let first_portfolio_date = dates[0];

        for date in &dates {
            let mut total_value_base = Decimal::ZERO;
            let mut net_contribution_base = Decimal::ZERO;
            let mut covered_accounts = 0usize;

            for (index, series) in included.iter().enumerate() {
                let cursor = &mut cursors[index];
                while *cursor + 1 < series.len() && series[*cursor + 1].valuation_date <= *date {
                    *cursor += 1;
                }

                let Some(valuation) = series.get(*cursor) else {
                    continue;
                };
                if valuation.valuation_date > *date {
                    continue;
                }

                total_value_base += valuation.total_value_base;
                covered_accounts += 1;

                // Flows on this date. An account whose series begins after the
                // portfolio start enters with its market value as an external
                // inflow, so its arrival cannot masquerade as investment return.
                let mut account_flow = if valuation.valuation_date == *date {
                    valuation.net_contribution_base
                } else {
                    Decimal::ZERO
                };
                let enters_here = series
                    .first()
                    .map(|first| {
                        first.valuation_date > first_portfolio_date && first.valuation_date == *date
                    })
                    .unwrap_or(false);
                if enters_here && account_flow.is_zero() {
                    account_flow = valuation.total_value_base;
                }
                net_contribution_base += account_flow;
            }

            points.push(ConsolidatedPerformancePoint {
                date: *date,
                total_value_base,
                net_contribution_base,
                covered_accounts,
                total_accounts: included.len(),
            });
        }

        let total_accounts = included.len();
        let start_value = points
            .first()
            .map(|point| point.total_value_base)
            .unwrap_or_default();
        let end_value = points
            .last()
            .map(|point| point.total_value_base)
            .unwrap_or_default();
        let net_contribution: Decimal =
            points.iter().map(|point| point.net_contribution_base).sum();
        let total_gain = end_value - start_value - net_contribution;

        let total_return_pct = if start_value.is_zero() {
            None
        } else {
            Some((total_gain * Decimal::ONE_HUNDRED) / start_value)
        };

        let first_date = dates[0];
        let last_date = *dates.last().expect("dates is non-empty");
        let mut cash_flows: Vec<(f64, f64)> = points
            .iter()
            .filter(|point| !point.net_contribution_base.is_zero())
            .map(|point| {
                let days = (point.date - first_date).num_days() as f64;
                (days, -point.net_contribution_base.to_f64().unwrap_or(0.0))
            })
            .collect();
        let terminal_days = (last_date - first_date).num_days() as f64;
        let terminal_value = end_value.to_f64().unwrap_or(0.0);
        cash_flows.push((terminal_days, terminal_value));
        let xirr_pct = solve_xirr(&cash_flows);

        let mut chain_factor = Decimal::ONE;
        for window in points.windows(2) {
            let previous = &window[0];
            let current = &window[1];
            if previous.total_value_base.is_zero() {
                continue;
            }
            let sub_return = (current.total_value_base
                - previous.total_value_base
                - current.net_contribution_base)
                / previous.total_value_base;
            chain_factor *= Decimal::ONE + sub_return;
        }
        let twr_pct = Some((chain_factor - Decimal::ONE) * Decimal::ONE_HUNDRED);

        let covered_account_count = included
            .iter()
            .filter(|series| {
                series
                    .iter()
                    .any(|valuation| valuation.valuation_date <= end_date)
            })
            .count();

        let partial_coverage = covered_account_count < total_accounts
            || points
                .iter()
                .any(|point| point.covered_accounts < point.total_accounts);

        Ok(ConsolidatedPerformance {
            base_currency,
            points,
            total_return_pct,
            xirr_pct,
            twr_pct,
            start_value,
            end_value,
            net_contribution,
            total_gain,
            account_count: total_accounts,
            covered_account_count,
            skipped_accounts,
            data_quality: if partial_coverage {
                "partial".to_string()
            } else {
                "complete".to_string()
            },
        })
    }

    /// Get the performance time-series (daily points) for an account.
    pub async fn get_time_series(
        &self,
        account_id: &str,
    ) -> Result<Vec<PerformancePoint>, AppError> {
        let series = self.valuation_repo.list_by_account(account_id).await?;

        let mut points = Vec::new();
        let mut prev_value = Decimal::ZERO;

        for v in &series {
            let daily_return_pct = if !prev_value.is_zero() {
                Some(
                    ((v.total_value - prev_value - v.net_contribution) * Decimal::ONE_HUNDRED)
                        / prev_value,
                )
            } else {
                None
            };

            // Cumulative return from start of series
            let cumulative_return_pct = points.last().and_then(|p: &PerformancePoint| {
                p.cumulative_return_pct.map(|_| Decimal::ZERO) // placeholder
            });

            points.push(PerformancePoint {
                date: v.valuation_date,
                total_value: v.total_value,
                total_value_base: v.total_value_base,
                net_contribution: v.net_contribution,
                net_contribution_base: v.net_contribution_base,
                cumulative_return_pct,
                daily_return_pct,
            });

            prev_value = v.total_value;
        }

        // Compute cumulative returns
        if let Some(first) = points.first() {
            let start_value = first.total_value;
            let mut cumulative_net_contribution = Decimal::ZERO;
            for point in &mut points {
                cumulative_net_contribution += point.net_contribution;
                let gain = point.total_value - start_value - cumulative_net_contribution;
                point.cumulative_return_pct = if !start_value.is_zero() {
                    Some((gain * Decimal::ONE_HUNDRED) / start_value)
                } else {
                    None
                };
            }
        }

        Ok(points)
    }

    /// Calculate XIRR using Newton's method on a series of valuation points.
    /// Each valuation point is treated as a cash flow (net_contribution) with
    /// the final total_value as the terminal flow.
    fn calculate_xirr(&self, series: &[&DailyAccountValuation]) -> Option<Decimal> {
        if series.len() < 2 {
            return None;
        }

        let (first, last) = match (series.first(), series.last()) {
            (Some(f), Some(l)) => (*f, *l),
            _ => return None,
        };
        let first_date = first.valuation_date;

        // Build cash flows: each valuation is a flow of net_contribution,
        // the final valuation also includes total_value as a positive flow.
        let mut cash_flows: Vec<(f64, f64)> = Vec::new();

        for valuation in series.iter() {
            let days = (valuation.valuation_date - first_date).num_days() as f64;
            let net_flow = valuation.net_contribution.to_f64().unwrap_or(0.0);
            if net_flow != 0.0 {
                cash_flows.push((days, -net_flow));
            }
        }

        // Add terminal value as positive flow
        let terminal_days = (last.valuation_date - first_date).num_days() as f64;
        let terminal_value = last.total_value.to_f64().unwrap_or(0.0);
        cash_flows.push((terminal_days, terminal_value));

        solve_xirr(&cash_flows)
    }

    /// Calculate time-weighted return by chaining daily sub-period returns.
    fn calculate_twr(&self, series: &[&DailyAccountValuation]) -> Option<Decimal> {
        if series.len() < 2 {
            return None;
        }

        let mut chain_factor = Decimal::ONE;

        for window in series.windows(2) {
            let prev = window[0];
            let curr = window[1];

            let start_value = prev.total_value;
            if start_value.is_zero() {
                continue;
            }

            // Sub-period return = (end_value - start_value - net_flow) / start_value
            let sub_return =
                (curr.total_value - prev.total_value - curr.net_contribution) / start_value;

            // Chain-link: (1 + r1) * (1 + r2) * ... - 1
            chain_factor *= Decimal::ONE + sub_return;
        }

        let twr = (chain_factor - Decimal::ONE) * Decimal::ONE_HUNDRED;
        Some(twr)
    }
}

/// Solve XIRR with Newton's method over `(days, amount)` cash flows.
///
/// Flows are expressed in one currency; `days` are offsets from the first
/// flow's date. Returns percentage points (`5` = 5%).
fn solve_xirr(cash_flows: &[(f64, f64)]) -> Option<Decimal> {
    if cash_flows.len() < 2 {
        return None;
    }

    let mut rate: f64 = 0.1; // initial guess (10%)
    let max_iterations = 100;
    let tolerance = 1e-10;

    for _ in 0..max_iterations {
        let mut f = 0.0;
        let mut f_prime = 0.0;

        for (days, amount) in cash_flows {
            let t = days / 365.0;
            let factor = (1.0 + rate).powf(t);
            f += amount / factor;
            f_prime += -amount * t / (factor * (1.0 + rate));
        }

        if f.abs() < tolerance {
            break;
        }

        if f_prime.abs() < 1e-15 {
            break; // avoid division by zero
        }

        let new_rate = rate - f / f_prime;

        if !new_rate.is_finite() || new_rate < -0.9999 {
            // XIRR failed to converge
            return None;
        }

        rate = new_rate;
    }

    let xirr_pct = rate * 100.0;
    if !xirr_pct.is_finite() {
        return None;
    }

    Decimal::from_f64(xirr_pct)
}
