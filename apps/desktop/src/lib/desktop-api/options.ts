import { invoke } from "@tauri-apps/api/core";
import type {
  OptionChain,
  OptionContract,
  OptionStrategy,
} from "@/types/option";

// ============================================
// Types matching backend commands
// ============================================

export interface FetchChainParams {
  symbol: string;
  workspaceId: string;
  provider?: "live" | "demo" | "file";
}

export interface CalculateGreeksParams {
  optionType: "call" | "put";
  underlyingPrice: number;
  strike: number;
  expirationYears: number;
  riskFreeRate: number;
  volatility: number;
  dividendYield?: number;
}

export interface CalculateIVParams {
  optionType: "call" | "put";
  underlyingPrice: number;
  strike: number;
  expirationYears: number;
  riskFreeRate: number;
  dividendYield?: number;
  marketPrice: number;
}

export interface GreeksResponse {
  delta: number;
  gamma: number;
  theta: number;
  vega: number;
  rho: number;
}

// ============================================
// Option Chain Commands
// ============================================

/**
 * Fetch an option chain for a symbol from a data provider
 */
export async function fetchOptionChain(
  params: FetchChainParams
): Promise<OptionChain> {
  return invoke("fetch_option_chain", { params });
}

/**
 * Get an existing option chain by ID
 */
export async function getOptionChain(id: string): Promise<OptionChain> {
  return invoke("get_option_chain", { id });
}

/**
 * Create a new option chain (typically from imported data)
 */
export async function createOptionChain(
  params: FetchChainParams
): Promise<OptionChain> {
  return invoke("create_option_chain", { params });
}

/**
 * Delete an option chain
 */
export async function deleteOptionChain(id: string): Promise<void> {
  return invoke("delete_option_chain", { id });
}

/**
 * List all option chains in a workspace
 */
export async function listOptionChains(workspaceId: string): Promise<OptionChain[]> {
  return invoke("list_option_chains", { workspaceId });
}

// ============================================
// Option Contract Commands
// ============================================

/**
 * List all contracts in an option chain
 */
export async function listOptionContracts(chainId: string): Promise<OptionContract[]> {
  return invoke("list_option_contracts", { chainId });
}

/**
 * Get a specific option contract by ID
 */
export async function getOptionContract(id: string): Promise<OptionContract> {
  return invoke("get_option_contract", { id });
}

/**
 * Create a new option contract
 */
export async function createOptionContract(
  params: Omit<OptionContract, "id" | "createdAt" | "updatedAt">
): Promise<OptionContract> {
  return invoke("create_option_contract", { params });
}

/**
 * Delete an option contract
 */
export async function deleteOptionContract(id: string): Promise<void> {
  return invoke("delete_option_contract", { id });
}

// ============================================
// Calculation Commands
// ============================================

/**
 * Calculate Greeks for an option
 */
export async function calculateGreeks(
  params: CalculateGreeksParams
): Promise<GreeksResponse> {
  return invoke("calculate_greeks", { params });
}

/**
 * Calculate option price using Black-Scholes
 */
export async function calculateOptionPrice(
  params: CalculateGreeksParams
): Promise<number> {
  return invoke("calculate_option_price", { params });
}

/**
 * Calculate implied volatility from market price
 */
export async function calculateImpliedVolatility(
  params: CalculateIVParams
): Promise<number> {
  return invoke("calculate_implied_volatility", { params });
}

// ============================================
// Option Strategy Commands
// ============================================

/**
 * List all option strategies in a workspace
 */
export async function listOptionStrategies(
  workspaceId: string
): Promise<OptionStrategy[]> {
  return invoke("list_option_strategies", { workspaceId });
}

/**
 * Get a specific option strategy by ID
 */
export async function getOptionStrategy(id: string): Promise<OptionStrategy> {
  return invoke("get_option_strategy", { id });
}

/**
 * Create a new option strategy
 */
export async function createOptionStrategy(
  params: Partial<OptionStrategy> & { name: string; strategyType: string; workspaceId: string }
): Promise<OptionStrategy> {
  return invoke("create_option_strategy", { params });
}

/**
 * Update an existing option strategy
 */
export async function updateOptionStrategy(
  params: Partial<OptionStrategy> & { id: string }
): Promise<OptionStrategy> {
  return invoke("update_option_strategy", { params });
}

/**
 * Delete an option strategy
 */
export async function deleteOptionStrategy(id: string): Promise<void> {
  return invoke("delete_option_strategy", { id });
}
