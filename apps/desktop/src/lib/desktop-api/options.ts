import { invoke } from "@tauri-apps/api/core";
import { OptionChainSchema, OptionContractSchema, OptionStrategySchema } from "@/types/option";
import type { OptionChain, OptionContract, OptionStrategy, StrategyType } from "@/types/option";
import { z } from "zod";

const finiteNumberSchema = z.number().finite();
const GreeksResponseSchema = z.object({
  delta: finiteNumberSchema,
  gamma: finiteNumberSchema,
  theta: finiteNumberSchema,
  vega: finiteNumberSchema,
  rho: finiteNumberSchema,
});
const VoidResponseSchema = z.union([z.null(), z.undefined()]);

async function invokeOption<T>(
  command: string,
  args: Record<string, unknown> | undefined,
  schema: z.ZodType<T>,
): Promise<T> {
  const response: unknown = await invoke(command, args);
  return schema.parse(response);
}

async function invokeOptionVoid(command: string, args: Record<string, unknown>): Promise<void> {
  const response: unknown = await invoke(command, args);
  VoidResponseSchema.parse(response);
}

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

export interface CreateOptionContractParams {
  workspaceId: string;
  chainId: string;
  symbol: string;
  optionType: "call" | "put";
  strike: number;
  expiration: string;
  contractMultiplier?: number;
  bid?: number;
  ask?: number;
  last?: number | null;
  volume?: number;
  openInterest?: number;
  impliedVolatility?: number;
}

export interface CreateOptionStrategyParams {
  workspaceId: string;
  name: string;
  strategyType: StrategyType;
  underlying: string;
  totalCost?: number;
  maxProfit?: number;
  maxLoss?: number;
  breakEvenPoints?: number[];
}

export interface UpdateOptionStrategyParams {
  id: string;
  name?: string;
  totalCost?: number;
  maxProfit?: number;
  maxLoss?: number;
  breakEvenPoints?: number[];
}

/**
 * Fetch an option chain for a symbol from a data provider.
 */
export async function fetchOptionChain(params: FetchChainParams): Promise<OptionChain> {
  return invokeOption("fetch_option_chain", { params }, OptionChainSchema);
}

/** Get an existing option chain by ID. */
export async function getOptionChain(id: string): Promise<OptionChain> {
  return invokeOption("get_option_chain", { id }, OptionChainSchema);
}

/** Delete an option chain. */
export async function deleteOptionChain(id: string): Promise<void> {
  return invokeOptionVoid("delete_option_chain", { id });
}

/** List all option chains in a workspace. */
export async function listOptionChains(workspaceId: string): Promise<OptionChain[]> {
  return invokeOption("list_option_chains", { workspaceId }, z.array(OptionChainSchema));
}

// ============================================
// Option Contract Commands
// ============================================

/** List all contracts in an option chain. */
export async function listOptionContracts(chainId: string): Promise<OptionContract[]> {
  return invokeOption("list_option_contracts", { chainId }, z.array(OptionContractSchema));
}

/** Get a specific option contract by ID. */
export async function getOptionContract(id: string): Promise<OptionContract> {
  return invokeOption("get_option_contract", { id }, OptionContractSchema);
}

/** Create a new option contract. */
export async function createOptionContract(
  params: CreateOptionContractParams,
): Promise<OptionContract> {
  return invokeOption("create_option_contract", { params }, OptionContractSchema);
}

/** Delete an option contract. */
export async function deleteOptionContract(id: string): Promise<void> {
  return invokeOptionVoid("delete_option_contract", { id });
}

// ============================================
// Calculation Commands
// ============================================

/** Calculate Greeks for an option. */
export async function calculateGreeks(params: CalculateGreeksParams): Promise<GreeksResponse> {
  return invokeOption("calculate_greeks", { params }, GreeksResponseSchema);
}

/** Calculate option price using Black-Scholes. */
export async function calculateOptionPrice(params: CalculateGreeksParams): Promise<number> {
  return invokeOption("calculate_option_price", { params }, finiteNumberSchema);
}

/** Calculate implied volatility from market price. */
export async function calculateImpliedVolatility(params: CalculateIVParams): Promise<number> {
  return invokeOption("calculate_implied_volatility", { params }, finiteNumberSchema);
}

// ============================================
// Option Strategy Commands
// ============================================

/** List all option strategies in a workspace. */
export async function listOptionStrategies(workspaceId: string): Promise<OptionStrategy[]> {
  return invokeOption("list_option_strategies", { workspaceId }, z.array(OptionStrategySchema));
}

/** Get a specific option strategy by ID. */
export async function getOptionStrategy(id: string): Promise<OptionStrategy> {
  return invokeOption("get_option_strategy", { id }, OptionStrategySchema);
}

/** Create a new option strategy. */
export async function createOptionStrategy(
  params: CreateOptionStrategyParams,
): Promise<OptionStrategy> {
  return invokeOption("create_option_strategy", { params }, OptionStrategySchema);
}

/** Update an existing option strategy. */
export async function updateOptionStrategy(
  params: UpdateOptionStrategyParams,
): Promise<OptionStrategy> {
  return invokeOption("update_option_strategy", { params }, OptionStrategySchema);
}

/** Delete an option strategy. */
export async function deleteOptionStrategy(id: string): Promise<void> {
  return invokeOptionVoid("delete_option_strategy", { id });
}
