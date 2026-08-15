//! Exchange metadata lookup functions.
//!
//! Provides functions for looking up exchange names, currencies, timezones,
//! and market close times by MIC code.

use crate::resolver::exchange_registry::{get_exchange_list, ExchangeInfo};

/// Look up the human-readable name for an exchange MIC.
pub fn mic_to_exchange_name(mic: &str) -> Option<&'static str> {
    get_exchange_list()
        .iter()
        .find(|e| e.mic == mic)
        .map(|e| e.name)
}

/// Look up the default currency for an exchange MIC.
pub fn mic_to_currency(mic: &str) -> Option<&'static str> {
    get_exchange_list()
        .iter()
        .find(|e| e.mic == mic)
        .map(|e| e.currency)
}

/// Get all exchanges that trade in a given currency.
pub fn exchanges_for_currency(currency: &str) -> Vec<&'static ExchangeInfo> {
    get_exchange_list()
        .iter()
        .filter(|e| e.currency == currency)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mic_to_exchange_name() {
        let name = mic_to_exchange_name("XNYS");
        assert!(name.is_some());
        assert!(name.unwrap().contains("NYSE"));
    }

    #[test]
    fn test_mic_to_currency() {
        let currency = mic_to_currency("XNYS");
        assert!(currency.is_some());
        assert_eq!(currency.unwrap(), "USD");
    }

    #[test]
    fn test_mic_to_currency_unknown() {
        assert!(mic_to_currency("UNKNOWN").is_none());
    }

    #[test]
    fn test_exchanges_for_currency() {
        let exchanges = exchanges_for_currency("USD");
        assert!(!exchanges.is_empty());
        assert!(exchanges.iter().any(|e| e.mic == "XNYS"));
    }

    #[test]
    fn test_exchanges_for_unknown_currency() {
        let exchanges = exchanges_for_currency("XXX");
        assert!(exchanges.is_empty());
    }
}
