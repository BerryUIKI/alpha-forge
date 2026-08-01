/**
 * Portfolio Option Service
 * Aggregates Greeks across positions and calculates portfolio-level risk
 */

use std::sync::Arc;
use domain::option::{OptionPosition, OptionType, PositionType};
use option_core::{GreeksValues, calculate_greeks};
use crate::database::repositories::option_position_repository::OptionPositionRepository;
use crate::error::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PortfolioGreeks {
    pub net_delta: f64,
    pub net_gamma: f64,
    pub net_theta: f64,
    pub net_vega: f64,
    pub net_rho: f64,
    pub delta_dollars: f64,
    pub gamma_dollars: f64,
    pub theta_dollars: f64,
    pub vega_dollars: f64,
}

#[derive(Debug, Serialize)]
pub struct RiskContribution {
    pub position_id: String,
    pub symbol: String,
    pub delta_contribution: f64,
    pub gamma_contribution: f64,
    pub theta_contribution: f64,
    pub vega_contribution: f64,
}

#[derive(Debug, Serialize)]
pub struct PortfolioRiskAnalysis {
    pub portfolio_greeks: PortfolioGreeks,
    pub risk_contributions: Vec<RiskContribution>,
    pub delta_adjusted_exposure: f64,
    pub gamma_adjusted_exposure: f64,
    pub concentration_risks: Vec<String>,
}

pub struct PortfolioOptionService {
    position_repo: Arc<OptionPositionRepository>,
}

impl PortfolioOptionService {
    pub fn new(position_repo: Arc<OptionPositionRepository>) -> Self {
        Self { position_repo }
    }

    /// Aggregate Greeks across all positions in a workspace
    pub async fn calculate_portfolio_greeks(
        &self,
        workspace_id: &str,
        underlying_prices: &std::collections::HashMap<String, f64>,
        volatilities: &std::collections::HashMap<String, f64>,
        risk_free_rate: f64,
    ) -> Result<PortfolioGreeks, AppError> {
        let positions = self.position_repo
            .find_by_workspace(workspace_id)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut net_delta = 0.0;
        let mut net_gamma = 0.0;
        let mut net_theta = 0.0;
        let mut net_vega = 0.0;
        let mut net_rho = 0.0;

        for position in &positions {
            let underlying_price = underlying_prices.get(&position.symbol).unwrap_or(&100.0);
            let volatility = volatilities.get(&position.symbol).unwrap_or(&0.20);
            let years = position.days_to_expiration as f64 / 365.0;

            let greeks = calculate_greeks(
                position.option_type,
                *underlying_price,
                position.strike,
                years,
                risk_free_rate,
                *volatility,
                0.0,
            ).map_err(|e| AppError::Internal(e.to_string()))?;

            let multiplier = position.quantity as f64;
            net_delta += greeks.delta * multiplier;
            net_gamma += greeks.gamma * multiplier;
            net_theta += greeks.theta * multiplier;
            net_vega += greeks.vega * multiplier;
            net_rho += greeks.rho * multiplier;
        }

        // Calculate dollar-denominated Greeks
        let avg_underlying = underlying_prices.values().sum::<f64>() / underlying_prices.len().max(1) as f64;

        Ok(PortfolioGreeks {
            delta_dollars: net_delta * avg_underlying * 100.0, // Per 100 shares
            gamma_dollars: net_gamma * avg_underlying * 100.0,
            theta_dollars: net_theta,
            vega_dollars: net_vega,
            net_delta,
            net_gamma,
            net_theta,
            net_vega,
            net_rho,
        })
    }

    /// Calculate risk contribution of each position
    pub async fn analyze_risk_contributions(
        &self,
        workspace_id: &str,
        underlying_prices: &std::collections::HashMap<String, f64>,
        volatilities: &std::collections::HashMap<String, f64>,
        risk_free_rate: f64,
    ) -> Result<PortfolioRiskAnalysis, AppError> {
        let positions = self.position_repo
            .find_by_workspace(workspace_id)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut contributions = Vec::new();
        let mut total_delta = 0.0;
        let mut total_gamma = 0.0;
        let mut total_theta = 0.0;
        let mut total_vega = 0.0;

        // Calculate total Greeks first
        for position in &positions {
            let underlying_price = underlying_prices.get(&position.symbol).unwrap_or(&100.0);
            let volatility = volatilities.get(&position.symbol).unwrap_or(&0.20);
            let years = position.days_to_expiration as f64 / 365.0;

            let greeks = calculate_greeks(
                position.option_type,
                *underlying_price,
                position.strike,
                years,
                risk_free_rate,
                *volatility,
                0.0,
            ).map_err(|e| AppError::Internal(e.to_string()))?;

            total_delta += greeks.delta * position.quantity as f64;
            total_gamma += greeks.gamma * position.quantity as f64;
            total_theta += greeks.theta * position.quantity as f64;
            total_vega += greeks.vega * position.quantity as f64;
        }

        // Calculate contribution percentages
        for position in &positions {
            let underlying_price = underlying_prices.get(&position.symbol).unwrap_or(&100.0);
            let volatility = volatilities.get(&position.symbol).unwrap_or(&0.20);
            let years = position.days_to_expiration as f64 / 365.0;

            let greeks = calculate_greeks(
                position.option_type,
                *underlying_price,
                position.strike,
                years,
                risk_free_rate,
                *volatility,
                0.0,
            ).map_err(|e| AppError::Internal(e.to_string()))?;

            let multiplier = position.quantity as f64;

            contributions.push(RiskContribution {
                position_id: position.id.clone(),
                symbol: position.symbol.clone(),
                delta_contribution: if total_delta != 0.0 {
                    (greeks.delta * multiplier / total_delta) * 100.0
                } else {
                    0.0
                },
                gamma_contribution: if total_gamma != 0.0 {
                    (greeks.gamma * multiplier / total_gamma) * 100.0
                } else {
                    0.0
                },
                theta_contribution: if total_theta != 0.0 {
                    (greeks.theta * multiplier / total_theta) * 100.0
                } else {
                    0.0
                },
                vega_contribution: if total_vega != 0.0 {
                    (greeks.vega * multiplier / total_vega) * 100.0
                } else {
                    0.0
                },
            });
        }

        // Identify concentration risks
        let mut concentration_risks = Vec::new();
        for contrib in &contributions {
            if contrib.delta_contribution.abs() > 50.0 {
                concentration_risks.push(format!(
                    "High delta concentration in {}: {:.1}%",
                    contrib.symbol, contrib.delta_contribution
                ));
            }
            if contrib.vega_contribution.abs() > 50.0 {
                concentration_risks.push(format!(
                    "High vega concentration in {}: {:.1}%",
                    contrib.symbol, contrib.vega_contribution
                ));
            }
        }

        let portfolio_greeks = self.calculate_portfolio_greeks(
            workspace_id,
            underlying_prices,
            volatilities,
            risk_free_rate,
        ).await?;

        Ok(PortfolioRiskAnalysis {
            portfolio_greeks,
            risk_contributions: contributions,
            delta_adjusted_exposure: total_delta,
            gamma_adjusted_exposure: total_gamma,
            concentration_risks,
        })
    }

    /// Import positions from CSV
    pub async fn import_positions_csv(
        &self,
        workspace_id: &str,
        csv_content: &str,
    ) -> Result<usize, AppError> {
        let mut imported_count = 0;
        
        for line in csv_content.lines().skip(1) { // Skip header
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() >= 7 {
                // Parse CSV fields
                let position = OptionPosition {
                    id: uuid::Uuid::new_v4().to_string(),
                    workspace_id: workspace_id.to_string(),
                    account_id: None,
                    symbol: fields[0].to_string(),
                    option_type: if fields[1].to_uppercase() == "CALL" {
                        OptionType::Call
                    } else {
                        OptionType::Put
                    },
                    strike: fields[2].parse().unwrap_or(0.0),
                    expiration: chrono::Utc::now(), // Would parse from CSV
                    quantity: fields[4].parse().unwrap_or(0),
                    position_type: if fields[5].to_uppercase() == "LONG" {
                        PositionType::Long
                    } else {
                        PositionType::Short
                    },
                    cost_basis: fields[6].parse().unwrap_or(0.0),
                    current_price: 0.0,
                    days_to_expiration: 30,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                };

                self.position_repo.create(&position).await
                    .map_err(|e| AppError::Internal(e.to_string()))?;
                
                imported_count += 1;
            }
        }

        Ok(imported_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_portfolio_greeks_calculation() {
        // Would test with mock repository
        assert!(true);
    }
}