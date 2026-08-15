//! Market data provider trait.
//!
//! Defines the interface that all market data providers must implement.
//! Providers are responsible for fetching quotes, profiles, search results,
//! splits, and dividends from external data sources.

use async_trait::async_trait;
use chrono::NaiveDate;

use crate::errors::MarketDataError;
use crate::models::{
    AssetProfile, DividendEvent, ProviderId, ProviderInstrument, Quote, SearchResult, SplitEvent,
};
use crate::provider::{ProviderCapabilities, RateLimit};

/// Market data provider trait.
///
/// All providers must implement this trait to be registered with the
/// [`ProviderRegistry`](crate::registry::ProviderRegistry).
///
/// Default implementations are provided for `search`, `get_profile`,
/// `get_splits`, and `get_dividends` — these return
/// [`NotSupported`](MarketDataError::NotSupported) by default.
#[async_trait]
pub trait MarketDataProvider: Send + Sync {
    /// Unique provider identifier (e.g., "YAHOO", "ALPHA_VANTAGE").
    fn id(&self) -> ProviderId;

    /// Priority order (lower = preferred first).
    fn priority(&self) -> u32;

    /// Capabilities advertised by this provider.
    fn capabilities(&self) -> ProviderCapabilities;

    /// Rate limit configuration for this provider.
    fn rate_limit(&self) -> RateLimit;

    /// Fetch the latest quote for a resolved provider instrument.
    async fn get_latest_quote(
        &self,
        instrument: &ProviderInstrument,
    ) -> Result<Quote, MarketDataError>;

    /// Fetch historical quotes for a date range.
    async fn get_historical_quotes(
        &self,
        instrument: &ProviderInstrument,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<Quote>, MarketDataError>;

    /// Search for symbols matching the given query.
    async fn search(&self, _query: &str) -> Result<Vec<SearchResult>, MarketDataError> {
        Err(MarketDataError::NotSupported {
            operation: "search".to_string(),
            provider: self.id().to_string(),
        })
    }

    /// Fetch the asset profile for a resolved provider instrument.
    async fn get_profile(
        &self,
        _instrument: &ProviderInstrument,
    ) -> Result<AssetProfile, MarketDataError> {
        Err(MarketDataError::NotSupported {
            operation: "get_profile".to_string(),
            provider: self.id().to_string(),
        })
    }

    /// Fetch split history for a resolved provider instrument.
    async fn get_splits(
        &self,
        _instrument: &ProviderInstrument,
        _start: NaiveDate,
        _end: NaiveDate,
    ) -> Result<Vec<SplitEvent>, MarketDataError> {
        Err(MarketDataError::NotSupported {
            operation: "get_splits".to_string(),
            provider: self.id().to_string(),
        })
    }

    /// Fetch dividend history for a resolved provider instrument.
    async fn get_dividends(
        &self,
        _instrument: &ProviderInstrument,
        _start: NaiveDate,
        _end: NaiveDate,
    ) -> Result<Vec<DividendEvent>, MarketDataError> {
        Err(MarketDataError::NotSupported {
            operation: "get_dividends".to_string(),
            provider: self.id().to_string(),
        })
    }
}
