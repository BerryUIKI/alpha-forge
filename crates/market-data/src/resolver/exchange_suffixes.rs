//! Provider-specific symbol resolution.
//!
//! This module provides mappings from canonical (ticker, MIC) pairs to
//! provider-specific symbols. Each provider (Yahoo, Alpha Vantage, etc.)
//! uses different suffixes to identify exchanges.

use std::borrow::Cow;
use std::collections::HashMap;

use crate::models::{Mic, ProviderId};

/// Provider-specific exchange suffix and currency.
#[derive(Clone, Debug)]
pub struct ExchangeSuffix {
    /// The suffix to append to the ticker (e.g., ".TO" for Yahoo TSX).
    pub suffix: Cow<'static, str>,
    /// The trading currency for this exchange (e.g., "CAD" for TSX).
    pub currency: Cow<'static, str>,
}

/// MIC to provider suffix mapping database.
///
/// Maps ISO 10383 Market Identifier Codes to provider-specific suffixes
/// for each supported provider.
pub struct ExchangeMap {
    mappings: HashMap<String, HashMap<String, ExchangeSuffix>>,
}

impl Default for ExchangeMap {
    fn default() -> Self {
        Self::new()
    }
}

impl ExchangeMap {
    /// Create a new ExchangeMap with default mappings.
    pub fn new() -> Self {
        let mut map = Self {
            mappings: HashMap::new(),
        };
        map.load_defaults();
        map
    }

    /// Load all default exchange mappings.
    fn load_defaults(&mut self) {
        // US exchanges (no suffix)
        for mic in ["XNYS", "XNAS", "XASE", "BATS", "ARCX"] {
            self.insert(
                mic,
                "YAHOO",
                ExchangeSuffix {
                    suffix: Cow::Borrowed(""),
                    currency: Cow::Borrowed("USD"),
                },
            );
        }

        // Canadian exchanges
        self.insert(
            "XTSE",
            "YAHOO",
            ExchangeSuffix {
                suffix: Cow::Borrowed(".TO"),
                currency: Cow::Borrowed("CAD"),
            },
        );
        self.insert(
            "XCNQ",
            "YAHOO",
            ExchangeSuffix {
                suffix: Cow::Borrowed(".CN"),
                currency: Cow::Borrowed("CAD"),
            },
        );
        self.insert(
            "XVAN",
            "YAHOO",
            ExchangeSuffix {
                suffix: Cow::Borrowed(".V"),
                currency: Cow::Borrowed("CAD"),
            },
        );

        // UK & Europe
        let europe: &[(&str, &str, &str)] = &[
            ("XLON", ".L", "GBP"), // Yahoo returns GBp (pence) for London
            ("XETR", ".DE", "EUR"),
            ("XFRA", ".F", "EUR"),
            ("XPAR", ".PA", "EUR"),
            ("XBRU", ".BR", "EUR"),
            ("XAMS", ".AS", "EUR"),
            ("XMIL", ".MI", "EUR"),
            ("XMAD", ".MC", "EUR"),
            ("XSTO", ".ST", "SEK"),
            ("XHEL", ".HE", "EUR"),
            ("XCPH", ".CO", "DKK"),
            ("XOSL", ".OL", "NOK"),
            ("XLIS", ".LS", "EUR"),
            ("XVIE", ".VI", "EUR"),
            ("XICE", ".IC", "ISK"),
            ("XPRA", ".PR", "CZK"),
            ("XWAR", ".WA", "PLN"),
            ("XATH", ".AT", "EUR"),
            ("XTAE", ".TA", "ILS"), // Yahoo returns ILA (agora) for Tel Aviv
        ];
        for (mic, suffix, currency) in europe {
            self.insert(
                mic,
                "YAHOO",
                ExchangeSuffix {
                    suffix: Cow::Borrowed(suffix),
                    currency: Cow::Borrowed(currency),
                },
            );
        }

        // Cboe Europe
        self.insert(
            "CXE",
            "YAHOO",
            ExchangeSuffix {
                suffix: Cow::Borrowed(".XC"),
                currency: Cow::Borrowed("GBP"),
            },
        );
        self.insert(
            "DXE",
            "YAHOO",
            ExchangeSuffix {
                suffix: Cow::Borrowed(".XD"),
                currency: Cow::Borrowed("EUR"),
            },
        );

        // Asia-Pacific
        let asia: &[(&str, &str, &str)] = &[
            ("XTKS", ".T", "JPY"),
            ("XHKG", ".HK", "HKD"),
            ("XSHG", ".SS", "CNY"),
            ("XSHE", ".SZ", "CNY"),
            ("XASX", ".AX", "AUD"),
            ("XNZE", ".NZ", "NZD"),
            ("XKRX", ".KS", "KRW"),
            ("XKOS", ".KQ", "KRW"),
            ("XSES", ".SI", "SGD"),
            ("XBOM", ".BO", "INR"),
            ("XNSE", ".NS", "INR"),
            ("XTAI", ".TW", "TWD"),
            ("XSET", ".BK", "THB"),
            ("XIDX", ".JK", "IDR"),
            ("XPHS", ".PS", "PHP"),
        ];
        for (mic, suffix, currency) in asia {
            self.insert(
                mic,
                "YAHOO",
                ExchangeSuffix {
                    suffix: Cow::Borrowed(suffix),
                    currency: Cow::Borrowed(currency),
                },
            );
        }

        // Americas (non-US)
        self.insert(
            "XMEX",
            "YAHOO",
            ExchangeSuffix {
                suffix: Cow::Borrowed(".MX"),
                currency: Cow::Borrowed("MXN"),
            },
        );
        self.insert(
            "XBSP",
            "YAHOO",
            ExchangeSuffix {
                suffix: Cow::Borrowed(".SA"),
                currency: Cow::Borrowed("BRL"),
            },
        );

        // Middle East & Africa
        let mea: &[(&str, &str, &str)] = &[
            ("XJSE", ".JO", "ZAR"),
            ("XDFM", ".DU", "AED"),
            ("XSAU", ".SR", "SAR"),
            ("XCAI", ".CA", "EGP"),
            ("XNAM", ".NM", "NAD"),
        ];
        for (mic, suffix, currency) in mea {
            self.insert(
                mic,
                "YAHOO",
                ExchangeSuffix {
                    suffix: Cow::Borrowed(suffix),
                    currency: Cow::Borrowed(currency),
                },
            );
        }
    }

    /// Insert a mapping from MIC to provider suffix.
    pub fn insert(&mut self, mic: &str, provider: &str, suffix: ExchangeSuffix) {
        self.mappings
            .entry(mic.to_string())
            .or_default()
            .insert(provider.to_string(), suffix);
    }

    /// Get the suffix for a MIC and provider.
    pub fn get_suffix(&self, mic: &Mic, provider: &ProviderId) -> Option<&str> {
        self.mappings
            .get(mic.as_ref())?
            .get(provider.as_ref())
            .map(|s| s.suffix.as_ref())
    }

    /// Get the currency for a MIC and provider.
    pub fn get_currency(&self, mic: &Mic, provider: &ProviderId) -> Option<&str> {
        self.mappings
            .get(mic.as_ref())?
            .get(provider.as_ref())
            .map(|s| s.currency.as_ref())
    }

    /// Check if a MIC is supported.
    pub fn has_mic(&self, mic: &Mic) -> bool {
        self.mappings.contains_key(mic.as_ref())
    }

    /// Check if a MIC/provider combination is supported.
    pub fn has_mapping(&self, mic: &Mic, provider: &ProviderId) -> bool {
        self.mappings
            .get(mic.as_ref())
            .is_some_and(|pm| pm.contains_key(provider.as_ref()))
    }
}

/// Map a Yahoo Finance exchange code to a MIC.
pub fn yahoo_exchange_to_mic(exchange: &str) -> Option<Mic> {
    let normalized = exchange.trim().to_uppercase();
    let mic = match normalized.as_str() {
        "NYQ" | "NYS" => "XNYS",
        "NAS" | "NMS" | "NCM" | "NGM" => "XNAS",
        "ASE" | "AMX" => "XASE",
        "BTS" | "BAT" => "BATS",
        "ARC" | "PCX" | "PCS" => "ARCX",
        "TOR" | "TSE" => "XTSE",
        "CNQ" => "XCNQ",
        "VAN" => "XVAN",
        "LSE" | "LON" => "XLON",
        "GER" | "XETRA" | "ETR" => "XETR",
        "FRA" | "FWB" => "XFRA",
        "PAR" => "XPAR",
        "BRU" => "XBRU",
        "AMS" => "XAMS",
        "MIL" => "XMIL",
        "MCE" | "MAD" => "XMAD",
        "STO" => "XSTO",
        "HEL" => "XHEL",
        "CPH" => "XCPH",
        "OSL" => "XOSL",
        "LIS" => "XLIS",
        "VIE" => "XVIE",
        "TKS" | "TKY" => "XTKS",
        "HKG" => "XHKG",
        "SHH" | "SS" => "XSHG",
        "SHZ" | "SZ" => "XSHE",
        "ASX" => "XASX",
        "NZE" => "XNZE",
        "KSC" | "KRX" => "XKRX",
        "KOS" | "KOSDAQ" => "XKOS",
        "SES" | "SES-A" | "SES-D" => "XSES",
        "BOM" => "XBOM",
        "NSI" => "XNSE",
        "TAI" | "TWO" => "XTAI",
        "SET" | "BKK" => "XSET",
        "IDX" | "JKT" => "XIDX",
        "PHS" => "XPHS",
        "MEX" => "XMEX",
        "SAO" | "BVMF" => "XBSP",
        "JSE" => "XJSE",
        "DFM" => "XDFM",
        "SAU" | "TADAWUL" => "XSAU",
        "CAI" => "XCAI",
        "NAM" => "XNAM",
        "CXE" => "CXE",
        "DXE" => "DXE",
        "XICE" => "XICE",
        _ => return None,
    };
    Some(Cow::Owned(mic.to_string()))
}

/// Known Yahoo exchange suffixes.
///
/// Returns the whitelist used by `strip_yahoo_suffix` to safely extract
/// the canonical ticker from a Yahoo symbol.
pub fn yahoo_exchange_suffixes() -> Vec<&'static str> {
    vec![
        ".TO", ".CN", ".V", ".L", ".DE", ".F", ".PA", ".BR", ".AS", ".MI", ".MC", ".ST", ".HE",
        ".CO", ".OL", ".LS", ".VI", ".IC", ".PR", ".WA", ".AT", ".TA", ".XC", ".XD", ".T", ".HK",
        ".SS", ".SZ", ".AX", ".NZ", ".KS", ".KQ", ".SI", ".BO", ".NS", ".TW", ".BK", ".JK", ".PS",
        ".MX", ".SA", ".JO", ".DU", ".SR", ".CA", ".NM",
    ]
}

/// Map a Yahoo Finance suffix to a MIC.
pub fn yahoo_suffix_to_mic(suffix: &str) -> Option<&'static str> {
    let normalized = suffix.trim().to_uppercase();
    match normalized.as_str() {
        ".TO" => Some("XTSE"),
        ".CN" => Some("XCNQ"),
        ".V" => Some("XTSX"), // Yahoo uses .V for TSX Venture
        ".L" => Some("XLON"),
        ".DE" => Some("XETR"),
        ".F" => Some("XFRA"),
        ".PA" => Some("XPAR"),
        ".BR" => Some("XBRU"),
        ".AS" => Some("XAMS"),
        ".MI" => Some("XMIL"),
        ".MC" => Some("XMAD"),
        ".ST" => Some("XSTO"),
        ".HE" => Some("XHEL"),
        ".CO" => Some("XCPH"),
        ".OL" => Some("XOSL"),
        ".LS" => Some("XLIS"),
        ".VI" => Some("XVIE"),
        ".IC" => Some("XICE"),
        ".PR" => Some("XPRA"),
        ".WA" => Some("XWAR"),
        ".AT" => Some("XATH"),
        ".TA" => Some("XTAE"),
        ".XC" => Some("CXE"),
        ".XD" => Some("DXE"),
        ".T" => Some("XTKS"),
        ".HK" => Some("XHKG"),
        ".SS" => Some("XSHG"),
        ".SZ" => Some("XSHE"),
        ".AX" => Some("XASX"),
        ".NZ" => Some("XNZE"),
        ".KS" => Some("XKRX"),
        ".KQ" => Some("XKOS"),
        ".SI" => Some("XSES"),
        ".BO" => Some("XBOM"),
        ".NS" => Some("XNSE"),
        ".TW" => Some("XTAI"),
        ".BK" => Some("XSET"),
        ".JK" => Some("XIDX"),
        ".PS" => Some("XPHS"),
        ".MX" => Some("XMEX"),
        ".SA" => Some("XBSP"),
        ".JO" => Some("XJSE"),
        ".DU" => Some("XDFM"),
        ".SR" => Some("XSAU"),
        ".CA" => Some("XCAI"),
        ".NM" => Some("XNAM"),
        _ => None,
    }
}

fn strip_ascii_suffix_ignore_case<'a>(value: &'a str, suffix: &str) -> Option<&'a str> {
    let start = value.len().checked_sub(suffix.len())?;
    let candidate = value.get(start..)?;
    if candidate.eq_ignore_ascii_case(suffix) {
        value.get(..start)
    } else {
        None
    }
}

fn split_known_yahoo_suffix(symbol: &str) -> (&str, Option<&'static str>, Option<&'static str>) {
    let trimmed = symbol.trim();
    for suffix in yahoo_exchange_suffixes() {
        if let Some(base) = strip_ascii_suffix_ignore_case(trimmed, suffix) {
            let suffix_code = suffix.strip_prefix('.').unwrap_or(suffix);
            return (base, yahoo_suffix_to_mic(suffix_code), Some(suffix));
        }
    }
    (trimmed, None, None)
}

/// Format an equity ticker base for Yahoo.
///
/// Yahoo uses dotted suffixes for exchanges (`SHOP.TO`, `VOD.L`) but hyphens
/// for share-class separators in the base ticker (`BRK-B`). Callers must strip
/// known exchange suffixes before formatting the base.
pub fn yahoo_equity_base_to_provider(base: &str) -> String {
    base.trim().replace('.', "-")
}

/// Convert a Yahoo equity provider symbol into the app's canonical ticker form.
///
/// Known exchange suffixes are preserved for the canonicalizer to strip into MIC.
/// For unsuffixed Yahoo equity symbols, a trailing one-letter hyphen class maps
/// back to the app's dotted share-class notation.
pub fn yahoo_equity_provider_symbol_to_canonical(symbol: &str) -> String {
    let trimmed = symbol.trim();
    let (base, _suffix_mic, known_suffix) = split_known_yahoo_suffix(trimmed);
    if known_suffix.is_some() {
        let suffix = trimmed.get(base.len()..).unwrap_or_default();
        return format!(
            "{}{}",
            yahoo_equity_provider_base_to_canonical(base),
            suffix
        );
    }

    yahoo_equity_provider_base_to_canonical(trimmed)
}

fn yahoo_equity_provider_base_to_canonical(base: &str) -> String {
    let trimmed = base.trim();
    let Some((base_part, class)) = trimmed.rsplit_once('-') else {
        return trimmed.to_string();
    };
    if base_part.is_empty() || class.len() != 1 || !class.chars().all(|c| c.is_ascii_alphabetic()) {
        return trimmed.to_string();
    }

    format!("{}.{}", base_part, class)
}

/// Build Yahoo search queries for an equity-like user query.
///
/// This keeps known exchange suffixes intact (`SHOP.TO` stays `SHOP.TO`) while
/// trying Yahoo's base share-class notation first (`BRK.B` -> `BRK-B`).
pub fn yahoo_equity_search_queries(query: &str) -> Vec<String> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return vec![];
    }

    let (base, _suffix_mic, known_suffix) = split_known_yahoo_suffix(trimmed);
    let provider_base = yahoo_equity_base_to_provider(base);
    let provider_query = if known_suffix.is_some() {
        let suffix = trimmed.get(base.len()..).unwrap_or_default();
        format!("{provider_base}{suffix}")
    } else {
        provider_base
    };

    let mut queries = vec![provider_query];
    if !queries[0].eq_ignore_ascii_case(trimmed) {
        queries.push(trimmed.to_string());
    }
    queries
}

/// Extract canonical ticker from Yahoo provider symbol.
///
/// Uses a whitelist approach to safely strip exchange suffixes while preserving
/// share classes like BRK.B or RDS.A (since .B and .A are not in the whitelist).
pub fn strip_yahoo_suffix(symbol: &str) -> &str {
    // Handle special suffixes first
    if let Some(base) = strip_ascii_suffix_ignore_case(symbol, "=X") {
        // FX pairs like EURUSD=X
        return base;
    }
    if let Some(base) = strip_ascii_suffix_ignore_case(symbol, "=F") {
        // Futures like GC=F
        return base;
    }

    // Only strip if suffix is in our known exchange whitelist
    for suffix in yahoo_exchange_suffixes() {
        if let Some(base) = strip_ascii_suffix_ignore_case(symbol, suffix) {
            return base;
        }
    }

    // No known suffix found - return as-is (preserves BRK.B, RDS.A, etc.)
    symbol
}

/// Convert a base ticker to a Yahoo Finance provider symbol.
#[allow(dead_code)]
pub fn yahoo_equity_base_to_provider_with_mic(base: &str, mic: &Mic) -> String {
    let suffixes = yahoo_exchange_suffixes();
    let suffix = suffixes
        .iter()
        .find(|s| yahoo_suffix_to_mic(s).is_some_and(|m| m.eq_ignore_ascii_case(mic.as_ref())));
    if let Some(suffix) = suffix {
        if suffix.is_empty() {
            yahoo_equity_base_to_provider(base)
        } else {
            format!("{}{}", yahoo_equity_base_to_provider(base), suffix)
        }
    } else {
        // For US exchanges, no suffix needed
        yahoo_equity_base_to_provider(base)
    }
}

/// Convert a Yahoo Finance provider symbol back to a canonical ticker.
#[allow(dead_code)]
pub fn yahoo_equity_provider_symbol_to_canonical_with_mic(symbol: &str, mic: &Mic) -> String {
    let suffixes = yahoo_exchange_suffixes();
    let suffix = suffixes
        .iter()
        .find(|s| yahoo_suffix_to_mic(s).is_some_and(|m| m.eq_ignore_ascii_case(mic.as_ref())));
    if let Some(suffix) = suffix {
        if !suffix.is_empty() && symbol.ends_with(suffix) {
            let base = &symbol[..symbol.len() - suffix.len()];
            return yahoo_equity_provider_base_to_canonical(base);
        }
    }
    // For US exchanges, return as-is
    strip_yahoo_suffix(symbol).to_string()
}

/// Generate search queries for Yahoo Finance equity search.
#[allow(dead_code)]
pub fn yahoo_equity_search_queries_with_mic(base: &str, _mic: &Mic) -> Vec<String> {
    yahoo_equity_search_queries(base)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_map_north_america() {
        let map = ExchangeMap::new();

        // NYSE - no suffix for US exchanges
        assert_eq!(
            map.get_suffix(&Cow::Borrowed("XNYS"), &Cow::Borrowed("YAHOO")),
            Some("")
        );
        assert_eq!(
            map.get_currency(&Cow::Borrowed("XNYS"), &Cow::Borrowed("YAHOO")),
            Some("USD")
        );

        // Toronto
        assert_eq!(
            map.get_suffix(&Cow::Borrowed("XTSE"), &Cow::Borrowed("YAHOO")),
            Some(".TO")
        );
        assert_eq!(
            map.get_currency(&Cow::Borrowed("XTSE"), &Cow::Borrowed("YAHOO")),
            Some("CAD")
        );
    }

    #[test]
    fn test_exchange_map_europe() {
        let map = ExchangeMap::new();

        // London - Yahoo returns GBp (pence)
        assert_eq!(
            map.get_suffix(&Cow::Borrowed("XLON"), &Cow::Borrowed("YAHOO")),
            Some(".L")
        );
        assert_eq!(
            map.get_currency(&Cow::Borrowed("XLON"), &Cow::Borrowed("YAHOO")),
            Some("GBP")
        );

        // XETRA
        assert_eq!(
            map.get_suffix(&Cow::Borrowed("XETR"), &Cow::Borrowed("YAHOO")),
            Some(".DE")
        );

        // Cboe UK
        assert_eq!(
            map.get_suffix(&Cow::Borrowed("CXE"), &Cow::Borrowed("YAHOO")),
            Some(".XC")
        );
        assert_eq!(
            map.get_currency(&Cow::Borrowed("CXE"), &Cow::Borrowed("YAHOO")),
            Some("GBP")
        );
    }

    #[test]
    fn test_yahoo_exchange_to_mic() {
        // NASDAQ variants
        assert_eq!(
            yahoo_exchange_to_mic("NMS"),
            Some(Cow::Owned("XNAS".to_string()))
        );
        assert_eq!(
            yahoo_exchange_to_mic("NGM"),
            Some(Cow::Owned("XNAS".to_string()))
        );
        assert_eq!(
            yahoo_exchange_to_mic("NYQ"),
            Some(Cow::Owned("XNYS".to_string()))
        );

        // Yahoo uses PCX for NYSE Arca ETFs; ASE is NYSE American.
        assert_eq!(
            yahoo_exchange_to_mic("PCX"),
            Some(Cow::Owned("ARCX".to_string()))
        );
        assert_eq!(
            yahoo_exchange_to_mic("ASE"),
            Some(Cow::Owned("XASE".to_string()))
        );

        // Toronto
        assert_eq!(
            yahoo_exchange_to_mic("TOR"),
            Some(Cow::Owned("XTSE".to_string()))
        );

        // Cboe UK
        assert_eq!(
            yahoo_exchange_to_mic("CXE"),
            Some(Cow::Owned("CXE".to_string()))
        );
        assert_eq!(
            yahoo_exchange_to_mic(" cxe "),
            Some(Cow::Owned("CXE".to_string()))
        );

        // Unknown
        assert_eq!(yahoo_exchange_to_mic("UNKNOWN"), None);
    }

    #[test]
    fn test_strip_yahoo_suffix() {
        // Normal exchange suffixes
        assert_eq!(strip_yahoo_suffix("SHOP.TO"), "SHOP");
        assert_eq!(strip_yahoo_suffix("shop.to"), "shop");
        assert_eq!(strip_yahoo_suffix("AAPL"), "AAPL");
        assert_eq!(strip_yahoo_suffix("VOD.L"), "VOD");
        assert_eq!(strip_yahoo_suffix("vod.l"), "vod");

        // Share classes preserved
        assert_eq!(strip_yahoo_suffix("BRK.B"), "BRK.B");
        assert_eq!(strip_yahoo_suffix("RDS.A"), "RDS.A");

        // Cboe Europe EUR suffix
        assert_eq!(strip_yahoo_suffix("SXLPM.XD"), "SXLPM");

        // Special suffixes
        assert_eq!(strip_yahoo_suffix("EURUSD=X"), "EURUSD");
        assert_eq!(strip_yahoo_suffix("eurusd=x"), "eurusd");
        assert_eq!(strip_yahoo_suffix("GC=F"), "GC");
        assert_eq!(strip_yahoo_suffix("gc=f"), "gc");
    }

    #[test]
    fn test_yahoo_suffix_helpers_handle_non_ascii_symbols() {
        for symbol in ["AAO", "XYZ"] {
            assert_eq!(strip_yahoo_suffix(symbol), symbol);
            assert_eq!(
                yahoo_equity_search_queries(symbol),
                vec![symbol.to_string()]
            );
            assert_eq!(yahoo_equity_provider_symbol_to_canonical(symbol), symbol);

            let suffixed = format!("{symbol}.TO");
            assert_eq!(strip_yahoo_suffix(&suffixed), symbol);
            assert_eq!(
                yahoo_equity_search_queries(&suffixed),
                vec![suffixed.clone()]
            );
            assert_eq!(
                yahoo_equity_provider_symbol_to_canonical(&suffixed),
                suffixed
            );
        }
    }

    #[test]
    fn test_yahoo_share_class_aliases_are_explicit() {
        assert_eq!(yahoo_equity_base_to_provider("BRK.B"), "BRK-B");
        assert_eq!(yahoo_equity_base_to_provider("brk.a"), "brk-a");
        assert_eq!(yahoo_equity_search_queries("BRK.B"), vec!["BRK-B", "BRK.B"]);
        assert_eq!(yahoo_equity_search_queries("SHOP.TO"), vec!["SHOP.TO"]);
        assert_eq!(yahoo_equity_search_queries("VOD.L"), vec!["VOD.L"]);
        assert_eq!(yahoo_equity_provider_symbol_to_canonical("BRK-B"), "BRK.B");
        assert_eq!(
            yahoo_equity_provider_symbol_to_canonical("BRK-B.TO"),
            "BRK.B.TO"
        );
        assert_eq!(
            yahoo_equity_provider_symbol_to_canonical("SHOP.TO"),
            "SHOP.TO"
        );
    }

    #[test]
    fn test_yahoo_exchange_suffixes_are_never_share_classes() {
        for suffix in yahoo_exchange_suffixes() {
            let query = format!("ABC{suffix}");
            assert_eq!(yahoo_equity_search_queries(&query), vec![query]);

            let provider_symbol = format!("BRK-B{suffix}");
            let canonical_symbol = format!("BRK.B{suffix}");
            assert_eq!(
                yahoo_equity_provider_symbol_to_canonical(&provider_symbol),
                canonical_symbol
            );
        }
    }

    #[test]
    fn test_yahoo_suffix_to_mic() {
        // North America
        assert_eq!(yahoo_suffix_to_mic(".TO"), Some("XTSE"));
        assert_eq!(yahoo_suffix_to_mic("to"), None); // Must include dot prefix

        // UK & Europe
        assert_eq!(yahoo_suffix_to_mic(".L"), Some("XLON"));
        assert_eq!(yahoo_suffix_to_mic(".XC"), Some("CXE"));
        assert_eq!(yahoo_suffix_to_mic(".XD"), Some("DXE"));
        assert_eq!(yahoo_suffix_to_mic(".DE"), Some("XETR"));
        assert_eq!(yahoo_suffix_to_mic(".PA"), Some("XPAR"));

        // Asia
        assert_eq!(yahoo_suffix_to_mic(".T"), Some("XTKS"));
        assert_eq!(yahoo_suffix_to_mic(".HK"), Some("XHKG"));

        // Unknown
        assert_eq!(yahoo_suffix_to_mic(".UNKNOWN"), None);
        assert_eq!(yahoo_suffix_to_mic(".B"), None); // Share class, not suffix
    }

    #[test]
    fn test_exchange_map_has_mic() {
        let map = ExchangeMap::new();
        assert!(map.has_mic(&Cow::Borrowed("XTSE")));
        assert!(!map.has_mic(&Cow::Borrowed("UNKNOWN")));
    }

    #[test]
    fn test_exchange_map_get_currency() {
        let map = ExchangeMap::new();
        assert_eq!(
            map.get_currency(&Cow::Borrowed("XTSE"), &Cow::Borrowed("YAHOO")),
            Some("CAD")
        );
    }

    #[test]
    fn test_yahoo_equity_base_to_provider_with_mic() {
        let mic = Cow::Borrowed("XTSE");
        assert_eq!(
            yahoo_equity_base_to_provider_with_mic("SHOP", &mic),
            "SHOP.TO"
        );
        assert_eq!(
            yahoo_equity_base_to_provider_with_mic("BRK.B", &mic),
            "BRK-B.TO"
        );

        let mic = Cow::Borrowed("XNAS");
        assert_eq!(yahoo_equity_base_to_provider_with_mic("AAPL", &mic), "AAPL");
        assert_eq!(
            yahoo_equity_base_to_provider_with_mic("BRK.B", &mic),
            "BRK-B"
        );
    }
}
