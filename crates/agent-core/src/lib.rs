//! Core Agent runtime, supervisor, registry, and execution engine for AlphaForge.
//!
//! Governed by ADR-0010 (Managed Agent Worker Subprocess Architecture) and SUBPROCESS_ROADMAP.md (AW2).

pub mod diagnostics;
pub mod error;
pub mod launch;
pub mod manager;
pub mod manifest;
pub mod supervisor;

pub use diagnostics::*;
pub use error::*;
pub use launch::*;
pub use manager::*;
pub use manifest::*;
pub use supervisor::*;

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
