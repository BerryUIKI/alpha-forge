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
    let d1 =
        ((s * (1.0 - q)).ln() - k.ln() + (r - q + 0.5 * sigma * sigma) * t) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();

    let delta = calculate_delta(option_type, d1, q, t)?;
    let gamma = calculate_gamma(s, sigma, t, q)?;
    let theta = calculate_theta(option_type, s, k, t, r, sigma, q, d1, d2)?;
    let vega = calculate_vega(s, t, sigma, q, d1)?;
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
fn calculate_delta(option_type: OptionType, d1: f64, q: f64, _t: f64) -> Result<f64> {
    match option_type {
        OptionType::Call => Ok((1.0 - q) * norm_cdf(d1)),
        OptionType::Put => Ok((1.0 - q) * (norm_cdf(d1) - 1.0)),
    }
}

/// Calculate Gamma (∂²V/∂S²)
fn calculate_gamma(s: f64, sigma: f64, t: f64, q: f64) -> Result<f64> {
    if s <= 0.0 || sigma <= 0.0 || t <= 0.0 {
        return Err(OptionError::InvalidParameters(
            "Invalid parameters for gamma".to_string(),
        ));
    }

    let d1 = (s.ln() + (0.5 * sigma * sigma) * t) / (sigma * t.sqrt());
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
fn calculate_vega(s: f64, t: f64, _sigma: f64, q: f64, d1: f64) -> Result<f64> {
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
    (-0.5 * x * x).exp() / (std::f64::consts::SQRT_2 * std::f64::consts::PI).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_atm_call() {
        // ATM call should have delta ≈ 0.6 with rates and vol
        let greeks = calculate_greeks(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        // Delta ≈ 0.64 with 5% rate and 20% vol
        assert!(
            (greeks.delta - 0.6).abs() < 0.1,
            "Delta {} not close to 0.6",
            greeks.delta
        );
    }

    #[test]
    fn test_gamma_positive() {
        // Gamma is always positive
        let greeks = calculate_greeks(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!(greeks.gamma > 0.0, "Gamma should be positive");
    }

    #[test]
    fn test_theta_negative() {
        // Theta is typically negative for long options (time decay)
        let greeks = calculate_greeks(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!(
            greeks.theta < 0.0,
            "Theta {} should be negative for long call",
            greeks.theta
        );
    }

    #[test]
    fn test_vega_positive() {
        // Vega is always positive for long options
        let greeks = calculate_greeks(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap();
        assert!(greeks.vega > 0.0, "Vega should be positive");
    }
}
