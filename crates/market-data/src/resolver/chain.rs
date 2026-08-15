//! Resolver chain - composite resolver that tries resolvers in order.
//!
//! The resolver chain is the main entry point for symbol resolution. It
//! combines multiple resolvers and tries them in order until one succeeds.

use async_trait::async_trait;

use crate::errors::MarketDataError;
use crate::models::{Currency, InstrumentId, QuoteContext};

use super::asset_resolver::AssetResolver;
use super::exchange_suffixes::ExchangeMap;
use super::rules_resolver::RulesResolver;
use super::traits::{ResolvedInstrument, Resolver, SymbolResolver};

/// Composite resolver that tries multiple resolvers in order.
///
/// The resolution order is:
/// 1. Asset overrides (from `Asset.provider_overrides`)
/// 2. Deterministic rules (MIC->suffix mappings)
///
/// The chain stops at the first resolver that returns a result.
/// A resolver returning `None` means it cannot handle the request,
/// and the next resolver is tried.
///
/// # Example
///
/// ```ignore
/// let chain = ResolverChain::new();
///
/// let context = QuoteContext { ... };
/// let resolved = chain.resolve(&context, "YAHOO").await?;
/// // resolved.instrument = ProviderInstrument { symbol: "SHOP.TO", ... }
/// // resolved.source = ResolutionSource::Rules
/// ```
pub struct ResolverChain {
    resolvers: Vec<Box<dyn Resolver>>,
    rules_resolver: RulesResolver,
}

impl ResolverChain {
    /// Create a new ResolverChain with the default resolver order.
    ///
    /// Default order:
    /// 1. AssetResolver (provider overrides)
    /// 2. RulesResolver (MIC->suffix rules)
    pub fn new() -> Self {
        Self::with_exchange_map(ExchangeMap::new())
    }

    /// Create a ResolverChain with a custom exchange map.
    pub fn with_exchange_map(exchange_map: ExchangeMap) -> Self {
        let rules_resolver = RulesResolver::with_exchange_map(exchange_map);

        Self {
            resolvers: vec![Box::new(AssetResolver::new())],
            rules_resolver,
        }
    }

    /// Add a custom resolver to the chain.
    ///
    /// The resolver is added before the rules resolver (which is always last).
    pub fn add_resolver(&mut self, resolver: Box<dyn Resolver>) {
        self.resolvers.push(resolver);
    }
}

impl Default for ResolverChain {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SymbolResolver for ResolverChain {
    async fn resolve(
        &self,
        context: &QuoteContext,
        provider_id: &str,
    ) -> Result<ResolvedInstrument, MarketDataError> {
        // Try each resolver in order
        for resolver in &self.resolvers {
            if let Some(result) = resolver.resolve(context, provider_id).await {
                return result;
            }
        }

        // Finally try the rules resolver (always last)
        if let Some(result) = self.rules_resolver.resolve(context, provider_id).await {
            return result;
        }

        // No resolver could handle this
        Err(MarketDataError::ResolutionFailed {
            provider: provider_id.to_string(),
        })
    }

    async fn get_currency(&self, context: &QuoteContext) -> Option<Currency> {
        match &context.instrument {
            InstrumentId::Equity { mic, .. } => {
                let provider = context.preferred_provider.as_ref()?;
                self.rules_resolver.get_equity_currency(mic, provider)
            }
            InstrumentId::Fx { quote, .. } => Some(quote.clone()),
            InstrumentId::Crypto { quote, .. } => Some(quote.clone()),
            InstrumentId::Metal { quote, .. } => Some(quote.clone()),
            InstrumentId::Option { .. } => None,
            InstrumentId::Bond { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::sync::Arc;

    use super::*;
    use crate::models::QuoteIdentifiers;
    use crate::ProviderOverrides;
    use crate::ResolutionSource;

    #[tokio::test]
    async fn test_chain_with_override() {
        let chain = ResolverChain::new();

        let overrides = ProviderOverrides {
            provider: "YAHOO".to_string(),
            symbol: "CUSTOM.SYMBOL".to_string(),
            kind: Some("equity".to_string()),
            currency: Some("CAD".to_string()),
        };

        let context = QuoteContext {
            instrument: InstrumentId::Equity {
                ticker: Arc::from("TEST"),
                mic: Some(Cow::Borrowed("XTSE")),
            },
            identifiers: QuoteIdentifiers::default(),
            overrides: Some(overrides),
            currency_hint: Some(Cow::Borrowed("CAD")),
            preferred_provider: None,
            bond_metadata: None,
            custom_provider_code: None,
        };

        let resolved = chain.resolve(&context, "YAHOO").await.unwrap();

        // Should use override, not rules
        assert_eq!(resolved.source, ResolutionSource::Override);
        assert_eq!(resolved.instrument.symbol, "CUSTOM.SYMBOL");
    }

    #[tokio::test]
    async fn test_chain_falls_through_to_rules() {
        let chain = ResolverChain::new();

        // Context without override - should fall through to rules
        let context = QuoteContext {
            instrument: InstrumentId::Equity {
                ticker: Arc::from("SHOP"),
                mic: Some(Cow::Borrowed("XTSE")),
            },
            identifiers: QuoteIdentifiers::default(),
            overrides: None,
            currency_hint: Some(Cow::Borrowed("CAD")),
            preferred_provider: None,
            bond_metadata: None,
            custom_provider_code: None,
        };

        let resolved = chain.resolve(&context, "YAHOO").await.unwrap();

        // Should use rules
        assert_eq!(resolved.source, ResolutionSource::Rules);
        assert_eq!(resolved.instrument.symbol, "SHOP.TO");
    }

    #[tokio::test]
    async fn test_chain_override_for_different_provider() {
        let chain = ResolverChain::new();

        let overrides = ProviderOverrides {
            provider: "YAHOO".to_string(),
            symbol: "SHOP.TO".to_string(),
            kind: Some("equity".to_string()),
            currency: Some("CAD".to_string()),
        };

        let context = QuoteContext {
            instrument: InstrumentId::Equity {
                ticker: Arc::from("SHOP"),
                mic: Some(Cow::Borrowed("XTSE")),
            },
            identifiers: QuoteIdentifiers::default(),
            overrides: Some(overrides),
            currency_hint: Some(Cow::Borrowed("CAD")),
            preferred_provider: None,
            bond_metadata: None,
            custom_provider_code: None,
        };

        // Resolve for ALPHA_VANTAGE (no override) - should use rules
        let resolved = chain.resolve(&context, "ALPHA_VANTAGE").await.unwrap();

        assert_eq!(resolved.source, ResolutionSource::Rules);
        assert_eq!(resolved.instrument.symbol, "SHOP.TO");
    }

    #[tokio::test]
    async fn test_chain_fx_resolution() {
        let chain = ResolverChain::new();

        let context = QuoteContext {
            instrument: InstrumentId::Fx {
                base: Cow::Borrowed("EUR"),
                quote: Cow::Borrowed("USD"),
            },
            identifiers: QuoteIdentifiers::default(),
            overrides: None,
            currency_hint: None,
            preferred_provider: None,
            bond_metadata: None,
            custom_provider_code: None,
        };

        let resolved = chain.resolve(&context, "YAHOO").await.unwrap();

        assert_eq!(resolved.instrument.symbol, "EURUSD=X");
        assert_eq!(resolved.instrument.kind, "fx");
    }

    #[tokio::test]
    async fn test_chain_resolution_failed() {
        let chain = ResolverChain::new();

        let context = QuoteContext {
            instrument: InstrumentId::Crypto {
                base: Arc::from("BTC"),
                quote: Cow::Borrowed("USD"),
            },
            identifiers: QuoteIdentifiers::default(),
            overrides: None,
            currency_hint: None,
            preferred_provider: None,
            bond_metadata: None,
            custom_provider_code: None,
        };

        let result = chain.resolve(&context, "UNKNOWN_PROVIDER").await;

        assert!(result.is_err());
        match result.unwrap_err() {
            MarketDataError::ResolutionFailed { provider } => {
                assert_eq!(provider, "UNKNOWN_PROVIDER");
            }
            _ => panic!("Expected ResolutionFailed error"),
        }
    }

    #[tokio::test]
    async fn test_get_currency_equity() {
        let chain = ResolverChain::new();

        let context = QuoteContext {
            instrument: InstrumentId::Equity {
                ticker: Arc::from("SHOP"),
                mic: Some(Cow::Borrowed("XTSE")),
            },
            identifiers: QuoteIdentifiers::default(),
            overrides: None,
            currency_hint: None,
            preferred_provider: Some(Cow::Borrowed("YAHOO")),
            bond_metadata: None,
            custom_provider_code: None,
        };

        let currency = chain.get_currency(&context).await;
        assert_eq!(currency.as_deref(), Some("CAD"));
    }

    #[tokio::test]
    async fn test_get_currency_fx() {
        let chain = ResolverChain::new();

        let context = QuoteContext {
            instrument: InstrumentId::Fx {
                base: Cow::Borrowed("EUR"),
                quote: Cow::Borrowed("USD"),
            },
            identifiers: QuoteIdentifiers::default(),
            overrides: None,
            currency_hint: None,
            preferred_provider: None,
            bond_metadata: None,
            custom_provider_code: None,
        };

        let currency = chain.get_currency(&context).await;
        assert_eq!(currency.as_deref(), Some("USD"));
    }
}
