// Strategy service — orchestrates strategy analysis and validation

use std::sync::Arc;

use crate::database::repositories::option_strategy_repository::OptionStrategyRepository;
use crate::database::repositories::strategy_leg_repository::StrategyLegRepository;
use crate::error::AppError;
use domain::option::{OptionStrategy, StrategyLeg};
use option_core::calculate_payoff_at_expiry;

pub struct StrategyService {
    strategy_repo: Arc<OptionStrategyRepository>,
    leg_repo: Arc<StrategyLegRepository>,
}

impl StrategyService {
    pub fn new(
        strategy_repo: Arc<OptionStrategyRepository>,
        leg_repo: Arc<StrategyLegRepository>,
    ) -> Self {
        Self {
            strategy_repo,
            leg_repo,
        }
    }

    /// Validate strategy legs for consistency
    pub fn validate_strategy(&self, legs: &[StrategyLeg]) -> Result<ValidationResult, AppError> {
        if legs.is_empty() {
            return Err(AppError::Validation(
                "Strategy must have at least one leg".to_string(),
            ));
        }

        // Check for valid quantities
        for leg in legs {
            if leg.quantity == 0 {
                return Err(AppError::Validation(
                    "Leg quantity cannot be zero".to_string(),
                ));
            }
        }

        Ok(ValidationResult {
            valid: true,
            warnings: vec![],
        })
    }

    /// Calculate strategy payoff at expiry
    pub fn calculate_payoff(
        &self,
        legs: &[StrategyLeg],
        underlying_price: f64,
    ) -> Result<f64, AppError> {
        // Convert domain::option::StrategyLeg to option_core::StrategyLeg
        let core_legs: Vec<option_core::StrategyLeg> = legs
            .iter()
            .map(|leg| option_core::StrategyLeg {
                option_type: leg.option_type,
                strike: leg.strike,
                expiration: 1.0, // Default for expiry calculation
                quantity: leg.quantity,
                premium: leg.premium,
            })
            .collect();

        let payoff = calculate_payoff_at_expiry(&core_legs, underlying_price)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(payoff)
    }

    /// Create and persist a new strategy with legs
    pub async fn create_strategy(
        &self,
        strategy: &OptionStrategy,
        legs: &[StrategyLeg],
    ) -> Result<(), AppError> {
        // Validate
        self.validate_strategy(legs)?;

        // Persist strategy
        self.strategy_repo.create(strategy).await?;

        // Persist legs
        for leg in legs {
            self.leg_repo.create(leg).await?;
        }

        Ok(())
    }
}

pub struct ValidationResult {
    pub valid: bool,
    pub warnings: Vec<String>,
}
