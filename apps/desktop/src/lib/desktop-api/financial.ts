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