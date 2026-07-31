// Provider abstraction core — Phase 1 placeholder.
// Will define provider traits for AI, market-data, news in later phases.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub base_url: String,
    pub api_key_env: String,
}
