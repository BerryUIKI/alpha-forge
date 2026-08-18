// Financial CRUD Tauri commands — Phase 3.5.
//
// Exposes the repository-level CRUD operations (platform, account, asset,
// quote, activity, import_run, lot, valuation, taxonomy, allocation_target)
// as Tauri commands. Follows the thin-command pattern: validate → delegate
// to repository → map result.
//
// Complex create commands accept a single `input: CreateXInput` struct.
// Simple read commands use flat string/number parameters.

use chrono::NaiveDate;
use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use domain::financial::{
    Activity, AllocationTarget, AllocationTargetConstraint, AllocationTargetConstraintInput,
    AllocationTargetWeight, AllocationTargetWeightInput, Asset, AssetTaxonomyAssignment,
    AssetTaxonomyAssignmentInput, CreateAccountInput, CreateActivityInput,
    CreateAllocationTargetInput, CreateAssetInput, CreateImportRunInput, CreateLotInput,
    CreatePlatformInput, CreateTaxonomyCategoryInput, CreateTaxonomyInput, DailyAccountValuation,
    FinancialAccount, ImportRun, Lot, Platform, Quote, Taxonomy, TaxonomyCategory,
    UpsertQuoteInput, UpsertValuationInput,
};

// ── Platform ────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_platform(
    input: CreatePlatformInput,
    state: State<'_, AppState>,
) -> Result<Platform, AppError> {
    state.platform_repo.create(input).await
}

#[tauri::command]
pub async fn list_platforms(state: State<'_, AppState>) -> Result<Vec<Platform>, AppError> {
    state.platform_repo.list().await
}

#[tauri::command]
pub async fn get_platform(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<Platform>, AppError> {
    state.platform_repo.get(&id).await
}

// ── Financial Account ───────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_financial_account(
    input: CreateAccountInput,
    state: State<'_, AppState>,
) -> Result<FinancialAccount, AppError> {
    state.account_repo.create(input).await
}

#[tauri::command]
pub async fn list_financial_accounts(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<FinancialAccount>, AppError> {
    state.account_repo.list_by_workspace(&workspace_id).await
}

#[tauri::command]
pub async fn list_all_financial_accounts(
    state: State<'_, AppState>,
) -> Result<Vec<FinancialAccount>, AppError> {
    state.account_repo.list().await
}

#[tauri::command]
pub async fn get_financial_account(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<FinancialAccount>, AppError> {
    state.account_repo.get(&id).await
}

#[tauri::command]
pub async fn archive_financial_account(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.account_repo.archive(&id).await
}

// ── Asset ───────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_asset(
    input: CreateAssetInput,
    state: State<'_, AppState>,
) -> Result<Asset, AppError> {
    state.asset_repo.create(input).await
}

#[tauri::command]
pub async fn get_asset(id: String, state: State<'_, AppState>) -> Result<Option<Asset>, AppError> {
    state.asset_repo.get(&id).await
}

#[tauri::command]
pub async fn find_asset_by_instrument_key(
    key: String,
    state: State<'_, AppState>,
) -> Result<Option<Asset>, AppError> {
    state.asset_repo.find_by_instrument_key(&key).await
}

#[tauri::command]
pub async fn list_active_assets(state: State<'_, AppState>) -> Result<Vec<Asset>, AppError> {
    state.asset_repo.list_active().await
}

// ── Quote ───────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn upsert_quote(
    input: UpsertQuoteInput,
    state: State<'_, AppState>,
) -> Result<Quote, AppError> {
    state.quote_repo.upsert(input).await
}

#[tauri::command]
pub async fn get_quote_for_day(
    asset_id: String,
    date: String,
    source: String,
    state: State<'_, AppState>,
) -> Result<Option<Quote>, AppError> {
    let parsed = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|e| AppError::Validation(format!("invalid date '{date}': {e}")))?;
    state
        .quote_repo
        .get_for_day(&asset_id, &parsed, &source)
        .await
}

#[tauri::command]
pub async fn list_quotes_for_asset(
    asset_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Quote>, AppError> {
    state.quote_repo.list_for_asset(&asset_id).await
}

// ── Activity ────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_activity(
    input: CreateActivityInput,
    state: State<'_, AppState>,
) -> Result<Activity, AppError> {
    state.activity_repo.create(input).await
}

#[tauri::command]
pub async fn get_activity(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<Activity>, AppError> {
    state.activity_repo.get(&id).await
}

#[tauri::command]
pub async fn list_activities_by_account(
    account_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Activity>, AppError> {
    state.activity_repo.list_by_account(&account_id).await
}

#[tauri::command]
pub async fn list_activities_by_asset(
    asset_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Activity>, AppError> {
    state.activity_repo.list_by_asset(&asset_id).await
}

// ── Import Run ──────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_import_run(
    input: CreateImportRunInput,
    state: State<'_, AppState>,
) -> Result<ImportRun, AppError> {
    state.import_run_repo.create(input).await
}

#[tauri::command]
pub async fn list_import_runs(
    account_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ImportRun>, AppError> {
    state.import_run_repo.list_by_account(&account_id).await
}

// ── Lot ─────────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_lot(
    input: CreateLotInput,
    state: State<'_, AppState>,
) -> Result<Lot, AppError> {
    state.lot_repo.create(input).await
}

#[tauri::command]
pub async fn get_lot(id: String, state: State<'_, AppState>) -> Result<Option<Lot>, AppError> {
    state.lot_repo.get(&id).await
}

// ── Valuation ───────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn upsert_valuation(
    input: UpsertValuationInput,
    state: State<'_, AppState>,
) -> Result<DailyAccountValuation, AppError> {
    state.valuation_repo.upsert(input).await
}

#[tauri::command]
pub async fn list_valuations_by_account(
    account_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<DailyAccountValuation>, AppError> {
    state.valuation_repo.list_by_account(&account_id).await
}

#[tauri::command]
pub async fn delete_valuation_for_date(
    account_id: String,
    date: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state
        .valuation_repo
        .delete_for_date(&account_id, &date)
        .await
}

// ── Taxonomy ────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_taxonomy(
    input: CreateTaxonomyInput,
    state: State<'_, AppState>,
) -> Result<Taxonomy, AppError> {
    state.taxonomy_repo.create(input).await
}

#[tauri::command]
pub async fn get_taxonomy(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<Taxonomy>, AppError> {
    state.taxonomy_repo.get(&id).await
}

#[tauri::command]
pub async fn list_taxonomies(state: State<'_, AppState>) -> Result<Vec<Taxonomy>, AppError> {
    state.taxonomy_repo.list().await
}

#[tauri::command]
pub async fn create_taxonomy_category(
    input: CreateTaxonomyCategoryInput,
    state: State<'_, AppState>,
) -> Result<TaxonomyCategory, AppError> {
    state.taxonomy_repo.create_category(input).await
}

#[tauri::command]
pub async fn list_taxonomy_categories(
    taxonomy_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<TaxonomyCategory>, AppError> {
    state.taxonomy_repo.list_categories(&taxonomy_id).await
}

#[tauri::command]
pub async fn assign_asset_to_taxonomy_category(
    input: AssetTaxonomyAssignmentInput,
    state: State<'_, AppState>,
) -> Result<AssetTaxonomyAssignment, AppError> {
    state.taxonomy_repo.assign_asset(input).await
}

#[tauri::command]
pub async fn list_assignments_for_asset(
    asset_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<AssetTaxonomyAssignment>, AppError> {
    state
        .taxonomy_repo
        .list_assignments_for_asset(&asset_id)
        .await
}

#[tauri::command]
pub async fn list_assignments_by_taxonomy(
    taxonomy_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<AssetTaxonomyAssignment>, AppError> {
    state
        .taxonomy_repo
        .list_assignments_by_taxonomy(&taxonomy_id)
        .await
}

#[tauri::command]
pub async fn remove_taxonomy_assignment(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.taxonomy_repo.remove_assignment(&id).await
}

// ── Allocation Target ───────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_allocation_target(
    input: CreateAllocationTargetInput,
    state: State<'_, AppState>,
) -> Result<AllocationTarget, AppError> {
    state.target_repo.create(input).await
}

#[tauri::command]
pub async fn get_allocation_target(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<AllocationTarget>, AppError> {
    state.target_repo.get(&id).await
}

#[tauri::command]
pub async fn list_allocation_targets(
    include_archived: bool,
    state: State<'_, AppState>,
) -> Result<Vec<AllocationTarget>, AppError> {
    state.target_repo.list(include_archived).await
}

#[tauri::command]
pub async fn archive_allocation_target(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.target_repo.archive(&id).await
}

#[tauri::command]
pub async fn add_allocation_weight(
    input: AllocationTargetWeightInput,
    state: State<'_, AppState>,
) -> Result<AllocationTargetWeight, AppError> {
    state.target_repo.add_weight(input).await
}

#[tauri::command]
pub async fn list_allocation_weights(
    target_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<AllocationTargetWeight>, AppError> {
    state.target_repo.list_weights(&target_id).await
}

#[tauri::command]
pub async fn add_allocation_constraint(
    input: AllocationTargetConstraintInput,
    state: State<'_, AppState>,
) -> Result<AllocationTargetConstraint, AppError> {
    state.target_repo.add_constraint(input).await
}

#[tauri::command]
pub async fn list_allocation_constraints(
    target_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<AllocationTargetConstraint>, AppError> {
    state.target_repo.list_constraints(&target_id).await
}
