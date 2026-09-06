// SEC EDGAR client service for AlphaForge.
// Compliant with SEC Fair Access policy (custom User-Agent required).

use domain::research::SecFiling;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, USER_AGENT};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::LazyLock;
use std::time::Duration;
use thiserror::Error;

const SEC_SUBMISSIONS_URL: &str = "https://data.sec.gov/submissions/CIK";
const SEC_USER_AGENT: &str = "AlphaForgeResearch research@alphaforge.local";

#[derive(Debug, Error)]
pub enum SecEdgarError {
    #[error("network request to SEC EDGAR failed: {0}")]
    Network(#[from] reqwest::Error),
    #[error("unknown or unmapped ticker: {0}")]
    UnknownTicker(String),
    #[error("failed to parse SEC EDGAR payload: {0}")]
    ParseError(String),
}

// Well-known 10-digit CIK map for key tech & portfolio equities
static TICKER_CIK_MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("NVDA", "0001045810");
    m.insert("AAPL", "0000320193");
    m.insert("MSFT", "0000789019");
    m.insert("GOOGL", "0001652044");
    m.insert("GOOG", "0001652044");
    m.insert("AMZN", "0001018724");
    m.insert("META", "0001326801");
    m.insert("TSLA", "0001318605");
    m.insert("ASML", "0000937966");
    m.insert("TSM", "0001046179");
    m.insert("AMD", "0000002488");
    m.insert("INTC", "0000050863");
    m.insert("AVGO", "0001730168");
    m.insert("QCOM", "0000804328");
    m.insert("ARM", "0001973239");
    m.insert("BABA", "0001577552");
    m.insert("BRK.A", "0001067983");
    m.insert("BRK.B", "0001067983");
    m.insert("JPM", "0000019617");
    m.insert("V", "0001403161");
    m
});

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct SecRecentFilings {
    accessionNumber: Vec<String>,
    filingDate: Vec<String>,
    reportDate: Vec<Option<String>>,
    form: Vec<String>,
    primaryDocument: Vec<String>,
    primaryDocDescription: Vec<Option<String>>,
}

#[derive(Debug, Deserialize)]
struct SecFilingsContainer {
    recent: SecRecentFilings,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct SecSubmissionsResponse {
    cik: String,
    name: String,
    tickers: Option<Vec<String>>,
    filings: SecFilingsContainer,
}

pub struct SecEdgarService {
    client: reqwest::Client,
}

impl Default for SecEdgarService {
    fn default() -> Self {
        Self::new()
    }
}

impl SecEdgarService {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(SEC_USER_AGENT));
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(12))
            .build()
            .unwrap_or_default();

        Self { client }
    }

    /// Resolve 10-digit CIK for a ticker symbol.
    pub fn resolve_cik(&self, ticker: &str) -> Option<&'static str> {
        let upper = ticker.to_ascii_uppercase();
        TICKER_CIK_MAP.get(upper.as_str()).copied()
    }

    /// Fetch recent filings for a company by ticker.
    /// Forms can include 10-K, 10-Q, 8-K, 20-F, 6-K.
    pub async fn fetch_recent_filings(
        &self,
        ticker: &str,
        limit: usize,
    ) -> Result<Vec<SecFiling>, SecEdgarError> {
        let upper_ticker = ticker.to_ascii_uppercase();
        let cik = self
            .resolve_cik(&upper_ticker)
            .ok_or_else(|| SecEdgarError::UnknownTicker(ticker.to_string()))?;

        let url = format!("{}{}.json", SEC_SUBMISSIONS_URL, cik);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(SecEdgarError::Network)?;

        if !response.status().is_success() {
            return Err(SecEdgarError::ParseError(format!(
                "SEC EDGAR returned status: {}",
                response.status()
            )));
        }

        let payload: SecSubmissionsResponse = response
            .json()
            .await
            .map_err(|e| SecEdgarError::ParseError(e.to_string()))?;

        let recent = payload.filings.recent;
        let mut filings = Vec::new();
        let count = recent.accessionNumber.len().min(recent.form.len());

        let raw_cik_digits = payload.cik.trim_start_matches('0');

        for i in 0..count {
            if filings.len() >= limit {
                break;
            }

            let form = &recent.form[i];
            // Filter to core financial/material regulatory filings
            let is_target_form = matches!(
                form.as_str(),
                "10-K" | "10-Q" | "8-K" | "20-F" | "6-K" | "10-K/A" | "10-Q/A"
            );
            if !is_target_form {
                continue;
            }

            let acc = &recent.accessionNumber[i];
            let acc_clean = acc.replace('-', "");
            let primary_doc = recent.primaryDocument.get(i).cloned().unwrap_or_default();

            // Construct SEC EDGAR direct viewer URL
            let filing_url = format!(
                "https://www.sec.gov/Archives/edgar/data/{}/{}/{}",
                raw_cik_digits, acc_clean, primary_doc
            );

            let filing_date = recent.filingDate.get(i).cloned().unwrap_or_default();
            let report_date = recent.reportDate.get(i).and_then(|r| r.clone());
            let primary_doc_description =
                recent.primaryDocDescription.get(i).and_then(|d| d.clone());

            filings.push(SecFiling {
                id: format!("{}-{}", payload.cik, acc),
                accession_number: acc.clone(),
                cik: payload.cik.clone(),
                ticker: upper_ticker.clone(),
                company_name: payload.name.clone(),
                form_type: form.clone(),
                filing_date,
                report_date,
                primary_document: Some(primary_doc),
                primary_doc_description,
                filing_url,
                summary: None,
            });
        }

        Ok(filings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_known_cik() {
        let svc = SecEdgarService::new();
        assert_eq!(svc.resolve_cik("NVDA"), Some("0001045810"));
        assert_eq!(svc.resolve_cik("aapl"), Some("0000320193"));
        assert_eq!(svc.resolve_cik("MSFT"), Some("0000789019"));
        assert_eq!(svc.resolve_cik("UNKNOWN_CO"), None);
    }

    #[test]
    fn test_parse_sec_submissions_mock() {
        let json_data = r#"{
            "cik": "0001045810",
            "name": "NVIDIA CORP",
            "tickers": ["NVDA"],
            "filings": {
                "recent": {
                    "accessionNumber": ["0001045810-24-000123", "0001045810-24-000120"],
                    "filingDate": ["2024-08-28", "2024-05-22"],
                    "reportDate": ["2024-07-28", "2024-04-28"],
                    "form": ["10-Q", "10-Q"],
                    "primaryDocument": ["nvda-20240728.htm", "nvda-20240428.htm"],
                    "primaryDocDescription": ["10-Q", "10-Q"]
                }
            }
        }"#;

        let parsed: SecSubmissionsResponse = serde_json::from_str(json_data).unwrap();
        assert_eq!(parsed.cik, "0001045810");
        assert_eq!(parsed.filings.recent.accessionNumber.len(), 2);
    }
}
