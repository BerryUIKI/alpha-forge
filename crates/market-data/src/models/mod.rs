//! Market data models
//!
//! This module contains the core data types for market data operations.

mod coverage;
mod dividend;
mod instrument;
mod profile;
mod provider_params;
mod quote;
mod search;
mod types;

pub use chrono::NaiveDate;
pub use coverage::Coverage;
pub use dividend::DividendEvent;
pub use instrument::{AssetKind, InstrumentId, InstrumentKind};
pub use profile::AssetProfile;
pub use provider_params::{ProviderInstrument, ProviderOverrides};
pub use quote::{BondQuoteMetadata, Quote, QuoteContext, QuoteIdentifiers};
pub use rust_decimal::Decimal;
pub use search::SearchResult;
pub use types::{Currency, Mic, ProviderId, ProviderSymbol};

/// A stock split event from a market data provider.
///
/// The `ratio` is numerator / denominator:
/// - Forward 3:1 split => ratio = 3.0 (shares triple)
/// - Reverse 1:5 split => ratio = 0.2 (shares become 1/5)
#[derive(Debug, Clone)]
pub struct SplitEvent {
    pub date: NaiveDate,
    pub ratio: Decimal,
}
