/**
 * Financial data TanStack Query hooks — Phase 3.
 *
 * Wraps desktopApi.financial.* calls with query key factories and
 * TanStack Query hooks for portfolio dashboard components.
 *
 * @module features/portfolio/hooks/useFinancialData
 */

import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import type {
  HoldingsSummary,
  DailyAccountValuation,
  AllocationBreakdown,
  NetWorthSnapshot,
  HoldingSnapshot,
  FinancialAccount,
  Asset,
  Activity,
  CreateAccountInput,
  CreateAssetInput,
  CreateActivityInput,
} from "@/types/financial";

// ── Query Key Factory ──────────────────────────────────────────────────────

export const financialKeys = {
  all: ["financial"] as const,
  holdings: (accountId: string, asOfDate: string) =>
    [...financialKeys.all, "holdings", accountId, asOfDate] as const,
  allHoldings: (asOfDate: string) =>
    [...financialKeys.all, "allHoldings", asOfDate] as const,
  valuations: (accountId: string) =>
    [...financialKeys.all, "valuations", accountId] as const,
  allocation: (scopeType: string, scopeId: string | null, asOfDate: string) =>
    [...financialKeys.all, "allocation", scopeType, scopeId, asOfDate] as const,
  netWorth: (asOfDate: string, baseCurrency?: string) =>
    [...financialKeys.all, "netWorth", asOfDate, baseCurrency] as const,
  performance: (accountId: string, startDate: string, endDate: string) =>
    [...financialKeys.all, "performance", accountId, startDate, endDate] as const,
  snapshots: (accountId: string) =>
    [...financialKeys.all, "snapshots", accountId] as const,
  accounts: (workspaceId: string) =>
    [...financialKeys.all, "accounts", workspaceId] as const,
  assets: () => [...financialKeys.all, "assets"] as const,
  activities: (accountId: string) =>
    [...financialKeys.all, "activities", accountId] as const,
};

// ── Holdings Hooks ─────────────────────────────────────────────────────────

/** Get current holdings for a single account. */
export function useHoldings(accountId: string | undefined, asOfDate: string) {
  return useQuery({
    queryKey: financialKeys.holdings(accountId ?? "", asOfDate),
    queryFn: () => desktopApi.financial.getHoldings(accountId!, asOfDate),
    enabled: Boolean(accountId),
  });
}

/** Get holdings for all non-archived accounts. */
export function useAllHoldings(asOfDate: string) {
  return useQuery({
    queryKey: financialKeys.allHoldings(asOfDate),
    queryFn: () => desktopApi.financial.getAllHoldings(asOfDate),
  });
}

// ── Valuation Hooks ────────────────────────────────────────────────────────

/** Get the full valuation series for an account. */
export function useValuationSeries(accountId: string | undefined) {
  return useQuery({
    queryKey: financialKeys.valuations(accountId ?? ""),
    queryFn: () => desktopApi.financial.getValuationSeries(accountId!),
    enabled: Boolean(accountId),
  });
}

// ── Allocation Hooks ───────────────────────────────────────────────────────

/** Compute allocation breakdown for a scope. */
export function useAllocation(
  scopeType: string | undefined,
  scopeId: string | null,
  asOfDate: string,
) {
  return useQuery({
    queryKey: financialKeys.allocation(scopeType ?? "", scopeId, asOfDate),
    queryFn: () => desktopApi.financial.getAllocation(scopeType!, scopeId, asOfDate),
    enabled: Boolean(scopeType),
  });
}

// ── Net Worth Hooks ─────────────────────────────────────────────────────────

/** Compute net worth as of a given date. */
export function useNetWorth(asOfDate: string, baseCurrency?: string) {
  return useQuery({
    queryKey: financialKeys.netWorth(asOfDate, baseCurrency),
    queryFn: () => desktopApi.financial.computeNetWorth(asOfDate, baseCurrency),
  });
}

// ── Snapshot Hooks ──────────────────────────────────────────────────────────

/** Create a snapshot from the current holdings of an account. */
export function useCreateSnapshot() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({
      accountId,
      snapshotDate,
      label,
    }: {
      accountId: string;
      snapshotDate: string;
      label?: string;
    }) => desktopApi.financial.createSnapshot(accountId, snapshotDate, label),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: financialKeys.all });
    },
  });
}

/** List snapshots for an account. */
export function useSnapshots(accountId: string | undefined) {
  return useQuery({
    queryKey: financialKeys.snapshots(accountId ?? ""),
    queryFn: () => desktopApi.financial.listSnapshots(accountId!),
    enabled: Boolean(accountId),
  });
}

// ── Account CRUD Mutations (Phase 3.5) ────────────────────────────────────

export function useCreateFinancialAccount() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: CreateAccountInput) =>
      desktopApi.financial.createFinancialAccount(input),
    onSuccess: (_, input) => {
      queryClient.invalidateQueries({
        queryKey: financialKeys.accounts(input.workspace_id ?? ""),
      });
      queryClient.invalidateQueries({ queryKey: financialKeys.all });
    },
  });
}

export function useListFinancialAccounts(workspaceId: string | undefined) {
  return useQuery({
    queryKey: financialKeys.accounts(workspaceId ?? ""),
    queryFn: () => desktopApi.financial.listFinancialAccounts(workspaceId!),
    enabled: Boolean(workspaceId),
  });
}

// ── Asset CRUD Mutations (Phase 3.5) ──────────────────────────────────────

export function useCreateAsset() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: CreateAssetInput) =>
      desktopApi.financial.createAsset(input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: financialKeys.assets() });
      queryClient.invalidateQueries({ queryKey: financialKeys.all });
    },
  });
}

export function useListActiveAssets() {
  return useQuery({
    queryKey: financialKeys.assets(),
    queryFn: () => desktopApi.financial.listActiveAssets(),
  });
}

// ── Activity CRUD Mutations (Phase 3.5) ───────────────────────────────────

export function useCreateActivity() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: CreateActivityInput) =>
      desktopApi.financial.createActivity(input),
    onSuccess: (_, input) => {
      queryClient.invalidateQueries({
        queryKey: financialKeys.activities(input.account_id),
      });
      queryClient.invalidateQueries({ queryKey: financialKeys.all });
    },
  });
}

export function useListActivitiesByAccount(accountId: string | undefined) {
  return useQuery({
    queryKey: financialKeys.activities(accountId ?? ""),
    queryFn: () => desktopApi.financial.listActivitiesByAccount(accountId!),
    enabled: Boolean(accountId),
  });
}