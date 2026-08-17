//! Skip reasons and diagnostics for provider fetch attempts.

use crate::errors::MarketDataError;

/// Reasons why a provider was skipped during quote fetching.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkipReason {
    /// Provider does not support this instrument kind.
    NotSupported,
    /// Provider circuit breaker is open.
    CircuitOpen,
    /// Provider is rate limited.
    RateLimited,
    /// Symbol resolution failed for this provider.
    ResolutionFailed,
    /// Provider returned an error.
    ProviderError,
}

/// Record of a single provider attempt during quote fetching.
#[derive(Debug, Clone)]
pub struct ProviderAttempt {
    /// Provider identifier.
    pub provider_id: String,
    /// Whether the provider was skipped (before attempting).
    pub skipped: bool,
    /// Skip reason if skipped.
    pub skip_reason: Option<SkipReason>,
    /// Error message if the attempt failed.
    pub error: Option<String>,
    /// Whether the attempt succeeded.
    pub success: bool,
}

/// Diagnostics collected during a quote fetch operation.
#[derive(Debug, Clone, Default)]
pub struct FetchDiagnostics {
    /// All provider attempts in order.
    pub attempts: Vec<ProviderAttempt>,
}

impl FetchDiagnostics {
    /// Create a new empty diagnostics collector.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a skipped provider.
    pub fn record_skip(&mut self, provider_id: impl Into<String>, reason: SkipReason) {
        self.attempts.push(ProviderAttempt {
            provider_id: provider_id.into(),
            skipped: true,
            skip_reason: Some(reason),
            error: None,
            success: false,
        });
    }

    /// Record a provider error.
    pub fn record_error(&mut self, provider_id: impl Into<String>, error: MarketDataError) {
        self.attempts.push(ProviderAttempt {
            provider_id: provider_id.into(),
            skipped: false,
            skip_reason: None,
            error: Some(error.to_string()),
            success: false,
        });
    }

    /// Record a successful provider attempt.
    pub fn record_success(&mut self, provider_id: impl Into<String>) {
        self.attempts.push(ProviderAttempt {
            provider_id: provider_id.into(),
            skipped: false,
            skip_reason: None,
            error: None,
            success: true,
        });
    }

    /// Whether any provider succeeded.
    pub fn has_success(&self) -> bool {
        self.attempts.iter().any(|a| a.success)
    }

    /// Get all skip reasons from the attempts.
    pub fn skip_reasons(&self) -> Vec<&SkipReason> {
        self.attempts
            .iter()
            .filter_map(|a| a.skip_reason.as_ref())
            .collect()
    }

    /// Get all error messages from the attempts.
    pub fn errors(&self) -> Vec<&str> {
        self.attempts
            .iter()
            .filter_map(|a| a.error.as_deref())
            .collect()
    }

    /// Get a human-readable summary of the diagnostics.
    pub fn summary(&self) -> String {
        let total = self.attempts.len();
        let successes = self.attempts.iter().filter(|a| a.success).count();
        let skips = self.attempts.iter().filter(|a| a.skipped).count();
        let failures = total - successes - skips;

        format!(
            "{} providers tried: {} succeeded, {} skipped, {} failed",
            total, successes, skips, failures
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_diagnostics() {
        let diag = FetchDiagnostics::new();
        assert!(!diag.has_success());
        assert!(diag.skip_reasons().is_empty());
    }

    #[test]
    fn test_record_skip() {
        let mut diag = FetchDiagnostics::new();
        diag.record_skip("YAHOO", SkipReason::CircuitOpen);
        assert_eq!(diag.attempts.len(), 1);
        assert!(diag.attempts[0].skipped);
        assert_eq!(diag.attempts[0].provider_id, "YAHOO");
    }

    #[test]
    fn test_record_error() {
        let mut diag = FetchDiagnostics::new();
        diag.record_error(
            "YAHOO",
            MarketDataError::RateLimited {
                provider: "YAHOO".to_string(),
            },
        );
        assert_eq!(diag.attempts.len(), 1);
        assert!(!diag.attempts[0].success);
        assert!(diag.attempts[0].error.is_some());
    }

    #[test]
    fn test_record_success() {
        let mut diag = FetchDiagnostics::new();
        diag.record_success("YAHOO");
        assert!(diag.has_success());
    }

    #[test]
    fn test_skip_reasons_collected() {
        let mut diag = FetchDiagnostics::new();
        diag.record_skip("YAHOO", SkipReason::CircuitOpen);
        diag.record_skip("ALPHA_VANTAGE", SkipReason::RateLimited);
        assert_eq!(diag.skip_reasons().len(), 2);
    }

    #[test]
    fn test_errors_collected() {
        let mut diag = FetchDiagnostics::new();
        diag.record_error(
            "YAHOO",
            MarketDataError::RateLimited {
                provider: "YAHOO".to_string(),
            },
        );
        assert_eq!(diag.errors().len(), 1);
    }

    #[test]
    fn test_summary() {
        let mut diag = FetchDiagnostics::new();
        diag.record_skip("YAHOO", SkipReason::CircuitOpen);
        diag.record_success("ALPHA_VANTAGE");
        let summary = diag.summary();
        assert!(summary.contains("2 providers"));
        assert!(summary.contains("1 succeeded"));
        assert!(summary.contains("1 skipped"));
    }
}
