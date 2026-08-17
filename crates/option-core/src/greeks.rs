//! Greeks calculations for options
//!
//! Implements analytical Greeks (Delta, Gamma, Theta, Vega, Rho) for European options.

use crate::pricing::norm_cdf;
use crate::OptionError;
use crate::Result;
use domain::option::OptionType;

/// Greeks structure holding all five first-order Greeks
#[derive(Debug, Clone, Copy)]
pub struct GreeksValues {
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
}

/// Calculate all Greeks for an option
///
/// # Arguments
/// * `option_type` - Call or Put
/// * `s` - Underlying price
/// * `k` - Strike price
/// * `t` - Time to expiration in years
/// * `r` - Risk-free interest rate
/// * `sigma` - Volatility
/// * `q` - Dividend yield
pub fn calculate_greeks(
    option_type: OptionType,
    s: f64,
    k: f64,
    t: f64,
    r: f64,
    sigma: f64,
    q: f64,
) -> Result<GreeksValues> {
    crate::pricing::validate_pricing_input(s, k, t, sigma)?;

    let d1 =
        ((s * (1.0 - q)).ln() - k.ln() + (r - q + 0.5 * sigma * sigma) * t) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();

    let delta = calculate_delta(option_type, d1, q)?;
    let gamma = calculate_gamma(s, sigma, t, q, d1)?;
    let theta = calculate_theta(option_type, s, k, t, r, sigma, q, d1, d2)?;
    let vega = calculate_vega(s, t, q, d1)?;
    let rho = calculate_rho(option_type, k, t, r, d2)?;

    Ok(GreeksValues {
        delta,
        gamma,
        theta,
        vega,
        rho,
    })
}

/// Calculate Delta (∂V/∂S)
fn calculate_delta(option_type: OptionType, d1: f64, q: f64) -> Result<f64> {
    match option_type {
        OptionType::Call => Ok((1.0 - q) * norm_cdf(d1)),
        OptionType::Put => Ok((1.0 - q) * (norm_cdf(d1) - 1.0)),
    }
}

/// Calculate Gamma (∂²V/∂S²)
///
/// Uses the same `d1` as pricing so the rate, strike, and dividend terms are
/// included. The previous implementation recomputed `d1` without the strike or
/// rate terms, which produced numerically meaningless gamma values.
fn calculate_gamma(s: f64, sigma: f64, t: f64, q: f64, d1: f64) -> Result<f64> {
    if s <= 0.0 || sigma <= 0.0 || t <= 0.0 {
        return Err(OptionError::InvalidParameters(
            "Invalid parameters for gamma".to_string(),
        ));
    }

    let gamma = (1.0 - q) * norm_pdf(d1) / (s * sigma * t.sqrt());

    Ok(gamma)
}

/// Calculate Theta (∂V/∂t) - per day
fn calculate_theta(
    option_type: OptionType,
    s: f64,
    k: f64,
    t: f64,
    r: f64,
    sigma: f64,
    q: f64,
    d1: f64,
    d2: f64,
) -> Result<f64> {
    let sqrt_t = t.sqrt();

    match option_type {
        OptionType::Call => {
            let theta = -s * (1.0 - q) * norm_pdf(d1) * sigma / (2.0 * sqrt_t)
                - r * k * (-r * t).exp() * norm_cdf(d2)
                + q * s * (1.0 - q) * norm_cdf(d1);
            Ok(theta / 365.0) // Convert to per-day
        }
        OptionType::Put => {
            let theta = -s * (1.0 - q) * norm_pdf(d1) * sigma / (2.0 * sqrt_t)
                + r * k * (-r * t).exp() * norm_cdf(-d2)
                - q * s * (1.0 - q) * norm_cdf(-d1);
            Ok(theta / 365.0)
        }
    }
}

/// Calculate Vega (∂V/∂σ) - per 1% IV change
fn calculate_vega(s: f64, t: f64, q: f64, d1: f64) -> Result<f64> {
    let vega = s * (1.0 - q) * norm_pdf(d1) * t.sqrt();
    Ok(vega / 100.0) // Convert to per 1% change
}

/// Calculate Rho (∂V/∂r) - per 1% rate change
fn calculate_rho(option_type: OptionType, k: f64, t: f64, r: f64, d2: f64) -> Result<f64> {
    match option_type {
        OptionType::Call => {
            let rho = k * t * (-r * t).exp() * norm_cdf(d2);
            Ok(rho / 100.0) // Convert to per 1% change
        }
        OptionType::Put => {
            let rho = -k * t * (-r * t).exp() * norm_cdf(-d2);
            Ok(rho / 100.0)
        }
    }
}

/// Standard normal probability density function
fn norm_pdf(x: f64) -> f64 {
    (-0.5 * x * x).exp() / (2.0 * std::f64::consts::PI).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------------------------------------------------------------------------
    // Reference fixtures
    // ---------------------------------------------------------------------------

    /// Hull-style ATM call: S=100, K=100, T=1, r=5%, σ=20%, q=0
    /// Reference values computed with the correct standard normal density:
    ///   Delta(call) ≈ 0.637, Put ≈ -0.363
    ///   Gamma ≈ 0.019
    ///   Theta(call) ≈ -0.020 (per day), Put ≈ -0.006
    ///   Vega(call) ≈ 0.375 (per 1% vol)
    ///   Rho(call) ≈ 0.532 (per 1% rate), Put ≈ -0.419
    #[test]
    fn test_hull_atm_call_greeks() {
        let g = calculate_greeks(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!((g.delta - 0.637).abs() < 0.01, "Delta call {}", g.delta);
        assert!((g.gamma - 0.019).abs() < 0.005, "Gamma {}", g.gamma);
        assert!(g.theta < 0.0, "Theta should be negative for long call");
        assert!(g.vega > 0.0, "Vega should be positive");
        assert!((g.rho - 0.532).abs() < 0.02, "Rho call {}", g.rho);
    }

    #[test]
    fn test_hull_atm_put_greeks() {
        let g = calculate_greeks(OptionType::Put, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!((g.delta - (-0.363)).abs() < 0.01, "Delta put {}", g.delta);
        assert!((g.gamma - 0.019).abs() < 0.005, "Gamma put {}", g.gamma);
        assert!((g.rho - (-0.419)).abs() < 0.02, "Rho put {}", g.rho);
    }

    // ---------------------------------------------------------------------------
    // Property-based Greeks tests
    // ---------------------------------------------------------------------------

    #[test]
    fn test_gamma_positive() {
        // Gamma is always positive for long options (European)
        let g = calculate_greeks(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!(g.gamma > 0.0, "Gamma should be positive");
    }

    #[test]
    fn test_theta_negative_for_long_call() {
        let g = calculate_greeks(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!(g.theta < 0.0, "Theta should be negative for long call");
    }

    #[test]
    fn test_vega_positive() {
        // Vega is always positive for long options
        let g = calculate_greeks(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!(g.vega > 0.0, "Vega should be positive");
    }

    #[test]
    fn test_call_delta_increasing_with_price() {
        // Delta increases as underlying price increases (OTM → ATM → ITM)
        let otm = calculate_greeks(OptionType::Call, 80.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        let at = calculate_greeks(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        let itm = calculate_greeks(OptionType::Call, 120.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!(
            otm.delta < at.delta,
            "OTM delta {} < ATM delta {}",
            otm.delta,
            at.delta
        );
        assert!(
            at.delta < itm.delta,
            "ATM delta {} < ITM delta {}",
            at.delta,
            itm.delta
        );
    }

    #[test]
    fn test_put_delta_decreasing_with_price() {
        // Put delta decreases (becomes more negative) as price increases
        let otm = calculate_greeks(OptionType::Put, 120.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        let at = calculate_greeks(OptionType::Put, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        let itm = calculate_greeks(OptionType::Put, 80.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        // OTM delta ≈ -0.16, ATM ≈ -0.36, ITM ≈ -0.64
        // So OTM > ATM > ITM (closer to 0 for OTM)
        assert!(
            otm.delta > at.delta,
            "OTM delta {} > ATM delta {}",
            otm.delta,
            at.delta
        );
        assert!(
            at.delta > itm.delta,
            "ATM delta {} > ITM delta {}",
            at.delta,
            itm.delta
        );
    }

    #[test]
    fn test_delta_call_put_relationship() {
        // For same parameters: delta_call - delta_put = exp(-qT) * exp(-rT) ... approximately 1
        // More precisely: Call delta - Put delta = (1-q) * (norm_cdf(d1) - (norm_cdf(d1)-1)) = 1-q
        let call_g = calculate_greeks(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        let put_g = calculate_greeks(OptionType::Put, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        let diff = call_g.delta - put_g.delta;
        // With q=0, this should be approximately 1.0
        assert!((diff - 1.0).abs() < 0.01, "Delta call-put {} != 1.0", diff);
    }

    #[test]
    fn test_gamma_identical_for_call_and_put() {
        // Gamma is the same for call and put with identical parameters
        let call_g = calculate_greeks(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        let put_g = calculate_greeks(OptionType::Put, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!(
            (call_g.gamma - put_g.gamma).abs() < 1e-10,
            "Gamma call {} != put {}",
            call_g.gamma,
            put_g.gamma
        );
    }

    #[test]
    fn test_gamma_peaks_near_atm() {
        // Gamma is highest near the money
        let otm = calculate_greeks(OptionType::Call, 80.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        let at = calculate_greeks(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        let itm = calculate_greeks(OptionType::Call, 120.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!(
            otm.gamma < at.gamma,
            "OTM gamma {} < ATM gamma {}",
            otm.gamma,
            at.gamma
        );
        assert!(
            itm.gamma < at.gamma,
            "ITM gamma {} < ATM gamma {}",
            itm.gamma,
            at.gamma
        );
    }

    #[test]
    fn test_vega_peaks_near_atm() {
        let otm = calculate_greeks(OptionType::Call, 80.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        let at = calculate_greeks(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        let itm = calculate_greeks(OptionType::Call, 120.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!(
            otm.vega < at.vega,
            "OTM vega {} < ATM vega {}",
            otm.vega,
            at.vega
        );
        assert!(
            itm.vega < at.vega,
            "ITM vega {} < ATM vega {}",
            itm.vega,
            at.vega
        );
    }

    // ---------------------------------------------------------------------------
    // Input validation
    // ---------------------------------------------------------------------------

    #[test]
    fn test_greeks_rejects_invalid_parameters() {
        let result = calculate_greeks(OptionType::Call, -100.0, 100.0, 1.0, 0.05, 0.2, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_greeks_rejects_nan() {
        let result = calculate_greeks(OptionType::Call, f64::NAN, 100.0, 1.0, 0.05, 0.2, 0.0);
        assert!(result.is_err());
    }
}
