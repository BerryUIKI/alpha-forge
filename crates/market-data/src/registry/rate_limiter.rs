//! Token-bucket rate limiter for provider request throttling.
//!
//! Each provider gets its own token bucket. Tokens refill at a configurable
//! rate. The `acquire` method waits until a token is available.
//! The `try_acquire` method returns immediately if no token is available.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Rate limit configuration for a single provider.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Tokens per minute (refill rate).
    pub requests_per_minute: u32,
    /// Maximum burst capacity (max tokens in the bucket).
    pub burst_capacity: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            burst_capacity: 10,
        }
    }
}

/// A single token bucket.
#[derive(Debug)]
struct TokenBucket {
    tokens: f64,
    last_update: Instant,
    rate: f64, // tokens per second
    capacity: f64,
}

impl TokenBucket {
    fn new(rate: f64, capacity: f64) -> Self {
        Self {
            tokens: capacity,
            last_update: Instant::now(),
            rate,
            capacity,
        }
    }

    /// Refill tokens based on elapsed time since last update.
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.rate).min(self.capacity);
        self.last_update = now;
    }

    /// Try to consume one token. Returns `true` if a token was available.
    fn try_consume(&mut self) -> bool {
        self.refill();
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    /// Get the wait time until a token is available.
    fn wait_duration(&self) -> Duration {
        if self.tokens >= 1.0 {
            Duration::ZERO
        } else {
            let needed = 1.0 - self.tokens;
            Duration::from_secs_f64(needed / self.rate)
        }
    }
}

/// Token-bucket rate limiter supporting per-provider buckets.
pub struct RateLimiter {
    buckets: Mutex<HashMap<String, TokenBucket>>,
    configs: HashMap<String, RateLimitConfig>,
}

impl RateLimiter {
    /// Create a new rate limiter with no configured providers.
    pub fn new() -> Self {
        Self {
            buckets: Mutex::new(HashMap::new()),
            configs: HashMap::new(),
        }
    }

    /// Configure the rate limit for a provider.
    pub fn configure(&mut self, provider: impl Into<String>, config: RateLimitConfig) {
        let provider = provider.into();
        let rate = config.requests_per_minute as f64 / 60.0;
        let capacity = config.burst_capacity as f64;

        let mut buckets = self.buckets.lock().expect("rate limiter lock poisoned");
        buckets.insert(provider.clone(), TokenBucket::new(rate, capacity));
        self.configs.insert(provider, config);
    }

    /// Acquire a token, waiting if necessary.
    ///
    /// Returns the duration waited.
    pub async fn acquire(&self, provider: &str) -> Duration {
        loop {
            let wait = {
                let mut buckets = self.buckets.lock().expect("rate limiter lock poisoned");
                if let Some(bucket) = buckets.get_mut(provider) {
                    if bucket.try_consume() {
                        // Successfully acquired
                        return Duration::ZERO;
                    }
                    bucket.wait_duration()
                } else {
                    // Provider not configured — allow through
                    return Duration::ZERO;
                }
            };

            if wait > Duration::ZERO {
                tokio::time::sleep(wait).await;
            }
        }
    }

    /// Try to acquire a token without waiting.
    ///
    /// Returns `true` if a token was acquired.
    pub fn try_acquire(&self, provider: &str) -> bool {
        let mut buckets = self.buckets.lock().expect("rate limiter lock poisoned");
        if let Some(bucket) = buckets.get_mut(provider) {
            bucket.try_consume()
        } else {
            // Provider not configured — allow through
            true
        }
    }

    /// Get the number of remaining tokens for a provider.
    pub fn remaining_tokens(&self, provider: &str) -> u32 {
        let mut buckets = self.buckets.lock().expect("rate limiter lock poisoned");
        if let Some(bucket) = buckets.get_mut(provider) {
            bucket.refill();
            bucket.tokens as u32
        } else {
            0
        }
    }

    /// Reset the bucket for a provider (refills to capacity).
    pub fn reset(&self, provider: &str) {
        let mut buckets = self.buckets.lock().expect("rate limiter lock poisoned");
        buckets.remove(provider);
        // Re-create if configured
        if let Some(config) = self.configs.get(provider) {
            let rate = config.requests_per_minute as f64 / 60.0;
            let capacity = config.burst_capacity as f64;
            buckets.insert(provider.to_string(), TokenBucket::new(rate, capacity));
        }
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_acquire_allows_within_burst() {
        let mut rl = RateLimiter::new();
        rl.configure(
            "YAHOO",
            RateLimitConfig {
                requests_per_minute: 60,
                burst_capacity: 10,
            },
        );

        // Should be able to acquire up to burst_capacity tokens
        for _ in 0..10 {
            assert!(rl.try_acquire("YAHOO"));
        }
    }

    #[test]
    fn test_try_acquire_blocks_after_burst() {
        let mut rl = RateLimiter::new();
        rl.configure(
            "YAHOO",
            RateLimitConfig {
                requests_per_minute: 60,
                burst_capacity: 3,
            },
        );

        // Use all burst tokens
        for _ in 0..3 {
            assert!(rl.try_acquire("YAHOO"));
        }

        // Should be blocked
        assert!(!rl.try_acquire("YAHOO"));
    }

    #[test]
    fn test_unconfigured_provider_allowed() {
        let rl = RateLimiter::new();
        assert!(rl.try_acquire("UNKNOWN"));
    }

    #[test]
    fn test_remaining_tokens() {
        let mut rl = RateLimiter::new();
        rl.configure(
            "YAHOO",
            RateLimitConfig {
                requests_per_minute: 60,
                burst_capacity: 5,
            },
        );

        assert_eq!(rl.remaining_tokens("YAHOO"), 5);
        rl.try_acquire("YAHOO");
        assert_eq!(rl.remaining_tokens("YAHOO"), 4);
    }

    #[test]
    fn test_reset_restores_tokens() {
        let mut rl = RateLimiter::new();
        rl.configure(
            "YAHOO",
            RateLimitConfig {
                requests_per_minute: 60,
                burst_capacity: 3,
            },
        );

        // Deplete
        for _ in 0..3 {
            rl.try_acquire("YAHOO");
        }
        assert!(!rl.try_acquire("YAHOO"));

        // Reset
        rl.reset("YAHOO");
        assert!(rl.try_acquire("YAHOO"));
    }

    #[test]
    fn test_acquire_waits() {
        // Use a very slow rate to test the wait calculation
        let mut rl = RateLimiter::new();
        rl.configure(
            "SLOW",
            RateLimitConfig {
                requests_per_minute: 1, // 1 token per minute
                burst_capacity: 1,
            },
        );

        // Consume the only token
        assert!(rl.try_acquire("SLOW"));
        assert!(!rl.try_acquire("SLOW"));

        // The wait should be > 0
        let buckets = rl.buckets.lock().unwrap();
        let bucket = buckets.get("SLOW").unwrap();
        assert!(bucket.wait_duration() > Duration::ZERO);
    }

    #[test]
    fn test_configure_updates_existing() {
        let mut rl = RateLimiter::new();
        rl.configure(
            "YAHOO",
            RateLimitConfig {
                requests_per_minute: 60,
                burst_capacity: 5,
            },
        );

        assert_eq!(rl.remaining_tokens("YAHOO"), 5);

        // Reconfigure with larger capacity
        rl.configure(
            "YAHOO",
            RateLimitConfig {
                requests_per_minute: 60,
                burst_capacity: 10,
            },
        );

        assert_eq!(rl.remaining_tokens("YAHOO"), 10);
    }

    #[test]
    fn test_remaining_tokens_unconfigured() {
        let rl = RateLimiter::new();
        assert_eq!(rl.remaining_tokens("UNKNOWN"), 0);
    }
}
