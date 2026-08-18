//! Goose integration configuration

use std::path::PathBuf;
use std::time::Duration;

/// Goose runtime configuration
#[derive(Debug, Clone)]
pub struct GooseConfig {
    /// Path to the Goose binary
    pub binary_path: PathBuf,

    /// Expected SHA-256 checksum of the binary (hex-encoded)
    pub binary_checksum: String,

    /// Maximum execution time per task
    pub timeout: Duration,

    /// Maximum output size in bytes
    pub max_output_bytes: usize,

    /// Maximum turns per execution
    pub max_turns: u32,

    /// Maximum concurrent Goose processes
    pub max_concurrent: usize,

    /// Working directory for Goose process (isolated temp directory)
    pub working_directory: PathBuf,
}

impl Default for GooseConfig {
    fn default() -> Self {
        Self {
            binary_path: PathBuf::from("goose"),
            binary_checksum: String::new(), // Must be set in production
            timeout: Duration::from_secs(300), // 5 minutes
            max_output_bytes: 1024 * 1024,  // 1MB
            max_turns: 50,
            max_concurrent: 1,
            working_directory: std::env::temp_dir().join("alphaforge-goose"),
        }
    }
}

/// Budget constraints for Goose execution
#[derive(Debug, Clone)]
pub struct ExecutionBudget {
    /// Maximum tokens to consume
    pub max_tokens: Option<u64>,

    /// Maximum cost in USD
    pub max_cost_usd: Option<f64>,

    /// Maximum execution time
    pub max_duration: Duration,

    /// Maximum turns
    pub max_turns: u32,
}

impl Default for ExecutionBudget {
    fn default() -> Self {
        Self {
            max_tokens: Some(100_000),
            max_cost_usd: Some(1.0),
            max_duration: Duration::from_secs(300),
            max_turns: 50,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_reasonable_limits() {
        let config = GooseConfig::default();
        assert!(config.timeout.as_secs() > 0);
        assert!(config.max_output_bytes > 0);
        assert!(config.max_turns > 0);
        assert!(config.max_concurrent > 0);
    }
}
