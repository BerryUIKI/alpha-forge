/**
 * Financial desktop API client — Phase 2/3 Wealthfolio port.
 *
 * Wraps all financial Tauri commands with typed validation schemas using Zod.
 *
 * @module lib/desktop-api/financial
 */

import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";
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

// ── Enums ───────────────────────────────────────────────────────────────────

export const AccountTypeSchema = z.enum([
  "securities",
  "cash",
  "credit_card",
  "cryptocurrency",
]);
export type AccountType = z.infer<typeof AccountTypeSchema>;

export const TrackingModeSchema = z.enum(["not_set", "transactions", "holdings"]);
export type TrackingMode = z.infer<typeof TrackingModeSchema>;

export const AssetKindSchema = z.enum([
  "investment",
  "property",
  "vehicle",
  "collectible",
  "precious_metal",
  "private_equity",
  "liability",
  "other",
  "fx",
]);
export type AssetKind = z.infer<typeof AssetKindSchema>;

export const QuoteModeSchema = z.enum(["market", "manual"]);
export type QuoteMode = z.infer<typeof QuoteModeSchema>;

export const InstrumentTypeSchema = z.enum([
  "equity",
  "crypto",
  "fx",
  "option",
  "metal",
]);
export type InstrumentType = z.infer<typeof InstrumentTypeSchema>;

export const ActivityTypeSchema = z.enum([
  "buy",
  "sell",
  "split",
  "dividend",
  "interest",
  "deposit",
  "withdrawal",
  "transfer_in",
  "transfer_out",
  "fee",
  "tax",
  "credit",
  "adjustment",
  "cash_journal",
  "unknown",
]);
export type ActivityType = z.infer<typeof ActivityTypeSchema>;

export const ActivityStatusSchema = z.enum(["posted", "pending", "canceled"]);
export type ActivityStatus = z.infer<typeof ActivityStatusSchema>;

export const ValuationStatusSchema = z.enum([
  "initial",
  "estimated",
  "calculated",
  "reviewed",
  "final",
]);
export type ValuationStatus = z.infer<typeof ValuationStatusSchema>;

export const BasisStatusSchema = z.enum(["filled", "settled", "partial"]);
export type BasisStatus = z.infer<typeof BasisStatusSchema>;

export const HoldingSnapshotSourceSchema = z.enum([
  "manual",
  "system",
  "import",
  "calculated",
  "manual_entry",
  "csv_import",
  "broker_imported",
  "synthetic",
]);
export type HoldingSnapshotSource = z.infer<typeof HoldingSnapshotSourceSchema>;

export const ScopeTypeSchema = z.enum(["account", "portfolio", "workspace", "all"]);
export type ScopeType = z.infer<typeof ScopeTypeSchema>;

// ── Models & Schemas ────────────────────────────────────────────────────────

export const PlatformSchema = z.object({
  id: z.string(),
  name: z.string().nullable(),
  url: z.string(),
  external_id: z.string().nullable(),
  kind: z.string(),
  website_url: z.string().nullable(),
  logo_url: z.string().nullable(),
  created_at: z.string(),
  updated_at: z.string(),
});
export type Platform = z.infer<typeof PlatformSchema>;

export const FinancialAccountSchema = z.object({
  id: z.string(),
  workspace_id: z.string().nullable(),
  name: z.string(),
  account_type: AccountTypeSchema,
  group_name: z.string().nullable(),
  currency: z.string(),
  is_default: z.boolean(),
  is_active: z.boolean(),
  platform_id: z.string().nullable(),
  account_number: z.string().nullable(),
  meta: z.record(z.unknown()).nullable(),
  provider: z.string().nullable(),
  provider_account_id: z.string().nullable(),
  is_archived: z.boolean(),
  tracking_mode: TrackingModeSchema,
  created_at: z.string(),
  updated_at: z.string(),
});
export type FinancialAccount = z.infer<typeof FinancialAccountSchema>;

export const AssetSchema = z.object({
  id: z.string(),
  kind: AssetKindSchema,
  name: z.string().nullable(),
  display_code: z.string().nullable(),
  notes: z.string().nullable(),
  metadata: z.record(z.unknown()).nullable(),
  is_active: z.boolean(),
  quote_mode: QuoteModeSchema,
  quote_ccy: z.string(),
  instrument_type: InstrumentTypeSchema.nullable(),
  instrument_symbol: z.string().nullable(),
  instrument_exchange_mic: z.string().nullable(),
  instrument_key: z.string().nullable(),
  provider_config: z.record(z.unknown()).nullable(),
  created_at: z.string(),
  updated_at: z.string(),
});
export type Asset = z.infer<typeof AssetSchema>;

export const QuoteSchema = z.object({
  id: z.string(),
  asset_id: z.string(),
  day: z.string(),
  source: z.string(),
  open: z.string().nullable(),
  high: z.string().nullable(),
  low: z.string().nullable(),
  close: z.string(),
  adjclose: z.string().nullable(),
  volume: z.string().nullable(),
  currency: z.string(),
  notes: z.string().nullable(),
  created_at: z.string(),
  timestamp: z.string(),
});
export type Quote = z.infer<typeof QuoteSchema>;

export const ActivitySchema = z.object({
  id: z.string(),
  account_id: z.string(),
  asset_id: z.string().nullable(),
  activity_type: ActivityTypeSchema,
  activity_type_override: z.string().nullable(),
  source_type: z.string().nullable(),
  subtype: z.string().nullable(),
  status: ActivityStatusSchema,
  activity_date: z.string(),
  settlement_date: z.string().nullable(),
  quantity: z.string().nullable(),
  unit_price: z.string().nullable(),
  amount: z.string().nullable(),
  fee: z.string().nullable(),
  tax: z.string().nullable(),
  currency: z.string(),
  fx_rate: z.string().nullable(),
  notes: z.string().nullable(),
  metadata: z.record(z.unknown()).nullable(),
  source_system: z.string().nullable(),
  source_record_id: z.string().nullable(),
  source_group_id: z.string().nullable(),
  idempotency_key: z.string().nullable(),
  import_run_id: z.string().nullable(),
  is_user_modified: z.boolean(),
  needs_review: z.boolean(),
  created_at: z.string(),
  updated_at: z.string(),
});
export type Activity = z.infer<typeof ActivitySchema>;

export const ImportRunSchema = z.object({
  id: z.string(),
  account_id: z.string(),
  source_system: z.string(),
  run_type: z.string(),
  mode: z.string(),
  status: z.string(),
  started_at: z.string(),
  finished_at: z.string().nullable(),
  review_mode: z.string(),
  applied_at: z.string().nullable(),
  checkpoint_in: z.string().nullable(),
  checkpoint_out: z.string().nullable(),
  summary: z.string().nullable(),
  warnings: z.string().nullable(),
  error: z.string().nullable(),
  created_at: z.string(),
  updated_at: z.string(),
});
export type ImportRun = z.infer<typeof ImportRunSchema>;

export const LotSchema = z.object({
  id: z.string(),
  account_id: z.string(),
  asset_id: z.string(),
  open_date: z.string(),
  open_activity_id: z.string().nullable(),
  original_quantity: z.string(),
  remaining_quantity: z.string(),
  open_price: z.string(),
  cost_basis: z.string(),
  cost_basis_base: z.string(),
  currency: z.string(),
  fx_rate: z.string(),
  realized_gain: z.string(),
  realized_gain_base: z.string(),
  status: BasisStatusSchema,
  created_at: z.string(),
  updated_at: z.string(),
});
export type Lot = z.infer<typeof LotSchema>;

export const DailyAccountValuationSchema = z.object({
  id: z.string(),
  account_id: z.string(),
  valuation_date: z.string(),
  total_value: z.string(),
  cash_balance: z.string(),
  investment_value: z.string(),
  currency: z.string(),
  fx_rate: z.string(),
  total_value_base: z.string(),
  status: ValuationStatusSchema,
  created_at: z.string(),
  updated_at: z.string(),
});
export type DailyAccountValuation = z.infer<typeof DailyAccountValuationSchema>;

export const HoldingSchema = z.object({
  account_id: z.string(),
  asset_id: z.string(),
  asset_name: z.string().nullable(),
  asset_symbol: z.string().nullable(),
  asset_kind: AssetKindSchema,
  currency: z.string(),
  quantity: z.string(),
  cost_basis: z.string(),
  market_value: z.string(),
  unrealized_gain: z.string(),
  unrealized_gain_pct: z.string().nullable(),
  realized_gain: z.string(),
  total_gain: z.string(),
  total_gain_pct: z.string().nullable(),
  fx_rate: z.string(),
  cost_basis_base: z.string(),
  market_value_base: z.string(),
  unrealized_gain_base: z.string(),
  realized_gain_base: z.string(),
  weight_pct: z.string(),
  open_lot_count: z.number(),
});
export type Holding = z.infer<typeof HoldingSchema>;

export const HoldingsSummarySchema = z.object({
  account_id: z.string(),
  as_of_date: z.string(),
  total_market_value: z.string(),
  total_cost_basis: z.string(),
  total_unrealized_gain: z.string(),
  total_realized_gain: z.string(),
  total_market_value_base: z.string(),
  total_cost_basis_base: z.string(),
  total_unrealized_gain_base: z.string(),
  total_realized_gain_base: z.string(),
  holdings: z.array(HoldingSchema),
  cash_balance: z.string(),
  cash_balance_base: z.string(),
});
export type HoldingsSummary = z.infer<typeof HoldingsSummarySchema>;

export const FifoReductionResultSchema = z.object({
  account_id: z.string(),
  asset_id: z.string(),
  disposal_date: z.string(),
  total_quantity: z.string(),
  total_proceeds: z.string(),
  total_cost_basis: z.string(),
  total_realized_pnl: z.string(),
  total_proceeds_base: z.string(),
  total_cost_basis_base: z.string(),
  total_realized_pnl_base: z.string(),
  lots_consumed: z.number(),
  lots_partially_consumed: z.number(),
});
export type FifoReductionResult = z.infer<typeof FifoReductionResultSchema>;

export const PerformancePointSchema = z.object({
  date: z.string(),
  total_value: z.string(),
  total_value_base: z.string(),
  net_contribution: z.string(),
  net_contribution_base: z.string(),
  cumulative_return_pct: z.string().nullable(),
  daily_return_pct: z.string().nullable(),
});
export type PerformancePoint = z.infer<typeof PerformancePointSchema>;

export const PerformanceSummarySchema = z.object({
  account_id: z.string(),
  start_date: z.string(),
  end_date: z.string(),
  total_return_pct: z.string().nullable(),
  xirr_pct: z.string().nullable(),
  twr_pct: z.string().nullable(),
  start_value: z.string(),
  end_value: z.string(),
  net_contribution: z.string(),
  total_gain: z.string(),
  total_gain_base: z.string(),
  data_quality: z.string(),
});
export type PerformanceSummary = z.infer<typeof PerformanceSummarySchema>;

export const AllocationCategorySchema = z.object({
  category_id: z.string(),
  category_name: z.string(),
  taxonomy_id: z.string(),
  taxonomy_name: z.string(),
  actual_bps: z.number(),
  target_bps: z.number().nullable(),
  difference_bps: z.number(),
  market_value: z.string(),
  market_value_base: z.string(),
  within_drift: z.boolean(),
});
export type AllocationCategory = z.infer<typeof AllocationCategorySchema>;

export const AllocationBreakdownSchema = z.object({
  scope_type: ScopeTypeSchema,
  scope_id: z.string().nullable(),
  total_market_value: z.string(),
  total_market_value_base: z.string(),
  categories: z.array(AllocationCategorySchema),
  unassigned_market_value: z.string(),
  unassigned_market_value_base: z.string(),
});
export type AllocationBreakdown = z.infer<typeof AllocationBreakdownSchema>;

export const SnapshotPositionSchema = z.object({
  id: z.string(),
  snapshot_id: z.string(),
  asset_id: z.string(),
  quantity: z.string(),
  cost_basis: z.string(),
  market_value: z.string(),
  currency: z.string(),
  created_at: z.string(),
});
export type SnapshotPosition = z.infer<typeof SnapshotPositionSchema>;

export const HoldingSnapshotSchema = z.object({
  id: z.string(),
  account_id: z.string(),
  snapshot_date: z.string(),
  label: z.string().nullable(),
  source: HoldingSnapshotSourceSchema,
  total_value: z.string(),
  total_value_base: z.string(),
  cash_balance: z.string(),
  cash_balance_base: z.string(),
  positions: z.array(SnapshotPositionSchema),
  notes: z.string().nullable(),
  created_at: z.string(),
});
export type HoldingSnapshot = z.infer<typeof HoldingSnapshotSchema>;

export const NetWorthAccountEntrySchema = z.object({
  account_id: z.string(),
  account_name: z.string(),
  account_type: AccountTypeSchema,
  currency: z.string(),
  total_value: z.string(),
  total_value_base: z.string(),
  cash_balance: z.string(),
  investment_value: z.string(),
});
export type NetWorthAccountEntry = z.infer<typeof NetWorthAccountEntrySchema>;

export const NetWorthSnapshotSchema = z.object({
  as_of_date: z.string(),
  base_currency: z.string(),
  total_assets: z.string(),
  total_liabilities: z.string(),
  net_worth: z.string(),
  accounts: z.array(NetWorthAccountEntrySchema),
});
export type NetWorthSnapshot = z.infer<typeof NetWorthSnapshotSchema>;

export const TaxonomySchema = z.object({
  id: z.string(),
  name: z.string(),
  color: z.string(),
  description: z.string().nullable(),
  is_system: z.boolean(),
  is_single_select: z.boolean(),
  sort_order: z.number(),
  created_at: z.string(),
  updated_at: z.string(),
});
export type Taxonomy = z.infer<typeof TaxonomySchema>;

export const TaxonomyCategorySchema = z.object({
  id: z.string(),
  taxonomy_id: z.string(),
  parent_id: z.string().nullable(),
  name: z.string(),
  key: z.string(),
  color: z.string(),
  description: z.string().nullable(),
  sort_order: z.number(),
  created_at: z.string(),
  updated_at: z.string(),
});
export type TaxonomyCategory = z.infer<typeof TaxonomyCategorySchema>;

export const AssetTaxonomyAssignmentSchema = z.object({
  id: z.string(),
  asset_id: z.string(),
  taxonomy_id: z.string(),
  category_id: z.string(),
  weight: z.number(),
  source: z.string(),
  created_at: z.string(),
  updated_at: z.string(),
});
export type AssetTaxonomyAssignment = z.infer<
  typeof AssetTaxonomyAssignmentSchema
>;

export const AllocationTargetSchema = z.object({
  id: z.string(),
  name: z.string(),
  scope_type: ScopeTypeSchema,
  scope_id: z.string().nullable(),
  taxonomy_id: z.string(),
  trigger_type: z.string(),
  drift_band_bps: z.number(),
  rebalance_goal: z.string(),
  min_trade_amount: z.string(),
  whole_shares_only: z.boolean(),
  allow_sells: z.boolean(),
  max_turnover_bps: z.number().nullable(),
  created_at: z.string(),
  updated_at: z.string(),
  archived_at: z.string().nullable(),
});
export type AllocationTarget = z.infer<typeof AllocationTargetSchema>;

export const AllocationTargetWeightSchema = z.object({
  id: z.string(),
  target_id: z.string(),
  taxonomy_id: z.string(),
  category_id: z.string(),
  target_bps: z.number(),
  is_locked: z.boolean(),
  is_required: z.boolean(),
  created_at: z.string(),
  updated_at: z.string(),
});
export type AllocationTargetWeight = z.infer<
  typeof AllocationTargetWeightSchema
>;

export const AllocationTargetConstraintSchema = z.object({
  id: z.string(),
  target_id: z.string(),
  subject_type: z.string(),
  subject_id: z.string(),
  action: z.string(),
  effect: z.string(),
  reason: z.string().nullable(),
  metadata_json: z.record(z.unknown()).nullable(),
  created_at: z.string(),
  updated_at: z.string(),
});
export type AllocationTargetConstraint = z.infer<
  typeof AllocationTargetConstraintSchema
>;

// ── Holdings ────────────────────────────────────────────────────────────────

/** Get current holdings for a single account. */
export async function getHoldings(
  accountId: string,
  asOfDate: string,
): Promise<HoldingsSummary> {
  const res = await invoke("get_holdings", {
    accountId,
    asOfDate,
  });
  return HoldingsSummarySchema.parse(res);
}

/** Get holdings for all non-archived accounts. */
export async function getAllHoldings(
  asOfDate: string,
): Promise<HoldingsSummary[]> {
  const res = await invoke("get_all_holdings", { asOfDate });
  return z.array(HoldingsSummarySchema).parse(res);
}

// ── Lots ────────────────────────────────────────────────────────────────────

/** Record a sell activity against the FIFO lot inventory. */
export async function recordSell(
  accountId: string,
  assetId: string,
  activityId: string,
): Promise<FifoReductionResult> {
  const res = await invoke("record_sell", { accountId, assetId, activityId });
  return FifoReductionResultSchema.parse(res);
}

/** Get open lots for an account + asset combination. */
export async function getOpenLots(
  accountId: string,
  assetId: string,
): Promise<Lot[]> {
  const res = await invoke("get_open_lots", { accountId, assetId });
  return z.array(LotSchema).parse(res);
}

/** Get all open lots for an account. */
export async function getOpenLotsForAccount(
  accountId: string,
): Promise<Lot[]> {
  const res = await invoke("get_open_lots_for_account", { accountId });
  return z.array(LotSchema).parse(res);
}

// ── Valuation ───────────────────────────────────────────────────────────────

/** Calculate and persist one day's valuation for an account. */
export async function calculateValuationDay(
  accountId: string,
  date: string,
): Promise<DailyAccountValuation> {
  const res = await invoke("calculate_valuation_day", { accountId, date });
  return DailyAccountValuationSchema.parse(res);
}

/** Get a single valuation row. */
export async function getValuation(
  accountId: string,
  date: string,
): Promise<DailyAccountValuation | null> {
  const res = await invoke("get_valuation", { accountId, date });
  return res ? DailyAccountValuationSchema.parse(res) : null;
}

/** Get the full valuation series for an account. */
export async function getValuationSeries(
  accountId: string,
): Promise<DailyAccountValuation[]> {
  const res = await invoke("get_valuation_series", { accountId });
  return z.array(DailyAccountValuationSchema).parse(res);
}

/** Calculate and persist valuations for all active accounts on a date. */
export async function calculateAllValuations(
  date: string,
): Promise<DailyAccountValuation[]> {
  const res = await invoke("calculate_all_valuations", { date });
  return z.array(DailyAccountValuationSchema).parse(res);
}

// ── Performance ─────────────────────────────────────────────────────────────

/** Compute performance summary (XIRR, TWR) for an account. */
export async function computePerformanceSummary(
  accountId: string,
  startDate: string,
  endDate: string,
): Promise<PerformanceSummary> {
  const res = await invoke("compute_performance_summary", {
    accountId,
    startDate,
    endDate,
  });
  return PerformanceSummarySchema.parse(res);
}

/** Get the performance time-series for an account. */
export async function getPerformanceTimeSeries(
  accountId: string,
): Promise<PerformancePoint[]> {
  const res = await invoke("get_performance_time_series", { accountId });
  return z.array(PerformancePointSchema).parse(res);
}

// ── Allocation ──────────────────────────────────────────────────────────────

/** Compute allocation breakdown for a scope. */
export async function getAllocation(
  scopeType: string,
  scopeId: string | null,
  asOfDate: string,
): Promise<AllocationBreakdown> {
  const res = await invoke("get_allocation", { scopeType, scopeId, asOfDate });
  return AllocationBreakdownSchema.parse(res);
}

/** Check constraints that apply to a scope. */
export async function checkAllocationConstraints(
  scopeType: string,
  scopeId: string | null,
  asOfDate: string,
): Promise<string[]> {
  const res = await invoke("check_allocation_constraints", {
    scopeType,
    scopeId,
    asOfDate,
  });
  return z.array(z.string()).parse(res);
}

// ── Snapshots ───────────────────────────────────────────────────────────────

/** Create a snapshot from the current holdings of an account. */
export async function createSnapshot(
  accountId: string,
  snapshotDate: string,
  label?: string,
): Promise<HoldingSnapshot> {
  const res = await invoke("create_snapshot", {
    accountId,
    snapshotDate,
    label: label ?? null,
  });
  return HoldingSnapshotSchema.parse(res);
}

/** Get a snapshot by ID. */
export async function getSnapshot(
  id: string,
): Promise<HoldingSnapshot | null> {
  const res = await invoke("get_snapshot", { id });
  return res ? HoldingSnapshotSchema.parse(res) : null;
}

/** List snapshots for an account. */
export async function listSnapshots(
  accountId: string,
): Promise<HoldingSnapshot[]> {
  const res = await invoke("list_snapshots", { accountId });
  return z.array(HoldingSnapshotSchema).parse(res);
}

/** Delete a snapshot. */
export async function deleteSnapshot(id: string): Promise<void> {
  await invoke("delete_snapshot", { id });
}

// ── Net Worth ───────────────────────────────────────────────────────────────

/** Compute net worth as of a given date. */
export async function computeNetWorth(
  asOfDate: string,
  baseCurrency?: string,
): Promise<NetWorthSnapshot> {
  const res = await invoke("compute_net_worth", {
    asOfDate,
    baseCurrency: baseCurrency ?? null,
  });
  return NetWorthSnapshotSchema.parse(res);
}

// ── Platform CRUD (Phase 3.5) ──────────────────────────────────────────────

export async function createPlatform(
  input: CreatePlatformInput,
): Promise<Platform> {
  const res = await invoke("create_platform", { input });
  return PlatformSchema.parse(res);
}

export async function listPlatforms(): Promise<Platform[]> {
  const res = await invoke("list_platforms");
  return z.array(PlatformSchema).parse(res);
}

export async function getPlatform(id: string): Promise<Platform | null> {
  const res = await invoke("get_platform", { id });
  return res ? PlatformSchema.parse(res) : null;
}

// ── Financial Account CRUD (Phase 3.5) ─────────────────────────────────────

export async function createFinancialAccount(
  input: CreateAccountInput,
): Promise<FinancialAccount> {
  const res = await invoke("create_financial_account", { input });
  return FinancialAccountSchema.parse(res);
}

export async function listFinancialAccounts(
  workspaceId: string,
): Promise<FinancialAccount[]> {
  const res = await invoke("list_financial_accounts", { workspaceId });
  return z.array(FinancialAccountSchema).parse(res);
}

export async function listAllFinancialAccounts(): Promise<FinancialAccount[]> {
  const res = await invoke("list_all_financial_accounts");
  return z.array(FinancialAccountSchema).parse(res);
}

export async function getFinancialAccount(
  id: string,
): Promise<FinancialAccount | null> {
  const res = await invoke("get_financial_account", { id });
  return res ? FinancialAccountSchema.parse(res) : null;
}

export async function archiveFinancialAccount(id: string): Promise<void> {
  await invoke("archive_financial_account", { id });
}

// ── Asset CRUD (Phase 3.5) ─────────────────────────────────────────────────

export async function createAsset(input: CreateAssetInput): Promise<Asset> {
  const res = await invoke("create_asset", { input });
  return AssetSchema.parse(res);
}

export async function getAsset(id: string): Promise<Asset | null> {
  const res = await invoke("get_asset", { id });
  return res ? AssetSchema.parse(res) : null;
}

export async function findAssetByInstrumentKey(
  key: string,
): Promise<Asset | null> {
  const res = await invoke("find_asset_by_instrument_key", { key });
  return res ? AssetSchema.parse(res) : null;
}

export async function listActiveAssets(): Promise<Asset[]> {
  const res = await invoke("list_active_assets");
  return z.array(AssetSchema).parse(res);
}

// ── Quote CRUD (Phase 3.5) ─────────────────────────────────────────────────

export async function upsertQuote(input: UpsertQuoteInput): Promise<Quote> {
  const res = await invoke("upsert_quote", { input });
  return QuoteSchema.parse(res);
}

export async function getQuoteForDay(
  assetId: string,
  date: string,
  source: string,
): Promise<Quote | null> {
  const res = await invoke("get_quote_for_day", { assetId, date, source });
  return res ? QuoteSchema.parse(res) : null;
}

export async function listQuotesForAsset(assetId: string): Promise<Quote[]> {
  const res = await invoke("list_quotes_for_asset", { assetId });
  return z.array(QuoteSchema).parse(res);
}

// ── Activity CRUD (Phase 3.5) ──────────────────────────────────────────────

export async function createActivity(
  input: CreateActivityInput,
): Promise<Activity> {
  const res = await invoke("create_activity", { input });
  return ActivitySchema.parse(res);
}

export async function getActivity(id: string): Promise<Activity | null> {
  const res = await invoke("get_activity", { id });
  return res ? ActivitySchema.parse(res) : null;
}

export async function listActivitiesByAccount(
  accountId: string,
): Promise<Activity[]> {
  const res = await invoke("list_activities_by_account", { accountId });
  return z.array(ActivitySchema).parse(res);
}

export async function listActivitiesByAsset(
  assetId: string,
): Promise<Activity[]> {
  const res = await invoke("list_activities_by_asset", { assetId });
  return z.array(ActivitySchema).parse(res);
}

// ── Import Run CRUD (Phase 3.5) ────────────────────────────────────

export async function createImportRun(
  input: CreateImportRunInput,
): Promise<ImportRun> {
  const res = await invoke("create_import_run", { input });
  return ImportRunSchema.parse(res);
}

export async function listImportRuns(accountId: string): Promise<ImportRun[]> {
  const res = await invoke("list_import_runs", { accountId });
  return z.array(ImportRunSchema).parse(res);
}

// ── Lot CRUD (Phase 3.5) ───────────────────────────────────────────────────

export async function createLot(input: CreateLotInput): Promise<Lot> {
  const res = await invoke("create_lot", { input });
  return LotSchema.parse(res);
}

export async function getLot(id: string): Promise<Lot | null> {
  const res = await invoke("get_lot", { id });
  return res ? LotSchema.parse(res) : null;
}

// ── Valuation CRUD (Phase 3.5) ─────────────────────────────────────────────

export async function upsertValuation(
  input: UpsertValuationInput,
): Promise<DailyAccountValuation> {
  const res = await invoke("upsert_valuation", { input });
  return DailyAccountValuationSchema.parse(res);
}

export async function listValuationsByAccount(
  accountId: string,
): Promise<DailyAccountValuation[]> {
  const res = await invoke("list_valuations_by_account", { accountId });
  return z.array(DailyAccountValuationSchema).parse(res);
}

export async function deleteValuationForDate(
  accountId: string,
  date: string,
): Promise<void> {
  await invoke("delete_valuation_for_date", { accountId, date });
}

// ── Taxonomy CRUD (Phase 3.5) ──────────────────────────────────────────────

export async function createTaxonomy(
  input: CreateTaxonomyInput,
): Promise<Taxonomy> {
  const res = await invoke("create_taxonomy", { input });
  return TaxonomySchema.parse(res);
}

export async function getTaxonomy(id: string): Promise<Taxonomy | null> {
  const res = await invoke("get_taxonomy", { id });
  return res ? TaxonomySchema.parse(res) : null;
}

export async function listTaxonomies(): Promise<Taxonomy[]> {
  const res = await invoke("list_taxonomies");
  return z.array(TaxonomySchema).parse(res);
}

export async function createTaxonomyCategory(
  input: CreateTaxonomyCategoryInput,
): Promise<TaxonomyCategory> {
  const res = await invoke("create_taxonomy_category", { input });
  return TaxonomyCategorySchema.parse(res);
}

export async function listTaxonomyCategories(
  taxonomyId: string,
): Promise<TaxonomyCategory[]> {
  const res = await invoke("list_taxonomy_categories", { taxonomyId });
  return z.array(TaxonomyCategorySchema).parse(res);
}

export async function assignAssetToTaxonomyCategory(
  input: AssetTaxonomyAssignmentInput,
): Promise<AssetTaxonomyAssignment> {
  const res = await invoke("assign_asset_to_taxonomy_category", { input });
  return AssetTaxonomyAssignmentSchema.parse(res);
}

export async function listAssignmentsForAsset(
  assetId: string,
): Promise<AssetTaxonomyAssignment[]> {
  const res = await invoke("list_assignments_for_asset", { assetId });
  return z.array(AssetTaxonomyAssignmentSchema).parse(res);
}

export async function listAssignmentsByTaxonomy(
  taxonomyId: string,
): Promise<AssetTaxonomyAssignment[]> {
  const res = await invoke("list_assignments_by_taxonomy", { taxonomyId });
  return z.array(AssetTaxonomyAssignmentSchema).parse(res);
}

export async function removeTaxonomyAssignment(id: string): Promise<void> {
  await invoke("remove_taxonomy_assignment", { id });
}

// ── Allocation Target CRUD (Phase 3.5) ─────────────────────────────────────

export async function createAllocationTarget(
  input: CreateAllocationTargetInput,
): Promise<AllocationTarget> {
  const res = await invoke("create_allocation_target", { input });
  return AllocationTargetSchema.parse(res);
}

export async function getAllocationTarget(
  id: string,
): Promise<AllocationTarget | null> {
  const res = await invoke("get_allocation_target", { id });
  return res ? AllocationTargetSchema.parse(res) : null;
}

export async function listAllocationTargets(
  includeArchived: boolean,
): Promise<AllocationTarget[]> {
  const res = await invoke("list_allocation_targets", { includeArchived });
  return z.array(AllocationTargetSchema).parse(res);
}

export async function archiveAllocationTarget(id: string): Promise<void> {
  await invoke("archive_allocation_target", { id });
}

export async function addAllocationWeight(
  input: AllocationTargetWeightInput,
): Promise<AllocationTargetWeight> {
  const res = await invoke("add_allocation_weight", { input });
  return AllocationTargetWeightSchema.parse(res);
}

export async function listAllocationWeights(
  targetId: string,
): Promise<AllocationTargetWeight[]> {
  const res = await invoke("list_allocation_weights", { targetId });
  return z.array(AllocationTargetWeightSchema).parse(res);
}

export async function addAllocationConstraint(
  input: AllocationTargetConstraintInput,
): Promise<AllocationTargetConstraint> {
  const res = await invoke("add_allocation_constraint", { input });
  return AllocationTargetConstraintSchema.parse(res);
}

export async function listAllocationConstraints(
  targetId: string,
): Promise<AllocationTargetConstraint[]> {
  const res = await invoke("list_allocation_constraints", { targetId });
  return z.array(AllocationTargetConstraintSchema).parse(res);
}