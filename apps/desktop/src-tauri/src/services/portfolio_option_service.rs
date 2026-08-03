// Portfolio option service — integrates option positions with portfolio risk analysis

use crate::database::repositories::option_position_repository::OptionPositionRepository;
use crate::database::repositories::portfolio_repository::PortfolioRepository;
use crate::error::AppError;
use domain::option::OptionPosition;

pub struct PortfolioOptionService {
    position_repo: OptionPositionRepository,
    #[allow(dead_code)]
    portfolio_repo: PortfolioRepository,
}

impl PortfolioOptionService {
    pub fn new(
        position_repo: OptionPositionRepository,
        portfolio_repo: PortfolioRepository,
    ) -> Self {
        Self {
            position_repo,
            portfolio_repo,
        }
    }

    /// List all option positions for a workspace
    pub async fn list_option_positions(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<OptionPosition>, AppError> {
        self.position_repo.list_by_workspace(workspace_id).await
    }

    /// Calculate aggregate Greeks for all option positions in a workspace
    pub async fn calculate_aggregate_greeks(
        &self,
        workspace_id: &str,
    ) -> Result<AggregateGreeks, AppError> {
        let positions = self.list_option_positions(workspace_id).await?;

        // Simplified aggregation - would use option-core in production
        let mut net_delta = 0.0;
        let mut net_gamma = 0.0;
        let mut net_theta = 0.0;
        let mut net_vega = 0.0;

        // Placeholder calculation
        for position in &positions {
            let quantity = position.quantity as f64;
            net_delta += quantity * 0.5; // Simplified
            net_gamma += quantity * 0.02;
            net_theta += quantity * -5.0;
            net_vega += quantity * 10.0;
        }

        Ok(AggregateGreeks {
            net_delta,
            net_gamma,
            net_theta,
            net_vega,
            position_count: positions.len(),
        })
    }

    /// Get option exposure summary for portfolio
    pub async fn get_option_exposure(
        &self,
        workspace_id: &str,
    ) -> Result<OptionExposure, AppError> {
        let positions = self.list_option_positions(workspace_id).await?;

        let total_cost = positions
            .iter()
            .map(|p| p.cost_basis * p.quantity as f64)
            .sum();

        Ok(OptionExposure {
            total_positions: positions.len(),
            total_cost_basis: total_cost,
            open_positions: positions.iter().filter(|p| p.closed_at.is_none()).count(),
        })
    }
}

pub struct AggregateGreeks {
    pub net_delta: f64,
    pub net_gamma: f64,
    pub net_theta: f64,
    pub net_vega: f64,
    pub position_count: usize,
}

pub struct OptionExposure {
    pub total_positions: usize,
    pub total_cost_basis: f64,
    pub open_positions: usize,
}
