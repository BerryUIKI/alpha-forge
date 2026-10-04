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
  CreateAccountInput,
  CreateAssetInput,
  CreateActivityInput,
  CreateLotInput,
  CreateTaxonomyInput,
  CreateTaxonomyCategoryInput,
  AssetTaxonomyAssignmentInput,
  CreateAllocationTargetInput,
  AllocationTargetWeightInput,
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
  // All accounts across every workspace (global portfolio dimension, ADR-0008).
  allAccounts: () => [...financialKeys.all, "accounts", "all"] as const,
  assets: () => [...financialKeys.all, "assets"] as const,
  quotes: (assetId: string) => [...financialKeys.all, "quotes", assetId] as const,
  activities: (accountId: string) =>
    [...financialKeys.all, "activities", accountId] as const,
  taxonomies: () => [...financialKeys.all, "taxonomies"] as const,
  taxonomyCategories: (taxonomyId: string) =>
    [...financialKeys.all, "taxonomyCategories", taxonomyId] as const,
  assetAssignments: (assetId: string) =>
    [...financialKeys.all, "assetAssignments", assetId] as const,
  taxonomyAssignments: (taxonomyId: string) =>
    [...financialKeys.all, "taxonomyAssignments", taxonomyId] as const,
  allocationTargets: (includeArchived: boolean) =>
    [...financialKeys.all, "allocationTargets", includeArchived] as const,
  allocationWeights: (targetId: string) =>
    [...financialKeys.all, "allocationWeights", targetId] as const,
  allocationConstraints: (
    scopeType: string,
    scopeId: string | null,
    asOfDate: string,
  ) =>
    [
      ...financialKeys.all,
      "allocationConstraints",
      scopeType,
      scopeId,
      asOfDate,
    ] as const,
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
      queryClient.invalidateQueries({ queryKey: financialKeys.allAccounts() });
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

/**
 * List all financial accounts, ignoring the research-workspace ownership
 * marker. Portfolio is a global dimension (ADR-0008): the Portfolio page
 * shows every account regardless of the active workspace.
 */
export function useListAllFinancialAccounts() {
  return useQuery({
    queryKey: financialKeys.allAccounts(),
    queryFn: () => desktopApi.financial.listAllFinancialAccounts(),
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

// ── Lot CRUD + Sell (Phase 3.5) ────────────────────────────────────────────

export function useCreateLot() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: CreateLotInput) =>
      desktopApi.financial.createLot(input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: financialKeys.all });
    },
  });
}

export function useRecordSell() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({
      accountId,
      assetId,
      activityId,
    }: {
      accountId: string;
      assetId: string;
      activityId: string;
    }) => desktopApi.financial.recordSell(accountId, assetId, activityId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: financialKeys.all });
    },
  });
}

// ── Market Data Quote Refresh ──────────────────────────────────────────────

export function useRefreshAssetQuote() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (assetId: string) =>
      desktopApi.financial.refreshAssetQuote(assetId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: financialKeys.all });
    },
  });
}

export function useRefreshAllActiveQuotes() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: () => desktopApi.financial.refreshAllActiveQuotes(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: financialKeys.all });
    },
  });
}

export function useListQuotesForAsset(assetId: string | undefined) {
  return useQuery({
    queryKey: financialKeys.quotes(assetId ?? ""),
    queryFn: () => desktopApi.financial.listQuotesForAsset(assetId!),
    enabled: Boolean(assetId),
  });
}

// ── Broker Activity Statement CSV Import ───────────────────────────────────

export function useImportActivitiesCsv() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({
      accountId,
      format,
      csvText,
    }: {
      accountId: string;
      format: "GENERIC" | "IBKR";
      csvText: string;
    }) => desktopApi.financial.importActivitiesCsv(accountId, format, csvText),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: financialKeys.all });
    },
  });
}

// ── Taxonomy Hooks (Category 11) ───────────────────────────────────────────

export function useListTaxonomies() {
  return useQuery({
    queryKey: financialKeys.taxonomies(),
    queryFn: () => desktopApi.financial.listTaxonomies(),
  });
}

export function useCreateTaxonomy() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: CreateTaxonomyInput) =>
      desktopApi.financial.createTaxonomy(input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: financialKeys.taxonomies() });
    },
  });
}

export function useListTaxonomyCategories(taxonomyId: string | undefined) {
  return useQuery({
    queryKey: financialKeys.taxonomyCategories(taxonomyId ?? ""),
    queryFn: () => desktopApi.financial.listTaxonomyCategories(taxonomyId!),
    enabled: Boolean(taxonomyId),
  });
}

export function useCreateTaxonomyCategory() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: CreateTaxonomyCategoryInput) =>
      desktopApi.financial.createTaxonomyCategory(input),
    onSuccess: (_, input) => {
      queryClient.invalidateQueries({
        queryKey: financialKeys.taxonomyCategories(input.taxonomy_id),
      });
    },
  });
}

export function useListAssignmentsForAsset(assetId: string | undefined) {
  return useQuery({
    queryKey: financialKeys.assetAssignments(assetId ?? ""),
    queryFn: () => desktopApi.financial.listAssignmentsForAsset(assetId!),
    enabled: Boolean(assetId),
  });
}

export function useListAssignmentsByTaxonomy(taxonomyId: string | undefined) {
  return useQuery({
    queryKey: financialKeys.taxonomyAssignments(taxonomyId ?? ""),
    queryFn: () => desktopApi.financial.listAssignmentsByTaxonomy(taxonomyId!),
    enabled: Boolean(taxonomyId),
  });
}

export function useAssignAssetToTaxonomyCategory() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: AssetTaxonomyAssignmentInput) =>
      desktopApi.financial.assignAssetToTaxonomyCategory(input),
    onSuccess: (_, input) => {
      queryClient.invalidateQueries({
        queryKey: financialKeys.assetAssignments(input.asset_id),
      });
      queryClient.invalidateQueries({
        queryKey: financialKeys.taxonomyAssignments(input.taxonomy_id),
      });
      queryClient.invalidateQueries({ queryKey: financialKeys.all });
    },
  });
}

export function useRemoveTaxonomyAssignment() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) =>
      desktopApi.financial.removeTaxonomyAssignment(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: financialKeys.all });
    },
  });
}

// ── Allocation Target Hooks (Category 11) ──────────────────────────────────

export function useListAllocationTargets(includeArchived = false) {
  return useQuery({
    queryKey: financialKeys.allocationTargets(includeArchived),
    queryFn: () => desktopApi.financial.listAllocationTargets(includeArchived),
  });
}

export function useCreateAllocationTarget() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: CreateAllocationTargetInput) =>
      desktopApi.financial.createAllocationTarget(input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: financialKeys.all });
    },
  });
}

export function useArchiveAllocationTarget() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) =>
      desktopApi.financial.archiveAllocationTarget(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: financialKeys.all });
    },
  });
}

export function useListAllocationWeights(targetId: string | undefined) {
  return useQuery({
    queryKey: financialKeys.allocationWeights(targetId ?? ""),
    queryFn: () => desktopApi.financial.listAllocationWeights(targetId!),
    enabled: Boolean(targetId),
  });
}

export function useAddAllocationWeight() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: AllocationTargetWeightInput) =>
      desktopApi.financial.addAllocationWeight(input),
    onSuccess: (_, input) => {
      queryClient.invalidateQueries({
        queryKey: financialKeys.allocationWeights(input.target_id),
      });
      queryClient.invalidateQueries({ queryKey: financialKeys.all });
    },
  });
}

export function useCheckAllocationConstraints(
  scopeType: string | undefined,
  scopeId: string | null,
  asOfDate: string,
) {
  return useQuery({
    queryKey: financialKeys.allocationConstraints(
      scopeType ?? "",
      scopeId,
      asOfDate,
    ),
    queryFn: () =>
      desktopApi.financial.checkAllocationConstraints(
        scopeType!,
        scopeId,
        asOfDate,
      ),
    enabled: Boolean(scopeType),
  });
}