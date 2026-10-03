// ActivityImportService — Parses generic CSV & IBKR activity statements,
// resolves/creates assets, persists activities atomically, and synchronizes FIFO lots.
//
// Complies with AlphaForge principles:
// - Explicit typed AppError (no unwrap / expect panics)
// - Thin command layer delegation
// - Idempotent import runs and deterministic activity deduplication
// - Full parity with FIFO tax lot creation and lot disposal

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use chrono::NaiveDate;
use domain::financial::{
    ActivityStatus, ActivityType, AssetKind, CreateActivityInput, CreateAssetInput,
    CreateImportRunInput, ImportRun, InstrumentType, QuoteMode,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::database::repositories::account_repository::AccountRepository;
use crate::database::repositories::activity_repository::{ActivityRepository, ImportRunRepository};
use crate::database::repositories::asset_repository::AssetRepository;
use crate::database::repositories::lot_repository::LotRepository;
use crate::error::AppError;
use crate::services::activity_service::ActivityService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportRunSummary {
    pub total_rows: usize,
    pub created_count: usize,
    pub skipped_count: usize,
    pub failed_count: usize,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportFormat {
    GenericCsv,
    IbkrCsv,
}

impl ImportFormat {
    pub fn parse(s: &str) -> Result<Self, AppError> {
        match s.to_ascii_uppercase().as_str() {
            "GENERIC" | "GENERIC_CSV" => Ok(Self::GenericCsv),
            "IBKR" | "IBKR_CSV" => Ok(Self::IbkrCsv),
            other => Err(AppError::Validation(format!(
                "Unsupported import format '{other}'. Supported formats: GENERIC, IBKR"
            ))),
        }
    }
}

pub struct ActivityImportService {
    account_repo: Arc<AccountRepository>,
    asset_repo: Arc<AssetRepository>,
    activity_repo: Arc<ActivityRepository>,
    import_run_repo: Arc<ImportRunRepository>,
    #[allow(dead_code)]
    lot_repo: Arc<LotRepository>,
    activity_service: Arc<ActivityService>,
}

impl ActivityImportService {
    pub fn new(
        account_repo: Arc<AccountRepository>,
        asset_repo: Arc<AssetRepository>,
        activity_repo: Arc<ActivityRepository>,
        import_run_repo: Arc<ImportRunRepository>,
        lot_repo: Arc<LotRepository>,
        activity_service: Arc<ActivityService>,
    ) -> Self {
        Self {
            account_repo,
            asset_repo,
            activity_repo,
            import_run_repo,
            lot_repo,
            activity_service,
        }
    }

    /// Import activities from CSV content into an account.
    pub async fn import_csv(
        &self,
        account_id: &str,
        format_str: &str,
        csv_text: &str,
    ) -> Result<ImportRun, AppError> {
        let format = ImportFormat::parse(format_str)?;
        let _account = self
            .account_repo
            .get(account_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("account {account_id} not found")))?;

        let source_system = match format {
            ImportFormat::GenericCsv => "GENERIC_CSV",
            ImportFormat::IbkrCsv => "IBKR_CSV",
        };

        // Create import run record in PROCESSING status
        let import_run = self
            .import_run_repo
            .create(CreateImportRunInput {
                account_id: account_id.to_string(),
                source_system: source_system.to_string(),
                run_type: "ACTIVITIES".to_string(),
                mode: "APPEND".to_string(),
                status: "PROCESSING".to_string(),
                review_mode: "AUTOMATIC".to_string(),
            })
            .await?;

        let parsed_result = match format {
            ImportFormat::GenericCsv => self.parse_generic_csv(csv_text),
            ImportFormat::IbkrCsv => self.parse_ibkr_csv(csv_text),
        };

        let mut parsed_items = match parsed_result {
            Ok(items) => items,
            Err(e) => {
                let err_msg = e.to_string();
                let summary_json = serde_json::to_string(&ImportRunSummary {
                    total_rows: 0,
                    created_count: 0,
                    skipped_count: 0,
                    failed_count: 1,
                    errors: vec![err_msg.clone()],
                })
                .ok();
                let _ = self
                    .import_run_repo
                    .finish(&import_run.id, "FAILED", summary_json, None, Some(err_msg))
                    .await;
                return Err(e);
            }
        };

        if parsed_items.is_empty() {
            let err_msg = "CSV contains no valid activity rows".to_string();
            let summary_json = serde_json::to_string(&ImportRunSummary {
                total_rows: 0,
                created_count: 0,
                skipped_count: 0,
                failed_count: 0,
                errors: vec![err_msg.clone()],
            })
            .ok();
            let _ = self
                .import_run_repo
                .finish(
                    &import_run.id,
                    "FAILED",
                    summary_json,
                    None,
                    Some(err_msg.clone()),
                )
                .await;
            return Err(AppError::Validation(err_msg));
        }

        // Sort parsed items chronologically (and buys before sells on the same day to maintain FIFO lot inventory)
        parsed_items.sort_by(|a, b| {
            let date_cmp = a.activity_date.cmp(&b.activity_date);
            if date_cmp != std::cmp::Ordering::Equal {
                return date_cmp;
            }
            let priority_a = match a.activity_type {
                ActivityType::Buy | ActivityType::Deposit | ActivityType::TransferIn => 0,
                _ => 1,
            };
            let priority_b = match b.activity_type {
                ActivityType::Buy | ActivityType::Deposit | ActivityType::TransferIn => 0,
                _ => 1,
            };
            priority_a.cmp(&priority_b)
        });

        let total_rows = parsed_items.len();
        let mut created_count = 0usize;
        let mut skipped_count = 0usize;
        let mut failed_count = 0usize;
        let mut errors = Vec::new();
        let mut seen_occurrences: HashMap<String, usize> = HashMap::new();
        let mut resolved_assets: HashMap<String, String> = HashMap::new();

        for (idx, item) in parsed_items.into_iter().enumerate() {
            // Compute deterministic row signature
            let row_content = format!(
                "{}:{}:{}:{}:{}:{}:{}",
                item.activity_date,
                item.activity_type,
                item.symbol.as_deref().unwrap_or(""),
                item.quantity.unwrap_or(Decimal::ZERO),
                item.unit_price.unwrap_or(Decimal::ZERO),
                item.amount.unwrap_or(Decimal::ZERO),
                item.currency
            );
            let count = seen_occurrences.entry(row_content.clone()).or_insert(0);
            let occurrence = *count;
            *count += 1;

            let hash_input = format!("{}:{}:{}", source_system, row_content, occurrence);
            let hash = format!("{:x}", Sha256::digest(hash_input.as_bytes()));
            let idempotency_key =
                format!("csv:{}:{}:{}", account_id, item.activity_date, &hash[..16]);

            // Deduplication: if activity with this key already exists in the ledger, skip without error
            match self
                .activity_repo
                .find_by_idempotency_key(&idempotency_key)
                .await
            {
                Ok(Some(_)) => {
                    skipped_count += 1;
                    continue;
                }
                Ok(None) => {}
                Err(e) => {
                    failed_count += 1;
                    errors.push(format!("Row {}: lookup failed: {e}", idx + 1));
                    continue;
                }
            }

            // Resolve or create asset if symbol is specified
            let asset_id = if let Some(sym) = item.symbol {
                if let Some(cached_id) = resolved_assets.get(&sym) {
                    Some(cached_id.clone())
                } else {
                    let key = format!("EQUITY:{}@XNAS", sym.to_ascii_uppercase());
                    match self.asset_repo.find_by_instrument_key(&key).await {
                        Ok(Some(a)) => {
                            resolved_assets.insert(sym.clone(), a.id.clone());
                            Some(a.id)
                        }
                        Ok(None) => {
                            match self
                                .asset_repo
                                .create(CreateAssetInput {
                                    kind: AssetKind::Investment,
                                    name: Some(sym.clone()),
                                    display_code: Some(sym.clone()),
                                    notes: Some(
                                        "Imported from broker activity statement".to_string(),
                                    ),
                                    is_active: true,
                                    quote_mode: QuoteMode::Market,
                                    quote_ccy: item.currency.clone(),
                                    instrument_type: Some(InstrumentType::Equity),
                                    instrument_symbol: Some(sym.clone()),
                                    instrument_exchange_mic: Some("XNAS".to_string()),
                                    provider_config: None,
                                })
                                .await
                            {
                                Ok(a) => {
                                    resolved_assets.insert(sym.clone(), a.id.clone());
                                    Some(a.id)
                                }
                                Err(e) => {
                                    failed_count += 1;
                                    errors.push(format!(
                                        "Row {}: failed to create asset for {sym}: {e}",
                                        idx + 1
                                    ));
                                    continue;
                                }
                            }
                        }
                        Err(e) => {
                            failed_count += 1;
                            errors.push(format!(
                                "Row {}: failed to lookup asset for {sym}: {e}",
                                idx + 1
                            ));
                            continue;
                        }
                    }
                }
            } else {
                None
            };

            let activity_input = CreateActivityInput {
                account_id: account_id.to_string(),
                asset_id,
                activity_type: item.activity_type,
                activity_type_override: None,
                source_type: Some(source_system.to_string()),
                subtype: None,
                status: ActivityStatus::Posted,
                activity_date: item.activity_date,
                settlement_date: item.settlement_date,
                quantity: item.quantity,
                unit_price: item.unit_price,
                amount: item.amount,
                fee: item.fee,
                tax: item.tax,
                currency: item.currency.clone(),
                fx_rate: item.fx_rate,
                notes: item.notes,
                metadata: None,
                source_system: Some(source_system.to_string()),
                source_record_id: None,
                source_group_id: None,
                idempotency_key: Some(idempotency_key),
                import_run_id: Some(import_run.id.clone()),
            };

            // Atomically create activity + handle lots via ActivityService
            match self.activity_service.create_activity(activity_input).await {
                Ok(_) => {
                    created_count += 1;
                }
                Err(e) => {
                    failed_count += 1;
                    errors.push(format!("Row {}: {e}", idx + 1));
                }
            }
        }

        // Determine terminal status
        let status = if failed_count == 0 {
            "COMPLETED"
        } else if created_count > 0 {
            "PARTIAL"
        } else {
            "FAILED"
        };

        let summary = ImportRunSummary {
            total_rows,
            created_count,
            skipped_count,
            failed_count,
            errors: errors.clone(),
        };
        let summary_json = serde_json::to_string(&summary).ok();
        let warnings = if !errors.is_empty() && created_count > 0 {
            Some(format!(
                "{} row(s) failed during import: {}",
                failed_count,
                errors.join("; ")
            ))
        } else {
            None
        };
        let error_msg = if failed_count > 0 && created_count == 0 {
            Some(format!("Import failed: {}", errors.join("; ")))
        } else {
            None
        };

        self.import_run_repo
            .finish(&import_run.id, status, summary_json, warnings, error_msg)
            .await
    }

    /// Parse generic broker CSV:
    /// Headers: date,type,symbol,quantity,price,amount,fee,currency,notes
    fn parse_generic_csv(&self, csv_text: &str) -> Result<Vec<ParsedActivityRow>, AppError> {
        let mut rdr = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .flexible(true)
            .from_reader(csv_text.as_bytes());

        let headers = rdr
            .headers()
            .map_err(|e| AppError::Validation(format!("Invalid generic CSV header: {e}")))?
            .clone();

        let header_map: std::collections::HashMap<String, usize> = headers
            .iter()
            .enumerate()
            .map(|(i, h)| (h.to_ascii_lowercase(), i))
            .collect();

        let date_idx = header_map
            .get("date")
            .or_else(|| header_map.get("activity_date"))
            .ok_or_else(|| {
                AppError::Validation("CSV missing required 'date' column".to_string())
            })?;
        let type_idx = header_map
            .get("type")
            .or_else(|| header_map.get("activity_type"))
            .ok_or_else(|| {
                AppError::Validation("CSV missing required 'type' column".to_string())
            })?;

        let symbol_idx = header_map
            .get("symbol")
            .or_else(|| header_map.get("ticker"));
        let qty_idx = header_map.get("quantity").or_else(|| header_map.get("qty"));
        let price_idx = header_map
            .get("price")
            .or_else(|| header_map.get("unit_price"));
        let amount_idx = header_map.get("amount").or_else(|| header_map.get("total"));
        let fee_idx = header_map.get("fee");
        let ccy_idx = header_map.get("currency").or_else(|| header_map.get("ccy"));
        let notes_idx = header_map
            .get("notes")
            .or_else(|| header_map.get("description"));

        let mut rows = Vec::new();
        for (line, record) in rdr.records().enumerate() {
            let rec = record.map_err(|e| {
                AppError::Validation(format!("Error parsing generic CSV row {}: {e}", line + 2))
            })?;

            let date_str = rec.get(*date_idx).unwrap_or("").trim();
            if date_str.is_empty() {
                continue;
            }
            let activity_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .or_else(|_| NaiveDate::parse_from_str(date_str, "%m/%d/%Y"))
                .or_else(|_| NaiveDate::parse_from_str(date_str, "%d/%m/%Y"))
                .map_err(|e| {
                    AppError::Validation(format!(
                        "Row {}: invalid date '{date_str}': {e}. Use YYYY-MM-DD or MM/DD/YYYY",
                        line + 2
                    ))
                })?;

            let type_str = rec.get(*type_idx).unwrap_or("").trim().to_ascii_uppercase();
            let activity_type = match type_str.as_str() {
                "BUY" => ActivityType::Buy,
                "SELL" => ActivityType::Sell,
                "DIVIDEND" | "DIV" => ActivityType::Dividend,
                "INTEREST" | "INT" => ActivityType::Interest,
                "DEPOSIT" | "DEP" => ActivityType::Deposit,
                "WITHDRAWAL" | "WITHDRAW" | "WITH" => ActivityType::Withdrawal,
                "FEE" => ActivityType::Fee,
                "SPLIT" => ActivityType::Split,
                _ => ActivityType::Unknown,
            };

            let symbol = symbol_idx
                .and_then(|&idx| rec.get(idx))
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty());
            let quantity = qty_idx
                .and_then(|&idx| rec.get(idx))
                .and_then(|s| Decimal::from_str(s.trim()).ok());
            let unit_price = price_idx
                .and_then(|&idx| rec.get(idx))
                .and_then(|s| Decimal::from_str(s.trim()).ok());
            let amount = amount_idx
                .and_then(|&idx| rec.get(idx))
                .and_then(|s| Decimal::from_str(s.trim()).ok());
            let fee = fee_idx
                .and_then(|&idx| rec.get(idx))
                .and_then(|s| Decimal::from_str(s.trim()).ok());
            let currency = ccy_idx
                .and_then(|&idx| rec.get(idx))
                .map(|s| s.trim().to_ascii_uppercase())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "USD".to_string());
            let notes = notes_idx
                .and_then(|&idx| rec.get(idx))
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty());

            rows.push(ParsedActivityRow {
                activity_date,
                settlement_date: None,
                activity_type,
                symbol,
                quantity,
                unit_price,
                amount,
                fee,
                tax: None,
                currency,
                fx_rate: None,
                notes,
            });
        }

        Ok(rows)
    }

    /// Parse Interactive Brokers (IBKR) Activity Statement CSV:
    /// Matches the "Trades,Data,Order,..." rows from default IBKR exports.
    fn parse_ibkr_csv(&self, csv_text: &str) -> Result<Vec<ParsedActivityRow>, AppError> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .trim(csv::Trim::All)
            .flexible(true)
            .from_reader(csv_text.as_bytes());

        let mut rows = Vec::new();
        for (line, record) in rdr.records().enumerate() {
            let rec = record.map_err(|e| {
                AppError::Validation(format!("Error parsing IBKR CSV line {}: {e}", line + 1))
            })?;

            if rec.len() < 12 {
                continue;
            }

            let section = rec.get(0).unwrap_or("");
            let row_type = rec.get(1).unwrap_or("");

            // IBKR Trades section: "Trades,Data,Order,Stocks,USD,AAPL,2026-08-10, 10:30:00,10,150.00,1500.00,-1.00,..."
            if section == "Trades" && row_type == "Data" {
                let asset_category = rec.get(3).unwrap_or("");
                if asset_category != "Stocks" && asset_category != "Equity" {
                    continue;
                }
                let currency = rec.get(4).unwrap_or("USD").trim().to_ascii_uppercase();
                let symbol = rec.get(5).unwrap_or("").trim().to_ascii_uppercase();
                let (date_time_str, col_offset) =
                    if rec.get(7).map(|s| s.contains(':')).unwrap_or(false) {
                        (rec.get(6).unwrap_or("").trim(), 1)
                    } else {
                        (rec.get(6).unwrap_or("").trim(), 0)
                    };
                let date_part = date_time_str.split([',', ' ']).next().unwrap_or("");

                let activity_date = NaiveDate::parse_from_str(date_part, "%Y-%m-%d")
                    .or_else(|_| NaiveDate::parse_from_str(date_part, "%Y%m%d"))
                    .map_err(|e| {
                        AppError::Validation(format!(
                            "IBKR trade row {}: invalid date '{date_part}': {e}",
                            line + 1
                        ))
                    })?;

                let qty_raw = rec.get(7 + col_offset).unwrap_or("0").replace(',', "");
                let qty_dec = Decimal::from_str(&qty_raw).unwrap_or(Decimal::ZERO);

                let activity_type = if qty_dec < Decimal::ZERO {
                    ActivityType::Sell
                } else {
                    ActivityType::Buy
                };
                let quantity = Some(qty_dec.abs());

                let price_raw = rec.get(8 + col_offset).unwrap_or("0").replace(',', "");
                let unit_price = Decimal::from_str(&price_raw).ok();

                let amount_raw = rec.get(9 + col_offset).unwrap_or("0").replace(',', "");
                let amount = Decimal::from_str(&amount_raw).ok().map(|d| d.abs());

                let fee_raw = rec.get(10 + col_offset).unwrap_or("0").replace(',', "");
                let fee = Decimal::from_str(&fee_raw).ok().map(|d| d.abs());

                rows.push(ParsedActivityRow {
                    activity_date,
                    settlement_date: None,
                    activity_type,
                    symbol: if symbol.is_empty() {
                        None
                    } else {
                        Some(symbol)
                    },
                    quantity,
                    unit_price,
                    amount,
                    fee,
                    tax: None,
                    currency: if currency.is_empty() {
                        "USD".to_string()
                    } else {
                        currency
                    },
                    fx_rate: None,
                    notes: Some("IBKR Activity Statement Trade".to_string()),
                });
            }
        }

        // If no IBKR Trade rows matched, return an explicit error
        if rows.is_empty() {
            return Err(AppError::Validation(
                "No 'Trades' section found in IBKR CSV statement".to_string(),
            ));
        }

        Ok(rows)
    }
}

pub struct ParsedActivityRow {
    pub activity_date: NaiveDate,
    pub settlement_date: Option<NaiveDate>,
    pub activity_type: ActivityType,
    pub symbol: Option<String>,
    pub quantity: Option<Decimal>,
    pub unit_price: Option<Decimal>,
    pub amount: Option<Decimal>,
    pub fee: Option<Decimal>,
    pub tax: Option<Decimal>,
    pub currency: String,
    pub fx_rate: Option<Decimal>,
    pub notes: Option<String>,
}
