use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Provider-sourced profile data for an asset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetProfile {
    pub symbol: String,
    pub name: String,
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,
    pub market_cap: Option<Decimal>,
    pub shares_outstanding: Option<Decimal>,
    pub currency: Option<String>,
    pub exchange: Option<String>,
    pub exchange_mic: Option<String>,
}
