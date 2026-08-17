//! Provider capabilities and rate limit configuration.

use crate::models::{InstrumentId, InstrumentKind};

/// Provider capabilities describing what asset types and operations are supported.
#[derive(Clone, Debug)]
pub struct ProviderCapabilities {
    /// Instrument kinds this provider supports.
    pub instrument_kinds: Vec<InstrumentKind>,
    /// Whether latest quote is supported.
    pub supports_latest: bool,
    /// Whether historical quotes are supported.
    pub supports_historical: bool,
    /// Whether symbol search is supported.
    pub supports_search: bool,
    /// Whether profile lookup is supported.
    pub supports_profile: bool,
    /// Whether dividend data is supported.
    pub supports_dividends: bool,
}

impl ProviderCapabilities {
    /// Check if this provider supports the given instrument kind.
    pub fn supports_instrument(&self, instrument: &InstrumentId) -> bool {
        let kind = instrument.instrument_kind();
        self.instrument_kinds.contains(&kind)
    }
}

/// Rate limit configuration for a provider.
#[derive(Clone, Debug)]
pub struct RateLimit {
    /// Maximum requests per minute.
    pub requests_per_minute: u32,
    /// Maximum concurrent requests.
    pub max_concurrency: u32,
    /// Minimum delay between requests in milliseconds.
    pub min_delay_ms: u64,
}

impl Default for RateLimit {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            max_concurrency: 5,
            min_delay_ms: 100,
        }
    }
}
