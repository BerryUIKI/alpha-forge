//! Circuit breaker for provider health tracking.
//!
//! Implements a simple circuit breaker per provider:
//! - **Closed**: Normal operation. Requests are allowed.
//! - **Open**: After `failure_threshold` consecutive failures. Requests are rejected.
//! - **HalfOpen**: After `recovery_timeout` elapsed. One test request is allowed.
//!   If it succeeds, the circuit closes. If it fails, the circuit re-opens.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Circuit breaker state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Normal operation — requests are allowed.
    Closed,
    /// Circuit is open — requests are rejected until the recovery timeout.
    Open,
    /// Trial mode — one request is allowed to test recovery.
    HalfOpen,
}

/// Configuration for the circuit breaker.
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of consecutive failures before opening the circuit.
    pub failure_threshold: u32,
    /// Duration to wait before transitioning to HalfOpen.
    pub recovery_timeout: Duration,
    /// Number of consecutive successes in HalfOpen to close the circuit.
    pub half_open_success_threshold: u32,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(60),
            half_open_success_threshold: 2,
        }
    }
}

/// Internal circuit state for a single provider.
#[derive(Debug)]
struct Circuit {
    state: CircuitState,
    failure_count: u32,
    half_open_successes: u32,
    last_failure: Option<Instant>,
}

impl Circuit {
    fn new() -> Self {
        Self {
            state: CircuitState::Closed,
            failure_count: 0,
            half_open_successes: 0,
            last_failure: None,
        }
    }
}

/// Metrics for a single circuit.
#[derive(Debug, Clone)]
pub struct CircuitMetrics {
    pub state: CircuitState,
    pub failure_count: u32,
}

/// Circuit breaker that tracks per-provider circuit state.
pub struct CircuitBreaker {
    circuits: Mutex<HashMap<String, Circuit>>,
    config: CircuitBreakerConfig,
}

impl CircuitBreaker {
    /// Create a new circuit breaker with default configuration.
    pub fn new() -> Self {
        Self {
            circuits: Mutex::new(HashMap::new()),
            config: CircuitBreakerConfig::default(),
        }
    }

    /// Create a new circuit breaker with custom configuration.
    pub fn with_config(config: CircuitBreakerConfig) -> Self {
        Self {
            circuits: Mutex::new(HashMap::new()),
            config,
        }
    }

    /// Check if a request is allowed for the given provider.
    ///
    /// Returns `true` if the circuit is closed or half-open (trial mode).
    /// Returns `false` if the circuit is open and the recovery timeout has not elapsed.
    pub fn is_allowed(&self, provider: &str) -> bool {
        let mut circuits = self.circuits.lock().expect("circuit lock poisoned");
        let circuit = circuits
            .entry(provider.to_string())
            .or_insert_with(Circuit::new);

        match circuit.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if recovery timeout has elapsed
                if let Some(last_failure) = circuit.last_failure {
                    if last_failure.elapsed() >= self.config.recovery_timeout {
                        circuit.state = CircuitState::HalfOpen;
                        circuit.half_open_successes = 0;
                        true
                    } else {
                        false
                    }
                } else {
                    true
                }
            }
            CircuitState::HalfOpen => true,
        }
    }

    /// Record a successful request for the given provider.
    pub fn record_success(&self, provider: &str) {
        let mut circuits = self.circuits.lock().expect("circuit lock poisoned");
        let circuit = circuits
            .entry(provider.to_string())
            .or_insert_with(Circuit::new);

        match circuit.state {
            CircuitState::HalfOpen => {
                circuit.half_open_successes += 1;
                if circuit.half_open_successes >= self.config.half_open_success_threshold {
                    circuit.state = CircuitState::Closed;
                    circuit.failure_count = 0;
                    circuit.half_open_successes = 0;
                }
            }
            CircuitState::Closed => {
                // Reset failure count on success
                circuit.failure_count = 0;
            }
            CircuitState::Open => {
                // Should not happen if is_allowed is checked first
            }
        }
    }

    /// Record a failed request for the given provider.
    pub fn record_failure(&self, provider: &str) {
        let mut circuits = self.circuits.lock().expect("circuit lock poisoned");
        let circuit = circuits
            .entry(provider.to_string())
            .or_insert_with(Circuit::new);

        circuit.failure_count += 1;
        circuit.last_failure = Some(Instant::now());

        match circuit.state {
            CircuitState::Closed => {
                if circuit.failure_count >= self.config.failure_threshold {
                    circuit.state = CircuitState::Open;
                }
            }
            CircuitState::HalfOpen => {
                circuit.state = CircuitState::Open;
                circuit.half_open_successes = 0;
            }
            CircuitState::Open => {
                // Already open, stay open
            }
        }
    }

    /// Get the current state of the circuit for a provider.
    pub fn state(&self, provider: &str) -> CircuitState {
        let circuits = self.circuits.lock().expect("circuit lock poisoned");
        circuits
            .get(provider)
            .map(|c| c.state)
            .unwrap_or(CircuitState::Closed)
    }

    /// Get the current failure count for a provider.
    pub fn failure_count(&self, provider: &str) -> u32 {
        let circuits = self.circuits.lock().expect("circuit lock poisoned");
        circuits.get(provider).map(|c| c.failure_count).unwrap_or(0)
    }

    /// Reset the circuit for a specific provider.
    pub fn reset(&self, provider: &str) {
        let mut circuits = self.circuits.lock().expect("circuit lock poisoned");
        circuits.remove(provider);
    }

    /// Reset all circuits.
    pub fn reset_all(&self) {
        let mut circuits = self.circuits.lock().expect("circuit lock poisoned");
        circuits.clear();
    }

    /// Get metrics for all providers.
    pub fn metrics(&self) -> Vec<(String, CircuitMetrics)> {
        let circuits = self.circuits.lock().expect("circuit lock poisoned");
        circuits
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    CircuitMetrics {
                        state: v.state,
                        failure_count: v.failure_count,
                    },
                )
            })
            .collect()
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state_is_closed() {
        let cb = CircuitBreaker::new();
        assert_eq!(cb.state("YAHOO"), CircuitState::Closed);
    }

    #[test]
    fn test_opens_after_threshold_failures() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            recovery_timeout: Duration::from_secs(60),
            half_open_success_threshold: 2,
        };
        let cb = CircuitBreaker::with_config(config);

        assert!(cb.is_allowed("YAHOO"));
        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");

        assert_eq!(cb.state("YAHOO"), CircuitState::Open);
        assert!(!cb.is_allowed("YAHOO"));
    }

    #[test]
    fn test_stays_closed_below_threshold() {
        let config = CircuitBreakerConfig {
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(60),
            half_open_success_threshold: 2,
        };
        let cb = CircuitBreaker::with_config(config);

        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");

        // Not yet at threshold of 5
        assert_eq!(cb.state("YAHOO"), CircuitState::Closed);
        assert!(cb.is_allowed("YAHOO"));
    }

    #[test]
    fn test_closes_after_half_open_successes() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            recovery_timeout: Duration::from_millis(1), // Very short timeout
            half_open_success_threshold: 2,
        };
        let cb = CircuitBreaker::with_config(config);

        // Open the circuit
        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");
        assert_eq!(cb.state("YAHOO"), CircuitState::Open);

        // Wait for recovery timeout
        std::thread::sleep(Duration::from_millis(2));

        // Should transition to HalfOpen
        assert!(cb.is_allowed("YAHOO"));
        assert_eq!(cb.state("YAHOO"), CircuitState::HalfOpen);

        // Record successes in HalfOpen
        cb.record_success("YAHOO");
        assert_eq!(cb.state("YAHOO"), CircuitState::HalfOpen);

        cb.record_success("YAHOO");
        assert_eq!(cb.state("YAHOO"), CircuitState::Closed);
    }

    #[test]
    fn test_half_open_failure_reopens() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            recovery_timeout: Duration::from_millis(1),
            half_open_success_threshold: 2,
        };
        let cb = CircuitBreaker::with_config(config);

        // Open the circuit
        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");

        // Wait for recovery timeout
        std::thread::sleep(Duration::from_millis(2));

        // Transition to HalfOpen
        assert!(cb.is_allowed("YAHOO"));

        // Record failure in HalfOpen — should re-open
        cb.record_failure("YAHOO");
        assert_eq!(cb.state("YAHOO"), CircuitState::Open);
        assert!(!cb.is_allowed("YAHOO"));
    }

    #[test]
    fn test_multiple_providers_independent() {
        let cb = CircuitBreaker::new();

        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");

        assert_eq!(cb.state("YAHOO"), CircuitState::Open);
        assert_eq!(cb.state("ALPHA_VANTAGE"), CircuitState::Closed);
    }

    #[test]
    fn test_reset_provider() {
        let cb = CircuitBreaker::new();

        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");

        assert_eq!(cb.state("YAHOO"), CircuitState::Open);

        cb.reset("YAHOO");
        assert_eq!(cb.state("YAHOO"), CircuitState::Closed);
        assert_eq!(cb.failure_count("YAHOO"), 0);
    }

    #[test]
    fn test_reset_all() {
        let cb = CircuitBreaker::new();

        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");
        cb.record_failure("ALPHA_VANTAGE");
        cb.record_failure("ALPHA_VANTAGE");
        cb.record_failure("ALPHA_VANTAGE");
        cb.record_failure("ALPHA_VANTAGE");
        cb.record_failure("ALPHA_VANTAGE");

        assert_eq!(cb.state("YAHOO"), CircuitState::Open);
        assert_eq!(cb.state("ALPHA_VANTAGE"), CircuitState::Open);

        cb.reset_all();
        assert_eq!(cb.state("YAHOO"), CircuitState::Closed);
        assert_eq!(cb.state("ALPHA_VANTAGE"), CircuitState::Closed);
    }

    #[test]
    fn test_metrics() {
        let cb = CircuitBreaker::new();

        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");
        cb.record_failure("YAHOO");

        let metrics = cb.metrics();
        assert!(!metrics.is_empty());

        let yahoo_metrics = metrics.iter().find(|(k, _)| k == "YAHOO");
        assert!(yahoo_metrics.is_some());
        assert_eq!(yahoo_metrics.unwrap().1.failure_count, 3);
    }
}
