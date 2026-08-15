use serde::{Deserialize, Serialize};

/// Search result from a market data provider.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub symbol: String,
    pub name: String,
    pub exchange: Option<String>,
    pub kind: Option<String>,
    pub currency: Option<String>,
    pub provider: String,
}
