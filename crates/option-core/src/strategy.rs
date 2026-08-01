//! Strategy analysis for multi-leg option positions

use crate::OptionError;
use crate::Result;
use domain::option::OptionType;

/// Strategy leg representation
#[derive(Debug, Clone)]
pub struct StrategyLeg {
    pub option_type: OptionType,
    pub strike: f64,
    pub expiration: f64, // Years
    pub quantity: i32,   // Positive for long, negative for short
    pub premium: f64,
}

/// Strategy payoff result
#[derive(Debug)]
pub struct StrategyPayoff {
    pub max_profit: Option<f64>,
    pub max_loss: Option<f64>,
    pub break_even_points: Vec<f64>,
    pub payoff_at_expiry: Vec<(f64, f64)>, // (price, payoff) pairs
}

/// Calculate strategy payoff at expiration
pub fn calculate_payoff_at_expiry(legs: &[StrategyLeg], underlying_price: f64) -> Result<f64> {
    let mut total_payoff = 0.0;

    for leg in legs {
        let intrinsic_value = match leg.option_type {
            OptionType::Call => (underlying_price - leg.strike).max(0.0),
            OptionType::Put => (leg.strike - underlying_price).max(0.0),
        };

        // Payoff = intrinsic value * quantity - premium paid/received
        let leg_payoff = intrinsic_value * leg.quantity as f64 - leg.premium * leg.quantity as f64;
        total_payoff += leg_payoff;
    }

    Ok(total_payoff)
}

/// Analyze strategy risk/reward profile
pub fn analyze_strategy(legs: &[StrategyLeg]) -> Result<StrategyPayoff> {
    if legs.is_empty() {
        return Err(OptionError::InvalidParameters(
            "Strategy must have at least one leg".to_string(),
        ));
    }

    // Generate payoff profile for range of underlying prices
    let min_strike = legs.iter().map(|l| l.strike).fold(f64::INFINITY, f64::min);
    let max_strike = legs
        .iter()
        .map(|l| l.strike)
        .fold(f64::NEG_INFINITY, f64::max);

    let price_start = min_strike * 0.5;
    let price_end = max_strike * 1.5;
    let step = ((price_end - price_start) / 100.0).max(1.0);

    let mut payoff_points = Vec::new();
    let mut break_even_points = Vec::new();

    let mut prev_payoff = None;
    let mut current_price = price_start;

    while current_price <= price_end {
        let payoff = calculate_payoff_at_expiry(legs, current_price)?;
        payoff_points.push((current_price, payoff));

        // Detect break-even points (payoff crosses zero)
        if let Some(prev) = prev_payoff {
            if (prev < 0.0 && payoff >= 0.0) || (prev >= 0.0 && payoff < 0.0) {
                break_even_points.push(current_price);
            }
        }
        prev_payoff = Some(payoff);
        current_price += step;
    }

    // Calculate max profit/loss
    let max_profit = payoff_points
        .iter()
        .map(|(_, p)| *p)
        .fold(f64::NEG_INFINITY, f64::max);
    let max_loss = payoff_points
        .iter()
        .map(|(_, p)| *p)
        .fold(f64::INFINITY, f64::min);

    Ok(StrategyPayoff {
        max_profit: if max_profit.is_finite() {
            Some(max_profit)
        } else {
            None
        },
        max_loss: if max_loss.is_finite() {
            Some(max_loss)
        } else {
            None
        },
        break_even_points,
        payoff_at_expiry: payoff_points,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_long_call_payoff() {
        let leg = StrategyLeg {
            option_type: OptionType::Call,
            strike: 100.0,
            expiration: 1.0,
            quantity: 1,
            premium: 5.0,
        };

        // ITM scenario
        let payoff_itm = calculate_payoff_at_expiry(&[leg.clone()], 110.0).unwrap();
        assert_eq!(payoff_itm, 5.0); // (110 - 100) - 5 = 5

        // OTM scenario
        let payoff_otm = calculate_payoff_at_expiry(&[leg], 90.0).unwrap();
        assert_eq!(payoff_otm, -5.0); // 0 - 5 = -5
    }
}
