// Option service — orchestrates option analysis operations.
//
// Services own business logic and coordination between repositories.
// This service handles option chain fetching, Greeks calculations, and pricing.

#![allow(clippy::too_many_arguments)]

use std::sync::Arc;

use crate::database::repositories::greeks_repository::GreeksRepository;
use crate::database::repositories::option_chain_repository::OptionChainRepository;
use crate::database::repositories::option_contract_repository::OptionContractRepository;
use crate::database::repositories::option_strategy_repository::OptionStrategyRepository;
use crate::error::AppError;
use domain::option::{DataSource, OptionChain, OptionContract, OptionStrategy, OptionType};
use option_core::{
    black_scholes_price, calculate_greeks, calculate_implied_volatility, GreeksValues,
    ProviderFactory,
};

/// Service for option analysis operations.
///
/// Coordinates between:
/// - Data providers (Demo, Live, File)
/// - Option repositories (chains, contracts, Greeks, strategies)
/// - Pricing and Greeks calculations (Black-Scholes)
#[allow(dead_code)]
pub struct OptionService {
    chain_repo: Arc<OptionChainRepository>,
    contract_repo: Arc<OptionContractRepository>,
    greeks_repo: Arc<GreeksRepository>,
    strategy_repo: Arc<OptionStrategyRepository>,
}

impl OptionService {
    /// Creates a new option service.
    pub fn new(
        chain_repo: Arc<OptionChainRepository>,
        contract_repo: Arc<OptionContractRepository>,
        greeks_repo: Arc<GreeksRepository>,
        strategy_repo: Arc<OptionStrategyRepository>,
    ) -> Self {
        Self {
            chain_repo,
            contract_repo,
            greeks_repo,
            strategy_repo,
        }
    }

    /// Fetches an option chain from a data provider and persists it.
    ///
    /// # Arguments
    /// * `symbol` - Underlying symbol (e.g., "AAPL")
    /// * `workspace_id` - Workspace ID for the chain
    /// * `source` - Data source (Demo, Live, File)
    ///
    /// # Returns
    /// The fetched option chain with metadata
    pub async fn fetch_chain(
        &self,
        symbol: &str,
        workspace_id: &str,
        source: DataSource,
    ) -> Result<OptionChain, AppError> {
        // Validate inputs
        let normalized_symbol = symbol.trim().to_uppercase();
        let normalized_workspace_id = workspace_id.trim();
        if normalized_symbol.is_empty() {
            return Err(AppError::Validation("Symbol is required".to_string()));
        }
        if normalized_workspace_id.is_empty() {
            return Err(AppError::Validation("Workspace ID is required".to_string()));
        }

        // Get appropriate provider
        let provider = ProviderFactory::create(source)
            .map_err(|e| AppError::Internal(format!("Failed to create provider: {}", e)))?;

        // Fetch chain from provider
        let fetched = provider
            .fetch_chain(&normalized_symbol, normalized_workspace_id)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to fetch option chain: {}", e)))?;

        // Persist the chain and generated contracts together so a successful
        // fetch always leaves a selectable contract view.
        self.chain_repo
            .create_with_contracts(&fetched.chain, &fetched.contracts)
            .await?;

        Ok(fetched.chain)
    }

    /// Calculates all Greeks for an option using Black-Scholes model.
    ///
    /// # Arguments
    /// * `option_type` - Call or Put
    /// * `underlying_price` - Current price of the underlying
    /// * `strike` - Strike price
    /// * `expiration_years` - Time to expiration in years
    /// * `risk_free_rate` - Risk-free interest rate (e.g., 0.05 for 5%)
    /// * `volatility` - Implied volatility (e.g., 0.25 for 25%)
    /// * `dividend_yield` - Dividend yield (e.g., 0.02 for 2%)
    ///
    /// # Returns
    /// GreeksValues containing delta, gamma, theta, vega, and rho
    pub fn calculate_greeks(
        &self,
        option_type: OptionType,
        underlying_price: f64,
        strike: f64,
        expiration_years: f64,
        risk_free_rate: f64,
        volatility: f64,
        dividend_yield: f64,
    ) -> Result<GreeksValues, AppError> {
        // Validate inputs
        self.validate_pricing_inputs(underlying_price, strike, expiration_years, volatility)?;

        calculate_greeks(
            option_type,
            underlying_price,
            strike,
            expiration_years,
            risk_free_rate,
            volatility,
            dividend_yield,
        )
        .map_err(|e| AppError::Internal(format!("Greeks calculation failed: {}", e)))
    }

    /// Calculates option price using Black-Scholes model.
    ///
    /// # Arguments
    /// * `option_type` - Call or Put
    /// * `underlying_price` - Current price of the underlying
    /// * `strike` - Strike price
    /// * `expiration_years` - Time to expiration in years
    /// * `risk_free_rate` - Risk-free interest rate
    /// * `volatility` - Implied volatility
    /// * `dividend_yield` - Dividend yield
    ///
    /// # Returns
    /// Theoretical option price
    pub fn calculate_price(
        &self,
        option_type: OptionType,
        underlying_price: f64,
        strike: f64,
        expiration_years: f64,
        risk_free_rate: f64,
        volatility: f64,
        dividend_yield: f64,
    ) -> Result<f64, AppError> {
        // Validate inputs
        self.validate_pricing_inputs(underlying_price, strike, expiration_years, volatility)?;

        black_scholes_price(
            option_type,
            underlying_price,
            strike,
            expiration_years,
            risk_free_rate,
            volatility,
            dividend_yield,
        )
        .map_err(|e| AppError::Internal(format!("Price calculation failed: {}", e)))
    }

    /// Calculates implied volatility from market price.
    ///
    /// Uses Newton-Raphson method to find the volatility that produces
    /// the given market price.
    ///
    /// # Arguments
    /// * `option_type` - Call or Put
    /// * `underlying_price` - Current price of the underlying
    /// * `strike` - Strike price
    /// * `expiration_years` - Time to expiration in years
    /// * `risk_free_rate` - Risk-free interest rate
    /// * `dividend_yield` - Dividend yield
    /// * `market_price` - Observed market price of the option
    ///
    /// # Returns
    /// Implied volatility as a decimal (e.g., 0.25 for 25%)
    pub fn calculate_iv(
        &self,
        option_type: OptionType,
        underlying_price: f64,
        strike: f64,
        expiration_years: f64,
        risk_free_rate: f64,
        dividend_yield: f64,
        market_price: f64,
    ) -> Result<f64, AppError> {
        // Validate inputs
        self.validate_pricing_inputs(underlying_price, strike, expiration_years, 0.01)?;

        if market_price <= 0.0 {
            return Err(AppError::Validation(
                "Market price must be positive".to_string(),
            ));
        }

        calculate_implied_volatility(
            option_type,
            underlying_price,
            strike,
            expiration_years,
            risk_free_rate,
            dividend_yield,
            market_price,
            100,    // max_iterations
            0.0001, // precision
        )
        .map_err(|e| AppError::Internal(format!("IV calculation failed: {}", e)))
    }

    /// Validates common pricing inputs.
    fn validate_pricing_inputs(
        &self,
        underlying_price: f64,
        strike: f64,
        expiration_years: f64,
        volatility: f64,
    ) -> Result<(), AppError> {
        if underlying_price <= 0.0 {
            return Err(AppError::Validation(
                "Underlying price must be positive".to_string(),
            ));
        }
        if strike <= 0.0 {
            return Err(AppError::Validation(
                "Strike price must be positive".to_string(),
            ));
        }
        if expiration_years <= 0.0 {
            return Err(AppError::Validation(
                "Time to expiration must be positive".to_string(),
            ));
        }
        if volatility <= 0.0 {
            return Err(AppError::Validation(
                "Volatility must be positive".to_string(),
            ));
        }
        Ok(())
    }

    // ============================================
    // Option Chain CRUD Operations
    // ============================================

    /// Gets an option chain by ID.
    pub async fn get_chain(&self, id: &str) -> Result<OptionChain, AppError> {
        self.chain_repo.get(id).await?.ok_or_else(|| {
            AppError::NotFound(format!("Option chain '{}' not found", id))
        })
    }

    /// Lists all option chains for a workspace.
    pub async fn list_chains(&self, workspace_id: &str) -> Result<Vec<OptionChain>, AppError> {
        self.chain_repo.list_by_workspace(workspace_id).await
    }

    /// Deletes an option chain and all its contracts.
    pub async fn delete_chain(&self, id: &str) -> Result<(), AppError> {
        self.chain_repo.delete(id).await
    }

    // ============================================
    // Option Contract CRUD Operations
    // ============================================

    /// Creates a new option contract.
    pub async fn create_contract(&self, contract: &OptionContract) -> Result<(), AppError> {
        // Validate required fields
        if contract.chain_id.trim().is_empty() {
            return Err(AppError::Validation("Chain ID is required".to_string()));
        }
        if contract.symbol.trim().is_empty() {
            return Err(AppError::Validation("Symbol is required".to_string()));
        }
        if contract.strike <= 0.0 {
            return Err(AppError::Validation("Strike must be positive".to_string()));
        }

        self.contract_repo.create(contract).await
    }

    /// Gets an option contract by ID.
    pub async fn get_contract(&self, id: &str) -> Result<OptionContract, AppError> {
        self.contract_repo.get(id).await?.ok_or_else(|| {
            AppError::NotFound(format!("Option contract '{}' not found", id))
        })
    }

    /// Lists all option contracts for a chain.
    pub async fn list_contracts(&self, chain_id: &str) -> Result<Vec<OptionContract>, AppError> {
        self.contract_repo.list_by_chain(chain_id).await
    }

    /// Deletes an option contract.
    pub async fn delete_contract(&self, id: &str) -> Result<(), AppError> {
        self.contract_repo.delete(id).await
    }

    // ============================================
    // Option Strategy CRUD Operations
    // ============================================

    /// Creates a new option strategy.
    pub async fn create_strategy(&self, strategy: &OptionStrategy) -> Result<(), AppError> {
        // Validate required fields
        if strategy.name.trim().is_empty() {
            return Err(AppError::Validation("Strategy name is required".to_string()));
        }
        if strategy.underlying.trim().is_empty() {
            return Err(AppError::Validation("Underlying symbol is required".to_string()));
        }

        self.strategy_repo.create(strategy).await
    }

    /// Gets an option strategy by ID.
    pub async fn get_strategy(&self, id: &str) -> Result<OptionStrategy, AppError> {
        self.strategy_repo.get(id).await?.ok_or_else(|| {
            AppError::NotFound(format!("Option strategy '{}' not found", id))
        })
    }

    /// Lists all option strategies for a workspace.
    pub async fn list_strategies(&self, workspace_id: &str) -> Result<Vec<OptionStrategy>, AppError> {
        self.strategy_repo.list_by_workspace(workspace_id).await
    }

    /// Updates an existing option strategy.
    pub async fn update_strategy(&self, strategy: &OptionStrategy) -> Result<(), AppError> {
        if strategy.name.trim().is_empty() {
            return Err(AppError::Validation("Strategy name is required".to_string()));
        }

        self.strategy_repo.update(strategy).await
    }

    /// Deletes an option strategy.
    pub async fn delete_strategy(&self, id: &str) -> Result<(), AppError> {
        self.strategy_repo.delete(id).await
    }
}
