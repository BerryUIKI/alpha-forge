// Performance service — XIRR and time-weighted return calculations.
//
// XIRR (Extended Internal Rate of Return) solves for the discount rate that
// makes the net present value of cash flows + ending value equal to zero.
// Time-weighted return (TWR) links daily sub-period returns to remove the
// effect of external cash flows.

use std::sync::Arc;

use chrono::NaiveDate;
use domain::financial::{DailyAccountValuation, PerformancePoint, PerformanceSummary};
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

        let start_value = filtered.first().unwrap().total_value;
        let end_value = filtered.last().unwrap().total_value;

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
        if !points.is_empty() {
            let start_value = points.first().unwrap().total_value;
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

        let first_date = series.first().unwrap().valuation_date;
        let last = series.last().unwrap();

        // Build cash flows: each valuation is a flow of net_contribution,
        // the final valuation also includes total_value as a positive flow.
        #[derive(Clone, Copy)]
        struct CashFlow {
            days: f64,
            amount: f64,
        }

        let mut cash_flows: Vec<CashFlow> = Vec::new();

        for v in series.iter() {
            let days = (v.valuation_date - first_date).num_days() as f64;
            let net_flow = v.net_contribution.to_f64().unwrap_or(0.0);
            if net_flow != 0.0 {
                cash_flows.push(CashFlow {
                    days,
                    amount: -net_flow,
                });
            }
        }

        // Add terminal value as positive flow
        let terminal_days = (last.valuation_date - first_date).num_days() as f64;
        let terminal_value = last.total_value.to_f64().unwrap_or(0.0);
        cash_flows.push(CashFlow {
            days: terminal_days,
            amount: terminal_value,
        });

        if cash_flows.len() < 2 {
            return None;
        }

        // Newton's method to solve for rate
        let mut rate: f64 = 0.1; // initial guess (10%)
        let max_iterations = 100;
        let tolerance = 1e-10;

        for _ in 0..max_iterations {
            let mut f = 0.0;
            let mut f_prime = 0.0;

            for cf in &cash_flows {
                let t = cf.days / 365.0;
                let factor = (1.0 + rate).powf(t);
                f += cf.amount / factor;
                f_prime += -cf.amount * t / (factor * (1.0 + rate));
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

        // Convert to percentage
        let xirr_pct = rate * 100.0;
        if !xirr_pct.is_finite() {
            return None;
        }

        Decimal::from_f64(xirr_pct)
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
