//! Registry module for provider management.
//!
//! This module provides the infrastructure for managing market data providers:
//! - [`ProviderRegistry`] - Central registry that orchestrates providers
//! - [`CircuitBreaker`] - Circuit breaker for provider health tracking
//! - [`RateLimiter`] - Token-bucket rate limiter
//! - [`QuoteValidator`] - Quote validation and data quality checks
//! - [`SkipReason`] / [`FetchDiagnostics`] - Diagnostics for fetch attempts

mod circuit_breaker;
mod provider_registry;
mod rate_limiter;
mod skip_reason;
mod validator;

pub use circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitMetrics, CircuitState};
pub use provider_registry::ProviderRegistry;
pub use rate_limiter::{RateLimitConfig, RateLimiter};
pub use skip_reason::{FetchDiagnostics, ProviderAttempt, SkipReason};
pub use validator::{QuoteValidator, ValidationIssue, ValidationSeverity, ValidatorConfig};
