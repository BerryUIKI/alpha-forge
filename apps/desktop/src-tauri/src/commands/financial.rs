// Financial Tauri commands — Phase 2 Wealthfolio port.
//
// Exposes the Phase 2 financial services (holdings, lots, valuation,
// performance, allocation, snapshots, net worth) to the React frontend
// via thin Tauri command wrappers.

use chrono::NaiveDate;
use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use domain::financial::{
    AllocationBreakdown, DailyAccountValuation, FifoReductionResult, HoldingSnapshot,
    HoldingsSummary, Lot, NetWorthSnapshot, PerformancePoint, PerformanceSummary, ScopeType,
};

// ── Holdings ────────────────────────────────────────────────────────────────

/// Get current holdings for a single account.
#[tauri::command]
pub async fn get_holdings(
    account_id: String,
    as_of_date: String,
    state: State<'_, AppState>,
) -> Result<HoldingsSummary, AppError> {
    let date = NaiveDate::parse_from_str(&as_of_date, "%Y-%m-%d")
        .map_err(|e| AppError::Validation(format!("invalid date '{as_of_date}': {e}")))?;
    state.holdings_service.get_holdings(&account_id, date).await
}

/// Get holdings for all non-archived accounts.
#[tauri::command]
pub async fn get_all_holdings(
    as_of_date: String,
    state: State<'_, AppState>,
) -> Result<Vec<HoldingsSummary>, AppError> {
    let date = NaiveDate::parse_from_str(&as_of_date, "%Y-%m-%d")
        .map_err(|e| AppError::Validation(format!("invalid date '{as_of_date}': {e}")))?;
    state.holdings_service.get_all_holdings(date).await
}

// ── Lots ────────────────────────────────────────────────────────────────────

/// Record a sell activity against the FIFO lot inventory.
#[tauri::command]
pub async fn record_sell(
    account_id: String,
    asset_id: String,
    activity_id: String,
    state: State<'_, AppState>,
) -> Result<FifoReductionResult, AppError> {
    state
        .lot_service
        .record_sell(&account_id, &asset_id, &activity_id)
        .await
}

/// Get open lots for an account + asset combination.
#[tauri::command]
pub async fn get_open_lots(
    account_id: String,
    asset_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Lot>, AppError> {
    state
        .lot_service
        .get_open_lots(&account_id, &asset_id)
        .await
}

/// Get all open lots for an account.
#[tauri::command]
pub async fn get_open_lots_for_account(
    account_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Lot>, AppError> {
    state
        .lot_service
        .get_open_lots_for_account(&account_id)
        .await
}

// ── Valuation ───────────────────────────────────────────────────────────────

/// Calculate and persist one day's valuation for an account.
#[tauri::command]
pub async fn calculate_valuation_day(
    account_id: String,
    date: String,
    state: State<'_, AppState>,
) -> Result<DailyAccountValuation, AppError> {
    let parsed = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|e| AppError::Validation(format!("invalid date '{date}': {e}")))?;
    state
        .valuation_service
        .calculate_day(&account_id, parsed)
        .await
}

/// Get a single valuation row.
#[tauri::command]
pub async fn get_valuation(
    account_id: String,
    date: String,
    state: State<'_, AppState>,
) -> Result<Option<DailyAccountValuation>, AppError> {
    state
        .valuation_service
        .get_valuation(&account_id, &date)
        .await
}

/// Get the full valuation series for an account.
#[tauri::command]
pub async fn get_valuation_series(
    account_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<DailyAccountValuation>, AppError> {
    state
        .valuation_service
        .get_valuation_series(&account_id)
        .await
}

/// Calculate and persist valuations for all active accounts on a date.
#[tauri::command]
pub async fn calculate_all_valuations(
    date: String,
    state: State<'_, AppState>,
) -> Result<Vec<DailyAccountValuation>, AppError> {
    let parsed = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|e| AppError::Validation(format!("invalid date '{date}': {e}")))?;
    state.valuation_service.calculate_all(parsed).await
}

// ── Performance ─────────────────────────────────────────────────────────────

/// Compute performance summary (XIRR, TWR) for an account.
#[tauri::command]
pub async fn compute_performance_summary(
    account_id: String,
    start_date: String,
    end_date: String,
    state: State<'_, AppState>,
) -> Result<PerformanceSummary, AppError> {
    let start = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d")
        .map_err(|e| AppError::Validation(format!("invalid start_date '{start_date}': {e}")))?;
    let end = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d")
        .map_err(|e| AppError::Validation(format!("invalid end_date '{end_date}': {e}")))?;
    state
        .performance_service
        .compute_summary(&account_id, start, end)
        .await
}

/// Get the performance time-series for an account.
#[tauri::command]
pub async fn get_performance_time_series(
    account_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<PerformancePoint>, AppError> {
    state.performance_service.get_time_series(&account_id).await
}

// ── Allocation ──────────────────────────────────────────────────────────────

/// Compute allocation breakdown for a scope.
#[tauri::command]
pub async fn get_allocation(
    scope_type: String,
    scope_id: Option<String>,
    as_of_date: String,
    state: State<'_, AppState>,
) -> Result<AllocationBreakdown, AppError> {
    let scope = ScopeType::parse(&scope_type)
        .ok_or_else(|| AppError::Validation(format!("invalid scope_type '{scope_type}'")))?;
    let date = NaiveDate::parse_from_str(&as_of_date, "%Y-%m-%d")
        .map_err(|e| AppError::Validation(format!("invalid date '{as_of_date}': {e}")))?;
    state
        .allocation_service
        .get_allocation(scope, scope_id.as_deref(), date)
        .await
}

/// Check constraints that apply to a scope.
#[tauri::command]
pub async fn check_allocation_constraints(
    scope_type: String,
    scope_id: Option<String>,
    as_of_date: String,
    state: State<'_, AppState>,
) -> Result<Vec<String>, AppError> {
    let scope = ScopeType::parse(&scope_type)
        .ok_or_else(|| AppError::Validation(format!("invalid scope_type '{scope_type}'")))?;
    let date = NaiveDate::parse_from_str(&as_of_date, "%Y-%m-%d")
        .map_err(|e| AppError::Validation(format!("invalid date '{as_of_date}': {e}")))?;
    state
        .allocation_service
        .check_constraints(scope, scope_id.as_deref(), date)
        .await
}

// ── Snapshots ───────────────────────────────────────────────────────────────

/// Create a snapshot from the current holdings of an account.
#[tauri::command]
pub async fn create_snapshot(
    account_id: String,
    snapshot_date: String,
    label: Option<String>,
    state: State<'_, AppState>,
) -> Result<HoldingSnapshot, AppError> {
    let date = NaiveDate::parse_from_str(&snapshot_date, "%Y-%m-%d")
        .map_err(|e| AppError::Validation(format!("invalid date '{snapshot_date}': {e}")))?;
    state
        .snapshot_service
        .create_snapshot(&account_id, date, label.as_deref())
        .await
}

/// Get a snapshot by ID.
#[tauri::command]
pub async fn get_snapshot(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<HoldingSnapshot>, AppError> {
    state.snapshot_service.get_snapshot(&id).await
}

/// List snapshots for an account.
#[tauri::command]
pub async fn list_snapshots(
    account_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<HoldingSnapshot>, AppError> {
    state.snapshot_service.list_snapshots(&account_id).await
}

/// Delete a snapshot.
#[tauri::command]
pub async fn delete_snapshot(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    state.snapshot_service.delete_snapshot(&id).await
}

// ── Net Worth ───────────────────────────────────────────────────────────────

/// Compute net worth as of a given date.
#[tauri::command]
pub async fn compute_net_worth(
    as_of_date: String,
    base_currency: Option<String>,
    state: State<'_, AppState>,
) -> Result<NetWorthSnapshot, AppError> {
    let date = NaiveDate::parse_from_str(&as_of_date, "%Y-%m-%d")
        .map_err(|e| AppError::Validation(format!("invalid date '{as_of_date}': {e}")))?;
    let currency = base_currency.unwrap_or_else(|| "USD".to_string());
    state
        .net_worth_service
        .compute_net_worth(date, &currency)
        .await
}
