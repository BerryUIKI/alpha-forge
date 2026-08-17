//! Quote validation utilities.
//!
//! Provides validation logic for market data quotes, including:
//! - Negative price rejection
//! - OHLC invariant checks (high >= low, etc.)
//! - Price range validation
//! - Volume warnings

use rust_decimal::Decimal;

use crate::models::Quote;

/// Severity of a validation issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationSeverity {
    /// The quote is invalid and should be rejected.
    Error,
    /// The quote is valid but may have data quality issues.
    Warning,
}

/// A single validation issue found during quote validation.
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    /// Severity of the issue.
    pub severity: ValidationSeverity,
    /// Human-readable description of the issue.
    pub message: String,
}

/// Configuration for the quote validator.
#[derive(Debug, Clone)]
pub struct ValidatorConfig {
    /// Reject quotes with negative prices.
    pub reject_negative_prices: bool,
    /// Reject quotes with invalid OHLC relationships (e.g., high < low).
    pub reject_invalid_ohlc: bool,
    /// Maximum allowed price (quotes above this are rejected).
    pub max_price: Option<Decimal>,
    /// Warn on zero-volume quotes.
    pub warn_on_zero_volume: bool,
    /// Warn on missing OHLC fields (only close is provided).
    pub warn_on_missing_ohlc: bool,
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        Self {
            reject_negative_prices: true,
            reject_invalid_ohlc: true,
            max_price: Some(Decimal::new(1_000_000, 0)), // 1,000,000
            warn_on_zero_volume: true,
            warn_on_missing_ohlc: true,
        }
    }
}

/// Validates market data quotes for data quality issues.
pub struct QuoteValidator {
    config: ValidatorConfig,
}

impl QuoteValidator {
    /// Create a new validator with default configuration.
    pub fn new() -> Self {
        Self {
            config: ValidatorConfig::default(),
        }
    }

    /// Create a new validator with custom configuration.
    pub fn with_config(config: ValidatorConfig) -> Self {
        Self { config }
    }

    /// Validate a single quote.
    ///
    /// Returns `Ok(())` if the quote passes validation.
    /// Returns `Err(Vec<ValidationIssue>)` with all issues found.
    pub fn validate(&self, quote: &Quote) -> Result<(), Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        // Validate close price
        self.validate_close_price(quote.close, &mut issues);

        // Validate OHLC invariants
        if self.config.reject_invalid_ohlc {
            self.validate_ohlc_invariants(quote, &mut issues);
        }

        // Validate price range
        self.validate_price_range(quote.close, &mut issues);

        // Validate volume
        if self.config.warn_on_zero_volume {
            self.validate_volume(quote, &mut issues);
        }

        // Warn on missing OHLC
        if self.config.warn_on_missing_ohlc {
            self.validate_missing_ohlc(quote, &mut issues);
        }

        if issues
            .iter()
            .any(|i| i.severity == ValidationSeverity::Error)
        {
            Err(issues)
        } else {
            Ok(())
        }
    }

    /// Validate a batch of quotes.
    ///
    /// Returns quotes that pass validation, and issues for those that fail.
    pub fn validate_batch(
        &self,
        quotes: &[Quote],
    ) -> (Vec<Quote>, Vec<(usize, Vec<ValidationIssue>)>) {
        let mut valid = Vec::new();
        let mut invalid = Vec::new();

        for (i, quote) in quotes.iter().enumerate() {
            match self.validate(quote) {
                Ok(()) => valid.push(quote.clone()),
                Err(issues) => invalid.push((i, issues)),
            }
        }

        (valid, invalid)
    }

    /// Validate close price is not negative.
    fn validate_close_price(&self, close: Decimal, issues: &mut Vec<ValidationIssue>) {
        if self.config.reject_negative_prices && close.is_zero() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Warning,
                message: "Close price is zero".to_string(),
            });
        }
        if self.config.reject_negative_prices && close.is_sign_negative() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                message: format!("Close price is negative: {}", close),
            });
        }
    }

    /// Validate OHLC invariants: high >= low, high >= open, high >= close, etc.
    fn validate_ohlc_invariants(&self, quote: &Quote, issues: &mut Vec<ValidationIssue>) {
        if let (Some(high), Some(low)) = (quote.high, quote.low) {
            if high < low {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Error,
                    message: format!("High ({}) < Low ({})", high, low),
                });
            }
        }
        if let Some(open) = quote.open {
            if let Some(high) = quote.high {
                if open > high {
                    issues.push(ValidationIssue {
                        severity: ValidationSeverity::Error,
                        message: format!("Open ({}) > High ({})", open, high),
                    });
                }
            }
            if let Some(low) = quote.low {
                if open < low {
                    issues.push(ValidationIssue {
                        severity: ValidationSeverity::Error,
                        message: format!("Open ({}) < Low ({})", open, low),
                    });
                }
            }
        }
        if let Some(high) = quote.high {
            if quote.close > high {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Error,
                    message: format!("Close ({}) > High ({})", quote.close, high),
                });
            }
        }
        if let Some(low) = quote.low {
            if quote.close < low {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Error,
                    message: format!("Close ({}) < Low ({})", quote.close, low),
                });
            }
        }
    }

    /// Validate that the price is within the configured range.
    fn validate_price_range(&self, close: Decimal, issues: &mut Vec<ValidationIssue>) {
        if let Some(max_price) = self.config.max_price {
            if close > max_price {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Error,
                    message: format!("Close price ({}) exceeds max ({})", close, max_price),
                });
            }
        }
    }

    /// Validate volume is not zero.
    fn validate_volume(&self, quote: &Quote, issues: &mut Vec<ValidationIssue>) {
        if let Some(volume) = quote.volume {
            if volume.is_zero() {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Warning,
                    message: "Volume is zero".to_string(),
                });
            }
        }
    }

    /// Warn if OHLC fields are missing (only close is provided).
    fn validate_missing_ohlc(&self, quote: &Quote, issues: &mut Vec<ValidationIssue>) {
        if quote.open.is_none() && quote.high.is_none() && quote.low.is_none() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Warning,
                message: "Quote has no OHLC data (close only)".to_string(),
            });
        }
    }
}

impl Default for QuoteValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rust_decimal_macros::dec;

    fn make_quote(close: Decimal) -> Quote {
        Quote {
            timestamp: Utc::now(),
            close,
            open: Some(close),
            high: Some(close),
            low: Some(close),
            volume: Some(dec!(1000)),
            currency: "USD".to_string(),
            source: "TEST".to_string(),
        }
    }

    #[test]
    fn test_valid_quote_passes() {
        let validator = QuoteValidator::new();
        let quote = make_quote(dec!(150.50));
        assert!(validator.validate(&quote).is_ok());
    }

    #[test]
    fn test_negative_price_rejected() {
        let validator = QuoteValidator::new();
        let quote = make_quote(dec!(-10.00));
        let result = validator.validate(&quote);
        assert!(result.is_err());
        let issues = result.unwrap_err();
        assert!(issues
            .iter()
            .any(|i| i.severity == ValidationSeverity::Error));
        assert!(issues.iter().any(|i| i.message.contains("negative")));
    }

    #[test]
    fn test_zero_price_is_warning_not_error() {
        let validator = QuoteValidator::new();
        let quote = make_quote(dec!(0));
        let result = validator.validate(&quote);
        // Zero price is a warning, not an error — but combined with volume
        // checks it may still pass since warnings don't fail validation.
        assert!(result.is_ok());
    }

    #[test]
    fn test_high_below_low_rejected() {
        let validator = QuoteValidator::new();
        let mut quote = make_quote(dec!(100));
        quote.high = Some(dec!(90));
        quote.low = Some(dec!(110));
        let result = validator.validate(&quote);
        assert!(result.is_err());
        let issues = result.unwrap_err();
        assert!(issues.iter().any(|i| i.message.contains("High")));
    }

    #[test]
    fn test_open_above_high_rejected() {
        let validator = QuoteValidator::new();
        let mut quote = make_quote(dec!(100));
        quote.open = Some(dec!(120));
        quote.high = Some(dec!(110));
        quote.low = Some(dec!(90));
        let result = validator.validate(&quote);
        assert!(result.is_err());
        let issues = result.unwrap_err();
        assert!(issues.iter().any(|i| i.message.contains("Open")));
    }

    #[test]
    fn test_close_above_high_rejected() {
        let validator = QuoteValidator::new();
        let mut quote = make_quote(dec!(120));
        quote.high = Some(dec!(110));
        quote.low = Some(dec!(90));
        let result = validator.validate(&quote);
        assert!(result.is_err());
        let issues = result.unwrap_err();
        assert!(issues.iter().any(|i| i.message.contains("Close")));
    }

    #[test]
    fn test_price_exceeds_max_rejected() {
        let config = ValidatorConfig {
            max_price: Some(dec!(1000)),
            ..ValidatorConfig::default()
        };
        let validator = QuoteValidator::with_config(config);
        let quote = make_quote(dec!(5000));
        let result = validator.validate(&quote);
        assert!(result.is_err());
        let issues = result.unwrap_err();
        assert!(issues.iter().any(|i| i.message.contains("exceeds max")));
    }

    #[test]
    fn test_zero_volume_warns() {
        let validator = QuoteValidator::new();
        let mut quote = make_quote(dec!(100));
        quote.volume = Some(dec!(0));
        let result = validator.validate(&quote);
        assert!(result.is_ok());
    }

    #[test]
    fn test_close_only_quote_warns() {
        let validator = QuoteValidator::new();
        let mut quote = make_quote(dec!(100));
        quote.open = None;
        quote.high = None;
        quote.low = None;
        let result = validator.validate(&quote);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_batch_separates_valid_and_invalid() {
        let validator = QuoteValidator::new();
        let valid = make_quote(dec!(100));
        let invalid = make_quote(dec!(-5));
        let (valid_quotes, invalid_indices) = validator.validate_batch(&[valid, invalid]);
        assert_eq!(valid_quotes.len(), 1);
        assert_eq!(invalid_indices.len(), 1);
        assert_eq!(invalid_indices[0].0, 1);
    }

    #[test]
    fn test_validation_disabled_passes() {
        let config = ValidatorConfig {
            reject_negative_prices: false,
            reject_invalid_ohlc: false,
            max_price: None,
            warn_on_zero_volume: false,
            warn_on_missing_ohlc: false,
        };
        let validator = QuoteValidator::with_config(config);
        let quote = make_quote(dec!(-10));
        assert!(validator.validate(&quote).is_ok());
    }

    #[test]
    fn test_warnings_do_not_fail_validation() {
        let validator = QuoteValidator::new();
        // A quote with zero volume produces only warnings, so validation passes.
        let mut quote = make_quote(dec!(100));
        quote.volume = Some(dec!(0));
        assert!(validator.validate(&quote).is_ok());
    }
}
