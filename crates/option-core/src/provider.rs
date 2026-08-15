//! Data provider abstraction for option chain data

use crate::OptionError;
use crate::Result;
use async_trait::async_trait;
use chrono::Utc;
use domain::option::{DataSource, OptionChain, OptionContract, OptionType};
use uuid::Uuid;

/// Provider trait for fetching option data
#[async_trait]
pub trait OptionsDataProvider: Send + Sync {
    /// Fetch option chain for a symbol
    async fn fetch_chain(&self, symbol: &str, workspace_id: &str) -> Result<FetchedOptionChain>;

    /// Provider name
    fn name(&self) -> &str;

    /// Check if provider is available
    fn is_available(&self) -> bool;
}

/// A provider result containing the chain metadata and its contracts.
///
/// Keeping the related identifiers together prevents the persistence layer from
/// accidentally storing a chain whose contracts use a different chain ID.
#[derive(Debug, Clone)]
pub struct FetchedOptionChain {
    pub chain: OptionChain,
    pub contracts: Vec<OptionContract>,
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
    async fn fetch_chain(&self, symbol: &str, workspace_id: &str) -> Result<FetchedOptionChain> {
        // Generate simulated chain
        let underlying_price = 150.0; // Demo price
        let chain_id = Uuid::new_v4().to_string();

        // Generate strikes around underlying price
        let mut contracts = Vec::new();
        for i in -5..=5 {
            let strike = underlying_price + (i as f64 * 5.0);

            // Generate call
            contracts.push(self.generate_contract(
                &Uuid::new_v4().to_string(),
                workspace_id,
                &chain_id,
                symbol,
                OptionType::Call,
                strike,
                underlying_price,
            ));

            // Generate put
            contracts.push(self.generate_contract(
                &Uuid::new_v4().to_string(),
                workspace_id,
                &chain_id,
                symbol,
                OptionType::Put,
                strike,
                underlying_price,
            ));
        }

        Ok(FetchedOptionChain {
            chain: OptionChain {
                id: chain_id,
                workspace_id: workspace_id.to_string(),
                symbol: symbol.to_string(),
                underlying_price,
                as_of: Utc::now(),
                data_source: DataSource::Demo,
                created_at: Utc::now(),
            },
            contracts,
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
        _underlying_price: f64,
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

        assert_eq!(chain.chain.symbol, "AAPL");
        assert_eq!(chain.chain.data_source, DataSource::Demo);
        assert!(Uuid::parse_str(&chain.chain.id).is_ok());
        assert_eq!(chain.contracts.len(), 22);
        assert!(chain.contracts.iter().all(|contract| {
            contract.chain_id == chain.chain.id && Uuid::parse_str(&contract.id).is_ok()
        }));
    }
}
