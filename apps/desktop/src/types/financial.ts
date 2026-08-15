/**
 * Financial domain types — Phase 2/3 Wealthfolio port.
 *
 * These types mirror the Rust domain models in crates/domain/src/financial.rs.
 * Tauri serializes Rust struct fields in snake_case (the default serde).
 * Service output models (Holding, HoldingsSummary, etc.) are also snake_case.
 */

// ── Enums ───────────────────────────────────────────────────────────────────

export type AccountType =
  | "securities"
  | "cash"
  | "credit_card"
  | "cryptocurrency";

export type TrackingMode = "not_set" | "transactions" | "holdings";

export type AssetKind =
  | "investment"
  | "property"
  | "vehicle"
  | "collectible"
  | "precious_metal"
  | "private_equity"
  | "liability"
  | "other"
  | "fx";

export type QuoteMode = "market" | "manual";

export type InstrumentType =
  | "equity"
  | "crypto"
  | "fx"
  | "option"
  | "metal";

export type ActivityType =
  | "buy"
  | "sell"
  | "split"
  | "dividend"
  | "interest"
  | "deposit"
  | "withdrawal"
  | "transfer_in"
  | "transfer_out"
  | "fee"
  | "tax"
  | "credit"
  | "adjustment"
  | "cash_journal";

export type ActivityStatus = "posted" | "pending" | "canceled";

export type ValuationStatus =
  | "initial"
  | "estimated"
  | "calculated"
  | "reviewed"
  | "final";

export type BasisStatus = "filled" | "settled" | "partial";

export type HoldingSnapshotSource = "manual" | "system" | "import";

export type CostBasisMethod = "fifo" | "lifo" | "average_cost" | "specific_id";

export type ScopeType = "account" | "portfolio" | "workspace";

// ── Domain Models (snake_case from Tauri) ────────────────────────────────────

export interface Platform {
  id: string;
  name: string | null;
  url: string;
  external_id: string | null;
  kind: string;
  website_url: string | null;
  logo_url: string | null;
  created_at: string;
  updated_at: string;
}

export interface FinancialAccount {
  id: string;
  workspace_id: string | null;
  name: string;
  account_type: AccountType;
  group_name: string | null;
  currency: string;
  is_default: boolean;
  is_active: boolean;
  platform_id: string | null;
  account_number: string | null;
  meta: Record<string, unknown> | null;
  provider: string | null;
  provider_account_id: string | null;
  is_archived: boolean;
  tracking_mode: TrackingMode;
  created_at: string;
  updated_at: string;
}

export interface Asset {
  id: string;
  kind: AssetKind;
  name: string | null;
  display_code: string | null;
  notes: string | null;
  metadata: Record<string, unknown> | null;
  is_active: boolean;
  quote_mode: QuoteMode;
  quote_ccy: string;
  instrument_type: InstrumentType | null;
  instrument_symbol: string | null;
  instrument_exchange_mic: string | null;
  instrument_key: string | null;
  provider_config: Record<string, unknown> | null;
  created_at: string;
  updated_at: string;
}

export interface Quote {
  id: string;
  asset_id: string;
  day: string;
  source: string;
  open: string | null;
  high: string | null;
  low: string | null;
  close: string;
  adjclose: string | null;
  volume: string | null;
  currency: string;
  notes: string | null;
  created_at: string;
  timestamp: string;
}

export interface Activity {
  id: string;
  account_id: string;
  asset_id: string | null;
  activity_type: ActivityType;
  activity_type_override: string | null;
  source_type: string | null;
  subtype: string | null;
  status: ActivityStatus;
  activity_date: string;
  settlement_date: string | null;
  quantity: string | null;
  unit_price: string | null;
  amount: string | null;
  fee: string | null;
  tax: string | null;
  currency: string;
  fx_rate: string | null;
  notes: string | null;
  metadata: Record<string, unknown> | null;
  source_system: string | null;
  source_record_id: string | null;
  source_group_id: string | null;
  idempotency_key: string | null;
  import_run_id: string | null;
  is_user_modified: boolean;
  needs_review: boolean;
  created_at: string;
  updated_at: string;
}

export interface Lot {
  id: string;
  account_id: string;
  asset_id: string;
  open_date: string;
  open_activity_id: string | null;
  original_quantity: string;
  remaining_quantity: string;
  open_price: string;
  cost_basis: string;
  cost_basis_base: string;
  currency: string;
  fx_rate: string;
  realized_gain: string;
  realized_gain_base: string;
  status: BasisStatus;
  created_at: string;
  updated_at: string;
}

export interface LotDisposal {
  id: string;
  lot_id: string;
  activity_id: string;
  quantity: string;
  cost_basis: string;
  cost_basis_base: string;
  proceeds: string;
  proceeds_base: string;
  realized_pnl: string;
  realized_pnl_base: string;
  created_at: string;
}

export interface HoldingSnapshot {
  id: string;
  account_id: string;
  snapshot_date: string;
  label: string | null;
  source: HoldingSnapshotSource;
  total_value: string;
  total_value_base: string;
  cash_balance: string;
  cash_balance_base: string;
  positions: SnapshotPosition[];
  notes: string | null;
  created_at: string;
}

export interface SnapshotPosition {
  id: string;
  snapshot_id: string;
  asset_id: string;
  quantity: string;
  cost_basis: string;
  market_value: string;
  currency: string;
  created_at: string;
}

export interface DailyAccountValuation {
  id: string;
  account_id: string;
  valuation_date: string;
  total_value: string;
  cash_balance: string;
  investment_value: string;
  currency: string;
  fx_rate: string;
  total_value_base: string;
  status: ValuationStatus;
  created_at: string;
  updated_at: string;
}

// ── Service Output Models (Phase 2 — computed) ──────────────────────────────

export interface Holding {
  account_id: string;
  asset_id: string;
  asset_name: string | null;
  asset_symbol: string | null;
  asset_kind: AssetKind;
  currency: string;
  quantity: string;
  cost_basis: string;
  market_value: string;
  unrealized_gain: string;
  unrealized_gain_pct: string | null;
  realized_gain: string;
  total_gain: string;
  total_gain_pct: string | null;
  fx_rate: string;
  cost_basis_base: string;
  market_value_base: string;
  unrealized_gain_base: string;
  realized_gain_base: string;
  weight_pct: string;
  open_lot_count: number;
}

export interface HoldingsSummary {
  account_id: string;
  as_of_date: string;
  total_market_value: string;
  total_cost_basis: string;
  total_unrealized_gain: string;
  total_realized_gain: string;
  total_market_value_base: string;
  total_cost_basis_base: string;
  total_unrealized_gain_base: string;
  total_realized_gain_base: string;
  holdings: Holding[];
  cash_balance: string;
  cash_balance_base: string;
}

export interface FifoReductionResult {
  account_id: string;
  asset_id: string;
  disposal_date: string;
  total_quantity: string;
  total_proceeds: string;
  total_cost_basis: string;
  total_realized_pnl: string;
  total_proceeds_base: string;
  total_cost_basis_base: string;
  total_realized_pnl_base: string;
  lots_consumed: number;
  lots_partially_consumed: number;
}

export interface PerformancePoint {
  date: string;
  total_value: string;
  total_value_base: string;
  net_contribution: string;
  net_contribution_base: string;
  cumulative_return_pct: string | null;
  daily_return_pct: string | null;
}

export interface PerformanceSummary {
  account_id: string;
  start_date: string;
  end_date: string;
  total_return_pct: string | null;
  xirr_pct: string | null;
  twr_pct: string | null;
  start_value: string;
  end_value: string;
  net_contribution: string;
  total_gain: string;
  total_gain_base: string;
  data_quality: string;
}

export interface AllocationCategory {
  category_id: string;
  category_name: string;
  taxonomy_id: string;
  taxonomy_name: string;
  actual_bps: number;
  target_bps: number | null;
  difference_bps: number;
  market_value: string;
  market_value_base: string;
  within_drift: boolean;
}

export interface AllocationBreakdown {
  scope_type: ScopeType;
  scope_id: string | null;
  total_market_value: string;
  total_market_value_base: string;
  categories: AllocationCategory[];
  unassigned_market_value: string;
  unassigned_market_value_base: string;
}

export interface NetWorthAccountEntry {
  account_id: string;
  account_name: string;
  account_type: AccountType;
  currency: string;
  total_value: string;
  total_value_base: string;
  cash_balance: string;
  investment_value: string;
}

export interface NetWorthSnapshot {
  as_of_date: string;
  base_currency: string;
  total_assets: string;
  total_liabilities: string;
  net_worth: string;
  accounts: NetWorthAccountEntry[];
}