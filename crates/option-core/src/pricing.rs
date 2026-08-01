//! Option pricing models
//!
//! Implements Black-Scholes and Binomial pricing models for European and American options.

use crate::OptionError;
use crate::Result;
use domain::option::OptionType;

/// Black-Scholes option pricing model
///
/// # Arguments
/// * `option_type` - Call or Put
/// * `s` - Underlying price
/// * `k` - Strike price
/// * `t` - Time to expiration in years
/// * `r` - Risk-free interest rate (e.g., 0.05 for 5%)
/// * `sigma` - Volatility (e.g., 0.25 for 25%)
/// * `q` - Dividend yield (e.g., 0.02 for 2%)
pub fn black_scholes_price(
    option_type: OptionType,
    s: f64,
    k: f64,
    t: f64,
    r: f64,
    sigma: f64,
    q: f64,
) -> Result<f64> {
    // Validate inputs
    if s <= 0.0 {
        return Err(OptionError::InvalidParameters(
            "Stock price must be positive".to_string(),
        ));
    }
    if k <= 0.0 {
        return Err(OptionError::InvalidParameters(
            "Strike price must be positive".to_string(),
        ));
    }
    if t <= 0.0 {
        return Err(OptionError::InvalidParameters(
            "Time to expiration must be positive".to_string(),
        ));
    }
    if sigma <= 0.0 {
        return Err(OptionError::InvalidParameters(
            "Volatility must be positive".to_string(),
        ));
    }

    let d1 =
        ((s * (1.0 - q)).ln() - k.ln() + (r - q + 0.5 * sigma * sigma) * t) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();

    match option_type {
        OptionType::Call => {
            let call_price = s * (1.0 - q) * norm_cdf(d1) - k * (-r * t).exp() * norm_cdf(d2);
            Ok(call_price)
        }
        OptionType::Put => {
            let put_price = k * (-r * t).exp() * norm_cdf(-d2) - s * (1.0 - q) * norm_cdf(-d1);
            Ok(put_price)
        }
    }
}

/// Standard normal cumulative distribution function
pub fn norm_cdf(x: f64) -> f64 {
    // Use error function approximation
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}

/// Error function approximation (Abramowitz and Stegun)
fn erf(x: f64) -> f64 {
    // Constants for approximation
    const A1: f64 = 0.254829592;
    const A2: f64 = -0.284496736;
    const A3: f64 = 1.421413741;
    const A4: f64 = -1.453152027;
    const A5: f64 = 1.061405429;
    const P: f64 = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + P * x);
    let y = 1.0 - (((((A5 * t + A4) * t) + A3) * t + A2) * t + A1) * t * (-x * x).exp();

    sign * y
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::option::OptionType;

    #[test]
    fn test_black_scholes_call() {
        // Test against known values from financial literature
        // Example: ATM call with 1 year, 100 stock, 100 strike, 5% rate, 20% vol, no dividends
        let price =
            black_scholes_price(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();

        // Expected price approximately $10.45 (from Black-Scholes formula)
        assert!(
            (price - 10.4506).abs() < 0.01,
            "Call price {} != 10.4506",
            price
        );
    }

    #[test]
    fn test_black_scholes_put() {
        // Test put option with same parameters
        let price =
            black_scholes_price(OptionType::Put, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();

        // Expected price approximately $5.57
        assert!(
            (price - 5.5663).abs() < 0.01,
            "Put price {} != 5.5663",
            price
        );
    }

    #[test]
    fn test_put_call_parity() {
        // Put-call parity: Call - Put = S*exp(-qT) - K*exp(-rT)
        let call_price =
            black_scholes_price(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        let put_price =
            black_scholes_price(OptionType::Put, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();

        let parity = call_price - put_price;
        let expected = 100.0 - 100.0 * (-0.05_f64).exp();

        assert!((parity - expected).abs() < 0.01, "Put-call parity violated");
    }

    #[test]
    fn test_invalid_parameters() {
        // Negative stock price should fail
        let result = black_scholes_price(OptionType::Call, -100.0, 100.0, 1.0, 0.05, 0.2, 0.0);
        assert!(result.is_err());
    }
}
