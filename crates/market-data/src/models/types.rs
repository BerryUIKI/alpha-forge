use std::borrow::Cow;

/// Provider identifier (e.g., "YAHOO", "ALPHA_VANTAGE").
pub type ProviderId = Cow<'static, str>;
/// Market Identifier Code (ISO 10383).
pub type Mic = Cow<'static, str>;
/// Currency code (ISO 4217).
pub type Currency = Cow<'static, str>;
/// Provider-specific symbol string.
pub type ProviderSymbol = String;
