//! Volatility models and implied volatility calculations

use crate::pricing::black_scholes_price;
use crate::OptionError;
use crate::Result;
use domain::option::OptionType;

/// Calculate implied volatility using Newton-Raphson method
///
/// # Arguments
/// * `option_type` - Call or Put
/// * `s` - Underlying price
/// * `k` - Strike price
/// * `t` - Time to expiration in years
/// * `r` - Risk-free interest rate
/// * `q` - Dividend yield
/// * `market_price` - Market price of the option
/// * `max_iterations` - Maximum iterations (default: 100)
/// * `precision` - Convergence tolerance (default: 0.0001)
pub fn calculate_implied_volatility(
    option_type: OptionType,
    s: f64,
    k: f64,
    t: f64,
    r: f64,
    q: f64,
    market_price: f64,
    max_iterations: usize,
    precision: f64,
) -> Result<f64> {
    // Initial guess for volatility
    let mut sigma = 0.5; // Start with 50% volatility as initial guess

    for _ in 0..max_iterations {
        // Calculate price with current sigma
        let price = black_scholes_price(option_type, s, k, t, r, sigma, q)?;

        // Calculate vega for Newton-Raphson step
        let vega = calculate_vega_approx(option_type, s, k, t, r, sigma, q)?;

        // Price difference
        let diff = price - market_price;

        // Check convergence
        if diff.abs() < precision {
            return Ok(sigma);
        }

        // Newton-Raphson update
        if vega.abs() < 1e-10 {
            return Err(OptionError::IvConvergenceFailed);
        }

        sigma = sigma - diff / vega;

        // Ensure sigma stays positive
        if sigma <= 0.0 {
            sigma = 0.01;
        }
    }

    Err(OptionError::IvConvergenceFailed)
}

/// Approximate vega for IV calculation
fn calculate_vega_approx(
    option_type: OptionType,
    s: f64,
    k: f64,
    t: f64,
    r: f64,
    sigma: f64,
    q: f64,
) -> Result<f64> {
    let d1 =
        ((s * (1.0 - q)).ln() - k.ln() + (r - q + 0.5 * sigma * sigma) * t) / (sigma * t.sqrt());

    let sqrt_t = t.sqrt();
    let vega = s * (1.0 - q) * norm_pdf(d1) * sqrt_t;

    Ok(vega)
}

/// Standard normal PDF
fn norm_pdf(x: f64) -> f64 {
    (-0.5 * x * x).exp() / (2.0 * std::f64::consts::PI).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iv_calculation() {
        // Test IV calculation: if we price with 20% vol, we should get back ~20% IV
        let s = 100.0;
        let k = 100.0;
        let t = 1.0;
        let r = 0.05;
        let q = 0.0;
        let true_sigma = 0.20;

        let market_price =
            black_scholes_price(OptionType::Call, s, k, t, r, true_sigma, q).unwrap();

        let iv = calculate_implied_volatility(
            OptionType::Call,
            s,
            k,
            t,
            r,
            q,
            market_price,
            100,
            0.0001,
        )
        .unwrap();

        assert!(
            (iv - true_sigma).abs() < 0.01,
            "IV {} != {}",
            iv,
            true_sigma
        );
    }
}
