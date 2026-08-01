//! Option Core Library
//!
//! Provides option pricing models, Greeks calculations, and strategy analysis
//! for the Investment OS Option Analysis Platform.

#![allow(clippy::too_many_arguments)]

pub mod greeks;
pub mod pricing;
pub mod provider;
pub mod strategy;
pub mod volatility;

pub use greeks::*;
pub use pricing::*;
pub use provider::*;
pub use strategy::*;
pub use volatility::*;

/// Error type for option calculations
#[derive(Debug, thiserror::Error)]
pub enum OptionError {
    #[error("Invalid option parameters: {0}")]
    InvalidParameters(String),

    #[error("Pricing calculation failed: {0}")]
    PricingFailed(String),

    #[error("Implied volatility calculation failed to converge")]
    IvConvergenceFailed,

    #[error("Data provider error: {0}")]
    ProviderError(String),
}

/// Result type for option operations
pub type Result<T> = std::result::Result<T, OptionError>;
