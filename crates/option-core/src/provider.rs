//! Data provider abstraction for option chain data

use crate::OptionError;
use crate::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::option::{DataSource, OptionChain, OptionContract, OptionType};

/// Provider trait for fetching option data
#[async_trait]
pub trait OptionsDataProvider: Send + Sync {
    /// Fetch option chain for a symbol
    async fn fetch_chain(&self, symbol: &str, workspace_id: &str) -> Result<OptionChain>;

    /// Provider name
    fn name(&self) -> &str;

    /// Check if provider is available
    fn is_available(&self) -> bool;
}

/// Demo provider - generates simulated option chains
pub struct DemoProvider {
    pub risk_free_rate: f64,
    pub default_volatility: f64,
    pub dividend_yield: f64,
}

impl DemoProvider {
    pub fn new() -> Self {
        Self {
            risk_free_rate: 0.05,
            default_volatility: 0.25,
            dividend_yield: 0.0,
        }
    }
}

#[async_trait]
impl OptionsDataProvider for DemoProvider {
    async fn fetch_chain(&self, symbol: &str, workspace_id: &str) -> Result<OptionChain> {
        // Generate simulated chain
        let underlying_price = 150.0; // Demo price
        let chain_id = format!("chain-{}-{}", symbol, Utc::now().timestamp());

        // Generate strikes around underlying price
        let mut contracts = Vec::new();
        for i in -5..=5 {
            let strike = underlying_price + (i as f64 * 5.0);

            // Generate call
            contracts.push(self.generate_contract(
                &format!("{}-{}-C", symbol, strike),
                workspace_id,
                &chain_id,
                symbol,
                OptionType::Call,
                strike,
                underlying_price,
            ));

            // Generate put
            contracts.push(self.generate_contract(
                &format!("{}-{}-P", symbol, strike),
                workspace_id,
                &chain_id,
                symbol,
                OptionType::Put,
                strike,
                underlying_price,
            ));
        }

        Ok(OptionChain {
            id: chain_id,
            workspace_id: workspace_id.to_string(),
            symbol: symbol.to_string(),
            underlying_price,
            as_of: Utc::now(),
            data_source: DataSource::Demo,
            created_at: Utc::now(),
        })
    }

    fn name(&self) -> &str {
        "demo"
    }

    fn is_available(&self) -> bool {
        true
    }
}

impl DemoProvider {
    fn generate_contract(
        &self,
        id: &str,
        workspace_id: &str,
        chain_id: &str,
        symbol: &str,
        option_type: OptionType,
        strike: f64,
        underlying_price: f64,
    ) -> OptionContract {
        let now = Utc::now();

        OptionContract {
            id: id.to_string(),
            workspace_id: workspace_id.to_string(),
            chain_id: chain_id.to_string(),
            symbol: symbol.to_string(),
            option_type,
            strike,
            expiration: now + chrono::Duration::days(30),
            contract_multiplier: 100,
            bid: 5.0,
            ask: 5.5,
            last: Some(5.25),
            volume: 1000,
            open_interest: 5000,
            implied_volatility: self.default_volatility,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Default for DemoProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// Provider factory for creating providers
pub struct ProviderFactory;

impl ProviderFactory {
    pub fn create(source: DataSource) -> Result<Box<dyn OptionsDataProvider>> {
        match source {
            DataSource::Demo => Ok(Box::new(DemoProvider::new())),
            DataSource::Live => Err(OptionError::ProviderError(
                "Live provider not implemented".to_string(),
            )),
            DataSource::File => Err(OptionError::ProviderError(
                "File provider not implemented".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_provider() {
        let provider = DemoProvider::new();
        let chain = provider
            .fetch_chain("AAPL", "test-workspace")
            .await
            .unwrap();

        assert_eq!(chain.symbol, "AAPL");
        assert_eq!(chain.data_source, DataSource::Demo);
    }
}
