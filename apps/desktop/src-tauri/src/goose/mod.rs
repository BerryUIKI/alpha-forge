//! Goose Agent Integration Module
//!
//! This module implements M10 Goose integration with the following components:
//! - `adapter`: GooseAdapter trait for process lifecycle management
//! - `recipe`: Recipe loader with validation
//! - `output`: Structured output parser
//! - `mcp`: AlphaForge MCP bridge (M10-G2)
//!
//! ## Security Model
//!
//! - Goose runs as a pinned sidecar process supervised by Rust
//! - No shell access; direct process spawn with fixed arguments
//! - Only allowlisted MCP extensions permitted
//! - Output validated against strict schema before use
//! - No auto-persist; user reviews all results
//!
//! See docs/DECISIONS/0004-goose-integration-topology.md for architecture decisions.
//! See docs/goose/THREAT_MODEL.md for security analysis.

pub mod adapter;
pub mod config;
pub mod error;
pub mod output;
pub mod recipe;

#[cfg(test)]
pub mod test_fixture;

pub use adapter::GooseAdapter;
pub use config::GooseConfig;
pub use error::GooseError;
pub use output::{GooseOutput, StructuredResponse};
pub use recipe::Recipe;