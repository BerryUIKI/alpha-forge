//! Synthetic market data provider for deterministic e2e tests.
//!
//! Placeholder: will be implemented with fixture data generation
//! following the Wealthfolio pattern.

use async_trait::async_trait;
use std::borrow::Cow;
use std::time::Duration;

use crate::errors::MarketDataError;
use crate::models::{InstrumentKind, ProviderInstrument, Quote, ProviderId};
use crate::provider::{MarketDataProvider, ProviderCapabilities, RateLimit};

/// Market data provider backed by synthetic fixture metadata.
pub struct FixtureProvider {
    provider_id: &'static str,
}

impl FixtureProvider {
    pub fn new() -> Self {
        Self::new_for_provider("YAHOO")
    }

    pub fn new_for_provider(provider_id: &'static str) -> Self {
        Self { provider_id }
    }
}

impl Default for FixtureProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MarketDataProvider for FixtureProvider {
    fn id(&self) -> ProviderId {
        Cow::Borrowed(self.provider_id)
    }

    fn priority(&self) -> u32 {
        1
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            instrument_kinds: vec![
                InstrumentKind::Equity,
                InstrumentKind::Crypto,
                InstrumentKind::Fx,
                InstrumentKind::Metal,
            ],
            supports_latest: true,
            supports_historical: true,
            supports_search: true,
            supports_profile: true,
            supports_dividends: false,
        }
    }

    fn rate_limit(&self) -> RateLimit {
        RateLimit {
            requests_per_minute: 60_000,
            max_concurrency: 100,
            min_delay_ms: 0,
        }
    }

    async fn get_latest_quote(
        &self,
        _instrument: &ProviderInstrument,
    ) -> Result<Quote, MarketDataError> {
        Err(MarketDataError::NotSupported {
            operation: "get_latest_quote".to_string(),
            provider: self.id().to_string(),
        })
    }

    async fn get_historical_quotes(
        &self,
        _instrument: &ProviderInstrument,
        _start: chrono::NaiveDate,
        _end: chrono::NaiveDate,
    ) -> Result<Vec<Quote>, MarketDataError> {
        Err(MarketDataError::NotSupported {
            operation: "get_historical_quotes".to_string(),
            provider: self.id().to_string(),
        })
    }
}