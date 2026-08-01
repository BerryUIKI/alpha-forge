/**
 * Scenario Analysis Service
 * Performs multi-factor stress testing and scenario analysis
 */

use domain::option::{OptionPosition, OptionType};
use option_core::{GreeksValues, calculate_greeks};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub underlying_shock: Option<f64>,      // % price change
    pub volatility_shock: Option<f64>,      // % IV change
    pub days_forward: Option<i64>,           // Time decay
    pub interest_rate_shock: Option<f64>,   // % rate change
}

#[derive(Debug, Serialize)]
pub struct ScenarioResult {
    pub scenario_name: String,
    pub new_position_value: f64,
    pub pnl: f64,
    pub pnl_percent: f64,
    pub new_greeks: GreeksValues,
}

pub struct ScenarioService;

impl ScenarioService {
    /// Run a single scenario analysis
    pub fn run_scenario(
        &self,
        positions: &[OptionPosition],
        scenario: &Scenario,
        current_underlying: f64,
        current_volatility: f64,
        current_rate: f64,
    ) -> Result<ScenarioResult, String> {
        // Calculate new parameters
        let new_underlying = match scenario.underlying_shock {
            Some(shock) => current_underlying * (1.0 + shock),
            None => current_underlying,
        };

        let new_volatility = match scenario.volatility_shock {
            Some(shock) => current_volatility * (1.0 + shock),
            None => current_volatility,
        };

        let new_rate = match scenario.interest_rate_shock {
            Some(shock) => current_rate + shock,
            None => current_rate,
        };

        // Calculate new position values
        let mut total_value = 0.0;
        let mut net_delta = 0.0;
        let mut net_gamma = 0.0;
        let mut net_theta = 0.0;
        let mut net_vega = 0.0;
        let mut net_rho = 0.0;

        for position in positions {
            // Calculate new option price and Greeks
            let years = position.days_to_expiration as f64 / 365.0;
            
            let greeks = calculate_greeks(
                position.option_type,
                new_underlying,
                position.strike,
                years,
                new_rate,
                new_volatility,
                0.0,
            )?;

            // Apply position multiplier
            let multiplier = position.quantity as f64;
            net_delta += greeks.delta * multiplier;
            net_gamma += greeks.gamma * multiplier;
            net_theta += greeks.theta * multiplier;
            net_vega += greeks.vega * multiplier;
            net_rho += greeks.rho * multiplier;

            // Calculate position value
            total_value += greeks.delta * position.quantity as f64; // Simplified
        }

        // Calculate P&L
        let original_value = positions.iter().map(|p| p.cost_basis).sum::<f64>();
        let pnl = total_value - original_value;
        let pnl_percent = if original_value != 0.0 {
            (pnl / original_value.abs()) * 100.0
        } else {
            0.0
        };

        Ok(ScenarioResult {
            scenario_name: scenario.name.clone(),
            new_position_value: total_value,
            pnl,
            pnl_percent,
            new_greeks: GreeksValues {
                delta: net_delta,
                gamma: net_gamma,
                theta: net_theta,
                vega: net_vega,
                rho: net_rho,
            },
        })
    }

    /// Run multiple scenarios and return results
    pub fn run_scenarios(
        &self,
        positions: &[OptionPosition],
        scenarios: &[Scenario],
        current_underlying: f64,
        current_volatility: f64,
        current_rate: f64,
    ) -> Result<Vec<ScenarioResult>, String> {
        scenarios
            .iter()
            .map(|scenario| {
                self.run_scenario(
                    positions,
                    scenario,
                    current_underlying,
                    current_volatility,
                    current_rate,
                )
            })
            .collect()
    }

    /// Generate predefined stress test scenarios
    pub fn get_standard_stress_scenarios() -> Vec<Scenario> {
        vec![
            Scenario {
                name: "Bullish +10%".to_string(),
                underlying_shock: Some(0.10),
                volatility_shock: None,
                days_forward: None,
                interest_rate_shock: None,
            },
            Scenario {
                name: "Bearish -10%".to_string(),
                underlying_shock: Some(-0.10),
                volatility_shock: None,
                days_forward: None,
                interest_rate_shock: None,
            },
            Scenario {
                name: "Vol Spike +50%".to_string(),
                underlying_shock: None,
                volatility_shock: Some(0.50),
                days_forward: None,
                interest_rate_shock: None,
            },
            Scenario {
                name: "Vol Crush -30%".to_string(),
                underlying_shock: None,
                volatility_shock: Some(-0.30),
                days_forward: None,
                interest_rate_shock: None,
            },
            Scenario {
                name: "Time Decay 7 days".to_string(),
                underlying_shock: None,
                volatility_shock: None,
                days_forward: Some(7),
                interest_rate_shock: None,
            },
            Scenario {
                name: "Market Crash -20% + Vol +100%".to_string(),
                underlying_shock: Some(-0.20),
                volatility_shock: Some(1.00),
                days_forward: None,
                interest_rate_shock: None,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::option::PositionType;

    #[test]
    fn test_standard_scenarios() {
        let scenarios = ScenarioService::get_standard_stress_scenarios();
        assert_eq!(scenarios.len(), 6);
        assert!(scenarios.iter().any(|s| s.name == "Bullish +10%"));
    }
}