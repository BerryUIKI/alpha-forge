// Option service — orchestrates option analysis operations.
//
// Services own business logic and coordination between repositories.
// This service handles option chain fetching, Greeks calculations, and pricing.

#![allow(clippy::too_many_arguments)]

use std::sync::Arc;

use crate::database::repositories::greeks_repository::GreeksRepository;
use crate::database::repositories::option_chain_repository::OptionChainRepository;
use crate::database::repositories::option_contract_repository::OptionContractRepository;
use crate::error::AppError;
use domain::option::{DataSource, OptionChain, OptionType};
use option_core::{
    black_scholes_price, calculate_greeks, calculate_implied_volatility, GreeksValues,
    ProviderFactory,
};

/// Service for option analysis operations.
///
/// Coordinates between:
/// - Data providers (Demo, Live, File)
/// - Option repositories (chains, contracts, Greeks)
/// - Pricing and Greeks calculations (Black-Scholes)
#[allow(dead_code)]
pub struct OptionService {
    chain_repo: Arc<OptionChainRepository>,
    contract_repo: Arc<OptionContractRepository>,
    greeks_repo: Arc<GreeksRepository>,
}

impl OptionService {
    /// Creates a new option service.
    pub fn new(
        chain_repo: Arc<OptionChainRepository>,
        contract_repo: Arc<OptionContractRepository>,
        greeks_repo: Arc<GreeksRepository>,
    ) -> Self {
        Self {
            chain_repo,
            contract_repo,
            greeks_repo,
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
        if symbol.trim().is_empty() {
            return Err(AppError::Validation("Symbol is required".to_string()));
        }
        if workspace_id.trim().is_empty() {
            return Err(AppError::Validation("Workspace ID is required".to_string()));
        }

        // Get appropriate provider
        let provider = ProviderFactory::create(source)
            .map_err(|e| AppError::Internal(format!("Failed to create provider: {}", e)))?;

        // Fetch chain from provider
        let chain = provider
            .fetch_chain(symbol, workspace_id)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to fetch option chain: {}", e)))?;

        // Persist chain to database
        self.chain_repo.create(&chain).await?;

        Ok(chain)
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
}
