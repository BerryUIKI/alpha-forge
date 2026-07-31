// Agent runtime core — Phase 1 placeholder.
// Will contain agent execution engine in later phases.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f64,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            model: "gpt-4o".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
        }
    }
}
