//! Synthetic market data provider for deterministic e2e tests.
//!
//! Placeholder: will be implemented with fixture data generation
//! following the Wealthfolio pattern.

use async_trait::async_trait;
use chrono::Datelike;
use std::borrow::Cow;

use crate::errors::MarketDataError;
use crate::models::{InstrumentKind, ProviderId, ProviderInstrument, Quote};
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
        instrument: &ProviderInstrument,
    ) -> Result<Quote, MarketDataError> {
        let now = chrono::Utc::now();
        // Deterministic baseline price derived from symbol characters (e.g. AAPL -> ~185.00)
        let hash_val: u32 = instrument.symbol.chars().map(|c| c as u32).sum();
        let base_price = rust_decimal::Decimal::from(100 + (hash_val % 200));
        let open = base_price - rust_decimal::Decimal::new(15, 1);
        let high = base_price + rust_decimal::Decimal::new(25, 1);
        let low = base_price - rust_decimal::Decimal::new(20, 1);
        let volume = rust_decimal::Decimal::from(1_500_000 + (hash_val as u64 * 100));
        let currency = instrument.currency.as_deref().unwrap_or("USD").to_string();

        Ok(Quote::ohlcv(
            now,
            open,
            high,
            low,
            base_price,
            volume,
            currency,
            self.provider_id.to_string(),
        ))
    }

    async fn get_historical_quotes(
        &self,
        instrument: &ProviderInstrument,
        start: chrono::NaiveDate,
        end: chrono::NaiveDate,
    ) -> Result<Vec<Quote>, MarketDataError> {
        if start > end {
            return Ok(Vec::new());
        }
        let currency = instrument.currency.as_deref().unwrap_or("USD").to_string();
        let hash_val: u32 = instrument.symbol.chars().map(|c| c as u32).sum();
        let base_price = rust_decimal::Decimal::from(100 + (hash_val % 200));
        let mut current = start;
        let mut quotes = Vec::new();

        while current <= end {
            // Monday=1 .. Sunday=7
            let weekday = current.weekday().number_from_monday();
            if weekday <= 5 {
                let timestamp = current.and_hms_opt(16, 0, 0).unwrap_or_default().and_utc();
                let day_offset = rust_decimal::Decimal::new(weekday as i64, 1);
                let close = base_price + day_offset;
                quotes.push(Quote::ohlcv(
                    timestamp,
                    close - rust_decimal::Decimal::new(10, 1),
                    close + rust_decimal::Decimal::new(20, 1),
                    close - rust_decimal::Decimal::new(15, 1),
                    close,
                    rust_decimal::Decimal::from(1_000_000),
                    currency.clone(),
                    self.provider_id.to_string(),
                ));
            }
            if let Some(next) = current.succ_opt() {
                current = next;
            } else {
                break;
            }
        }

        Ok(quotes)
    }
}
