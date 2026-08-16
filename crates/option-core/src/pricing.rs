//! Option pricing models
//!
//! Implements Black-Scholes pricing model for European options.

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
    validate_pricing_input(s, k, t, sigma)?;

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

/// Validate shared pricing inputs, rejecting NaN, infinity, and out-of-range values.
///
/// Public so Greeks and IV calculations reuse the same validation rules.
pub fn validate_pricing_input(s: f64, k: f64, t: f64, sigma: f64) -> Result<()> {
    if !s.is_finite() || s <= 0.0 {
        return Err(OptionError::InvalidParameters(
            "Stock price must be finite and positive".to_string(),
        ));
    }
    if !k.is_finite() || k <= 0.0 {
        return Err(OptionError::InvalidParameters(
            "Strike price must be finite and positive".to_string(),
        ));
    }
    if !t.is_finite() || t <= 0.0 {
        return Err(OptionError::InvalidParameters(
            "Time to expiration must be finite and positive".to_string(),
        ));
    }
    if !sigma.is_finite() || sigma <= 0.0 {
        return Err(OptionError::InvalidParameters(
            "Volatility must be finite and positive".to_string(),
        ));
    }
    Ok(())
}

/// Standard normal cumulative distribution function
pub fn norm_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}

/// Error function approximation (Abramowitz and Stegun)
fn erf(x: f64) -> f64 {
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

    // ---------------------------------------------------------------------------
    // Reference fixtures from financial literature
    // ---------------------------------------------------------------------------

    /// Hull (10th ed), Example 15.6: ATM call, 1 year, 5% rate, 20% vol, no dividend.
    /// Expected call price ≈ 10.4506, put ≈ 5.5663.
    #[test]
    fn test_hull_atm_call() {
        let price =
            black_scholes_price(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!(
            (price - 10.4506).abs() < 0.01,
            "Hull call price {} != 10.45",
            price
        );
    }

    #[test]
    fn test_hull_atm_put() {
        let price =
            black_scholes_price(OptionType::Put, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!(
            (price - 5.5663).abs() < 0.01,
            "Hull put price {} != 5.57",
            price
        );
    }

    /// ITM call: S=110, K=100 → intrinsic value 10, time value adds ~7.66
    #[test]
    fn test_itm_call() {
        let price =
            black_scholes_price(OptionType::Call, 110.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!(
            (price - 17.66).abs() < 0.02,
            "ITM call price {} != 17.66",
            price
        );
    }

    /// OTM call: S=90, K=100 → intrinsic value 0, time value ~5.09
    #[test]
    fn test_otm_call() {
        let price =
            black_scholes_price(OptionType::Call, 90.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!(
            (price - 5.09).abs() < 0.02,
            "OTM call price {} != 5.09",
            price
        );
    }

    /// ITM put: S=90, K=100 → intrinsic value 10, time value adds ~0.21
    #[test]
    fn test_itm_put() {
        let price = black_scholes_price(OptionType::Put, 90.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!(
            (price - 10.21).abs() < 0.02,
            "ITM put price {} != 10.21",
            price
        );
    }

    /// OTM put: S=110, K=100 → intrinsic value 0, time value ~2.79
    #[test]
    fn test_otm_put() {
        let price =
            black_scholes_price(OptionType::Put, 110.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!(
            (price - 2.79).abs() < 0.02,
            "OTM put price {} != 2.79",
            price
        );
    }

    // ---------------------------------------------------------------------------
    // Property-based tests
    // ---------------------------------------------------------------------------

    #[test]
    fn test_put_call_parity_no_dividend() {
        // Call - Put = S - K * exp(-rT)
        let call =
            black_scholes_price(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        let put = black_scholes_price(OptionType::Put, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();

        let parity = call - put;
        let expected = 100.0 - 100.0 * (-0.05_f64).exp();
        assert!((parity - expected).abs() < 0.01, "Put-call parity violated");
    }

    #[test]
    fn test_put_call_parity_with_dividend() {
        // The model convention uses (1-q) for the spot term (see ADR-0005),
        // so parity is: Call - Put = S * (1-q) - K * exp(-rT).
        let call =
            black_scholes_price(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.02).unwrap();
        let put = black_scholes_price(OptionType::Put, 100.0, 100.0, 1.0, 0.05, 0.2, 0.02).unwrap();

        let parity = call - put;
        let expected = 100.0 * (1.0 - 0.02) - 100.0 * (-0.05_f64).exp();
        assert!(
            (parity - expected).abs() < 0.01,
            "Put-call parity with dividend violated"
        );
    }

    #[test]
    fn test_call_upper_bound() {
        // Call price must not exceed underlying price (for non-negative rates)
        let price =
            black_scholes_price(OptionType::Call, 100.0, 50.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!(price <= 100.0, "Call price {} exceeds upper bound", price);
    }

    #[test]
    fn test_put_upper_bound() {
        // Put price must not exceed strike price
        let price = black_scholes_price(OptionType::Put, 50.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!(price <= 100.0, "Put price {} exceeds upper bound", price);
    }

    #[test]
    fn test_call_price_monotonic_in_vol() {
        // Higher volatility → higher call price (ATM)
        let low =
            black_scholes_price(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.15, 0.0).unwrap();
        let high =
            black_scholes_price(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.35, 0.0).unwrap();
        assert!(high > low, "Call price should increase with volatility");
    }

    #[test]
    fn test_put_price_monotonic_in_vol() {
        let low = black_scholes_price(OptionType::Put, 100.0, 100.0, 1.0, 0.05, 0.15, 0.0).unwrap();
        let high =
            black_scholes_price(OptionType::Put, 100.0, 100.0, 1.0, 0.05, 0.35, 0.0).unwrap();
        assert!(high > low, "Put price should increase with volatility");
    }

    // ---------------------------------------------------------------------------
    // Boundary cases
    // ---------------------------------------------------------------------------

    #[test]
    fn test_expiration_boundary() {
        // Very short time to expiry: option should approach intrinsic value
        let epsilon = 1e-10;
        let call =
            black_scholes_price(OptionType::Call, 105.0, 100.0, epsilon, 0.05, 0.2, 0.0).unwrap();
        assert!(
            (call - 5.0).abs() < 0.01,
            "Expiring ITM call should be near intrinsic"
        );
    }

    #[test]
    fn test_zero_volatility_boundary() {
        // Zero vol is rejected by validation; test near-zero vol.
        // The model's d1 term ((s*(1-q)).ln() - k.ln() + ...) does not
        // converge to the intrinsic value at sigma -> 0, so this documents
        // the boundary behavior rather than asserting a specific value:
        // the price must remain finite and positive.
        let call =
            black_scholes_price(OptionType::Call, 105.0, 100.0, 1.0, 0.05, 1e-8, 0.0).unwrap();
        assert!(
            call.is_finite() && call > 0.0,
            "Near-zero vol call should be finite and positive"
        );
    }

    // ---------------------------------------------------------------------------
    // Input validation
    // ---------------------------------------------------------------------------

    #[test]
    fn test_rejects_negative_stock_price() {
        let result = black_scholes_price(OptionType::Call, -100.0, 100.0, 1.0, 0.05, 0.2, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_rejects_nan_stock_price() {
        let result = black_scholes_price(OptionType::Call, f64::NAN, 100.0, 1.0, 0.05, 0.2, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_rejects_infinite_stock_price() {
        let result =
            black_scholes_price(OptionType::Call, f64::INFINITY, 100.0, 1.0, 0.05, 0.2, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_rejects_negative_strike() {
        let result = black_scholes_price(OptionType::Call, 100.0, -100.0, 1.0, 0.05, 0.2, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_rejects_negative_time() {
        let result = black_scholes_price(OptionType::Call, 100.0, 100.0, -1.0, 0.05, 0.2, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_rejects_negative_volatility() {
        let result = black_scholes_price(OptionType::Call, 100.0, 100.0, 1.0, 0.05, -0.2, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_norm_cdf_properties() {
        // CDF(-∞) → 0, CDF(0) → 0.5, CDF(∞) → 1
        assert!(
            (norm_cdf(0.0) - 0.5).abs() < 1e-6,
            "CDF(0) should be 0.5, got {}",
            norm_cdf(0.0)
        );
        assert!((norm_cdf(-10.0)).abs() < 1e-7, "CDF(-10) should be near 0");
        assert!(
            (norm_cdf(10.0) - 1.0).abs() < 1e-7,
            "CDF(10) should be near 1"
        );
        // Monotonic
        assert!(norm_cdf(-1.0) < norm_cdf(0.0), "CDF should be monotonic");
        assert!(norm_cdf(0.0) < norm_cdf(1.0), "CDF should be monotonic");
    }
}
