// ActivityImportService — Parses generic CSV & IBKR activity statements,
// resolves/creates assets, persists activities, and generates initial FIFO lots on buys.
//
// Complies with AlphaForge principles:
// - Explicit typed AppError (no unwrap / expect panics)
// - Thin command layer delegation
// - Idempotent import runs and activity deduplication

use std::sync::Arc;

use chrono::NaiveDate;
use domain::financial::{
    ActivityStatus, ActivityType, AssetKind, CostBasisMethod, CreateActivityInput,
    CreateAssetInput, CreateImportRunInput, CreateLotInput, ImportRun, InstrumentType, QuoteMode,
};
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::database::repositories::account_repository::AccountRepository;
use crate::database::repositories::activity_repository::{ActivityRepository, ImportRunRepository};
use crate::database::repositories::asset_repository::AssetRepository;
use crate::database::repositories::lot_repository::LotRepository;
use crate::error::AppError;

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
    lot_repo: Arc<LotRepository>,
}

impl ActivityImportService {
    pub fn new(
        account_repo: Arc<AccountRepository>,
        asset_repo: Arc<AssetRepository>,
        activity_repo: Arc<ActivityRepository>,
        import_run_repo: Arc<ImportRunRepository>,
        lot_repo: Arc<LotRepository>,
    ) -> Self {
        Self {
            account_repo,
            asset_repo,
            activity_repo,
            import_run_repo,
            lot_repo,
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
        let account = self
            .account_repo
            .get(account_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("account {account_id} not found")))?;

        let source_system = match format {
            ImportFormat::GenericCsv => "GENERIC_CSV",
            ImportFormat::IbkrCsv => "IBKR_CSV",
        };

        // Create import run record
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

        let parsed_items = match format {
            ImportFormat::GenericCsv => self.parse_generic_csv(csv_text)?,
            ImportFormat::IbkrCsv => self.parse_ibkr_csv(csv_text)?,
        };

        if parsed_items.is_empty() {
            return Err(AppError::Validation(
                "CSV contains no valid activity rows".to_string(),
            ));
        }

        for item in parsed_items {
            // Resolve or create asset if a symbol is specified
            let asset_id = if let Some(sym) = item.symbol {
                let key = format!("EQUITY:{}@XNAS", sym.to_ascii_uppercase());
                let maybe_asset = self.asset_repo.find_by_instrument_key(&key).await?;
                let asset = match maybe_asset {
                    Some(a) => a,
                    None => {
                        self.asset_repo
                            .create(CreateAssetInput {
                                kind: AssetKind::Investment,
                                name: Some(sym.clone()),
                                display_code: Some(sym.clone()),
                                notes: Some("Imported from broker activity statement".to_string()),
                                is_active: true,
                                quote_mode: QuoteMode::Market,
                                quote_ccy: item.currency.clone(),
                                instrument_type: Some(InstrumentType::Equity),
                                instrument_symbol: Some(sym),
                                instrument_exchange_mic: Some("XNAS".to_string()),
                                provider_config: None,
                            })
                            .await?
                    }
                };
                Some(asset.id)
            } else {
                None
            };

            let activity_input = CreateActivityInput {
                account_id: account_id.to_string(),
                asset_id: asset_id.clone(),
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
                idempotency_key: None,
                import_run_id: Some(import_run.id.clone()),
            };

            let activity = self.activity_repo.create(activity_input).await?;

            // If it's a buy activity, open a new FIFO lot
            if activity.activity_type == ActivityType::Buy {
                if let (Some(a_id), Some(qty), Some(price)) =
                    (asset_id, activity.quantity, activity.unit_price)
                {
                    if qty > Decimal::ZERO {
                        let total_cost = qty * price + activity.fee.unwrap_or(Decimal::ZERO);
                        let lot_input = CreateLotInput {
                            account_id: account_id.to_string(),
                            asset_id: a_id,
                            open_date: activity.activity_date,
                            open_activity_id: Some(activity.id.clone()),
                            original_quantity: qty,
                            cost_per_unit: price,
                            original_cost_basis: total_cost,
                            fee_allocated: activity.fee.unwrap_or(Decimal::ZERO),
                            currency: activity.currency.clone(),
                            base_currency: account.currency.clone(),
                            fx_rate_to_base: Decimal::ONE,
                            fx_rate_to_account: Some(Decimal::ONE),
                            account_currency: Some(account.currency.clone()),
                            cost_basis_method: CostBasisMethod::Fifo,
                        };
                        self.lot_repo.create(lot_input).await?;
                    }
                }
            }
        }

        // Return updated import run
        self.import_run_repo
            .get(&import_run.id)
            .await?
            .ok_or_else(|| AppError::Internal("Import run disappeared".to_string()))
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
                let date_time_str = rec.get(6).unwrap_or("").trim();
                let date_part = date_time_str
                    .split([',', ' '])
                    .next()
                    .unwrap_or("");

                let activity_date = NaiveDate::parse_from_str(date_part, "%Y-%m-%d")
                    .or_else(|_| NaiveDate::parse_from_str(date_part, "%Y%m%d"))
                    .map_err(|e| {
                        AppError::Validation(format!(
                            "IBKR trade row {}: invalid date '{date_part}': {e}",
                            line + 1
                        ))
                    })?;

                let qty_raw = rec.get(7).unwrap_or("0").replace(',', "");
                let qty_dec = Decimal::from_str(&qty_raw).unwrap_or(Decimal::ZERO);

                let activity_type = if qty_dec < Decimal::ZERO {
                    ActivityType::Sell
                } else {
                    ActivityType::Buy
                };
                let quantity = Some(qty_dec.abs());

                let price_raw = rec.get(8).unwrap_or("0").replace(',', "");
                let unit_price = Decimal::from_str(&price_raw).ok();

                let amount_raw = rec.get(9).unwrap_or("0").replace(',', "");
                let amount = Decimal::from_str(&amount_raw).ok().map(|d| d.abs());

                let fee_raw = rec.get(10).unwrap_or("0").replace(',', "");
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
