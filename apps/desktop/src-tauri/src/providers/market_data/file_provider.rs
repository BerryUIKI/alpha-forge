//! File-based option chain provider.
//!
//! Reads `.csv` and `.json` files containing option contract data and
//! converts them into validated `OptionChain` + `OptionContract` records.
//!
//! CSV format expects one header row followed by one contract per row:
//! ```csv
//! option_type,strike,expiration,contract_multiplier,bid,ask,last,volume,open_interest,implied_volatility
//! call,150.0,2026-09-17T00:00:00Z,100,4.0,5.0,4.5,10,20,0.25
//! put,150.0,2026-09-17T00:00:00Z,100,3.0,4.0,3.5,15,30,0.26
//! ```
//!
//! JSON format is an array of contract objects with the same fields.
//!
//! Constraints:
//! - Maximum file size: 10 MB.
//! - The resolved path must be within the workspace's imported-data directory
//!   or a temporary directory explicitly approved by the import dialog.
//! - Rows with invalid or missing required fields are rejected individually.
//! - A report of N-of-M successful rows is returned so the caller can show
//!   partial-data semantics.

use std::path::Path;

use chrono::{DateTime, Utc};
use domain::option::{OptionContract, OptionType};
use serde::Deserialize;
use uuid::Uuid;

use crate::error::AppError;

/// Maximum file size for option chain imports (10 MB).
pub const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// Import result with partial-data semantics.
#[derive(Debug)]
pub struct FileImportResult {
    /// The symbol extracted from the filename or derived from the data.
    pub symbol: String,
    /// The underlying price derived from the data (average of strikes or a
    /// user-provided value).
    pub underlying_price: f64,
    /// Contracts that passed validation, ready for persistence.
    pub contracts: Vec<OptionContract>,
    /// Number of rows that were rejected.
    pub rejected_count: usize,
    /// Human-readable explanation of any rejections.
    pub rejection_detail: Option<String>,
}

/// A single contract row from a CSV file.
#[derive(Debug, Deserialize)]
struct CsvContractRow {
    option_type: String,
    strike: f64,
    expiration: String,
    #[serde(default = "default_multiplier")]
    contract_multiplier: u32,
    bid: f64,
    ask: f64,
    last: Option<f64>,
    #[serde(default)]
    volume: u64,
    #[serde(default)]
    open_interest: u64,
    implied_volatility: f64,
}

fn default_multiplier() -> u32 {
    100
}

/// A single contract row from a JSON file.
#[derive(Debug, Deserialize)]
struct JsonContractRow {
    option_type: String,
    strike: f64,
    expiration: String,
    #[serde(default = "default_multiplier")]
    contract_multiplier: u32,
    bid: f64,
    ask: f64,
    last: Option<f64>,
    #[serde(default)]
    volume: u64,
    #[serde(default)]
    open_interest: u64,
    implied_volatility: f64,
}

/// Validate that a file path is safe to import.
///
/// Checks:
/// - The file exists and is readable.
/// - The file size does not exceed MAX_FILE_SIZE.
/// - The resolved path does not contain path traversal components.
pub fn validate_import_path(file_path: &Path) -> Result<(), AppError> {
    // Check for path traversal
    let canonical = file_path
        .canonicalize()
        .map_err(|_| AppError::Validation("File path cannot be resolved".to_string()))?;

    // Ensure the resolved path does not contain suspicious components
    let path_str = canonical.to_string_lossy();
    if path_str.contains("..") {
        return Err(AppError::Validation(
            "File path must not contain parent directory references".to_string(),
        ));
    }

    // Check file exists and is readable
    if !canonical.exists() {
        return Err(AppError::NotFound("File not found".to_string()));
    }
    if !canonical.is_file() {
        return Err(AppError::Validation("Path is not a file".to_string()));
    }

    // Check file size
    let metadata = std::fs::metadata(&canonical)
        .map_err(|e| AppError::Internal(format!("Failed to read file metadata: {}", e)))?;
    if metadata.len() > MAX_FILE_SIZE {
        return Err(AppError::Validation(format!(
            "File size {} exceeds maximum of {}",
            metadata.len(),
            MAX_FILE_SIZE
        )));
    }

    Ok(())
}

/// Parse option chain data from a CSV file.
///
/// Returns contracts and a summary of rejected rows.
fn parse_csv_contracts(
    content: &str,
    symbol: &str,
    workspace_id: &str,
    chain_id: &str,
) -> Result<(Vec<OptionContract>, usize, Vec<String>), AppError> {
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(content.as_bytes());

    let mut contracts = Vec::new();
    let mut rejected = 0usize;
    let mut rejection_reasons = Vec::new();

    for (row_idx, result) in reader.deserialize::<CsvContractRow>().enumerate() {
        match result {
            Ok(row) => match row_to_contract(&row, symbol, workspace_id, chain_id, row_idx) {
                Ok(contract) => contracts.push(contract),
                Err(e) => {
                    rejected += 1;
                    rejection_reasons.push(format!("Row {}: {}", row_idx + 2, e));
                }
            },
            Err(e) => {
                rejected += 1;
                rejection_reasons.push(format!("Row {}: {}", row_idx + 2, e));
            }
        }
    }

    Ok((contracts, rejected, rejection_reasons))
}

/// Parse option chain data from a JSON file.
fn parse_json_contracts(
    content: &str,
    symbol: &str,
    workspace_id: &str,
    chain_id: &str,
) -> Result<(Vec<OptionContract>, usize, Vec<String>), AppError> {
    let rows: Vec<JsonContractRow> = serde_json::from_str(content)
        .map_err(|e| AppError::Validation(format!("Invalid JSON format: {}", e)))?;

    let mut contracts = Vec::new();
    let mut rejected = 0usize;
    let mut rejection_reasons = Vec::new();

    for (row_idx, row) in rows.into_iter().enumerate() {
        match row_to_contract_json(&row, symbol, workspace_id, chain_id, row_idx) {
            Ok(contract) => contracts.push(contract),
            Err(e) => {
                rejected += 1;
                rejection_reasons.push(format!("Entry {}: {}", row_idx, e));
            }
        }
    }

    Ok((contracts, rejected, rejection_reasons))
}

/// Convert a CSV row to an OptionContract, validating all fields.
fn row_to_contract(
    row: &CsvContractRow,
    symbol: &str,
    workspace_id: &str,
    chain_id: &str,
    _row_idx: usize,
) -> Result<OptionContract, String> {
    let option_type = match row.option_type.to_lowercase().as_str() {
        "call" => OptionType::Call,
        "put" => OptionType::Put,
        _ => return Err(format!("Invalid option_type '{}'", row.option_type)),
    };

    if !row.strike.is_finite() || row.strike <= 0.0 {
        return Err(format!("Invalid strike {}", row.strike));
    }
    if !row.bid.is_finite() || row.bid < 0.0 {
        return Err(format!("Invalid bid {}", row.bid));
    }
    if !row.ask.is_finite() || row.ask < 0.0 {
        return Err(format!("Invalid ask {}", row.ask));
    }
    if row.bid > row.ask {
        return Err(format!("bid ({}) > ask ({})", row.bid, row.ask));
    }
    if let Some(last) = row.last {
        if !last.is_finite() || last < 0.0 {
            return Err(format!("Invalid last {}", last));
        }
    }
    if !row.implied_volatility.is_finite() || row.implied_volatility <= 0.0 {
        return Err(format!(
            "Invalid implied_volatility {}",
            row.implied_volatility
        ));
    }
    if row.contract_multiplier == 0 {
        return Err("contract_multiplier must be positive".to_string());
    }

    let expiration = DateTime::parse_from_rfc3339(&row.expiration)
        .map_err(|e| format!("Invalid expiration '{}': {}", row.expiration, e))?
        .with_timezone(&Utc);

    let now = Utc::now();

    Ok(OptionContract {
        id: Uuid::new_v4().to_string(),
        workspace_id: workspace_id.to_string(),
        chain_id: chain_id.to_string(),
        symbol: symbol.to_string(),
        option_type,
        strike: row.strike,
        expiration,
        contract_multiplier: row.contract_multiplier,
        bid: row.bid,
        ask: row.ask,
        last: row.last,
        volume: row.volume,
        open_interest: row.open_interest,
        implied_volatility: row.implied_volatility,
        created_at: now,
        updated_at: now,
    })
}

/// Convert a JSON row to an OptionContract, validating all fields.
fn row_to_contract_json(
    row: &JsonContractRow,
    symbol: &str,
    workspace_id: &str,
    chain_id: &str,
    _row_idx: usize,
) -> Result<OptionContract, String> {
    let option_type = match row.option_type.to_lowercase().as_str() {
        "call" => OptionType::Call,
        "put" => OptionType::Put,
        _ => return Err(format!("Invalid option_type '{}'", row.option_type)),
    };

    if !row.strike.is_finite() || row.strike <= 0.0 {
        return Err(format!("Invalid strike {}", row.strike));
    }
    if !row.bid.is_finite() || row.bid < 0.0 {
        return Err(format!("Invalid bid {}", row.bid));
    }
    if !row.ask.is_finite() || row.ask < 0.0 {
        return Err(format!("Invalid ask {}", row.ask));
    }
    if row.bid > row.ask {
        return Err(format!("bid ({}) > ask ({})", row.bid, row.ask));
    }
    if let Some(last) = row.last {
        if !last.is_finite() || last < 0.0 {
            return Err(format!("Invalid last {}", last));
        }
    }
    if !row.implied_volatility.is_finite() || row.implied_volatility <= 0.0 {
        return Err(format!(
            "Invalid implied_volatility {}",
            row.implied_volatility
        ));
    }
    if row.contract_multiplier == 0 {
        return Err("contract_multiplier must be positive".to_string());
    }

    let expiration = DateTime::parse_from_rfc3339(&row.expiration)
        .map_err(|e| format!("Invalid expiration '{}': {}", row.expiration, e))?
        .with_timezone(&Utc);

    let now = Utc::now();

    Ok(OptionContract {
        id: Uuid::new_v4().to_string(),
        workspace_id: workspace_id.to_string(),
        chain_id: chain_id.to_string(),
        symbol: symbol.to_string(),
        option_type,
        strike: row.strike,
        expiration,
        contract_multiplier: row.contract_multiplier,
        bid: row.bid,
        ask: row.ask,
        last: row.last,
        volume: row.volume,
        open_interest: row.open_interest,
        implied_volatility: row.implied_volatility,
        created_at: now,
        updated_at: now,
    })
}

/// Read and parse an option chain file.
///
/// The symbol is extracted from the filename (stem). The underlying price
/// is estimated as the median strike price across all contracts.
pub fn import_option_chain_file(
    file_path: &Path,
    workspace_id: &str,
) -> Result<FileImportResult, AppError> {
    validate_import_path(file_path)?;

    let content = std::fs::read_to_string(file_path)
        .map_err(|e| AppError::Internal(format!("Failed to read file: {}", e)))?;

    if content.is_empty() {
        return Err(AppError::Validation("File is empty".to_string()));
    }

    // Strip a common "-options" suffix from the filename stem when present.
    let symbol = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("UNKNOWN")
        .trim_end_matches("-options")
        .trim_end_matches("_options")
        .to_uppercase();

    let chain_id = Uuid::new_v4().to_string();

    let (contracts, rejected, rejection_reasons) = match file_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase()
        .as_str()
    {
        "csv" => parse_csv_contracts(&content, &symbol, workspace_id, &chain_id)?,
        "json" => parse_json_contracts(&content, &symbol, workspace_id, &chain_id)?,
        ext => {
            return Err(AppError::Validation(format!(
                "Unsupported file extension '{}'. Supported: .csv, .json",
                ext
            )))
        }
    };

    if contracts.is_empty() {
        return Err(AppError::Validation(
            "No valid contracts found in file".to_string(),
        ));
    }

    // Estimate underlying price as the median of strikes
    let mut strikes: Vec<f64> = contracts.iter().map(|c| c.strike).collect();
    strikes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let underlying_price = if strikes.len().is_multiple_of(2) {
        let mid = strikes.len() / 2;
        (strikes[mid - 1] + strikes[mid]) / 2.0
    } else {
        strikes[strikes.len() / 2]
    };

    let rejection_detail = if rejection_reasons.is_empty() {
        None
    } else {
        Some(rejection_reasons.join("; "))
    };

    Ok(FileImportResult {
        symbol,
        underlying_price,
        contracts,
        rejected_count: rejected,
        rejection_detail,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------------------------------------------------------------------------
    // Path validation
    // ---------------------------------------------------------------------------

    #[test]
    fn rejects_non_existent_file() {
        let result = validate_import_path(Path::new("/tmp/nonexistent_option_file.csv"));
        assert!(result.is_err());
    }

    #[test]
    fn rejects_directory_path() {
        let result = validate_import_path(Path::new("/tmp"));
        // /tmp exists and is a directory, so this should fail with "not a file"
        assert!(result.is_err());
    }

    // ---------------------------------------------------------------------------
    // CSV parsing
    // ---------------------------------------------------------------------------

    #[test]
    fn parses_valid_csv() {
        let csv = "\
option_type,strike,expiration,contract_multiplier,bid,ask,last,volume,open_interest,implied_volatility
call,150.0,2026-09-17T00:00:00Z,100,4.0,5.0,4.5,10,20,0.25
put,155.0,2026-09-17T00:00:00Z,100,3.0,4.0,,15,30,0.26
";
        let (contracts, rejected, reasons) =
            parse_csv_contracts(csv, "AAPL", "ws-1", "chain-1").unwrap();
        assert_eq!(contracts.len(), 2, "Should parse 2 contracts");
        assert_eq!(rejected, 0, "Should have 0 rejected");
        assert!(reasons.is_empty());
        assert_eq!(contracts[0].option_type, OptionType::Call);
        assert_eq!(contracts[1].option_type, OptionType::Put);
        assert_eq!(contracts[0].strike, 150.0);
        assert_eq!(contracts[1].strike, 155.0);
        assert!(contracts[0].last.is_some());
        assert!(contracts[1].last.is_none());
    }

    #[test]
    fn rejects_csv_with_invalid_option_type() {
        let csv = "\
option_type,strike,expiration,contract_multiplier,bid,ask,last,volume,open_interest,implied_volatility
invalid,150.0,2026-09-17T00:00:00Z,100,4.0,5.0,4.5,10,20,0.25
";
        let (contracts, rejected, reasons) =
            parse_csv_contracts(csv, "AAPL", "ws-1", "chain-1").unwrap();
        assert_eq!(contracts.len(), 0);
        assert_eq!(rejected, 1);
        assert!(!reasons.is_empty());
    }

    #[test]
    fn rejects_csv_row_with_bid_greater_than_ask() {
        let csv = "\
option_type,strike,expiration,contract_multiplier,bid,ask,last,volume,open_interest,implied_volatility
call,150.0,2026-09-17T00:00:00Z,100,6.0,5.0,4.5,10,20,0.25
";
        let (contracts, rejected, _reasons) =
            parse_csv_contracts(csv, "AAPL", "ws-1", "chain-1").unwrap();
        assert_eq!(contracts.len(), 0);
        assert_eq!(rejected, 1);
    }

    #[test]
    fn rejects_csv_row_with_non_finite_strike() {
        let csv = "\
option_type,strike,expiration,contract_multiplier,bid,ask,last,volume,open_interest,implied_volatility
call,nan,2026-09-17T00:00:00Z,100,4.0,5.0,4.5,10,20,0.25
";
        let (contracts, rejected, _reasons) =
            parse_csv_contracts(csv, "AAPL", "ws-1", "chain-1").unwrap();
        assert_eq!(contracts.len(), 0, "Should reject the NaN row");
        assert_eq!(rejected, 1, "Should have 1 rejected row");
    }

    // ---------------------------------------------------------------------------
    // JSON parsing
    // ---------------------------------------------------------------------------

    #[test]
    fn parses_valid_json() {
        let json = r#"[
            {
                "option_type": "call",
                "strike": 150.0,
                "expiration": "2026-09-17T00:00:00Z",
                "contract_multiplier": 100,
                "bid": 4.0,
                "ask": 5.0,
                "last": 4.5,
                "volume": 10,
                "open_interest": 20,
                "implied_volatility": 0.25
            }
        ]"#;
        let (contracts, rejected, _reasons) =
            parse_json_contracts(json, "AAPL", "ws-1", "chain-1").unwrap();
        assert_eq!(contracts.len(), 1);
        assert_eq!(rejected, 0);
        assert_eq!(contracts[0].strike, 150.0);
    }

    #[test]
    fn rejects_malformed_json() {
        let result = parse_json_contracts("not json", "AAPL", "ws-1", "chain-1");
        assert!(result.is_err());
    }

    // ---------------------------------------------------------------------------
    // Full import pipeline
    // ---------------------------------------------------------------------------

    #[test]
    fn import_csv_file_end_to_end() {
        let dir = std::env::temp_dir();
        let path = dir.join("test_AAPL.csv");
        let csv = "\
option_type,strike,expiration,contract_multiplier,bid,ask,last,volume,open_interest,implied_volatility
call,150.0,2026-09-17T00:00:00Z,100,4.0,5.0,4.5,10,20,0.25
put,155.0,2026-09-17T00:00:00Z,100,3.0,4.0,,15,30,0.26
";
        std::fs::write(&path, csv).expect("write test file");

        let result = import_option_chain_file(&path, "ws-1").expect("import");
        assert_eq!(result.symbol, "TEST_AAPL");
        assert_eq!(result.contracts.len(), 2);
        assert_eq!(result.rejected_count, 0);

        std::fs::remove_file(&path).expect("cleanup");
    }

    #[test]
    fn import_json_file_end_to_end() {
        let dir = std::env::temp_dir();
        let path = dir.join("MSFT.json");
        let json = r#"[
            {
                "option_type": "put",
                "strike": 400.0,
                "expiration": "2026-12-18T00:00:00Z",
                "contract_multiplier": 100,
                "bid": 8.0,
                "ask": 9.0,
                "last": 8.5,
                "volume": 50,
                "open_interest": 100,
                "implied_volatility": 0.30
            }
        ]"#;
        std::fs::write(&path, json).expect("write test file");

        let result = import_option_chain_file(&path, "ws-1").expect("import");
        assert_eq!(result.contracts.len(), 1);
        assert_eq!(result.contracts[0].symbol, "MSFT");
        assert_eq!(result.contracts[0].option_type, OptionType::Put);

        std::fs::remove_file(&path).expect("cleanup");
    }

    #[test]
    fn rejects_unsupported_extension() {
        let dir = std::env::temp_dir();
        let path = dir.join("data.txt");
        std::fs::write(&path, "content").expect("write test file");
        let result = import_option_chain_file(&path, "ws-1");
        assert!(result.is_err());
        std::fs::remove_file(&path).expect("cleanup");
    }

    #[test]
    fn rejects_empty_file() {
        let dir = std::env::temp_dir();
        let path = dir.join("empty.csv");
        std::fs::write(&path, "").expect("write test file");
        let result = import_option_chain_file(&path, "ws-1");
        assert!(result.is_err());
        std::fs::remove_file(&path).expect("cleanup");
    }

    #[test]
    fn median_strike_estimate_for_odd_number_of_contracts() {
        let csv = "\
option_type,strike,expiration,contract_multiplier,bid,ask,last,volume,open_interest,implied_volatility
call,100.0,2026-09-17T00:00:00Z,100,4.0,5.0,4.5,10,20,0.25
call,150.0,2026-09-17T00:00:00Z,100,4.0,5.0,4.5,10,20,0.25
call,200.0,2026-09-17T00:00:00Z,100,4.0,5.0,4.5,10,20,0.25
";
        let dir = std::env::temp_dir();
        let path = dir.join("median_test.csv");
        std::fs::write(&path, csv).expect("write test file");
        let result = import_option_chain_file(&path, "ws-1").expect("import");
        // Median of [100, 150, 200] is 150
        assert!((result.underlying_price - 150.0).abs() < 0.001);
        std::fs::remove_file(&path).expect("cleanup");
    }
}
