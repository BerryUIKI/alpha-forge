//! Provider registry for managing and orchestrating market data providers.
//!
//! The [`ProviderRegistry`] manages a collection of market data providers
//! and coordinates request routing, rate limiting, circuit breaking, and
//! validation.
//!
//! # Architecture
//!
//! ```text
//! QuoteContext
//!     |
//!     v
//! ProviderRegistry
//!     |-- ordered_providers()  (sorted by priority, respecting circuit state)
//!     |-- rate_limiter.acquire()
//!     |-- provider.get_quote()
//!     |-- circuit_breaker.record_success/failure()
//!     |-- validator.validate()
//!     v
//! Quote
//! ```

use std::sync::Arc;

use crate::errors::MarketDataError;
use crate::models::{AssetProfile, InstrumentId, NaiveDate, Quote, QuoteContext, SearchResult};
use crate::provider::MarketDataProvider;
use crate::registry::{
    CircuitBreaker, CircuitBreakerConfig, CircuitState, FetchDiagnostics, QuoteValidator,
    RateLimitConfig, RateLimiter, SkipReason, ValidatorConfig,
};
use crate::resolver::SymbolResolver;

#[cfg(test)]
use crate::models::ProviderInstrument;
#[cfg(test)]
use crate::provider::{ProviderCapabilities, RateLimit};
#[cfg(test)]
use async_trait::async_trait;
#[cfg(test)]
use chrono::Utc;

/// Mock provider for testing the registry.
#[cfg(test)]
struct MockProvider {
    id: &'static str,
    priority: u32,
    capabilities: ProviderCapabilities,
    rate_limit: RateLimit,
    should_fail: bool,
}

#[cfg(test)]
#[async_trait]
impl MarketDataProvider for MockProvider {
    fn id(&self) -> crate::ProviderId {
        std::borrow::Cow::Borrowed(self.id)
    }

    fn priority(&self) -> u32 {
        self.priority
    }

    fn capabilities(&self) -> ProviderCapabilities {
        self.capabilities.clone()
    }

    fn rate_limit(&self) -> RateLimit {
        self.rate_limit.clone()
    }

    async fn get_latest_quote(
        &self,
        _instrument: &ProviderInstrument,
    ) -> Result<Quote, MarketDataError> {
        if self.should_fail {
            return Err(MarketDataError::ProviderError {
                provider: self.id.to_string(),
                message: "mock failure".to_string(),
            });
        }
        Ok(Quote::new(
            Utc::now(),
            rust_decimal::Decimal::new(150, 0),
            "USD".to_string(),
            self.id.to_string(),
        ))
    }

    async fn get_historical_quotes(
        &self,
        _instrument: &ProviderInstrument,
        _start: NaiveDate,
        _end: NaiveDate,
    ) -> Result<Vec<Quote>, MarketDataError> {
        Ok(vec![Quote::new(
            Utc::now(),
            rust_decimal::Decimal::new(150, 0),
            "USD".to_string(),
            self.id.to_string(),
        )])
    }
}

/// Mock resolver for testing.
#[cfg(test)]
struct MockResolver;

#[cfg(test)]
#[async_trait]
impl SymbolResolver for MockResolver {
    async fn resolve(
        &self,
        _context: &QuoteContext,
        _provider_id: &str,
    ) -> Result<crate::ResolvedInstrument, MarketDataError> {
        Ok(crate::ResolvedInstrument {
            instrument: ProviderInstrument {
                symbol: "AAPL".to_string(),
                kind: "equity".to_string(),
                currency: Some("USD".to_string()),
                exchange: Some("XNAS".to_string()),
            },
            source: crate::ResolutionSource::Rules,
        })
    }

    async fn get_currency(
        &self,
        _context: &QuoteContext,
    ) -> Option<std::borrow::Cow<'static, str>> {
        Some(std::borrow::Cow::Borrowed("USD"))
    }
}

/// The provider registry manages and orchestrates market data providers.
pub struct ProviderRegistry {
    /// Registered providers.
    providers: Vec<Arc<dyn MarketDataProvider>>,
    /// Symbol resolver for resolving instruments.
    resolver: Arc<dyn SymbolResolver>,
    /// Rate limiter for each provider.
    rate_limiter: RateLimiter,
    /// Circuit breaker for each provider.
    circuit_breaker: CircuitBreaker,
    /// Quote validator.
    validator: QuoteValidator,
    /// Custom priority overrides.
    custom_priorities: std::collections::HashMap<String, u32>,
}

impl ProviderRegistry {
    /// Create a new empty provider registry.
    ///
    /// A resolver must be provided for instrument resolution.
    pub fn new(resolver: Arc<dyn SymbolResolver>) -> Self {
        Self {
            providers: Vec::new(),
            resolver,
            rate_limiter: RateLimiter::new(),
            circuit_breaker: CircuitBreaker::new(),
            validator: QuoteValidator::new(),
            custom_priorities: std::collections::HashMap::new(),
        }
    }

    /// Set custom priority overrides for providers.
    ///
    /// Lower values mean higher priority.
    pub fn with_priorities(mut self, priorities: std::collections::HashMap<String, u32>) -> Self {
        self.custom_priorities = priorities;
        self
    }

    /// Set custom configuration for the circuit breaker.
    pub fn with_circuit_breaker_config(mut self, config: CircuitBreakerConfig) -> Self {
        self.circuit_breaker = CircuitBreaker::with_config(config);
        self
    }

    /// Set custom configuration for the validator.
    pub fn with_validator_config(mut self, config: ValidatorConfig) -> Self {
        self.validator = QuoteValidator::with_config(config);
        self
    }

    /// Register a provider with the registry.
    ///
    /// This also configures rate limiting for the provider.
    pub fn register(&mut self, provider: Arc<dyn MarketDataProvider>) {
        let id = provider.id().to_string();
        let rl = provider.rate_limit();

        // Configure rate limiter
        self.rate_limiter.configure(
            id.clone(),
            RateLimitConfig {
                requests_per_minute: rl.requests_per_minute,
                burst_capacity: rl.max_concurrency,
            },
        );

        self.providers.push(provider);
    }

    /// Get the list of providers in priority order, respecting circuit state.
    fn ordered_providers(&self) -> Vec<&dyn MarketDataProvider> {
        let mut sorted: Vec<&dyn MarketDataProvider> =
            self.providers.iter().map(|p| p.as_ref()).collect();
        sorted.sort_by_key(|p| {
            let id = p.id().to_string();
            self.custom_priorities
                .get(&id)
                .copied()
                .unwrap_or_else(|| p.priority())
        });
        sorted
    }

    /// Filter providers that support the given instrument.
    fn filter_providers<'a>(
        &'a self,
        providers: Vec<&'a dyn MarketDataProvider>,
        instrument: &InstrumentId,
    ) -> Vec<&'a dyn MarketDataProvider> {
        providers
            .into_iter()
            .filter(|p| p.capabilities().supports_instrument(instrument))
            .collect()
    }

    /// Sort providers by preference for a given context.
    fn sort_by_preference<'a>(
        &'a self,
        providers: Vec<&'a dyn MarketDataProvider>,
        context: &QuoteContext,
    ) -> Vec<&'a dyn MarketDataProvider> {
        let mut sorted = providers;
        sorted.sort_by_key(|p| {
            let id = p.id().to_string();
            let is_preferred = context
                .preferred_provider
                .as_ref()
                .is_some_and(|pref| id.as_str() == pref.as_ref());
            // Preferred providers come first
            !is_preferred
        });
        sorted
    }

    /// Get all registered providers.
    pub fn providers(&self) -> &[Arc<dyn MarketDataProvider>] {
        &self.providers
    }

    /// Check if a circuit is open for a provider.
    pub fn is_circuit_open(&self, provider: &str) -> bool {
        self.circuit_breaker.state(provider) == CircuitState::Open
    }

    /// Reset the circuit breaker for a provider.
    pub fn reset_circuit(&self, provider: &str) {
        self.circuit_breaker.reset(provider);
    }

    /// Reset all circuit breakers.
    pub fn reset_all_circuits(&self) {
        self.circuit_breaker.reset_all();
    }

    /// Fetch the latest quote for an instrument, trying providers in order.
    pub async fn get_latest_quote(&self, context: &QuoteContext) -> Result<Quote, MarketDataError> {
        let (quote, _diagnostics) = self.fetch_quotes_with_diagnostics(context).await?;
        quote.ok_or(MarketDataError::AllProvidersFailed)
    }

    /// Fetch the latest quote with diagnostics (for debugging).
    pub async fn fetch_quotes_with_diagnostics(
        &self,
        context: &QuoteContext,
    ) -> Result<(Option<Quote>, FetchDiagnostics), MarketDataError> {
        let mut diagnostics = FetchDiagnostics::new();
        let providers = self.ordered_providers();
        let providers = self.filter_providers(providers, &context.instrument);
        let providers = self.sort_by_preference(providers, context);

        for provider in &providers {
            let provider_id = provider.id().to_string();

            // Check circuit breaker
            if !self.circuit_breaker.is_allowed(&provider_id) {
                diagnostics.record_skip(&provider_id, SkipReason::CircuitOpen);
                continue;
            }

            // Resolve instrument for this provider
            let resolved = match self.resolver.resolve(context, &provider_id).await {
                Ok(r) => r,
                Err(_) => {
                    diagnostics.record_skip(&provider_id, SkipReason::ResolutionFailed);
                    continue;
                }
            };

            // Acquire rate limit token
            self.rate_limiter.acquire(&provider_id).await;

            // Fetch latest quote
            match provider.get_latest_quote(&resolved.instrument).await {
                Ok(quote) => {
                    self.circuit_breaker.record_success(&provider_id);

                    // Validate
                    match self.validator.validate(&quote) {
                        Ok(()) => {
                            diagnostics.record_success(&provider_id);
                            return Ok((Some(quote), diagnostics));
                        }
                        Err(_issues) => {
                            diagnostics.record_error(
                                &provider_id,
                                MarketDataError::ValidationFailed {
                                    message: "Quote validation failed".to_string(),
                                },
                            );
                            continue;
                        }
                    }
                }
                Err(e) => {
                    self.circuit_breaker.record_failure(&provider_id);
                    let retry = e.retry_class();
                    match retry {
                        crate::errors::RetryClass::CircuitOpen => {
                            diagnostics.record_skip(&provider_id, SkipReason::CircuitOpen);
                        }
                        _ => {
                            diagnostics.record_error(&provider_id, e);
                        }
                    }
                }
            }
        }

        Ok((None, diagnostics))
    }

    /// Fetch historical quotes for a date range.
    pub async fn get_historical_quotes(
        &self,
        context: &QuoteContext,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<Quote>, MarketDataError> {
        let providers = self.ordered_providers();
        let providers = self.filter_providers(providers, &context.instrument);
        let providers = self.sort_by_preference(providers, context);

        let mut last_error = MarketDataError::NoProvidersAvailable;

        for provider in &providers {
            let provider_id = provider.id().to_string();

            // Check circuit breaker
            if !self.circuit_breaker.is_allowed(&provider_id) {
                continue;
            }

            // Resolve instrument for this provider
            let resolved = self.resolver.resolve(context, &provider_id).await?;

            // Acquire rate limit token
            self.rate_limiter.acquire(&provider_id).await;

            // Fetch historical quotes
            match provider
                .get_historical_quotes(&resolved.instrument, start, end)
                .await
            {
                Ok(quotes) => {
                    self.circuit_breaker.record_success(&provider_id);
                    // Validate and filter quotes
                    let (valid, _invalid) = self.validator.validate_batch(&quotes);
                    if !valid.is_empty() {
                        return Ok(valid);
                    }
                }
                Err(e) => {
                    self.circuit_breaker.record_failure(&provider_id);
                    last_error = e;
                }
            }
        }

        Err(last_error)
    }

    /// Search for symbols across all providers.
    pub async fn search(&self, query: &str) -> Result<Vec<SearchResult>, MarketDataError> {
        let mut results = Vec::new();
        for provider in &self.providers {
            if provider.capabilities().supports_search {
                match provider.search(query).await {
                    Ok(mut r) => results.append(&mut r),
                    Err(_) => continue,
                }
            }
        }
        if results.is_empty() {
            Err(MarketDataError::SymbolNotFound(query.to_string()))
        } else {
            Ok(results)
        }
    }

    /// Fetch the asset profile for an instrument.
    pub async fn get_profile(
        &self,
        context: &QuoteContext,
    ) -> Result<AssetProfile, MarketDataError> {
        let providers = self.ordered_providers();
        let providers = self.filter_providers(providers, &context.instrument);
        let providers = self.sort_by_preference(providers, context);

        let mut last_error = MarketDataError::NoProvidersAvailable;

        for provider in &providers {
            let provider_id = provider.id().to_string();

            if !provider.capabilities().supports_profile {
                continue;
            }

            if !self.circuit_breaker.is_allowed(&provider_id) {
                continue;
            }

            let resolved = self.resolver.resolve(context, &provider_id).await?;
            self.rate_limiter.acquire(&provider_id).await;

            match provider.get_profile(&resolved.instrument).await {
                Ok(profile) => {
                    self.circuit_breaker.record_success(&provider_id);
                    return Ok(profile);
                }
                Err(e) => {
                    self.circuit_breaker.record_failure(&provider_id);
                    last_error = e;
                }
            }
        }

        Err(last_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::sync::Arc;

    fn make_context() -> QuoteContext {
        QuoteContext {
            instrument: InstrumentId::Equity {
                ticker: Arc::from("AAPL"),
                mic: Some(Cow::Borrowed("XNAS")),
            },
            identifiers: crate::models::QuoteIdentifiers::default(),
            overrides: None,
            currency_hint: Some(Cow::Borrowed("USD")),
            preferred_provider: None,
            bond_metadata: None,
            custom_provider_code: None,
        }
    }

    fn make_provider(id: &'static str, priority: u32, should_fail: bool) -> Arc<MockProvider> {
        Arc::new(MockProvider {
            id,
            priority,
            capabilities: ProviderCapabilities {
                instrument_kinds: vec![crate::models::InstrumentKind::Equity],
                supports_latest: true,
                supports_historical: true,
                supports_search: false,
                supports_profile: false,
                supports_dividends: false,
            },
            rate_limit: RateLimit::default(),
            should_fail,
        })
    }

    #[tokio::test]
    async fn test_register_provider() {
        let resolver = Arc::new(MockResolver);
        let mut registry = ProviderRegistry::new(resolver);

        let provider = make_provider("YAHOO", 1, false);
        registry.register(provider);
        assert_eq!(registry.providers().len(), 1);
    }

    #[tokio::test]
    async fn test_get_latest_quote_success() {
        let resolver = Arc::new(MockResolver);
        let mut registry = ProviderRegistry::new(resolver);
        registry.register(make_provider("YAHOO", 1, false));

        let context = make_context();
        let result = registry.get_latest_quote(&context).await;
        assert!(result.is_ok());
        let quote = result.unwrap();
        assert_eq!(quote.close, rust_decimal::Decimal::new(150, 0));
    }

    #[tokio::test]
    async fn test_fallback_to_next_provider() {
        let resolver = Arc::new(MockResolver);
        let mut registry = ProviderRegistry::new(resolver);
        registry.register(make_provider("FAILING", 1, true));
        registry.register(make_provider("WORKING", 2, false));

        let context = make_context();
        let result = registry.get_latest_quote(&context).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_all_providers_fail() {
        let resolver = Arc::new(MockResolver);
        let mut registry = ProviderRegistry::new(resolver);
        registry.register(make_provider("FAILING", 1, true));

        let context = make_context();
        let result = registry.get_latest_quote(&context).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_preferred_provider_is_tried_first() {
        let resolver = Arc::new(MockResolver);
        let mut registry = ProviderRegistry::new(resolver);
        registry.register(make_provider("PROVIDER_A", 10, true));
        registry.register(make_provider("PROVIDER_B", 5, false));

        let mut context = make_context();
        context.preferred_provider = Some(Cow::Borrowed("PROVIDER_A"));

        // PROVIDER_A is preferred (and cheap), but fails; falls back to PROVIDER_B
        let result = registry.get_latest_quote(&context).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_diagnostics_collected() {
        let resolver = Arc::new(MockResolver);
        let mut registry = ProviderRegistry::new(resolver);
        registry.register(make_provider("YAHOO", 1, false));

        let context = make_context();
        let (quote, diagnostics) = registry
            .fetch_quotes_with_diagnostics(&context)
            .await
            .unwrap();
        assert!(quote.is_some());
        assert!(diagnostics.has_success());
    }

    #[tokio::test]
    async fn test_historical_quotes() {
        let resolver = Arc::new(MockResolver);
        let mut registry = ProviderRegistry::new(resolver);
        registry.register(make_provider("YAHOO", 1, false));

        let context = make_context();
        let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2025, 1, 31).unwrap();
        let result = registry.get_historical_quotes(&context, start, end).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_search_no_results() {
        let resolver = Arc::new(MockResolver);
        let registry = ProviderRegistry::new(resolver);
        let result = registry.search("UNKNOWN").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_circuit_isolation() {
        let resolver = Arc::new(MockResolver);
        let mut registry = ProviderRegistry::new(resolver);
        registry.register(make_provider("YAHOO", 1, false));

        assert!(!registry.is_circuit_open("YAHOO"));
    }
}
