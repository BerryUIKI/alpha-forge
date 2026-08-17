//! Exchange registry providing exchange metadata.
//!
//! This module provides a static registry of exchange information
//! including MIC codes, exchange names, currencies, and Yahoo Finance
//! suffix mappings.

/// Public exchange information.
#[derive(Debug, Clone)]
pub struct ExchangeInfo {
    /// MIC code (ISO 10383).
    pub mic: &'static str,
    /// Human-readable exchange name.
    pub name: &'static str,
    /// Default currency (ISO 4217).
    pub currency: &'static str,
    /// Yahoo Finance exchange code.
    pub yahoo_code: Option<&'static str>,
    /// Yahoo Finance symbol suffix.
    pub yahoo_suffix: Option<&'static str>,
}

/// Get the list of all known exchanges.
pub fn get_exchange_list() -> &'static [ExchangeInfo] {
    EXCHANGES
}

/// Static exchange registry.
static EXCHANGES: &[ExchangeInfo] = &[
    // US
    ExchangeInfo {
        mic: "XNYS",
        name: "NYSE",
        currency: "USD",
        yahoo_code: Some("NYQ"),
        yahoo_suffix: None,
    },
    ExchangeInfo {
        mic: "XNAS",
        name: "NASDAQ",
        currency: "USD",
        yahoo_code: Some("NAS"),
        yahoo_suffix: None,
    },
    ExchangeInfo {
        mic: "XASE",
        name: "NYSE American",
        currency: "USD",
        yahoo_code: Some("ASE"),
        yahoo_suffix: None,
    },
    ExchangeInfo {
        mic: "BATS",
        name: "BATS Exchange",
        currency: "USD",
        yahoo_code: Some("BTS"),
        yahoo_suffix: None,
    },
    ExchangeInfo {
        mic: "ARCX",
        name: "NYSE Arca",
        currency: "USD",
        yahoo_code: Some("ARC"),
        yahoo_suffix: None,
    },
    // Canada
    ExchangeInfo {
        mic: "XTSE",
        name: "Toronto Stock Exchange",
        currency: "CAD",
        yahoo_code: Some("TOR"),
        yahoo_suffix: Some(".TO"),
    },
    ExchangeInfo {
        mic: "XCNQ",
        name: "Canadian Securities Exchange",
        currency: "CAD",
        yahoo_code: Some("CNQ"),
        yahoo_suffix: Some(".CN"),
    },
    ExchangeInfo {
        mic: "XVAN",
        name: "TSX Venture Exchange",
        currency: "CAD",
        yahoo_code: Some("VAN"),
        yahoo_suffix: Some(".V"),
    },
    // UK & Europe
    ExchangeInfo {
        mic: "XLON",
        name: "London Stock Exchange",
        currency: "GBP",
        yahoo_code: Some("LSE"),
        yahoo_suffix: Some(".L"),
    },
    ExchangeInfo {
        mic: "XETR",
        name: "Xetra",
        currency: "EUR",
        yahoo_code: Some("GER"),
        yahoo_suffix: Some(".DE"),
    },
    ExchangeInfo {
        mic: "XFRA",
        name: "Frankfurt Stock Exchange",
        currency: "EUR",
        yahoo_code: Some("FRA"),
        yahoo_suffix: Some(".F"),
    },
    ExchangeInfo {
        mic: "XPAR",
        name: "Euronext Paris",
        currency: "EUR",
        yahoo_code: Some("PAR"),
        yahoo_suffix: Some(".PA"),
    },
    ExchangeInfo {
        mic: "XBRU",
        name: "Euronext Brussels",
        currency: "EUR",
        yahoo_code: Some("BRU"),
        yahoo_suffix: Some(".BR"),
    },
    ExchangeInfo {
        mic: "XAMS",
        name: "Euronext Amsterdam",
        currency: "EUR",
        yahoo_code: Some("AMS"),
        yahoo_suffix: Some(".AS"),
    },
    ExchangeInfo {
        mic: "XMIL",
        name: "Borsa Italiana",
        currency: "EUR",
        yahoo_code: Some("MIL"),
        yahoo_suffix: Some(".MI"),
    },
    ExchangeInfo {
        mic: "XMAD",
        name: "Bolsa de Madrid",
        currency: "EUR",
        yahoo_code: Some("MCE"),
        yahoo_suffix: Some(".MC"),
    },
    ExchangeInfo {
        mic: "XSTO",
        name: "Nasdaq Stockholm",
        currency: "SEK",
        yahoo_code: Some("STO"),
        yahoo_suffix: Some(".ST"),
    },
    ExchangeInfo {
        mic: "XHEL",
        name: "Nasdaq Helsinki",
        currency: "EUR",
        yahoo_code: Some("HEL"),
        yahoo_suffix: Some(".HE"),
    },
    ExchangeInfo {
        mic: "XCPH",
        name: "Nasdaq Copenhagen",
        currency: "DKK",
        yahoo_code: Some("CPH"),
        yahoo_suffix: Some(".CO"),
    },
    ExchangeInfo {
        mic: "XOSL",
        name: "Oslo Bors",
        currency: "NOK",
        yahoo_code: Some("OSL"),
        yahoo_suffix: Some(".OL"),
    },
    ExchangeInfo {
        mic: "XLIS",
        name: "Euronext Lisbon",
        currency: "EUR",
        yahoo_code: Some("LIS"),
        yahoo_suffix: Some(".LS"),
    },
    ExchangeInfo {
        mic: "XVIE",
        name: "Vienna Stock Exchange",
        currency: "EUR",
        yahoo_code: Some("VIE"),
        yahoo_suffix: Some(".VI"),
    },
    ExchangeInfo {
        mic: "XICE",
        name: "Iceland Stock Exchange",
        currency: "ISK",
        yahoo_code: None,
        yahoo_suffix: Some(".IC"),
    },
    ExchangeInfo {
        mic: "XPRA",
        name: "Prague Stock Exchange",
        currency: "CZK",
        yahoo_code: None,
        yahoo_suffix: Some(".PR"),
    },
    ExchangeInfo {
        mic: "XWAR",
        name: "Warsaw Stock Exchange",
        currency: "PLN",
        yahoo_code: None,
        yahoo_suffix: Some(".WA"),
    },
    ExchangeInfo {
        mic: "XATH",
        name: "Athens Stock Exchange",
        currency: "EUR",
        yahoo_code: None,
        yahoo_suffix: Some(".AT"),
    },
    ExchangeInfo {
        mic: "XTAE",
        name: "Tel Aviv Stock Exchange",
        currency: "ILS",
        yahoo_code: None,
        yahoo_suffix: Some(".TA"),
    },
    // Cboe Europe
    ExchangeInfo {
        mic: "CXE",
        name: "Cboe UK Equities",
        currency: "GBP",
        yahoo_code: Some("CXE"),
        yahoo_suffix: Some(".XC"),
    },
    ExchangeInfo {
        mic: "DXE",
        name: "Cboe Europe Equities (EUR)",
        currency: "EUR",
        yahoo_code: Some("DXE"),
        yahoo_suffix: Some(".XD"),
    },
    // Asia-Pacific
    ExchangeInfo {
        mic: "XTKS",
        name: "Tokyo Stock Exchange",
        currency: "JPY",
        yahoo_code: Some("TKS"),
        yahoo_suffix: Some(".T"),
    },
    ExchangeInfo {
        mic: "XHKG",
        name: "Hong Kong Stock Exchange",
        currency: "HKD",
        yahoo_code: Some("HKG"),
        yahoo_suffix: Some(".HK"),
    },
    ExchangeInfo {
        mic: "XSHG",
        name: "Shanghai Stock Exchange",
        currency: "CNY",
        yahoo_code: Some("SHH"),
        yahoo_suffix: Some(".SS"),
    },
    ExchangeInfo {
        mic: "XSHE",
        name: "Shenzhen Stock Exchange",
        currency: "CNY",
        yahoo_code: Some("SHZ"),
        yahoo_suffix: Some(".SZ"),
    },
    ExchangeInfo {
        mic: "XASX",
        name: "ASX",
        currency: "AUD",
        yahoo_code: Some("ASX"),
        yahoo_suffix: Some(".AX"),
    },
    ExchangeInfo {
        mic: "XNZE",
        name: "New Zealand Exchange",
        currency: "NZD",
        yahoo_code: Some("NZE"),
        yahoo_suffix: Some(".NZ"),
    },
    ExchangeInfo {
        mic: "XKRX",
        name: "Korea Exchange",
        currency: "KRW",
        yahoo_code: Some("KSC"),
        yahoo_suffix: Some(".KS"),
    },
    ExchangeInfo {
        mic: "XKOS",
        name: "KOSDAQ",
        currency: "KRW",
        yahoo_code: Some("KOS"),
        yahoo_suffix: Some(".KQ"),
    },
    ExchangeInfo {
        mic: "XSES",
        name: "Singapore Exchange",
        currency: "SGD",
        yahoo_code: Some("SES"),
        yahoo_suffix: Some(".SI"),
    },
    ExchangeInfo {
        mic: "XBOM",
        name: "BSE India",
        currency: "INR",
        yahoo_code: Some("BOM"),
        yahoo_suffix: Some(".BO"),
    },
    ExchangeInfo {
        mic: "XNSE",
        name: "NSE India",
        currency: "INR",
        yahoo_code: Some("NSI"),
        yahoo_suffix: Some(".NS"),
    },
    ExchangeInfo {
        mic: "XTAI",
        name: "Taiwan Stock Exchange",
        currency: "TWD",
        yahoo_code: Some("TAI"),
        yahoo_suffix: Some(".TW"),
    },
    ExchangeInfo {
        mic: "XSET",
        name: "Stock Exchange of Thailand",
        currency: "THB",
        yahoo_code: Some("SET"),
        yahoo_suffix: Some(".BK"),
    },
    ExchangeInfo {
        mic: "XIDX",
        name: "Indonesia Stock Exchange",
        currency: "IDR",
        yahoo_code: Some("IDX"),
        yahoo_suffix: Some(".JK"),
    },
    ExchangeInfo {
        mic: "XPHS",
        name: "Philippine Stock Exchange",
        currency: "PHP",
        yahoo_code: Some("PHS"),
        yahoo_suffix: Some(".PS"),
    },
    // Americas (non-US)
    ExchangeInfo {
        mic: "XMEX",
        name: "Mexican Stock Exchange",
        currency: "MXN",
        yahoo_code: Some("MEX"),
        yahoo_suffix: Some(".MX"),
    },
    ExchangeInfo {
        mic: "XBSP",
        name: "B3 (Brazil)",
        currency: "BRL",
        yahoo_code: Some("SAO"),
        yahoo_suffix: Some(".SA"),
    },
    // Middle East & Africa
    ExchangeInfo {
        mic: "XDFM",
        name: "Dubai Financial Market",
        currency: "AED",
        yahoo_code: None,
        yahoo_suffix: Some(".DU"),
    },
    ExchangeInfo {
        mic: "XSAU",
        name: "Saudi Stock Exchange",
        currency: "SAR",
        yahoo_code: None,
        yahoo_suffix: Some(".SR"),
    },
    ExchangeInfo {
        mic: "XCAI",
        name: "Egyptian Exchange",
        currency: "EGP",
        yahoo_code: None,
        yahoo_suffix: Some(".CA"),
    },
    ExchangeInfo {
        mic: "XJSE",
        name: "Johannesburg Stock Exchange",
        currency: "ZAR",
        yahoo_code: None,
        yahoo_suffix: Some(".JO"),
    },
    ExchangeInfo {
        mic: "XNAM",
        name: "Namibia Stock Exchange",
        currency: "NAD",
        yahoo_code: None,
        yahoo_suffix: Some(".NM"),
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_exchange_list() {
        let exchanges = get_exchange_list();
        assert!(!exchanges.is_empty());
    }

    #[test]
    fn test_xnys_exists() {
        let exchanges = get_exchange_list();
        let nyse = exchanges.iter().find(|e| e.mic == "XNYS");
        assert!(nyse.is_some());
        assert_eq!(nyse.unwrap().currency, "USD");
    }

    #[test]
    fn test_xtse_suffix() {
        let exchanges = get_exchange_list();
        let tse = exchanges.iter().find(|e| e.mic == "XTSE");
        assert!(tse.is_some());
        assert_eq!(tse.unwrap().yahoo_suffix, Some(".TO"));
        assert_eq!(tse.unwrap().currency, "CAD");
    }

    #[test]
    fn test_no_duplicate_mics() {
        let exchanges = get_exchange_list();
        let mut mics: Vec<&str> = exchanges.iter().map(|e| e.mic).collect();
        let original_len = mics.len();
        mics.sort();
        mics.dedup();
        assert_eq!(mics.len(), original_len, "Duplicate MICs found");
    }

    #[test]
    fn test_all_entries_have_currency() {
        let exchanges = get_exchange_list();
        for entry in exchanges {
            assert!(
                !entry.currency.is_empty(),
                "Missing currency for {}",
                entry.mic
            );
        }
    }

    #[test]
    fn test_cxe_and_dxe_present() {
        let exchanges = get_exchange_list();
        assert!(exchanges.iter().any(|e| e.mic == "CXE"));
        assert!(exchanges.iter().any(|e| e.mic == "DXE"));
    }
}
