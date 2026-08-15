//! Investment OS Market Data Crate
//!
//! This crate provides provider-agnostic market data fetching capabilities
//! for the Investment OS application.
//!
//! # Overview
//!
//! The market data crate supports:
//! - Multiple asset types: equities, crypto, FX, precious metals
//! - Multiple providers: Yahoo Finance, Alpha Vantage, etc.
//! - Provider-agnostic instrument resolution
//! - Rate limiting and circuit breaking
//!
//! # Architecture
//!
//! ```text
//! +------------------+     +------------------+
//! |   Domain Layer   | --> |  InstrumentId    |  (canonical identity)
//! +------------------+     +------------------+
//!                                  |
//!                                  v
//!                          +------------------+
//!                          |    Resolver      |  (chain of responsibility)
//!                          +------------------+
//!                                  |
//!                                  v
//!                         +-------------------+
//!                         | ProviderInstrument|  (provider-specific)
//!                         +-------------------+
//!                                  |
//!                                  v
//!                          +------------------+
//!                          |    Provider      |  (Yahoo, AlphaVantage, etc.)
//!                          +------------------+
//!                                  |
//!                                  v
//!                          +------------------+
//!                          |     Quote        |  (market data)
//!                          +------------------+
//! ```
//!
//! # Core Types
//!
//! - [`InstrumentId`] - Provider-agnostic instrument identifier
//! - [`ProviderInstrument`] - Provider-specific lookup parameters
//! - [`Quote`] - Market data quote with OHLCV data
//! - [`QuoteContext`] - Request context including overrides and preferences
//! - [`AssetProfile`] - Provider-sourced profile data (sector, industry, etc.)
//! - [`AssetKind`] - Classification of asset types

pub mod errors;
pub mod models;
pub mod provider;
pub mod registry;
pub mod resolver;

// Re-export all public types from models
pub use models::{
    AssetKind, AssetProfile, BondQuoteMetadata, Coverage, Currency, DividendEvent, InstrumentId,
    InstrumentKind, Mic, ProviderId, ProviderInstrument, ProviderOverrides, ProviderSymbol, Quote,
    QuoteContext, QuoteIdentifiers, SearchResult, SplitEvent,
};

// Re-export provider types
pub use provider::{MarketDataProvider, ProviderCapabilities, RateLimit};

// Re-export registry types
pub use registry::{
    CircuitBreaker, CircuitBreakerConfig, CircuitMetrics, CircuitState, FetchDiagnostics,
    ProviderAttempt, ProviderRegistry, QuoteValidator, RateLimitConfig, RateLimiter, SkipReason,
    ValidationIssue, ValidationSeverity, ValidatorConfig,
};

// Re-export resolver types
pub use resolver::{
    AssetResolver, ExchangeInfo, ExchangeMap, ExchangeSuffix, ResolutionSource, ResolvedInstrument,
    Resolver, ResolverChain, RulesResolver, SymbolResolver,
};
