/**
 * Strategy Service
 * Orchestrates strategy analysis and validation
 */

use std::sync::Arc;
use domain::option::{OptionStrategy, StrategyLeg, OptionType, PositionType};
use option_core::{calculate_strategy_payoff, calculate_greeks, StrategyPayoff, GreeksValues};
use crate::database::repositories::option_strategy_repository::OptionStrategyRepository;
use crate::error::AppError;

pub struct StrategyService {
    strategy_repo: Arc<OptionStrategyRepository>,
}

impl StrategyService {
    pub fn new(strategy_repo: Arc<OptionStrategyRepository>) -> Self {
        Self { strategy_repo }
    }

    /// Validate strategy legs for consistency
    pub fn validate_strategy(&self, legs: &[StrategyLeg]) -> Result<ValidationResult, AppError> {
        if legs.is_empty() {
            return Err(AppError::Validation("Strategy must have at least one leg".to_string()));
        }

        // Check for valid quantities
        for leg in legs {
            if leg.quantity == 0 {
                return Err(AppError::Validation("Leg quantity cannot be zero".to_string()));
            }
        }

        // Check expiration consistency (all legs should have same expiration for now)
        let expirations: std::collections::HashSet<_> = legs.iter().map(|l| l.expiration).collect();
        if expirations.len() > 1 {
            return Err(AppError::Validation("All legs must have the same expiration date".to_string()));
        }

        Ok(ValidationResult { valid: true, warnings: vec![] })
    }

    /// Calculate comprehensive risk metrics for a strategy
    pub fn analyze_strategy(
        &self,
        legs: &[StrategyLeg],
        underlying_price: f64,
        volatility: f64,
        risk_free_rate: f64,
        days_to_expiration: i64,
    ) -> Result<StrategyAnalysis, AppError> {
        let years_to_expiration = days_to_expiration as f64 / 365.0;

        // Calculate payoff at current price
        let current_payoff = calculate_strategy_payoff(legs, underlying_price, years_to_expiration)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        // Calculate net Greeks
        let net_greeks = self.calculate_net_greeks(legs, underlying_price, volatility, risk_free_rate, years_to_expiration)?;

        // Calculate break-even points
        let break_evens = self.calculate_break_evens(legs)?;

        // Estimate max profit/loss
        let (max_profit, max_loss) = self.estimate_max_profit_loss(legs)?;

        // Calculate probability of profit (simplified)
        let pop = self.calculate_probability_of_profit(&break_evens, underlying_price, volatility, years_to_expiration)?;

        Ok(StrategyAnalysis {
            net_greeks,
            break_even_points: break_evens,
            max_profit,
            max_loss,
            probability_of_profit: pop,
            current_payoff: current_payoff.total,
        })
    }

    /// Calculate net Greeks for entire strategy
    fn calculate_net_greeks(
        &self,
        legs: &[StrategyLeg],
        underlying_price: f64,
        volatility: f64,
        risk_free_rate: f64,
        years_to_expiration: f64,
    ) -> Result<GreeksValues, AppError> {
        let mut net_delta = 0.0;
        let mut net_gamma = 0.0;
        let mut net_theta = 0.0;
        let mut net_vega = 0.0;
        let mut net_rho = 0.0;

        for leg in legs {
            let greeks = calculate_greeks(
                leg.option_type,
                underlying_price,
                leg.strike,
                years_to_expiration,
                risk_free_rate,
                volatility,
                0.0, // dividend yield
            ).map_err(|e| AppError::Internal(e.to_string()))?;

            let multiplier = if leg.position_type == PositionType::Long { 1.0 } else { -1.0 };
            let quantity = leg.quantity as f64;

            net_delta += greeks.delta * multiplier * quantity;
            net_gamma += greeks.gamma * multiplier * quantity;
            net_theta += greeks.theta * multiplier * quantity;
            net_vega += greeks.vega * multiplier * quantity;
            net_rho += greeks.rho * multiplier * quantity;
        }

        Ok(GreeksValues {
            delta: net_delta,
            gamma: net_gamma,
            theta: net_theta,
            vega: net_vega,
            rho: net_rho,
        })
    }

    /// Calculate break-even points for strategy
    fn calculate_break_evens(&self, legs: &[StrategyLeg]) -> Result<Vec<f64>, AppError> {
        // Simplified break-even calculation
        // For a single-leg call: break-even = strike + premium
        // For a single-leg put: break-even = strike - premium
        // For multi-leg strategies, need to solve payoff = 0

        let mut break_evens = Vec::new();

        // Use numerical method to find break-even points
        for test_price in (50..200).step_by(1) {
            let price = test_price as f64;
            if let Ok(payoff) = calculate_strategy_payoff(legs, price, 0.0) {
                if payoff.total.abs() < 0.5 {
                    break_evens.push(price);
                }
            }
        }

        Ok(break_evens)
    }

    /// Estimate max profit and loss
    fn estimate_max_profit_loss(&self, legs: &[StrategyLeg]) -> Result<(Option<f64>, Option<f64>), AppError> {
        // Test extreme price scenarios
        let test_prices = [0.0, 50.0, 100.0, 150.0, 200.0, 300.0, 500.0];

        let mut max_profit: Option<f64> = None;
        let mut max_loss: Option<f64> = None;

        for price in test_prices {
            if let Ok(payoff) = calculate_strategy_payoff(legs, price, 0.0) {
                let p = payoff.total;

                match max_profit {
                    None => max_profit = Some(p),
                    Some(current) if p > current => max_profit = Some(p),
                    _ => {}
                }

                match max_loss {
                    None => max_loss = Some(p),
                    Some(current) if p < current => max_loss = Some(p),
                    _ => {}
                }
            }
        }

        Ok((max_profit, max_loss))
    }

    /// Calculate probability of profit using simplified model
    fn calculate_probability_of_profit(
        &self,
        break_evens: &[f64],
        current_price: f64,
        volatility: f64,
        years: f64,
    ) -> Result<f64, AppError> {
        if break_evens.is_empty() {
            return Ok(0.5); // Default to 50% if no break-even found
        }

        // Simplified: assume one break-even point
        let break_even = break_evens[0];

        // Use normal distribution approximation
        let drift = 0.0; // Assume no drift for simplicity
        let std_dev = current_price * volatility * years.sqrt();

        // Calculate z-score
        let z = (break_even - current_price - drift) / std_dev;

        // Simplified normal CDF approximation
        let prob = 0.5 * (1.0 + erf(z / std::f64::consts::SQRT_2));

        Ok(prob)
    }

    /// Save strategy to database
    pub async fn save_strategy(&self, strategy: &OptionStrategy) -> Result<String, AppError> {
        self.strategy_repo.create(strategy).await
            .map_err(|e| AppError::Internal(e.to_string()))
    }
}

/// Error function approximation for normal CDF
fn erf(x: f64) -> f64 {
    // Approximation of error function
    let a1 =  0.254829592;
    let a2 = -0.284496736;
    let a3 =  1.421413741;
    let a4 = -1.453152027;
    let a5 =  1.061405429;
    let p  =  0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}

#[derive(Debug)]
pub struct ValidationResult {
    pub valid: bool,
    pub warnings: Vec<String>,
}

#[derive(Debug)]
pub struct StrategyAnalysis {
    pub net_greeks: GreeksValues,
    pub break_even_points: Vec<f64>,
    pub max_profit: Option<f64>,
    pub max_loss: Option<f64>,
    pub probability_of_profit: f64,
    pub current_payoff: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_strategy() {
        let repo = Arc::new(OptionStrategyRepository::new(sqlx::SqlitePool::new_lazy()));
        let service = StrategyService::new(repo);

        let legs = vec![StrategyLeg {
            id: "1".to_string(),
            strategy_id: "test".to_string(),
            option_contract_id: "contract1".to_string(),
            quantity: 1,
            position_type: PositionType::Long,
            premium: 5.0,
            strike: 100.0,
            expiration: chrono::Utc::now() + chrono::Duration::days(30),
            option_type: OptionType::Call,
        }];

        let result = service.validate_strategy(&legs);
        assert!(result.is_ok());
    }
}