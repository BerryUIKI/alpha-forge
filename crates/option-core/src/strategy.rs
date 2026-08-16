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
        let payoff_itm = calculate_payoff_at_expiry(std::slice::from_ref(&leg), 110.0).unwrap();
        assert_eq!(payoff_itm, 5.0); // (110 - 100) - 5 = 5

        // OTM scenario
        let payoff_otm = calculate_payoff_at_expiry(std::slice::from_ref(&leg), 90.0).unwrap();
        assert_eq!(payoff_otm, -5.0); // 0 - 5 = -5
    }

    #[test]
    fn test_short_put_payoff() {
        let leg = StrategyLeg {
            option_type: OptionType::Put,
            strike: 100.0,
            expiration: 1.0,
            quantity: -1,
            premium: 4.0,
        };

        // Above strike: put expires worthless, collect premium
        let payoff = calculate_payoff_at_expiry(std::slice::from_ref(&leg), 120.0).unwrap();
        assert!(
            (payoff - 4.0).abs() < 1e-9,
            "OTM short put should keep premium"
        );

        // Below strike: put is ITM, loss = strike - price - premium
        let payoff_itm = calculate_payoff_at_expiry(std::slice::from_ref(&leg), 90.0).unwrap();
        assert!((payoff_itm - (-6.0)).abs() < 1e-9, "ITM short put payoff");
    }

    #[test]
    fn test_empty_legs_rejected() {
        let result = analyze_strategy(&[]);
        assert!(matches!(result, Err(OptionError::InvalidParameters(_))));
    }

    #[test]
    fn test_bull_call_spread_risk_reward() {
        // Long 100C @ 5.0, short 110C @ 2.0 → net debit 3.0
        // Max profit = (110-100) - 3.0 = 7.0 (bounded)
        // Max loss = -3.0 (bounded)
        // Break-even ≈ 103.0
        let legs = vec![
            StrategyLeg {
                option_type: OptionType::Call,
                strike: 100.0,
                expiration: 1.0,
                quantity: 1,
                premium: 5.0,
            },
            StrategyLeg {
                option_type: OptionType::Call,
                strike: 110.0,
                expiration: 1.0,
                quantity: -1,
                premium: 2.0,
            },
        ];

        let profile = analyze_strategy(&legs).expect("strategy should analyze");

        // Max profit at or above the short strike
        let payoff_deep_itm = calculate_payoff_at_expiry(&legs, 200.0).unwrap();
        assert!((payoff_deep_itm - 7.0).abs() < 1e-9, "Deep ITM payoff");

        // Max loss below the long strike
        let payoff_deep_otm = calculate_payoff_at_expiry(&legs, 50.0).unwrap();
        assert!((payoff_deep_otm - (-3.0)).abs() < 1e-9, "Deep OTM payoff");

        // Reported bounds
        let max_profit = profile.max_profit.expect("max profit should be bounded");
        let max_loss = profile.max_loss.expect("max loss should be bounded");
        assert!((max_profit - 7.0).abs() < 0.5, "Max profit {}", max_profit);
        assert!((max_loss - (-3.0)).abs() < 0.5, "Max loss {}", max_loss);

        // At least one break-even point near 103
        assert!(
            !profile.break_even_points.is_empty(),
            "Should have break-even"
        );
        let nearest = profile
            .break_even_points
            .iter()
            .min_by(|a, b| (**a - 103.0).abs().total_cmp(&(**b - 103.0).abs()))
            .expect("break-even points");
        assert!(
            (nearest - 103.0).abs() < 2.0,
            "Break-even {} != 103",
            nearest
        );
    }
}
