// Option service — orchestrates option analysis operations

use std::sync::Arc;
use domain::option::{OptionChain, OptionContract, OptionType, DataSource};
use option_core::{black_scholes_price, calculate_greeks, calculate_implied_volatility, GreeksValues, DemoProvider, OptionsDataProvider, ProviderFactory};
use crate::database::repositories::{
    OptionChainRepository, OptionContractRepository, GreeksRepository
};
use crate::error::AppError;

pub struct OptionService {
    chain_repo: Arc<OptionChainRepository>,
    contract_repo: Arc<OptionContractRepository>,
    greeks_repo: Arc<GreeksRepository>,
}

impl OptionService {
    pub fn new(
        chain_repo: Arc<OptionChainRepository>,
        contract_repo: Arc<OptionContractRepository>,
        greeks_repo: Arc<GreeksRepository>,
    ) -> Self {
        Self { chain_repo, contract_repo, greeks_repo }
    }

    /// Fetch option chain from provider
    pub async fn fetch_chain(&self, symbol: &str, workspace_id: &str, source: DataSource) -> Result<OptionChain, AppError> {
        let provider = ProviderFactory::create(source)
            .map_err(|e| AppError::Internal(e.to_string()))?;
        
        let chain = provider.fetch_chain(symbol, workspace_id).await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        
        // Save chain to database
        self.chain_repo.create(&chain).await?;
        
        Ok(chain)
    }

    /// Calculate Greeks for an option
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
        calculate_greeks(option_type, underlying_price, strike, expiration_years, risk_free_rate, volatility, dividend_yield)
            .map_err(|e| AppError::Internal(e.to_string()))
    }

    /// Calculate option price using Black-Scholes
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
        black_scholes_price(option_type, underlying_price, strike, expiration_years, risk_free_rate, volatility, dividend_yield)
            .map_err(|e| AppError::Internal(e.to_string()))
    }

    /// Calculate implied volatility
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
        calculate_implied_volatility(option_type, underlying_price, strike, expiration_years, risk_free_rate, dividend_yield, market_price, 100, 0.0001)
            .map_err(|e| AppError::Internal(e.to_string()))
    }
}