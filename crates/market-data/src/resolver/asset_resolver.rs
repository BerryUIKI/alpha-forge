//! Asset resolver - resolves from Asset.provider_overrides.
//!
//! This is the first resolver in the chain. It checks if the asset has
//! an explicit override for the requested provider.

use async_trait::async_trait;

use crate::errors::MarketDataError;
use crate::models::{ProviderInstrument, QuoteContext};
use crate::resolver::traits::{ResolutionSource, ResolvedInstrument, Resolver};

/// Resolves provider instruments from Asset.provider_overrides.
///
/// When a user explicitly sets a provider-specific symbol for an asset,
/// it is stored in the `provider_overrides` field. This resolver checks
/// for those overrides first.
///
/// # Resolution Order
///
/// This resolver is typically first in the chain:
/// 1. AssetResolver (this) - check explicit overrides
/// 2. RulesResolver - apply deterministic MIC->suffix rules
pub struct AssetResolver;

impl AssetResolver {
    /// Create a new AssetResolver.
    pub fn new() -> Self {
        Self
    }
}

impl Default for AssetResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Resolver for AssetResolver {
    async fn resolve(
        &self,
        context: &QuoteContext,
        provider_id: &str,
    ) -> Option<Result<ResolvedInstrument, MarketDataError>> {
        // Check if the context has overrides
        let overrides = context.overrides.as_ref()?;

        // Check if the override's provider matches the requested provider
        if overrides.provider != provider_id {
            return None;
        }

        // Found an override - construct a ProviderInstrument from it
        let instrument = ProviderInstrument {
            symbol: overrides.symbol.clone(),
            kind: overrides
                .kind
                .clone()
                .unwrap_or_else(|| "equity".to_string()),
            currency: overrides.currency.clone(),
            exchange: None,
        };

        Some(Ok(ResolvedInstrument {
            instrument,
            source: ResolutionSource::Override,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::sync::Arc;

    use crate::models::QuoteIdentifiers;
    use crate::InstrumentId;
    use crate::ProviderOverrides;

    #[tokio::test]
    async fn test_resolve_with_override() {
        let resolver = AssetResolver::new();

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

        let result = resolver.resolve(&context, "YAHOO").await;

        assert!(result.is_some());
        let resolved = result.unwrap().unwrap();
        assert_eq!(resolved.source, ResolutionSource::Override);
        assert_eq!(resolved.instrument.symbol, "SHOP.TO");
        assert_eq!(resolved.instrument.kind, "equity");
    }

    #[tokio::test]
    async fn test_resolve_no_override() {
        let resolver = AssetResolver::new();

        // Context without overrides
        let context = QuoteContext {
            instrument: InstrumentId::Equity {
                ticker: Arc::from("AAPL"),
                mic: Some(Cow::Borrowed("XNAS")),
            },
            identifiers: QuoteIdentifiers::default(),
            overrides: None,
            currency_hint: Some(Cow::Borrowed("USD")),
            preferred_provider: None,
            bond_metadata: None,
            custom_provider_code: None,
        };

        let result = resolver.resolve(&context, "YAHOO").await;

        // Should return None when no override exists
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_resolve_override_for_different_provider() {
        let resolver = AssetResolver::new();

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

        // Try to resolve for ALPHA_VANTAGE (no override)
        let result = resolver.resolve(&context, "ALPHA_VANTAGE").await;

        // Should return None when no override for this provider
        assert!(result.is_none());
    }
}
