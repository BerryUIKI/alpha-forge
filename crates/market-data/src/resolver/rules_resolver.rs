//! Rules resolver - deterministic MIC->suffix resolution.
//!
//! This resolver applies deterministic rules to convert canonical instruments
//! to provider-specific symbols. It uses the exchange map for securities
//! and provider-specific format rules for FX, crypto, and metals.

use async_trait::async_trait;
use std::borrow::Cow;
use std::sync::Arc;

use crate::errors::MarketDataError;
use crate::models::{Currency, InstrumentId, ProviderInstrument, QuoteContext};
use crate::resolver::exchange_suffixes::{yahoo_equity_base_to_provider, ExchangeMap};
use crate::resolver::traits::{ResolutionSource, ResolvedInstrument, Resolver};

/// Resolves provider instruments from deterministic MIC->suffix rules.
///
/// This resolver handles:
/// - Securities: Uses exchange map to add provider-specific suffixes
/// - FX: Formats currency pairs according to provider conventions
/// - Crypto: Formats crypto pairs according to provider conventions
/// - Metals: Maps metal codes to provider-specific symbols
///
/// # Supported Providers
///
/// - `YAHOO`: Yahoo Finance format (SHOP.TO, BTC-USD, EURUSD=X)
/// - `ALPHA_VANTAGE`: AlphaVantage format (SHOP.TO, CryptoPair, FxPair)
/// - `METAL_PRICE_API`: Metal Price API format
pub struct RulesResolver {
    exchange_map: ExchangeMap,
}

impl RulesResolver {
    /// Create a new RulesResolver with the default exchange map.
    pub fn new() -> Self {
        Self {
            exchange_map: ExchangeMap::new(),
        }
    }

    /// Create a RulesResolver with a custom exchange map.
    pub fn with_exchange_map(exchange_map: ExchangeMap) -> Self {
        Self { exchange_map }
    }

    /// Get the expected currency for an equity on a provider.
    pub fn get_equity_currency(
        &self,
        mic: &Option<Cow<'static, str>>,
        provider: &str,
    ) -> Option<Currency> {
        let mic = mic.as_ref()?;
        let provider = Cow::Owned(provider.to_string());
        self.exchange_map
            .get_currency(mic, &provider)
            .map(|s| Currency::from(s.to_string()))
    }

    /// Resolve an equity instrument.
    fn resolve_equity(
        &self,
        ticker: &Arc<str>,
        mic: &Option<Cow<'static, str>>,
        provider: &str,
    ) -> Option<ProviderInstrument> {
        // Yahoo normalizes share-class dots to hyphens (BRK.B -> BRK-B)
        let base = if provider == "YAHOO" {
            yahoo_equity_base_to_provider(ticker)
        } else {
            ticker.to_string()
        };

        let symbol = match mic {
            Some(mic) => {
                let provider_ref = Cow::Owned(provider.to_string());
                // Look up suffix for this MIC and provider
                match self.exchange_map.get_suffix(mic, &provider_ref) {
                    Some(suffix) => format!("{}{}", base, suffix),
                    None => {
                        // No mapping for this provider — fall back to YAHOO suffix
                        // (most providers use the same exchange suffixes as Yahoo)
                        let yahoo_ref: Cow<'static, str> = Cow::Borrowed("YAHOO");
                        match self.exchange_map.get_suffix(mic, &yahoo_ref) {
                            Some(suffix) => format!("{}{}", base, suffix),
                            None => {
                                // No mapping found at all — try ticker only
                                base
                            }
                        }
                    }
                }
            }
            None => {
                // No MIC = assume US market, no suffix needed
                base
            }
        };

        Some(ProviderInstrument {
            symbol,
            kind: "equity".to_string(),
            currency: None,
            exchange: mic.as_deref().map(|m| m.to_string()),
        })
    }

    /// Resolve a crypto instrument.
    fn resolve_crypto(
        &self,
        base: &Arc<str>,
        quote: &Currency,
        provider: &str,
    ) -> Option<ProviderInstrument> {
        match provider {
            "YAHOO" => {
                // Yahoo uses "BTC-USD" format
                Some(ProviderInstrument {
                    symbol: format!("{}-{}", base, quote),
                    kind: "crypto".to_string(),
                    currency: Some(quote.to_string()),
                    exchange: None,
                })
            }
            "ALPHA_VANTAGE" => {
                // AlphaVantage uses "BTC/USD" style via separate symbol and market
                Some(ProviderInstrument {
                    symbol: format!("{}/{}", base, quote),
                    kind: "crypto".to_string(),
                    currency: Some(quote.to_string()),
                    exchange: None,
                })
            }
            _ => None,
        }
    }

    /// Resolve an FX instrument.
    fn resolve_fx(
        &self,
        base: &Currency,
        quote: &Currency,
        provider: &str,
    ) -> Option<ProviderInstrument> {
        match provider {
            "YAHOO" => {
                // Yahoo uses "EURUSD=X" format
                Some(ProviderInstrument {
                    symbol: format!("{}{}=X", base, quote),
                    kind: "fx".to_string(),
                    currency: Some(quote.to_string()),
                    exchange: None,
                })
            }
            "ALPHA_VANTAGE" => {
                // AlphaVantage uses from/to pair in the symbol
                Some(ProviderInstrument {
                    symbol: format!("{}{}", base, quote),
                    kind: "fx".to_string(),
                    currency: Some(quote.to_string()),
                    exchange: None,
                })
            }
            _ => None,
        }
    }

    /// Resolve a bond instrument by ISIN.
    ///
    /// Bonds use ISIN directly — no provider-specific symbol transformation needed.
    fn resolve_bond(&self, isin: &Arc<str>, _provider: &str) -> Option<ProviderInstrument> {
        Some(ProviderInstrument {
            symbol: isin.to_string(),
            kind: "bond".to_string(),
            currency: None,
            exchange: None,
        })
    }

    /// Resolve an option instrument.
    /// Yahoo and Alpha Vantage accept OCC symbols as equity-like symbols.
    fn resolve_option(&self, occ_symbol: &Arc<str>, provider: &str) -> Option<ProviderInstrument> {
        match provider {
            "YAHOO" | "ALPHA_VANTAGE" => Some(ProviderInstrument {
                symbol: occ_symbol.to_string(),
                kind: "option".to_string(),
                currency: None,
                exchange: None,
            }),
            _ => None,
        }
    }

    /// Resolve a metal instrument.
    fn resolve_metal(
        &self,
        code: &Arc<str>,
        quote: &Currency,
        provider: &str,
    ) -> Option<ProviderInstrument> {
        match provider {
            "METAL_PRICE_API" => Some(ProviderInstrument {
                symbol: code.to_string(),
                kind: "metal".to_string(),
                currency: Some(quote.to_string()),
                exchange: None,
            }),
            "YAHOO" => {
                // Yahoo uses futures symbols for metals
                let futures = match code.as_ref() {
                    "XAU" => "GC=F", // Gold
                    "XAG" => "SI=F", // Silver
                    "XPT" => "PL=F", // Platinum
                    "XPD" => "PA=F", // Palladium
                    _ => return None,
                };
                Some(ProviderInstrument {
                    symbol: futures.to_string(),
                    kind: "metal".to_string(),
                    currency: Some(quote.to_string()),
                    exchange: None,
                })
            }
            _ => None,
        }
    }
}

impl Default for RulesResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Resolver for RulesResolver {
    async fn resolve(
        &self,
        context: &QuoteContext,
        provider: &str,
    ) -> Option<Result<ResolvedInstrument, MarketDataError>> {
        // CUSTOM_SCRAPER: extract symbol from any instrument variant
        if provider == "CUSTOM_SCRAPER" {
            let symbol = match &context.instrument {
                InstrumentId::Equity { ticker, .. } => ticker.to_string(),
                InstrumentId::Crypto { base, .. } => base.to_string(),
                InstrumentId::Fx { base, quote } => format!("{}{}", base, quote),
                InstrumentId::Metal { code, .. } => code.to_string(),
                InstrumentId::Bond { isin } => isin.to_string(),
                InstrumentId::Option { occ_symbol } => occ_symbol.to_string(),
            };
            return Some(Ok(ResolvedInstrument {
                instrument: ProviderInstrument {
                    symbol,
                    kind: "equity".to_string(),
                    currency: None,
                    exchange: None,
                },
                source: ResolutionSource::Rules,
            }));
        }

        let instrument = match &context.instrument {
            InstrumentId::Equity { ticker, mic } => self.resolve_equity(ticker, mic, provider)?,

            InstrumentId::Crypto { base, quote } => self.resolve_crypto(base, quote, provider)?,

            InstrumentId::Fx { base, quote } => self.resolve_fx(base, quote, provider)?,

            InstrumentId::Metal { code, quote } => self.resolve_metal(code, quote, provider)?,

            InstrumentId::Option { occ_symbol } => self.resolve_option(occ_symbol, provider)?,

            InstrumentId::Bond { isin } => self.resolve_bond(isin, provider)?,
        };

        Some(Ok(ResolvedInstrument {
            instrument,
            source: ResolutionSource::Rules,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::QuoteIdentifiers;

    fn make_equity_context(ticker: &str, mic: Option<&'static str>) -> QuoteContext {
        QuoteContext {
            instrument: InstrumentId::Equity {
                ticker: Arc::from(ticker),
                mic: mic.map(Cow::Borrowed),
            },
            identifiers: QuoteIdentifiers::default(),
            overrides: None,
            currency_hint: None,
            preferred_provider: None,
            bond_metadata: None,
            custom_provider_code: None,
        }
    }

    fn make_fx_context(base: &'static str, quote: &'static str) -> QuoteContext {
        QuoteContext {
            instrument: InstrumentId::Fx {
                base: Cow::Borrowed(base),
                quote: Cow::Borrowed(quote),
            },
            identifiers: QuoteIdentifiers::default(),
            overrides: None,
            currency_hint: None,
            preferred_provider: None,
            bond_metadata: None,
            custom_provider_code: None,
        }
    }

    fn make_crypto_context(base: &str, quote: &'static str) -> QuoteContext {
        QuoteContext {
            instrument: InstrumentId::Crypto {
                base: Arc::from(base),
                quote: Cow::Borrowed(quote),
            },
            identifiers: QuoteIdentifiers::default(),
            overrides: None,
            currency_hint: None,
            preferred_provider: None,
            bond_metadata: None,
            custom_provider_code: None,
        }
    }

    fn make_metal_context(code: &str, quote: &'static str) -> QuoteContext {
        QuoteContext {
            instrument: InstrumentId::Metal {
                code: Arc::from(code),
                quote: Cow::Borrowed(quote),
            },
            identifiers: QuoteIdentifiers::default(),
            overrides: None,
            currency_hint: None,
            preferred_provider: None,
            bond_metadata: None,
            custom_provider_code: None,
        }
    }

    #[tokio::test]
    async fn test_resolve_us_equity_yahoo() {
        let resolver = RulesResolver::new();
        let context = make_equity_context("AAPL", None);

        let result = resolver.resolve(&context, "YAHOO").await;

        assert!(result.is_some());
        let resolved = result.unwrap().unwrap();
        assert_eq!(resolved.source, ResolutionSource::Rules);
        assert_eq!(resolved.instrument.symbol, "AAPL");
        assert_eq!(resolved.instrument.kind, "equity");
    }

    #[tokio::test]
    async fn test_resolve_canadian_equity_yahoo() {
        let resolver = RulesResolver::new();
        let context = make_equity_context("SHOP", Some("XTSE"));

        let result = resolver.resolve(&context, "YAHOO").await;

        assert!(result.is_some());
        let resolved = result.unwrap().unwrap();
        assert_eq!(resolved.instrument.symbol, "SHOP.TO");
    }

    #[tokio::test]
    async fn test_resolve_yahoo_share_class_uses_provider_hyphen() {
        let resolver = RulesResolver::new();
        let context = make_equity_context("BRK.B", None);

        let result = resolver.resolve(&context, "YAHOO").await;

        assert!(result.is_some());
        let resolved = result.unwrap().unwrap();
        assert_eq!(resolved.instrument.symbol, "BRK-B");
    }

    #[tokio::test]
    async fn test_resolve_yahoo_known_exchange_suffix_keeps_dot() {
        let resolver = RulesResolver::new();
        let context = make_equity_context("VOD", Some("XLON"));

        let result = resolver.resolve(&context, "YAHOO").await;

        assert!(result.is_some());
        let resolved = result.unwrap().unwrap();
        assert_eq!(resolved.instrument.symbol, "VOD.L");
    }

    #[tokio::test]
    async fn test_resolve_yahoo_share_class_with_exchange_suffix_formats_base_then_suffix() {
        let resolver = RulesResolver::new();
        let context = make_equity_context("BRK.B", Some("XTSE"));

        let result = resolver.resolve(&context, "YAHOO").await;

        assert!(result.is_some());
        let resolved = result.unwrap().unwrap();
        assert_eq!(resolved.instrument.symbol, "BRK-B.TO");
    }

    #[tokio::test]
    async fn test_resolve_alphavantage_share_class_keeps_dot() {
        let resolver = RulesResolver::new();
        let context = make_equity_context("BRK.B", None);

        let result = resolver.resolve(&context, "ALPHA_VANTAGE").await;

        assert!(result.is_some());
        let resolved = result.unwrap().unwrap();
        assert_eq!(resolved.instrument.symbol, "BRK.B");
    }

    #[tokio::test]
    async fn test_resolve_canadian_equity_alphavantage() {
        let resolver = RulesResolver::new();
        let context = make_equity_context("SHOP", Some("XTSE"));

        let result = resolver.resolve(&context, "ALPHA_VANTAGE").await;

        assert!(result.is_some());
        let resolved = result.unwrap().unwrap();
        // Alpha Vantage expects the same Yahoo-style suffix for TSX
        assert_eq!(resolved.instrument.symbol, "SHOP.TO");
    }

    #[tokio::test]
    async fn test_resolve_fx_yahoo() {
        let resolver = RulesResolver::new();
        let context = make_fx_context("EUR", "USD");

        let result = resolver.resolve(&context, "YAHOO").await;

        assert!(result.is_some());
        let resolved = result.unwrap().unwrap();
        assert_eq!(resolved.instrument.symbol, "EURUSD=X");
        assert_eq!(resolved.instrument.kind, "fx");
    }

    #[tokio::test]
    async fn test_resolve_fx_alphavantage() {
        let resolver = RulesResolver::new();
        let context = make_fx_context("EUR", "USD");

        let result = resolver.resolve(&context, "ALPHA_VANTAGE").await;

        assert!(result.is_some());
        let resolved = result.unwrap().unwrap();
        assert_eq!(resolved.instrument.symbol, "EURUSD");
    }

    #[tokio::test]
    async fn test_resolve_crypto_yahoo() {
        let resolver = RulesResolver::new();
        let context = make_crypto_context("BTC", "USD");

        let result = resolver.resolve(&context, "YAHOO").await;

        assert!(result.is_some());
        let resolved = result.unwrap().unwrap();
        assert_eq!(resolved.instrument.symbol, "BTC-USD");
        assert_eq!(resolved.instrument.kind, "crypto");
    }

    #[tokio::test]
    async fn test_resolve_crypto_alphavantage() {
        let resolver = RulesResolver::new();
        let context = make_crypto_context("BTC", "USD");

        let result = resolver.resolve(&context, "ALPHA_VANTAGE").await;

        assert!(result.is_some());
        let resolved = result.unwrap().unwrap();
        assert_eq!(resolved.instrument.symbol, "BTC/USD");
    }

    #[tokio::test]
    async fn test_resolve_metal_yahoo() {
        let resolver = RulesResolver::new();
        let context = make_metal_context("XAU", "USD");

        let result = resolver.resolve(&context, "YAHOO").await;

        assert!(result.is_some());
        let resolved = result.unwrap().unwrap();
        assert_eq!(resolved.instrument.symbol, "GC=F");
        assert_eq!(resolved.instrument.kind, "metal");
    }

    #[tokio::test]
    async fn test_resolve_metal_api() {
        let resolver = RulesResolver::new();
        let context = make_metal_context("XAU", "USD");

        let result = resolver.resolve(&context, "METAL_PRICE_API").await;

        assert!(result.is_some());
        let resolved = result.unwrap().unwrap();
        assert_eq!(resolved.instrument.symbol, "XAU");
        assert_eq!(resolved.instrument.currency.as_deref(), Some("USD"));
    }

    #[tokio::test]
    async fn test_resolve_unknown_provider() {
        let resolver = RulesResolver::new();
        let context = make_fx_context("EUR", "USD");

        let result = resolver.resolve(&context, "UNKNOWN_PROVIDER").await;

        // Should return None for unknown providers
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_resolve_unknown_mic() {
        let resolver = RulesResolver::new();
        let context = make_equity_context("TEST", Some("UNKNOWN_MIC"));

        let result = resolver.resolve(&context, "YAHOO").await;

        // Unknown MICs fall back to bare ticker
        assert!(result.is_some());
        let resolved = result.unwrap().unwrap();
        assert_eq!(resolved.instrument.symbol, "TEST");
    }

    #[tokio::test]
    async fn test_get_equity_currency() {
        let resolver = RulesResolver::new();

        // Toronto
        let currency = resolver.get_equity_currency(&Some(Cow::Borrowed("XTSE")), "YAHOO");
        assert_eq!(currency.as_deref(), Some("CAD"));

        // London
        let currency = resolver.get_equity_currency(&Some(Cow::Borrowed("XLON")), "YAHOO");
        assert_eq!(currency.as_deref(), Some("GBP"));

        // No MIC
        let currency = resolver.get_equity_currency(&None, "YAHOO");
        assert!(currency.is_none());
    }
}
