//! Provider market coverage restrictions.

use super::InstrumentId;

/// Provider market coverage restrictions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Coverage {
    /// Global coverage, best-effort mode (accepts mic=None).
    GlobalBestEffort,
    /// US exchanges only, strict mode (rejects mic=None).
    UsOnlyStrict,
    /// US exchanges only, best-effort mode (accepts mic=None).
    UsOnlyBestEffort,
    /// Custom coverage with allow/deny lists.
    Custom {
        /// Allowed equity MICs (None = all).
        equity_mic_allow: Option<&'static [&'static str]>,
        /// Denied equity MICs (None = none).
        equity_mic_deny: Option<&'static [&'static str]>,
        /// Whether to accept equities with mic=None (unknown venue).
        allow_unknown_mic: bool,
        /// Allowed metal quote currencies (None = all).
        metal_quote_ccy_allow: Option<&'static [&'static str]>,
    },
}

/// Linear contains check for static slices.
#[inline]
fn slice_contains(list: &[&'static str], value: &str) -> bool {
    list.contains(&value)
}

impl Coverage {
    /// Check if this coverage supports the given instrument.
    pub fn supports(&self, inst: &InstrumentId) -> bool {
        match self {
            Self::GlobalBestEffort => true,
            Self::UsOnlyStrict => is_us_equity(inst, false),
            Self::UsOnlyBestEffort => is_us_equity(inst, true),
            Self::Custom {
                equity_mic_allow,
                equity_mic_deny,
                allow_unknown_mic,
                metal_quote_ccy_allow,
            } => match inst {
                InstrumentId::Equity { mic, .. } => {
                    // Check deny list first
                    if let Some(deny) = equity_mic_deny {
                        if mic.as_deref().is_some_and(|m| slice_contains(deny, m)) {
                            return false;
                        }
                    }

                    // Handle allowlist + mic=None case
                    match (equity_mic_allow, mic.as_deref()) {
                        (Some(allow), Some(m)) => slice_contains(allow, m),
                        (Some(_), None) => *allow_unknown_mic,
                        (None, Some(_)) => true,
                        (None, None) => *allow_unknown_mic,
                    }
                }
                // FX: No currency filtering
                InstrumentId::Fx { .. } => true,
                // Crypto: No currency filtering
                InstrumentId::Crypto { .. } => true,
                // Metal: Apply quote currency filter
                InstrumentId::Metal { quote, .. } => {
                    metal_quote_ccy_allow.is_none_or(|a| slice_contains(a, quote.as_ref()))
                }
                // Options: No geographic filtering
                InstrumentId::Option { .. } => true,
                // Bonds: No geographic filtering
                InstrumentId::Bond { .. } => true,
            },
        }
    }
}

/// Check if an equity instrument is from a US exchange.
fn is_us_equity(inst: &InstrumentId, allow_unknown_mic: bool) -> bool {
    if let InstrumentId::Equity { mic, .. } = inst {
        const US_MICS: &[&str] = &["XNYS", "XNAS", "XASE", "BATS", "ARCX"];
        match mic.as_deref() {
            Some(m) => US_MICS.contains(&m),
            None => allow_unknown_mic,
        }
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::sync::Arc;

    use super::*;

    #[test]
    fn test_us_strict_allows_nasdaq() {
        let coverage = Coverage::UsOnlyStrict;
        let inst = InstrumentId::Equity {
            ticker: Arc::from("AAPL"),
            mic: Some(Cow::Borrowed("XNAS")),
        };
        assert!(coverage.supports(&inst));
    }

    #[test]
    fn test_us_strict_rejects_toronto() {
        let coverage = Coverage::UsOnlyStrict;
        let inst = InstrumentId::Equity {
            ticker: Arc::from("SHOP"),
            mic: Some(Cow::Borrowed("XTSE")),
        };
        assert!(!coverage.supports(&inst));
    }

    #[test]
    fn test_us_strict_rejects_unknown_mic() {
        let coverage = Coverage::UsOnlyStrict;
        let inst = InstrumentId::Equity {
            ticker: Arc::from("AAPL"),
            mic: None,
        };
        assert!(!coverage.supports(&inst));
    }

    #[test]
    fn test_us_best_effort_allows_unknown_mic() {
        let coverage = Coverage::UsOnlyBestEffort;
        let inst = InstrumentId::Equity {
            ticker: Arc::from("AAPL"),
            mic: None,
        };
        assert!(coverage.supports(&inst));
    }

    #[test]
    fn test_global_best_effort_allows_any() {
        let coverage = Coverage::GlobalBestEffort;
        let inst = InstrumentId::Equity {
            ticker: Arc::from("AAPL"),
            mic: Some(Cow::Borrowed("XTSE")),
        };
        assert!(coverage.supports(&inst));
    }

    #[test]
    fn test_custom_deny_list() {
        let coverage = Coverage::Custom {
            equity_mic_allow: None,
            equity_mic_deny: Some(&["XTSE"]),
            allow_unknown_mic: true,
            metal_quote_ccy_allow: None,
        };
        let inst = InstrumentId::Equity {
            ticker: Arc::from("SHOP"),
            mic: Some(Cow::Borrowed("XTSE")),
        };
        assert!(!coverage.supports(&inst));
    }

    #[test]
    fn test_fx_ignores_equity_coverage() {
        let coverage = Coverage::UsOnlyStrict;
        let inst = InstrumentId::Fx {
            base: Cow::Borrowed("EUR"),
            quote: Cow::Borrowed("GBP"),
        };
        assert!(coverage.supports(&inst));
    }
}
