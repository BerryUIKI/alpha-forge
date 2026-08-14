// Financial domain models (Wealthfolio port, Phase 1 storage).
//
// Pure data structures with no I/O. Money and quantity values use
// `rust_decimal::Decimal` — never `f64` — matching the TEXT decimal columns
// in migrations 0015-0020. Date-only fields use `NaiveDate` (YYYY-MM-DD);
// timestamps use `DateTime<Utc>` (RFC3339).
//
// Each enum's `Display` writes the canonical database value and `parse`
// reads it back, so repositories can round-trip CHECK-constrained columns
// without ever inventing values the schema does not allow.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

// ────────────────────────────────────────────────────────────────────────────
// Enums (canonical values from migrations 0015-0020)
// ────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    Securities,
    Cash,
    CreditCard,
    Cryptocurrency,
}

impl AccountType {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "SECURITIES" => Some(Self::Securities),
            "CASH" => Some(Self::Cash),
            "CREDIT_CARD" => Some(Self::CreditCard),
            "CRYPTOCURRENCY" => Some(Self::Cryptocurrency),
            _ => None,
        }
    }
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Securities => write!(f, "SECURITIES"),
            Self::Cash => write!(f, "CASH"),
            Self::CreditCard => write!(f, "CREDIT_CARD"),
            Self::Cryptocurrency => write!(f, "CRYPTOCURRENCY"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrackingMode {
    NotSet,
    Transactions,
    Holdings,
}

impl TrackingMode {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "NOT_SET" => Some(Self::NotSet),
            "TRANSACTIONS" => Some(Self::Transactions),
            "HOLDINGS" => Some(Self::Holdings),
            _ => None,
        }
    }
}

impl std::fmt::Display for TrackingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotSet => write!(f, "NOT_SET"),
            Self::Transactions => write!(f, "TRANSACTIONS"),
            Self::Holdings => write!(f, "HOLDINGS"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetKind {
    Investment,
    Property,
    Vehicle,
    Collectible,
    PreciousMetal,
    PrivateEquity,
    Liability,
    Other,
    Fx,
}

impl AssetKind {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "INVESTMENT" => Some(Self::Investment),
            "PROPERTY" => Some(Self::Property),
            "VEHICLE" => Some(Self::Vehicle),
            "COLLECTIBLE" => Some(Self::Collectible),
            "PRECIOUS_METAL" => Some(Self::PreciousMetal),
            "PRIVATE_EQUITY" => Some(Self::PrivateEquity),
            "LIABILITY" => Some(Self::Liability),
            "OTHER" => Some(Self::Other),
            "FX" => Some(Self::Fx),
            _ => None,
        }
    }
}

impl std::fmt::Display for AssetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Investment => write!(f, "INVESTMENT"),
            Self::Property => write!(f, "PROPERTY"),
            Self::Vehicle => write!(f, "VEHICLE"),
            Self::Collectible => write!(f, "COLLECTIBLE"),
            Self::PreciousMetal => write!(f, "PRECIOUS_METAL"),
            Self::PrivateEquity => write!(f, "PRIVATE_EQUITY"),
            Self::Liability => write!(f, "LIABILITY"),
            Self::Other => write!(f, "OTHER"),
            Self::Fx => write!(f, "FX"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuoteMode {
    Market,
    Manual,
}

impl QuoteMode {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "MARKET" => Some(Self::Market),
            "MANUAL" => Some(Self::Manual),
            _ => None,
        }
    }
}

impl std::fmt::Display for QuoteMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Market => write!(f, "MARKET"),
            Self::Manual => write!(f, "MANUAL"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstrumentType {
    Equity,
    Crypto,
    Fx,
    Option,
    Metal,
}

impl InstrumentType {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "EQUITY" => Some(Self::Equity),
            "CRYPTO" => Some(Self::Crypto),
            "FX" => Some(Self::Fx),
            "OPTION" => Some(Self::Option),
            "METAL" => Some(Self::Metal),
            _ => None,
        }
    }
}

impl std::fmt::Display for InstrumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Equity => write!(f, "EQUITY"),
            Self::Crypto => write!(f, "CRYPTO"),
            Self::Fx => write!(f, "FX"),
            Self::Option => write!(f, "OPTION"),
            Self::Metal => write!(f, "METAL"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityType {
    Buy,
    Sell,
    Split,
    Dividend,
    Interest,
    Deposit,
    Withdrawal,
    TransferIn,
    TransferOut,
    Fee,
    Tax,
    Credit,
    Adjustment,
    Unknown,
}

impl ActivityType {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "BUY" => Some(Self::Buy),
            "SELL" => Some(Self::Sell),
            "SPLIT" => Some(Self::Split),
            "DIVIDEND" => Some(Self::Dividend),
            "INTEREST" => Some(Self::Interest),
            "DEPOSIT" => Some(Self::Deposit),
            "WITHDRAWAL" => Some(Self::Withdrawal),
            "TRANSFER_IN" => Some(Self::TransferIn),
            "TRANSFER_OUT" => Some(Self::TransferOut),
            "FEE" => Some(Self::Fee),
            "TAX" => Some(Self::Tax),
            "CREDIT" => Some(Self::Credit),
            "ADJUSTMENT" => Some(Self::Adjustment),
            "UNKNOWN" => Some(Self::Unknown),
            _ => None,
        }
    }
}

impl std::fmt::Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Buy => write!(f, "BUY"),
            Self::Sell => write!(f, "SELL"),
            Self::Split => write!(f, "SPLIT"),
            Self::Dividend => write!(f, "DIVIDEND"),
            Self::Interest => write!(f, "INTEREST"),
            Self::Deposit => write!(f, "DEPOSIT"),
            Self::Withdrawal => write!(f, "WITHDRAWAL"),
            Self::TransferIn => write!(f, "TRANSFER_IN"),
            Self::TransferOut => write!(f, "TRANSFER_OUT"),
            Self::Fee => write!(f, "FEE"),
            Self::Tax => write!(f, "TAX"),
            Self::Credit => write!(f, "CREDIT"),
            Self::Adjustment => write!(f, "ADJUSTMENT"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityStatus {
    Posted,
    Pending,
    Canceled,
}

impl ActivityStatus {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "POSTED" => Some(Self::Posted),
            "PENDING" => Some(Self::Pending),
            "CANCELED" => Some(Self::Canceled),
            _ => None,
        }
    }
}

impl std::fmt::Display for ActivityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Posted => write!(f, "POSTED"),
            Self::Pending => write!(f, "PENDING"),
            Self::Canceled => write!(f, "CANCELED"),
        }
    }
}

/// Valuation coverage quality (canonical; legacy codes are folded by the
/// domain layer on read).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValuationStatus {
    Complete,
    PartialUnpriced,
    Unavailable,
}

impl ValuationStatus {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "COMPLETE" => Some(Self::Complete),
            "PARTIAL_UNPRICED" => Some(Self::PartialUnpriced),
            "UNAVAILABLE" => Some(Self::Unavailable),
            _ => None,
        }
    }
}

impl std::fmt::Display for ValuationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Complete => write!(f, "COMPLETE"),
            Self::PartialUnpriced => write!(f, "PARTIAL_UNPRICED"),
            Self::Unavailable => write!(f, "UNAVAILABLE"),
        }
    }
}

/// Cost-basis coverage quality (canonical; legacy codes are folded by the
/// domain layer on read).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BasisStatus {
    Complete,
    PartialUnknown,
    Unknown,
    NotApplicable,
}

impl BasisStatus {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "COMPLETE" => Some(Self::Complete),
            "PARTIAL_UNKNOWN" => Some(Self::PartialUnknown),
            "UNKNOWN" => Some(Self::Unknown),
            "NOT_APPLICABLE" => Some(Self::NotApplicable),
            _ => None,
        }
    }
}

impl std::fmt::Display for BasisStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Complete => write!(f, "COMPLETE"),
            Self::PartialUnknown => write!(f, "PARTIAL_UNKNOWN"),
            Self::Unknown => write!(f, "UNKNOWN"),
            Self::NotApplicable => write!(f, "NOT_APPLICABLE"),
        }
    }
}

/// Provenance of the external-flow figures in a daily valuation row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalFlowSource {
    NoFlow,
    Unknown,
    CashAmount,
    QuoteDerivedMarketValue,
    CostBasisFallback,
    RemovedLotBasisFallback,
    LegacyActivityAmountFallback,
    UnknownBoundaryTransfer,
    UnpricedHoldingsTransition,
    ActivityDerived,
    StoredGross,
    NetContributionFallback,
    Mixed,
}

impl ExternalFlowSource {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "NO_FLOW" => Some(Self::NoFlow),
            "UNKNOWN" => Some(Self::Unknown),
            "CASH_AMOUNT" => Some(Self::CashAmount),
            "QUOTE_DERIVED_MARKET_VALUE" => Some(Self::QuoteDerivedMarketValue),
            "COST_BASIS_FALLBACK" => Some(Self::CostBasisFallback),
            "REMOVED_LOT_BASIS_FALLBACK" => Some(Self::RemovedLotBasisFallback),
            "LEGACY_ACTIVITY_AMOUNT_FALLBACK" => Some(Self::LegacyActivityAmountFallback),
            "UNKNOWN_BOUNDARY_TRANSFER" => Some(Self::UnknownBoundaryTransfer),
            "UNPRICED_HOLDINGS_TRANSITION" => Some(Self::UnpricedHoldingsTransition),
            "ACTIVITY_DERIVED" => Some(Self::ActivityDerived),
            "STORED_GROSS" => Some(Self::StoredGross),
            "NET_CONTRIBUTION_FALLBACK" => Some(Self::NetContributionFallback),
            "MIXED" => Some(Self::Mixed),
            _ => None,
        }
    }
}

impl std::fmt::Display for ExternalFlowSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoFlow => write!(f, "NO_FLOW"),
            Self::Unknown => write!(f, "UNKNOWN"),
            Self::CashAmount => write!(f, "CASH_AMOUNT"),
            Self::QuoteDerivedMarketValue => write!(f, "QUOTE_DERIVED_MARKET_VALUE"),
            Self::CostBasisFallback => write!(f, "COST_BASIS_FALLBACK"),
            Self::RemovedLotBasisFallback => write!(f, "REMOVED_LOT_BASIS_FALLBACK"),
            Self::LegacyActivityAmountFallback => write!(f, "LEGACY_ACTIVITY_AMOUNT_FALLBACK"),
            Self::UnknownBoundaryTransfer => write!(f, "UNKNOWN_BOUNDARY_TRANSFER"),
            Self::UnpricedHoldingsTransition => write!(f, "UNPRICED_HOLDINGS_TRANSITION"),
            Self::ActivityDerived => write!(f, "ACTIVITY_DERIVED"),
            Self::StoredGross => write!(f, "STORED_GROSS"),
            Self::NetContributionFallback => write!(f, "NET_CONTRIBUTION_FALLBACK"),
            Self::Mixed => write!(f, "MIXED"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldingSnapshotSource {
    Calculated,
    ManualEntry,
    CsvImport,
    BrokerImported,
    Synthetic,
}

impl HoldingSnapshotSource {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "CALCULATED" => Some(Self::Calculated),
            "MANUAL_ENTRY" => Some(Self::ManualEntry),
            "CSV_IMPORT" => Some(Self::CsvImport),
            "BROKER_IMPORTED" => Some(Self::BrokerImported),
            "SYNTHETIC" => Some(Self::Synthetic),
            _ => None,
        }
    }
}

impl std::fmt::Display for HoldingSnapshotSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Calculated => write!(f, "CALCULATED"),
            Self::ManualEntry => write!(f, "MANUAL_ENTRY"),
            Self::CsvImport => write!(f, "CSV_IMPORT"),
            Self::BrokerImported => write!(f, "BROKER_IMPORTED"),
            Self::Synthetic => write!(f, "SYNTHETIC"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CostBasisMethod {
    Fifo,
}

impl CostBasisMethod {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "FIFO" => Some(Self::Fifo),
            _ => None,
        }
    }
}

impl std::fmt::Display for CostBasisMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fifo => write!(f, "FIFO"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeType {
    All,
    Portfolio,
    Account,
}

impl ScopeType {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "all" => Some(Self::All),
            "portfolio" => Some(Self::Portfolio),
            "account" => Some(Self::Account),
            _ => None,
        }
    }
}

impl std::fmt::Display for ScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Portfolio => write!(f, "portfolio"),
            Self::Account => write!(f, "account"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintSubjectType {
    Asset,
    Account,
    Category,
}

impl ConstraintSubjectType {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "asset" => Some(Self::Asset),
            "account" => Some(Self::Account),
            "category" => Some(Self::Category),
            _ => None,
        }
    }
}

impl std::fmt::Display for ConstraintSubjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Asset => write!(f, "asset"),
            Self::Account => write!(f, "account"),
            Self::Category => write!(f, "category"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintAction {
    Buy,
    Sell,
    Trade,
}

impl ConstraintAction {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "buy" => Some(Self::Buy),
            "sell" => Some(Self::Sell),
            "trade" => Some(Self::Trade),
            _ => None,
        }
    }
}

impl std::fmt::Display for ConstraintAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Buy => write!(f, "buy"),
            Self::Sell => write!(f, "sell"),
            Self::Trade => write!(f, "trade"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintEffect {
    Block,
    Avoid,
}

impl ConstraintEffect {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "block" => Some(Self::Block),
            "avoid" => Some(Self::Avoid),
            _ => None,
        }
    }
}

impl std::fmt::Display for ConstraintEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Block => write!(f, "block"),
            Self::Avoid => write!(f, "avoid"),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Models
// ────────────────────────────────────────────────────────────────────────────

/// A brokerage / custodian that hosts one or more accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Platform {
    pub id: String,
    pub name: Option<String>,
    pub url: String,
    pub external_id: Option<String>,
    pub kind: String,
    pub website_url: Option<String>,
    pub logo_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A financial account scoped to an optional research workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialAccount {
    pub id: String,
    pub workspace_id: Option<String>,
    pub name: String,
    pub account_type: AccountType,
    pub group_name: Option<String>,
    pub currency: String,
    pub is_default: bool,
    pub is_active: bool,
    pub platform_id: Option<String>,
    pub account_number: Option<String>,
    pub meta: Option<serde_json::Value>,
    pub provider: Option<String>,
    pub provider_account_id: Option<String>,
    pub is_archived: bool,
    pub tracking_mode: TrackingMode,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A tradeable or trackable instrument (equity, crypto, FX, metal, cash...).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub kind: AssetKind,
    pub name: Option<String>,
    pub display_code: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub is_active: bool,
    pub quote_mode: QuoteMode,
    pub quote_ccy: String,
    pub instrument_type: Option<InstrumentType>,
    pub instrument_symbol: Option<String>,
    pub instrument_exchange_mic: Option<String>,
    /// Derived by the storage engine; read-only in the domain model.
    pub instrument_key: Option<String>,
    pub provider_config: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// One price point for an asset on a day from a source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub id: String,
    pub asset_id: String,
    pub day: NaiveDate,
    pub source: String,
    pub open: Option<Decimal>,
    pub high: Option<Decimal>,
    pub low: Option<Decimal>,
    pub close: Decimal,
    pub adjclose: Option<Decimal>,
    pub volume: Option<Decimal>,
    pub currency: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub timestamp: DateTime<Utc>,
}

/// A batch import of activities from a CSV file or broker sync.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportRun {
    pub id: String,
    pub account_id: String,
    pub source_system: String,
    pub run_type: String,
    pub mode: String,
    pub status: String,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub review_mode: String,
    pub applied_at: Option<String>,
    pub checkpoint_in: Option<String>,
    pub checkpoint_out: Option<String>,
    pub summary: Option<String>,
    pub warnings: Option<String>,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A single transaction / cash movement in the canonical ledger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub account_id: String,
    pub asset_id: Option<String>,
    pub activity_type: ActivityType,
    pub activity_type_override: Option<String>,
    pub source_type: Option<String>,
    pub subtype: Option<String>,
    pub status: ActivityStatus,
    pub activity_date: NaiveDate,
    pub settlement_date: Option<NaiveDate>,
    pub quantity: Option<Decimal>,
    pub unit_price: Option<Decimal>,
    pub amount: Option<Decimal>,
    pub fee: Option<Decimal>,
    pub tax: Option<Decimal>,
    pub currency: String,
    pub fx_rate: Option<Decimal>,
    pub notes: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub source_system: Option<String>,
    pub source_record_id: Option<String>,
    pub source_group_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub import_run_id: Option<String>,
    pub is_user_modified: bool,
    pub needs_review: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// An open tax lot — the FIFO cost-basis inventory for one asset in one account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lot {
    pub id: String,
    pub account_id: String,
    pub asset_id: String,
    pub open_date: NaiveDate,
    pub open_activity_id: Option<String>,
    pub original_quantity: Decimal,
    pub cost_per_unit: Decimal,
    pub original_cost_basis: Decimal,
    pub remaining_cost_basis: Decimal,
    pub fee_allocated: Decimal,
    pub tax_allocated: Decimal,
    pub currency: String,
    pub base_currency: String,
    pub fx_rate_to_base: Decimal,
    pub fx_rate_to_account: Option<Decimal>,
    pub account_currency: Option<String>,
    pub cost_basis_method: CostBasisMethod,
    pub remaining_quantity: Decimal,
    pub split_ratio: Decimal,
    pub is_closed: bool,
    pub close_date: Option<NaiveDate>,
    pub close_activity_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A realized disposal of part of a lot (one sell activity, one lot).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LotDisposal {
    pub id: String,
    pub lot_id: String,
    pub account_id: String,
    pub asset_id: String,
    pub disposal_activity_id: String,
    pub disposal_date: NaiveDate,
    pub quantity: Decimal,
    pub proceeds: Decimal,
    pub cost_basis: Decimal,
    pub realized_pnl: Decimal,
    pub proceeds_base: Decimal,
    pub cost_basis_base: Decimal,
    pub realized_pnl_base: Decimal,
    pub currency: String,
    pub base_currency: String,
    pub fx_rate_to_base: Decimal,
    pub cost_basis_method: CostBasisMethod,
    pub created_at: DateTime<Utc>,
}

/// A point-in-time position record for an account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldingSnapshot {
    pub id: String,
    pub account_id: String,
    pub snapshot_date: NaiveDate,
    pub currency: String,
    /// Relational per-snapshot positions (also mirrored in the `positions` JSON).
    pub positions: Vec<SnapshotPosition>,
    pub cash_balances: serde_json::Value,
    pub cost_basis: Decimal,
    pub net_contribution: Decimal,
    pub net_contribution_base: Decimal,
    pub cash_total_account_currency: Decimal,
    pub cash_total_base_currency: Decimal,
    pub source: HoldingSnapshotSource,
    pub calculated_at: DateTime<Utc>,
}

/// One holding inside a snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotPosition {
    pub id: i64,
    pub snapshot_id: String,
    pub asset_id: String,
    pub quantity: Decimal,
    pub average_cost: Decimal,
    pub total_cost_basis: Decimal,
    pub currency: String,
    pub contract_multiplier: Decimal,
    pub inception_date: NaiveDate,
    pub is_alternative: bool,
    pub cost_basis_base: Option<Decimal>,
    pub cost_basis_account: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

/// Derived daily valuation of one account (performance series read model).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyAccountValuation {
    pub id: String,
    pub account_id: String,
    pub valuation_date: NaiveDate,
    pub account_currency: String,
    pub base_currency: String,
    pub fx_rate_to_base: Decimal,
    pub cash_balance: Decimal,
    pub investment_market_value: Decimal,
    pub total_value: Decimal,
    pub cost_basis: Decimal,
    pub net_contribution: Decimal,
    pub cash_balance_base: Decimal,
    pub investment_market_value_base: Decimal,
    pub total_value_base: Decimal,
    pub cost_basis_base: Decimal,
    pub net_contribution_base: Decimal,
    pub external_inflow_base: Decimal,
    pub external_outflow_base: Decimal,
    pub performance_eligible_value_base: Decimal,
    pub external_flow_source: ExternalFlowSource,
    pub value_status: ValuationStatus,
    pub basis_status: BasisStatus,
    pub calculated_at: DateTime<Utc>,
}

/// A classification taxonomy (e.g. asset_classes, instrument_type).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Taxonomy {
    pub id: String,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
    pub is_system: bool,
    pub is_single_select: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A category within a taxonomy (hierarchical via parent_id).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyCategory {
    pub id: String,
    pub taxonomy_id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub key: String,
    pub color: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A weighted assignment of an asset to a taxonomy category (0..10000 bps).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetTaxonomyAssignment {
    pub id: String,
    pub asset_id: String,
    pub taxonomy_id: String,
    pub category_id: String,
    pub weight: i32,
    pub source: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A rebalancing target expressed against one taxonomy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationTarget {
    pub id: String,
    pub name: String,
    pub scope_type: ScopeType,
    pub scope_id: Option<String>,
    pub taxonomy_id: String,
    pub trigger_type: String,
    pub drift_band_bps: i32,
    pub rebalance_goal: String,
    pub min_trade_amount: Decimal,
    pub whole_shares_only: bool,
    pub allow_sells: bool,
    pub max_turnover_bps: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub archived_at: Option<String>,
}

/// One category weight of an allocation target (0..10000 bps).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationTargetWeight {
    pub id: String,
    pub target_id: String,
    pub taxonomy_id: String,
    pub category_id: String,
    pub target_bps: i32,
    pub is_locked: bool,
    pub is_required: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A buy/sell/trade rule that constrains rebalancing for a target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationTargetConstraint {
    pub id: String,
    pub target_id: String,
    pub subject_type: ConstraintSubjectType,
    pub subject_id: String,
    pub action: ConstraintAction,
    pub effect: ConstraintEffect,
    pub reason: Option<String>,
    pub metadata_json: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ────────────────────────────────────────────────────────────────────────────
// Create inputs (domain-layer contracts consumed by the financial repositories)
// ────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlatformInput {
    pub name: Option<String>,
    pub url: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountInput {
    pub workspace_id: Option<String>,
    pub name: String,
    pub account_type: AccountType,
    pub group_name: Option<String>,
    pub currency: String,
    pub is_default: bool,
    pub platform_id: Option<String>,
    pub account_number: Option<String>,
    pub tracking_mode: TrackingMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAssetInput {
    pub kind: AssetKind,
    pub name: Option<String>,
    pub display_code: Option<String>,
    pub notes: Option<String>,
    pub is_active: bool,
    pub quote_mode: QuoteMode,
    pub quote_ccy: String,
    pub instrument_type: Option<InstrumentType>,
    pub instrument_symbol: Option<String>,
    pub instrument_exchange_mic: Option<String>,
    pub provider_config: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertQuoteInput {
    pub asset_id: String,
    pub day: NaiveDate,
    pub source: String,
    pub open: Option<Decimal>,
    pub high: Option<Decimal>,
    pub low: Option<Decimal>,
    pub close: Decimal,
    pub adjclose: Option<Decimal>,
    pub volume: Option<Decimal>,
    pub currency: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImportRunInput {
    pub account_id: String,
    pub source_system: String,
    pub run_type: String,
    pub mode: String,
    pub status: String,
    pub review_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateActivityInput {
    pub account_id: String,
    pub asset_id: Option<String>,
    pub activity_type: ActivityType,
    pub activity_type_override: Option<String>,
    pub source_type: Option<String>,
    pub subtype: Option<String>,
    pub status: ActivityStatus,
    pub activity_date: NaiveDate,
    pub settlement_date: Option<NaiveDate>,
    pub quantity: Option<Decimal>,
    pub unit_price: Option<Decimal>,
    pub amount: Option<Decimal>,
    pub fee: Option<Decimal>,
    pub tax: Option<Decimal>,
    pub currency: String,
    pub fx_rate: Option<Decimal>,
    pub notes: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub source_system: Option<String>,
    pub source_record_id: Option<String>,
    pub source_group_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub import_run_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLotInput {
    pub account_id: String,
    pub asset_id: String,
    pub open_date: NaiveDate,
    pub open_activity_id: Option<String>,
    pub original_quantity: Decimal,
    pub cost_per_unit: Decimal,
    /// Cost basis at acquisition (business-computed; the repository mirrors it
    /// into `remaining_cost_basis` for a freshly opened lot).
    pub original_cost_basis: Decimal,
    pub fee_allocated: Decimal,
    pub currency: String,
    pub base_currency: String,
    pub fx_rate_to_base: Decimal,
    pub fx_rate_to_account: Option<Decimal>,
    pub account_currency: Option<String>,
    pub cost_basis_method: CostBasisMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLotDisposalInput {
    pub lot_id: String,
    pub account_id: String,
    pub asset_id: String,
    pub disposal_activity_id: String,
    pub disposal_date: NaiveDate,
    pub quantity: Decimal,
    pub proceeds: Decimal,
    pub cost_basis: Decimal,
    pub realized_pnl: Decimal,
    pub proceeds_base: Decimal,
    pub cost_basis_base: Decimal,
    pub realized_pnl_base: Decimal,
    pub currency: String,
    pub base_currency: String,
    pub fx_rate_to_base: Decimal,
    pub cost_basis_method: CostBasisMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotPositionInput {
    pub asset_id: String,
    pub quantity: Decimal,
    pub average_cost: Decimal,
    pub total_cost_basis: Decimal,
    pub currency: String,
    pub contract_multiplier: Decimal,
    pub inception_date: NaiveDate,
    pub is_alternative: bool,
    pub cost_basis_base: Option<Decimal>,
    pub cost_basis_account: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSnapshotInput {
    pub account_id: String,
    pub snapshot_date: NaiveDate,
    pub currency: String,
    pub positions: Vec<SnapshotPositionInput>,
    pub cash_balances: serde_json::Value,
    pub cost_basis: Decimal,
    pub net_contribution: Decimal,
    pub net_contribution_base: Decimal,
    pub cash_total_account_currency: Decimal,
    pub cash_total_base_currency: Decimal,
    pub source: HoldingSnapshotSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertValuationInput {
    pub account_id: String,
    pub valuation_date: NaiveDate,
    pub account_currency: String,
    pub base_currency: String,
    pub fx_rate_to_base: Decimal,
    pub cash_balance: Decimal,
    pub investment_market_value: Decimal,
    pub total_value: Decimal,
    pub cost_basis: Decimal,
    pub net_contribution: Decimal,
    pub cash_balance_base: Decimal,
    pub investment_market_value_base: Decimal,
    pub total_value_base: Decimal,
    pub cost_basis_base: Decimal,
    pub net_contribution_base: Decimal,
    pub external_inflow_base: Decimal,
    pub external_outflow_base: Decimal,
    pub performance_eligible_value_base: Decimal,
    pub external_flow_source: ExternalFlowSource,
    pub value_status: ValuationStatus,
    pub basis_status: BasisStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaxonomyInput {
    pub name: String,
    pub color: String,
    pub description: Option<String>,
    pub is_system: bool,
    pub is_single_select: bool,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaxonomyCategoryInput {
    pub taxonomy_id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub key: String,
    pub color: String,
    pub description: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetTaxonomyAssignmentInput {
    pub asset_id: String,
    pub taxonomy_id: String,
    pub category_id: String,
    pub weight: i32,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAllocationTargetInput {
    pub name: String,
    pub scope_type: ScopeType,
    pub scope_id: Option<String>,
    pub taxonomy_id: String,
    pub trigger_type: String,
    pub drift_band_bps: i32,
    pub rebalance_goal: String,
    pub min_trade_amount: Decimal,
    pub whole_shares_only: bool,
    pub allow_sells: bool,
    pub max_turnover_bps: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationTargetWeightInput {
    pub target_id: String,
    pub taxonomy_id: String,
    pub category_id: String,
    pub target_bps: i32,
    pub is_locked: bool,
    pub is_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationTargetConstraintInput {
    pub target_id: String,
    pub subject_type: ConstraintSubjectType,
    pub subject_id: String,
    pub action: ConstraintAction,
    pub effect: ConstraintEffect,
    pub reason: Option<String>,
    pub metadata_json: Option<serde_json::Value>,
}
