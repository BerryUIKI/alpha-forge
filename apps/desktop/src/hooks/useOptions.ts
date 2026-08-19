/**
 * Option Data Fetching Hooks
 *
 * Provides TanStack Query hooks for option operations.
 * Follows integration standards from docs/FRONTEND_BACKEND_INTEGRATION.md
 *
 * Backend Commands: src-tauri/src/commands/options.rs
 * Domain Types: crates/domain/src/option.rs
 * API Layer: src/lib/desktop-api/options.ts
 *
 * @module hooks/options
 */

import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import type { Locale } from "@/lib/i18n/locale";
import { processAppError } from "@/lib/errors";

// Import types from option types module
export type {
  OptionChain,
  OptionContract,
  OptionStrategy,
} from "@/types/option";

// Import API-specific types
export type {
  GreeksResponse,
  FetchChainParams,
  CalculateGreeksParams,
  CalculateIVParams,
} from "@/lib/desktop-api/options";

// ============================================================================
// Query Hooks - Option Chains
// ============================================================================

/**
 * Hook to fetch option chain for a symbol
 *
 * @example
 * const { data: chain, isLoading, error } = useOptionChain(workspaceId, "AAPL");
 */
export function useOptionChain(
  workspaceId: string,
  symbol: string,
  provider?: "live" | "demo" | "file"
) {
  return useQuery({
    queryKey: ["optionChain", workspaceId, symbol, provider],
    queryFn: () =>
      desktopApi.options.fetchOptionChain({
        symbol,
        workspaceId,
        provider: provider ?? "demo",
      }),
    enabled: !!workspaceId && !!symbol,
    staleTime: 30000, // 30 seconds - option data can be stale for demo
  });
}

/**
 * Hook to list all option chains in a workspace
 *
 * @example
 * const { data: chains } = useOptionChains(workspaceId);
 */
export function useOptionChains(workspaceId: string) {
  return useQuery({
    queryKey: ["optionChains", workspaceId],
    queryFn: () => desktopApi.options.listOptionChains(workspaceId),
    enabled: !!workspaceId,
    staleTime: 60000, // 1 minute
  });
}

/**
 * Hook to get a specific option chain by ID
 *
 * @example
 * const { data: chain } = useOptionChainById(chainId);
 */
export function useOptionChainById(id: string) {
  return useQuery({
    queryKey: ["optionChain", id],
    queryFn: () => desktopApi.options.getOptionChain(id),
    enabled: !!id,
    staleTime: 60000,
  });
}

// ============================================================================
// Query Hooks - Option Contracts
// ============================================================================

/**
 * Hook to list all contracts in an option chain
 *
 * @example
 * const { data: contracts } = useOptionContracts(chainId);
 */
export function useOptionContracts(chainId: string) {
  return useQuery({
    queryKey: ["optionContracts", chainId],
    queryFn: () => desktopApi.options.listOptionContracts(chainId),
    enabled: !!chainId,
    staleTime: 60000,
  });
}

// ============================================================================
// Query Hooks - Option Strategies
// ============================================================================

/**
 * Hook to list all option strategies in a workspace
 *
 * @example
 * const { data: strategies } = useOptionStrategies(workspaceId);
 */
export function useOptionStrategies(workspaceId: string) {
  return useQuery({
    queryKey: ["optionStrategies", workspaceId],
    queryFn: () => desktopApi.options.listOptionStrategies(workspaceId),
    enabled: !!workspaceId,
    staleTime: 60000,
  });
}

/**
 * Hook to get a specific option strategy by ID
 *
 * @example
 * const { data: strategy } = useOptionStrategy(strategyId);
 */
export function useOptionStrategy(id: string) {
  return useQuery({
    queryKey: ["optionStrategy", id],
    queryFn: () => desktopApi.options.getOptionStrategy(id),
    enabled: !!id,
    staleTime: 60000,
  });
}

// ============================================================================
// Mutation Hooks - Option Chains
// ============================================================================

/**
 * Hook to fetch a new option chain from provider
 *
 * @example
 * const fetchMutation = useFetchOptionChain();
 * fetchMutation.mutate({ workspaceId, symbol: "AAPL" });
 */
export function useFetchOptionChain(_locale: Locale) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (params: {
      workspaceId: string;
      symbol: string;
      provider?: "live" | "demo" | "file";
    }) =>
      desktopApi.options.fetchOptionChain({
        symbol: params.symbol,
        workspaceId: params.workspaceId,
        provider: params.provider,
      }),
    onSuccess: (data) => {
      // Invalidate chains list
      queryClient.invalidateQueries({
        queryKey: ["optionChains", data.workspaceId],
      });
      // Set the individual chain cache
      queryClient.setQueryData(["optionChain", data.id], data);
    },
  });
}

/**
 * Hook to delete an option chain
 *
 * @example
 * const deleteMutation = useDeleteOptionChain();
 * deleteMutation.mutate(chainId);
 */
export function useDeleteOptionChain(_locale: Locale) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => desktopApi.options.deleteOptionChain(id),
    onSuccess: () => {
      // Invalidate all chain lists
      queryClient.invalidateQueries({ queryKey: ["optionChains"] });
    },
  });
}

// ============================================================================
// Mutation Hooks - Calculations
// ============================================================================

/**
 * Hook to calculate Greeks for an option
 *
 * @example
 * const calcMutation = useCalculateGreeks();
 * calcMutation.mutate({ optionType: "call", underlyingPrice: 100, ... });
 */
export function useCalculateGreeks() {
  return useMutation({
    mutationFn: desktopApi.options.calculateGreeks,
  });
}

/**
 * Hook to calculate option price
 *
 * @example
 * const calcMutation = useCalculateOptionPrice();
 * calcMutation.mutate({ optionType: "call", underlyingPrice: 100, ... });
 */
export function useCalculateOptionPrice() {
  return useMutation({
    mutationFn: desktopApi.options.calculateOptionPrice,
  });
}

/**
 * Hook to calculate implied volatility
 *
 * @example
 * const calcMutation = useCalculateIV();
 * calcMutation.mutate({ optionType: "call", marketPrice: 5.50, ... });
 */
export function useCalculateIV() {
  return useMutation({
    mutationFn: desktopApi.options.calculateImpliedVolatility,
  });
}

// ============================================================================
// Mutation Hooks - Option Strategies
// ============================================================================

/**
 * Hook to create a new option strategy
 *
 * @example
 * const createMutation = useCreateOptionStrategy();
 * createMutation.mutate({ name: "Bull Call Spread", strategyType: "bull_call_spread", workspaceId });
 */
export function useCreateOptionStrategy(_locale: Locale) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: desktopApi.options.createOptionStrategy,
    onSuccess: (data) => {
      queryClient.invalidateQueries({
        queryKey: ["optionStrategies", data.workspaceId],
      });
      queryClient.setQueryData(["optionStrategy", data.id], data);
    },
  });
}

/**
 * Hook to update an option strategy
 *
 * @example
 * const updateMutation = useUpdateOptionStrategy();
 * updateMutation.mutate({ id: strategyId, name: "Updated Name" });
 */
export function useUpdateOptionStrategy(_locale: Locale) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: desktopApi.options.updateOptionStrategy,
    onSuccess: (data) => {
      queryClient.setQueryData(["optionStrategy", data.id], data);
      queryClient.invalidateQueries({
        queryKey: ["optionStrategies", data.workspaceId],
      });
    },
  });
}

/**
 * Hook to delete an option strategy
 *
 * @example
 * const deleteMutation = useDeleteOptionStrategy();
 * deleteMutation.mutate(strategyId);
 */
export function useDeleteOptionStrategy(_locale: Locale) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: desktopApi.options.deleteOptionStrategy,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["optionStrategies"] });
    },
  });
}

/**
 * Hook to delete an option contract
 *
 * @example
 * const deleteMutation = useDeleteOptionContract();
 * deleteMutation.mutate(contractId);
 */
export function useDeleteOptionContract(_locale: Locale) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => desktopApi.options.deleteOptionContract(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["optionContracts"] });
    },
  });
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Process option error for display
 *
 * @param locale - Current locale
 * @param error - Error from mutation
 * @returns Localized error messages
 */
export function processOptionError(locale: Locale, error: unknown) {
  return processAppError(locale, error);
}