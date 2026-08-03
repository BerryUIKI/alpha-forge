import { invoke } from "@tauri-apps/api/core";
import type {
  OptionChain,
  OptionContract,
  OptionStrategy,
  FetchOptionChainParams,
  CreateStrategyParams,
} from "@/types/option";

// ============================================
// Option Chain Commands
// ============================================

/**
 * Fetch an option chain for a symbol from a data provider
 */
export function fetchOptionChain(
  workspaceId: string,
  symbol: string
): Promise<OptionChain> {
  return invoke("fetch_option_chain", { workspaceId, symbol });
}

/**
 * Get an existing option chain by ID
 */
export function getOptionChain(id: string): Promise<OptionChain> {
  return invoke("get_option_chain", { id });
}

/**
 * Create a new option chain (typically from imported data)
 */
export function createOptionChain(
  params: FetchOptionChainParams
): Promise<OptionChain> {
  return invoke("create_option_chain", { params });
}

/**
 * Delete an option chain
 */
export function deleteOptionChain(id: string): Promise<void> {
  return invoke("delete_option_chain", { id });
}

/**
 * List all option chains in a workspace
 */
export function listOptionChains(workspaceId: string): Promise<OptionChain[]> {
  return invoke("list_option_chains", { workspaceId });
}

// ============================================
// Option Contract Commands
// ============================================

/**
 * List all contracts in an option chain
 */
export function listOptionContracts(chainId: string): Promise<OptionContract[]> {
  return invoke("list_option_contracts", { chainId });
}

/**
 * Get a specific option contract by ID
 */
export function getOptionContract(id: string): Promise<OptionContract> {
  return invoke("get_option_contract", { id });
}

/**
 * Create a new option contract
 */
export function createOptionContract(
  params: Omit<OptionContract, "id" | "createdAt" | "updatedAt">
): Promise<OptionContract> {
  return invoke("create_option_contract", { params });
}

/**
 * Delete an option contract
 */
export function deleteOptionContract(id: string): Promise<void> {
  return invoke("delete_option_contract", { id });
}

// ============================================
// Option Strategy Commands
// ============================================

/**
 * List all option strategies in a workspace
 */
export function listOptionStrategies(
  workspaceId: string
): Promise<OptionStrategy[]> {
  return invoke("list_option_strategies", { workspaceId });
}

/**
 * Get a specific option strategy by ID
 */
export function getOptionStrategy(id: string): Promise<OptionStrategy> {
  return invoke("get_option_strategy", { id });
}

/**
 * Create a new option strategy
 */
export function createOptionStrategy(
  params: CreateStrategyParams
): Promise<OptionStrategy> {
  return invoke("create_option_strategy", { params });
}

/**
 * Update an existing option strategy
 */
export function updateOptionStrategy(
  params: Partial<OptionStrategy> & { id: string }
): Promise<OptionStrategy> {
  return invoke("update_option_strategy", { params });
}

/**
 * Delete an option strategy
 */
export function deleteOptionStrategy(id: string): Promise<void> {
  return invoke("delete_option_strategy", { id });
}
