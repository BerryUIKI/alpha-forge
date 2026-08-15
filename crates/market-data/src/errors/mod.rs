//! Error types and retry classification for the market data crate.

mod retry;
pub use retry::RetryClass;

use thiserror::Error;

/// Errors that can occur during market data operations.
#[derive(Error, Debug)]
pub enum MarketDataError {
    #[error("Symbol not found: {0}")]
    SymbolNotFound(String),
    #[error("Unsupported asset type: {0}")]
    UnsupportedAssetType(String),
    #[error("No data for date range")]
    NoDataForRange,
    #[error("Rate limited: {provider}")]
    RateLimited { provider: String },
    #[error("Timeout: {provider}")]
    Timeout { provider: String },
    #[error("Provider error: {provider} - {message}")]
    ProviderError { provider: String, message: String },
    #[error("Resolution failed for provider: {provider}")]
    ResolutionFailed { provider: String },
    #[error("Circuit open: {provider}")]
    CircuitOpen { provider: String },
    #[error("Validation failed: {message}")]
    ValidationFailed { message: String },
    #[error("No providers available")]
    NoProvidersAvailable,
    #[error("All providers failed")]
    AllProvidersFailed,
    #[error("Operation '{operation}' not supported by provider '{provider}'")]
    NotSupported { operation: String, provider: String },
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
}

impl MarketDataError {
    pub fn retry_class(&self) -> RetryClass {
        match self {
            Self::SymbolNotFound(_)
            | Self::UnsupportedAssetType(_)
            | Self::ValidationFailed { .. } => RetryClass::Never,
            Self::Timeout { .. } => RetryClass::FailoverWithPenalty,
            Self::RateLimited { .. } => RetryClass::NextProvider,
            Self::ProviderError { .. }
            | Self::ResolutionFailed { .. }
            | Self::NotSupported { .. }
            | Self::NoDataForRange
            | Self::Network(_) => RetryClass::NextProvider,
            Self::CircuitOpen { .. } => RetryClass::CircuitOpen,
            Self::NoProvidersAvailable | Self::AllProvidersFailed => RetryClass::Never,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_symbol_not_found_never_retries() {
        let error = MarketDataError::SymbolNotFound("INVALID".to_string());
        assert_eq!(error.retry_class(), RetryClass::Never);
    }
    #[test]
    fn test_timeout_retries_with_backoff() {
        let error = MarketDataError::Timeout {
            provider: "YAHOO".to_string(),
        };
        assert_eq!(error.retry_class(), RetryClass::FailoverWithPenalty);
    }
    #[test]
    fn test_rate_limited_tries_next_provider() {
        let error = MarketDataError::RateLimited {
            provider: "YAHOO".to_string(),
        };
        assert_eq!(error.retry_class(), RetryClass::NextProvider);
    }
    #[test]
    fn test_circuit_open_returns_circuit_open() {
        let error = MarketDataError::CircuitOpen {
            provider: "YAHOO".to_string(),
        };
        assert_eq!(error.retry_class(), RetryClass::CircuitOpen);
    }
    #[test]
    fn test_error_display() {
        let error = MarketDataError::SymbolNotFound("INVALID".to_string());
        assert_eq!(format!("{}", error), "Symbol not found: INVALID");
    }
}
