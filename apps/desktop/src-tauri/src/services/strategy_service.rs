// Strategy service — orchestrates strategy analysis and validation

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::database::repositories::option_contract_repository::OptionContractRepository;
use crate::database::repositories::option_strategy_repository::OptionStrategyRepository;
use crate::database::repositories::strategy_leg_repository::StrategyLegRepository;
use crate::error::AppError;
use chrono::Utc;
use domain::option::{OptionStrategy, PositionType, StrategyLeg, StrategyType};
use option_core::calculate_payoff_at_expiry;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateStrategyLegInput {
    pub contract_id: String,
    pub quantity: i32,
    pub position_type: PositionType,
}

#[derive(Debug, Clone)]
pub struct CreateStrategyInput {
    pub workspace_id: String,
    pub name: String,
    pub strategy_type: StrategyType,
    pub legs: Vec<CreateStrategyLegInput>,
}

#[derive(Debug, Clone)]
pub struct StrategyWithLegs {
    pub strategy: OptionStrategy,
    pub legs: Vec<StrategyLeg>,
}

pub struct StrategyService {
    strategy_repo: Arc<OptionStrategyRepository>,
    leg_repo: Arc<StrategyLegRepository>,
    contract_repo: Arc<OptionContractRepository>,
}

impl StrategyService {
    pub fn new(
        strategy_repo: Arc<OptionStrategyRepository>,
        leg_repo: Arc<StrategyLegRepository>,
        contract_repo: Arc<OptionContractRepository>,
    ) -> Self {
        Self {
            strategy_repo,
            leg_repo,
            contract_repo,
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
            if leg.quantity <= 0 {
                return Err(AppError::Validation(
                    "Leg quantity must be positive".to_string(),
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

    /// Creates a strategy from contract references and persists it atomically.
    pub async fn create_strategy(
        &self,
        input: CreateStrategyInput,
    ) -> Result<StrategyWithLegs, AppError> {
        let workspace_id = input.workspace_id.trim();
        let name = input.name.trim();
        if Uuid::parse_str(workspace_id).is_err() {
            return Err(AppError::Validation(
                "Workspace ID must be a UUID".to_string(),
            ));
        }
        if name.is_empty() || name.chars().count() > 100 {
            return Err(AppError::Validation(
                "Strategy name must contain 1 to 100 characters".to_string(),
            ));
        }
        if input.legs.is_empty() {
            return Err(AppError::Validation(
                "Strategy must have at least one leg".to_string(),
            ));
        }

        let strategy_id = Uuid::new_v4().to_string();
        let mut seen_contracts = HashSet::new();
        let mut underlying: Option<String> = None;
        let mut total_cost = 0.0;
        let mut legs = Vec::with_capacity(input.legs.len());

        for leg_input in input.legs {
            if leg_input.quantity <= 0 {
                return Err(AppError::Validation(
                    "Leg quantity must be positive".to_string(),
                ));
            }
            if Uuid::parse_str(&leg_input.contract_id).is_err()
                || !seen_contracts.insert(leg_input.contract_id.clone())
            {
                return Err(AppError::Validation(
                    "Each leg must reference a unique contract UUID".to_string(),
                ));
            }

            let contract = self
                .contract_repo
                .get(&leg_input.contract_id)
                .await?
                .ok_or_else(|| {
                    AppError::NotFound(format!(
                        "Option contract '{}' not found",
                        leg_input.contract_id
                    ))
                })?;
            if contract.workspace_id != workspace_id {
                return Err(AppError::Validation(
                    "Strategy contracts must belong to the selected workspace".to_string(),
                ));
            }
            if underlying
                .as_ref()
                .is_some_and(|symbol| symbol != &contract.symbol)
            {
                return Err(AppError::Validation(
                    "Strategy contracts must share one underlying symbol".to_string(),
                ));
            }
            underlying.get_or_insert_with(|| contract.symbol.clone());

            // Long legs pay the ask; short legs receive the bid.
            let premium = match leg_input.position_type {
                PositionType::Long => contract.ask,
                PositionType::Short => contract.bid,
            };
            let signed_cost =
                premium * f64::from(leg_input.quantity) * f64::from(contract.contract_multiplier);
            total_cost += match leg_input.position_type {
                PositionType::Long => signed_cost,
                PositionType::Short => -signed_cost,
            };
            legs.push(StrategyLeg {
                id: Uuid::new_v4().to_string(),
                strategy_id: strategy_id.clone(),
                option_contract_id: contract.id,
                quantity: leg_input.quantity,
                position_type: leg_input.position_type,
                premium,
                strike: contract.strike,
                expiration: contract.expiration,
                option_type: contract.option_type,
            });
        }

        if !total_cost.is_finite() {
            return Err(AppError::Validation(
                "Strategy total cost must be finite".to_string(),
            ));
        }
        self.validate_strategy(&legs)?;
        let now = Utc::now();
        let strategy = OptionStrategy {
            id: strategy_id,
            workspace_id: workspace_id.to_string(),
            name: name.to_string(),
            strategy_type: input.strategy_type,
            underlying: underlying.unwrap_or_default(),
            total_cost,
            max_profit: None,
            max_loss: None,
            break_even_points: Vec::new(),
            created_at: now,
            updated_at: now,
        };
        self.strategy_repo
            .create_with_legs(&strategy, &legs)
            .await?;

        Ok(StrategyWithLegs { strategy, legs })
    }

    pub async fn get_strategy(&self, id: &str) -> Result<StrategyWithLegs, AppError> {
        let strategy = self
            .strategy_repo
            .get(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Option strategy '{}' not found", id)))?;
        let legs = self.leg_repo.list_by_strategy(id).await?;
        Ok(StrategyWithLegs { strategy, legs })
    }

    pub async fn list_strategies(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<StrategyWithLegs>, AppError> {
        let strategies = self.strategy_repo.list_by_workspace(workspace_id).await?;
        let mut legs_by_strategy: HashMap<String, Vec<StrategyLeg>> = HashMap::new();
        for leg in self.leg_repo.list_by_workspace(workspace_id).await? {
            legs_by_strategy
                .entry(leg.strategy_id.clone())
                .or_default()
                .push(leg);
        }
        Ok(strategies
            .into_iter()
            .map(|strategy| StrategyWithLegs {
                legs: legs_by_strategy.remove(&strategy.id).unwrap_or_default(),
                strategy,
            })
            .collect())
    }

    pub async fn delete_strategy(&self, id: &str) -> Result<(), AppError> {
        self.strategy_repo.delete(id).await
    }
}

pub struct ValidationResult {
    pub valid: bool,
    pub warnings: Vec<String>,
}
