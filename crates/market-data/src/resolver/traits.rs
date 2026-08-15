//! Core traits for the symbol resolution chain.
//!
//! The resolution system uses a chain-of-responsibility pattern:
//! 1. [`AssetResolver`] checks for provider-specific overrides
//! 2. [`RulesResolver`] applies exchange suffix rules
//! 3. [`ResolverChain`] composes multiple resolvers

use async_trait::async_trait;

use crate::errors::MarketDataError;
use crate::models::{Currency, ProviderInstrument, QuoteContext};

/// Source of a resolved instrument mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionSource {
    /// Resolved from a user-configured provider override.
    Override,
    /// Resolved from exchange suffix rules.
    Rules,
}

/// Result of resolving an instrument for a specific provider.
#[derive(Debug, Clone)]
pub struct ResolvedInstrument {
    /// Provider-specific instrument parameters.
    pub instrument: ProviderInstrument,
    /// Source of the resolution.
    pub source: ResolutionSource,
}

/// A single resolver in the chain-of-responsibility.
///
/// Returns `None` if this resolver cannot handle the request
/// (passing to the next resolver in the chain).
#[async_trait]
pub trait Resolver: Send + Sync {
    /// Try to resolve the instrument for the given provider.
    ///
    /// Returns `None` if this resolver cannot handle the request.
    /// Returns `Some(Err(...))` if resolution was attempted but failed.
    /// Returns `Some(Ok(...))` if resolution succeeded.
    async fn resolve(
        &self,
        context: &QuoteContext,
        provider_id: &str,
    ) -> Option<Result<ResolvedInstrument, MarketDataError>>;
}

/// Full symbol resolver that either succeeds or fails.
///
/// This is the top-level interface used by the [`ProviderRegistry`](crate::registry::ProviderRegistry).
#[async_trait]
pub trait SymbolResolver: Send + Sync {
    /// Resolve an instrument for the given provider.
    async fn resolve(
        &self,
        context: &QuoteContext,
        provider_id: &str,
    ) -> Result<ResolvedInstrument, MarketDataError>;

    /// Get the quote currency for the instrument.
    async fn get_currency(&self, context: &QuoteContext) -> Option<Currency>;
}
