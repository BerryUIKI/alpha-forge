use serde::{Deserialize, Serialize};

/// Provider-specific instrument parameters (already resolved from canonical InstrumentId).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProviderInstrument {
    pub symbol: String,
    pub kind: String,
    pub currency: Option<String>,
    pub exchange: Option<String>,
}

/// User-configured provider overrides for a specific asset.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProviderOverrides {
    pub provider: String,
    pub symbol: String,
    pub kind: Option<String>,
    pub currency: Option<String>,
}
