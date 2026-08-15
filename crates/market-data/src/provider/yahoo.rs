//! Yahoo Finance market data provider.
//!
//! Placeholder: will be implemented with the yahoo_finance_api crate
//! following the Wealthfolio pattern.

use async_trait::async_trait;
use std::borrow::Cow;
use std::time::Duration;

use crate::errors::MarketDataError;
use crate::models::{InstrumentKind, ProviderInstrument, Quote, ProviderId};
use crate::provider::{MarketDataProvider, ProviderCapabilities, RateLimit};

/// Market data provider backed by Yahoo Finance.
pub struct YahooProvider;

impl YahooProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for YahooProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MarketDataProvider for YahooProvider {
    fn id(&self) -> ProviderId {
        Cow::Borrowed("YAHOO")
    }

    fn priority(&self) -> u32 {
        5
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            instrument_kinds: vec![
                InstrumentKind::Equity,
                InstrumentKind::Crypto,
                InstrumentKind::Fx,
                InstrumentKind::Metal,
                InstrumentKind::Option,
            ],
            supports_latest: true,
            supports_historical: true,
            supports_search: true,
            supports_profile: true,
            supports_dividends: true,
        }
    }

    fn rate_limit(&self) -> RateLimit {
        RateLimit {
            requests_per_minute: 2000,
            max_concurrency: 10,
            min_delay_ms: 50,
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