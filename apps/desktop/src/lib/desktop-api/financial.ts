/**
 * Financial desktop API client — Phase 2/3 Wealthfolio port.
 *
 * Wraps the 18 Phase 2 Tauri commands (commands/financial.rs) with camelCase
 * function names and typed return values. Follows the existing pattern in
 * portfolio.ts and options.ts.
 *
 * @module lib/desktop-api/financial
 */

import { invoke } from "@tauri-apps/api/core";
import type {
  HoldingsSummary,
  FifoReductionResult,
  Lot,
  DailyAccountValuation,
  PerformanceSummary,
  PerformancePoint,
  AllocationBreakdown,
  HoldingSnapshot,
  NetWorthSnapshot,
  Platform,
  FinancialAccount,
  Asset,
  Quote,
  Activity,
  ImportRun,
  Taxonomy,
  TaxonomyCategory,
  AssetTaxonomyAssignment,
  AllocationTarget,
  AllocationTargetWeight,
  AllocationTargetConstraint,
  // Input types are imported from domain structs — Tauri accepts them as-is
} from "@/types/financial";
import type {
  CreatePlatformInput,
  CreateAccountInput,
  CreateAssetInput,
  UpsertQuoteInput,
  CreateActivityInput,
  CreateImportRunInput,
  CreateLotInput,
  UpsertValuationInput,
  CreateTaxonomyInput,
  CreateTaxonomyCategoryInput,
  AssetTaxonomyAssignmentInput,
  CreateAllocationTargetInput,
  AllocationTargetWeightInput,
  AllocationTargetConstraintInput,
} from "@/types/financial";

// ── Holdings ────────────────────────────────────────────────────────────────

/** Get current holdings for a single account. */
export function getHoldings(
  accountId: string,
  asOfDate: string,
): Promise<HoldingsSummary> {
  return invoke("get_holdings", {
    accountId,
    asOfDate,
  });
}

/** Get holdings for all non-archived accounts. */
export function getAllHoldings(
  asOfDate: string,
): Promise<HoldingsSummary[]> {
  return invoke("get_all_holdings", { asOfDate });
}

// ── Lots ────────────────────────────────────────────────────────────────────

/** Record a sell activity against the FIFO lot inventory. */
export function recordSell(
  accountId: string,
  assetId: string,
  activityId: string,
): Promise<FifoReductionResult> {
  return invoke("record_sell", { accountId, assetId, activityId });
}

/** Get open lots for an account + asset combination. */
export function getOpenLots(
  accountId: string,
  assetId: string,
): Promise<Lot[]> {
  return invoke("get_open_lots", { accountId, assetId });
}

/** Get all open lots for an account. */
export function getOpenLotsForAccount(
  accountId: string,
): Promise<Lot[]> {
  return invoke("get_open_lots_for_account", { accountId });
}

// ── Valuation ───────────────────────────────────────────────────────────────

/** Calculate and persist one day's valuation for an account. */
export function calculateValuationDay(
  accountId: string,
  date: string,
): Promise<DailyAccountValuation> {
  return invoke("calculate_valuation_day", { accountId, date });
}

/** Get a single valuation row. */
export function getValuation(
  accountId: string,
  date: string,
): Promise<DailyAccountValuation | null> {
  return invoke("get_valuation", { accountId, date });
}

/** Get the full valuation series for an account. */
export function getValuationSeries(
  accountId: string,
): Promise<DailyAccountValuation[]> {
  return invoke("get_valuation_series", { accountId });
}

/** Calculate and persist valuations for all active accounts on a date. */
export function calculateAllValuations(
  date: string,
): Promise<DailyAccountValuation[]> {
  return invoke("calculate_all_valuations", { date });
}

// ── Performance ─────────────────────────────────────────────────────────────

/** Compute performance summary (XIRR, TWR) for an account. */
export function computePerformanceSummary(
  accountId: string,
  startDate: string,
  endDate: string,
): Promise<PerformanceSummary> {
  return invoke("compute_performance_summary", {
    accountId,
    startDate,
    endDate,
  });
}

/** Get the performance time-series for an account. */
export function getPerformanceTimeSeries(
  accountId: string,
): Promise<PerformancePoint[]> {
  return invoke("get_performance_time_series", { accountId });
}

// ── Allocation ──────────────────────────────────────────────────────────────

/** Compute allocation breakdown for a scope. */
export function getAllocation(
  scopeType: string,
  scopeId: string | null,
  asOfDate: string,
): Promise<AllocationBreakdown> {
  return invoke("get_allocation", { scopeType, scopeId, asOfDate });
}

/** Check constraints that apply to a scope. */
export function checkAllocationConstraints(
  scopeType: string,
  scopeId: string | null,
  asOfDate: string,
): Promise<string[]> {
  return invoke("check_allocation_constraints", {
    scopeType,
    scopeId,
    asOfDate,
  });
}

// ── Snapshots ───────────────────────────────────────────────────────────────

/** Create a snapshot from the current holdings of an account. */
export function createSnapshot(
  accountId: string,
  snapshotDate: string,
  label?: string,
): Promise<HoldingSnapshot> {
  return invoke("create_snapshot", {
    accountId,
    snapshotDate,
    label: label ?? null,
  });
}

/** Get a snapshot by ID. */
export function getSnapshot(
  id: string,
): Promise<HoldingSnapshot | null> {
  return invoke("get_snapshot", { id });
}

/** List snapshots for an account. */
export function listSnapshots(
  accountId: string,
): Promise<HoldingSnapshot[]> {
  return invoke("list_snapshots", { accountId });
}

/** Delete a snapshot. */
export function deleteSnapshot(id: string): Promise<void> {
  return invoke("delete_snapshot", { id });
}

// ── Net Worth ───────────────────────────────────────────────────────────────

/** Compute net worth as of a given date. */
export function computeNetWorth(
  asOfDate: string,
  baseCurrency?: string,
): Promise<NetWorthSnapshot> {
  return invoke("compute_net_worth", {
    asOfDate,
    baseCurrency: baseCurrency ?? null,
  });
}

// ── Platform CRUD (Phase 3.5) ──────────────────────────────────────────────

export function createPlatform(
  input: CreatePlatformInput,
): Promise<Platform> {
  return invoke("create_platform", { input });
}

export function listPlatforms(): Promise<Platform[]> {
  return invoke("list_platforms");
}

export function getPlatform(id: string): Promise<Platform | null> {
  return invoke("get_platform", { id });
}

// ── Financial Account CRUD (Phase 3.5) ─────────────────────────────────────

export function createFinancialAccount(
  input: CreateAccountInput,
): Promise<FinancialAccount> {
  return invoke("create_financial_account", { input });
}

export function listFinancialAccounts(
  workspaceId: string,
): Promise<FinancialAccount[]> {
  return invoke("list_financial_accounts", { workspaceId });
}

export function listAllFinancialAccounts(): Promise<FinancialAccount[]> {
  return invoke("list_all_financial_accounts");
}

export function getFinancialAccount(
  id: string,
): Promise<FinancialAccount | null> {
  return invoke("get_financial_account", { id });
}

export function archiveFinancialAccount(id: string): Promise<void> {
  return invoke("archive_financial_account", { id });
}

// ── Asset CRUD (Phase 3.5) ─────────────────────────────────────────────────

export function createAsset(input: CreateAssetInput): Promise<Asset> {
  return invoke("create_asset", { input });
}

export function getAsset(id: string): Promise<Asset | null> {
  return invoke("get_asset", { id });
}

export function findAssetByInstrumentKey(
  key: string,
): Promise<Asset | null> {
  return invoke("find_asset_by_instrument_key", { key });
}

export function listActiveAssets(): Promise<Asset[]> {
  return invoke("list_active_assets");
}

// ── Quote CRUD (Phase 3.5) ─────────────────────────────────────────────────

export function upsertQuote(input: UpsertQuoteInput): Promise<Quote> {
  return invoke("upsert_quote", { input });
}

export function getQuoteForDay(
  assetId: string,
  date: string,
  source: string,
): Promise<Quote | null> {
  return invoke("get_quote_for_day", { assetId, date, source });
}

export function listQuotesForAsset(assetId: string): Promise<Quote[]> {
  return invoke("list_quotes_for_asset", { assetId });
}

// ── Activity CRUD (Phase 3.5) ──────────────────────────────────────────────

export function createActivity(
  input: CreateActivityInput,
): Promise<Activity> {
  return invoke("create_activity", { input });
}

export function getActivity(id: string): Promise<Activity | null> {
  return invoke("get_activity", { id });
}

export function listActivitiesByAccount(
  accountId: string,
): Promise<Activity[]> {
  return invoke("list_activities_by_account", { accountId });
}

export function listActivitiesByAsset(assetId: string): Promise<Activity[]> {
  return invoke("list_activities_by_asset", { assetId });
}

// ── Import Run CRUD (Phase 3.5) ────────────────────────────────────────────

export function createImportRun(
  input: CreateImportRunInput,
): Promise<ImportRun> {
  return invoke("create_import_run", { input });
}

export function listImportRuns(accountId: string): Promise<ImportRun[]> {
  return invoke("list_import_runs", { accountId });
}

// ── Lot CRUD (Phase 3.5) ───────────────────────────────────────────────────

export function createLot(input: CreateLotInput): Promise<Lot> {
  return invoke("create_lot", { input });
}

export function getLot(id: string): Promise<Lot | null> {
  return invoke("get_lot", { id });
}

// ── Valuation CRUD (Phase 3.5) ─────────────────────────────────────────────

export function upsertValuation(
  input: UpsertValuationInput,
): Promise<DailyAccountValuation> {
  return invoke("upsert_valuation", { input });
}

export function listValuationsByAccount(
  accountId: string,
): Promise<DailyAccountValuation[]> {
  return invoke("list_valuations_by_account", { accountId });
}

export function deleteValuationForDate(
  accountId: string,
  date: string,
): Promise<void> {
  return invoke("delete_valuation_for_date", { accountId, date });
}

// ── Taxonomy CRUD (Phase 3.5) ──────────────────────────────────────────────

export function createTaxonomy(
  input: CreateTaxonomyInput,
): Promise<Taxonomy> {
  return invoke("create_taxonomy", { input });
}

export function getTaxonomy(id: string): Promise<Taxonomy | null> {
  return invoke("get_taxonomy", { id });
}

export function listTaxonomies(): Promise<Taxonomy[]> {
  return invoke("list_taxonomies");
}

export function createTaxonomyCategory(
  input: CreateTaxonomyCategoryInput,
): Promise<TaxonomyCategory> {
  return invoke("create_taxonomy_category", { input });
}

export function listTaxonomyCategories(
  taxonomyId: string,
): Promise<TaxonomyCategory[]> {
  return invoke("list_taxonomy_categories", { taxonomyId });
}

export function assignAssetToTaxonomyCategory(
  input: AssetTaxonomyAssignmentInput,
): Promise<AssetTaxonomyAssignment> {
  return invoke("assign_asset_to_taxonomy_category", { input });
}

export function listAssignmentsForAsset(
  assetId: string,
): Promise<AssetTaxonomyAssignment[]> {
  return invoke("list_assignments_for_asset", { assetId });
}

export function listAssignmentsByTaxonomy(
  taxonomyId: string,
): Promise<AssetTaxonomyAssignment[]> {
  return invoke("list_assignments_by_taxonomy", { taxonomyId });
}

export function removeTaxonomyAssignment(id: string): Promise<void> {
  return invoke("remove_taxonomy_assignment", { id });
}

// ── Allocation Target CRUD (Phase 3.5) ─────────────────────────────────────

export function createAllocationTarget(
  input: CreateAllocationTargetInput,
): Promise<AllocationTarget> {
  return invoke("create_allocation_target", { input });
}

export function getAllocationTarget(
  id: string,
): Promise<AllocationTarget | null> {
  return invoke("get_allocation_target", { id });
}

export function listAllocationTargets(
  includeArchived: boolean,
): Promise<AllocationTarget[]> {
  return invoke("list_allocation_targets", { includeArchived });
}

export function archiveAllocationTarget(id: string): Promise<void> {
  return invoke("archive_allocation_target", { id });
}

export function addAllocationWeight(
  input: AllocationTargetWeightInput,
): Promise<AllocationTargetWeight> {
  return invoke("add_allocation_weight", { input });
}

export function listAllocationWeights(
  targetId: string,
): Promise<AllocationTargetWeight[]> {
  return invoke("list_allocation_weights", { targetId });
}

export function addAllocationConstraint(
  input: AllocationTargetConstraintInput,
): Promise<AllocationTargetConstraint> {
  return invoke("add_allocation_constraint", { input });
}

export function listAllocationConstraints(
  targetId: string,
): Promise<AllocationTargetConstraint[]> {
  return invoke("list_allocation_constraints", { targetId });
}