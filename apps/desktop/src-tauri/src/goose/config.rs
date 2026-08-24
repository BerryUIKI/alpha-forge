//! Goose integration configuration and security policies (M10-G5)

use serde::{Deserialize, Serialize};
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

/// Provider Policy governing allowed LLM endpoints and credential boundaries (M10-G5)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderPolicy {
    /// List of allowlisted LLM providers
    pub allowed_providers: Vec<String>,

    /// List of allowlisted model names
    pub allowed_models: Vec<String>,

    /// OS Keyring service namespace identifier
    pub keyring_service: String,

    /// Enforce zero plaintext credential fallback
    pub disallow_plaintext_fallback: bool,
}

impl Default for ProviderPolicy {
    fn default() -> Self {
        Self {
            allowed_providers: vec![
                "openai".to_string(),
                "anthropic".to_string(),
                "ollama".to_string(),
                "demo".to_string(),
            ],
            allowed_models: vec![
                "gpt-4o".to_string(),
                "gpt-4o-mini".to_string(),
                "o1".to_string(),
                "o3-mini".to_string(),
                "claude-3-5-sonnet-20241022".to_string(),
                "claude-3-5-haiku-20241022".to_string(),
                "llama3.2".to_string(),
                "deepseek-r1".to_string(),
                "qwen2.5".to_string(),
                "synthetic-v1".to_string(),
            ],
            keyring_service: "alphaforge-goose".to_string(),
            disallow_plaintext_fallback: true,
        }
    }
}

impl ProviderPolicy {
    /// Check if a provider is allowlisted
    pub fn is_provider_allowed(&self, provider: &str) -> bool {
        self.allowed_providers
            .iter()
            .any(|p| p.eq_ignore_ascii_case(provider))
    }

    /// Check if a model is allowlisted
    pub fn is_model_allowed(&self, model: &str) -> bool {
        self.allowed_models
            .iter()
            .any(|m| m.eq_ignore_ascii_case(model))
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

    #[test]
    fn provider_policy_allowlist_validation() {
        let policy = ProviderPolicy::default();

        // Allowed providers
        assert!(policy.is_provider_allowed("openai"));
        assert!(policy.is_provider_allowed("ANTHROPIC"));
        assert!(policy.is_provider_allowed("ollama"));
        assert!(policy.is_provider_allowed("demo"));

        // Disallowed providers
        assert!(!policy.is_provider_allowed("untrusted_provider"));
        assert!(!policy.is_provider_allowed("unknown_proxy"));

        // Allowed models
        assert!(policy.is_model_allowed("gpt-4o"));
        assert!(policy.is_model_allowed("claude-3-5-sonnet-20241022"));
        assert!(policy.is_model_allowed("llama3.2"));

        // Disallowed models
        assert!(!policy.is_model_allowed("arbitrary_model_xyz"));
    }
}
