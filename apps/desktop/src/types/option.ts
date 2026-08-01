/**
 * Option Analysis Platform - Type Definitions
 * 
 * TypeScript interfaces matching Rust domain models for the Option platform.
 * All entities are workspace-scoped and follow AlphaForge's data model patterns.
 */

// ============================================
// Enums
// ============================================

export type OptionType = 'call' | 'put';

export type PositionType = 'long' | 'short';

export type DataSource = 'live' | 'demo' | 'file';

export type PricingModel = 'black_scholes' | 'binomial' | 'finite_difference';

export type StrategyType =
  | 'long_call'
  | 'long_put'
  | 'covered_call'
  | 'protective_put'
  | 'bull_call_spread'
  | 'bear_put_spread'
  | 'straddle'
  | 'strangle'
  | 'iron_condor'
  | 'butterfly'
  | 'custom';

// ============================================
// Entities
// ============================================

/**
 * Option chain - represents all option contracts for a symbol at a point in time
 */
export interface OptionChain {
  id: string;
  workspaceId: string;
  symbol: string;
  underlyingPrice: number;
  asOf: string;  // ISO 8601 datetime
  dataSource: DataSource;
  createdAt: string;
}

/**
 * Option contract - a single option within a chain
 */
export interface OptionContract {
  id: string;
  workspaceId: string;
  chainId: string;
  symbol: string;
  optionType: OptionType;
  strike: number;
  expiration: string;  // ISO 8601 datetime
  contractMultiplier: number;
  bid: number;
  ask: number;
  last?: number;
  volume: number;
  openInterest: number;
  impliedVolatility: number;
  createdAt: string;
  updatedAt: string;
}

/**
 * Greeks - risk sensitivities for an option
 */
export interface Greeks {
  id: string;
  optionContractId: string;
  delta: number;
  gamma: number;
  theta: number;
  vega: number;
  rho: number;
  iv: number;
  calculatedAt: string;
  calculationModel: PricingModel;
}

/**
 * Option strategy - a multi-leg option position
 */
export interface OptionStrategy {
  id: string;
  workspaceId: string;
  name: string;
  strategyType: StrategyType;
  underlying: string;
  totalCost: number;
  maxProfit?: number;
  maxLoss?: number;
  breakEvenPoints: number[];
  createdAt: string;
  updatedAt: string;
}

/**
 * Strategy leg - a single option within a strategy
 */
export interface StrategyLeg {
  id: string;
  strategyId: string;
  optionContractId: string;
  quantity: number;
  positionType: PositionType;
  premium: number;
  strike: number;
  expiration: string;
  optionType: OptionType;
}

/**
 * Option position - an actual position held in portfolio
 */
export interface OptionPosition {
  id: string;
  workspaceId: string;
  accountId?: string;
  optionContractId: string;
  quantity: number;
  costBasis: number;
  openedAt: string;
  closedAt?: string;
  notes?: string;
}

/**
 * Greeks snapshot - historical Greeks tracking for positions
 */
export interface GreeksSnapshot {
  id: string;
  workspaceId: string;
  positionId: string;
  snapshotDate: string;
  delta: number;
  gamma: number;
  theta: number;
  vega: number;
  rho: number;
  createdAt: string;
}

// ============================================
// API Parameter Types
// ============================================

export interface FetchOptionChainParams {
  symbol: string;
  workspaceId: string;
  provider?: DataSource;
}

export interface GetChainContractsParams {
  chainId: string;
  includeGreeks?: boolean;
}

export interface GreeksParams {
  optionType: OptionType;
  underlyingPrice: number;
  strike: number;
  expiration: string;
  riskFreeRate: number;
  volatility: number;
  dividendYield?: number;
  model?: PricingModel;
}

export interface ChainGreeksParams {
  chainId: string;
  riskFreeRate?: number;
  dividendYield?: number;
  model?: PricingModel;
}

export interface CreateStrategyParams {
  workspaceId: string;
  name: string;
  strategyType: StrategyType;
  legs: StrategyLegParams[];
}

export interface StrategyLegParams {
  contractId?: string;
  symbol?: string;
  optionType?: OptionType;
  strike?: number;
  expiration?: string;
  quantity: number;
  premium: number;
}

export interface AnalyzeStrategyParams {
  priceRange?: {
    min: number;
    max: number;
    step: number;
  };
  volatilityAssumption?: number;
  daysToExpiration?: number;
}

export interface CalculatePayoffParams {
  strategyId: string;
  underlyingPrice: number;
  daysForward?: number;
}

export interface PositionImportData {
  source: 'csv' | 'manual';
  accountId?: string;
  csvPath?: string;
  positions?: ManualPosition[];
}

export interface ManualPosition {
  symbol: string;
  optionType: OptionType;
  strike: number;
  expiration: string;
  quantity: number;
  costBasis: number;
  accountId?: string;
}

// ============================================
// API Response Types
// ============================================

export interface OptionContractWithGreeks extends OptionContract {
  greeks?: Greeks;
}

export interface ContractGreeksResult {
  contractId: string;
  greeks: Greeks;
  calculationTime: number;
}

export interface StrategyAnalysis {
  strategyId: string;
  underlying: string;
  currentPrice: number;
  totalCost: number;
  maxProfit: number | null;
  maxLoss: number | null;
  breakEvenPoints: number[];
  netGreeks: {
    delta: number;
    gamma: number;
    theta: number;
    vega: number;
    rho: number;
  };
  payoffAtExpiration: PayoffPoint[];
  probabilityOfProfit?: number;
  probabilityOfMaxProfit?: number;
  probabilityOfMaxLoss?: number;
}

export interface PayoffPoint {
  underlyingPrice: number;
  profitLoss: number;
}

export interface PayoffResult {
  strategyId: string;
  underlyingPrice: number;
  daysForward: number;
  profitLoss: number;
  profitLossPercent: number;
  currentGreeks: {
    delta: number;
    gamma: number;
    theta: number;
    vega: number;
  };
  legBreakdown: LegPayoff[];
}

export interface LegPayoff {
  legId: string;
  contractId: string;
  intrinsicValue: number;
  timeValue: number;
  profitLoss: number;
}

export interface ImportResult {
  success: number;
  failed: number;
  errors?: ImportError[];
  positions: OptionPosition[];
}

export interface ImportError {
  row: number;
  field: string;
  message: string;
}

export interface PortfolioGreeks {
  workspaceId: string;
  calculatedAt: string;
  netDelta: number;
  netGamma: number;
  netTheta: number;
  netVega: number;
  netRho: number;
  deltaExposure: number;
  gammaExposure: number;
  totalPositions: number;
  longPositions: number;
  shortPositions: number;
  byUnderlying: UnderlyingGreeks[];
}

export interface UnderlyingGreeks {
  symbol: string;
  netDelta: number;
  netGamma: number;
  netTheta: number;
  netVega: number;
  positionCount: number;
}

export interface VolatilitySurface {
  symbol: string;
  underlyingPrice: number;
  asOf: string;
  points: SurfacePoint[];
  termStructure: TermStructurePoint[];
  skewAnalysis: SkewAnalysis;
}

export interface SurfacePoint {
  strike: number;
  expirationDays: number;
  impliedVolatility: number;
  moneyness: number;
}

export interface TermStructurePoint {
  expirationDays: number;
  atmIV: number;
}

export interface SkewAnalysis {
  expiration: string;
  callSkew: number;
  putSkew: number;
}

export interface IVResult {
  impliedVolatility: number;
  impliedVolatilityPercent: number;
  iterations: number;
  converged: boolean;
}

export interface ScenarioResults {
  baselinePnL: number;
  scenarios: ScenarioResult[];
}

export interface ScenarioResult {
  name: string;
  profitLoss: number;
  profitLossPercent: number;
  deltaChange: number;
  gammaChange: number;
  vegaImpact: number;
  thetaImpact: number;
}

// ============================================
// Zod Schemas (for validation)
// ============================================

import { z } from 'zod';

export const OptionTypeSchema = z.enum(['call', 'put']);
export const PositionTypeSchema = z.enum(['long', 'short']);
export const DataSourceSchema = z.enum(['live', 'demo', 'file']);

export const StrategyTypeSchema = z.enum([
  'long_call',
  'long_put',
  'covered_call',
  'protective_put',
  'bull_call_spread',
  'bear_put_spread',
  'straddle',
  'strangle',
  'iron_condor',
  'butterfly',
  'custom',
]);

export const OptionChainSchema = z.object({
  id: z.string().uuid(),
  workspaceId: z.string().uuid(),
  symbol: z.string().min(1).max(10),
  underlyingPrice: z.number().positive(),
  asOf: z.string().datetime(),
  dataSource: DataSourceSchema,
  createdAt: z.string().datetime(),
});

export const OptionContractSchema = z.object({
  id: z.string().uuid(),
  workspaceId: z.string().uuid(),
  chainId: z.string().uuid(),
  symbol: z.string().min(1).max(10),
  optionType: OptionTypeSchema,
  strike: z.number().positive(),
  expiration: z.string().datetime(),
  contractMultiplier: z.number().int().positive().default(100),
  bid: z.number().nonnegative(),
  ask: z.number().nonnegative(),
  last: z.number().nonnegative().optional(),
  volume: z.number().int().nonnegative().default(0),
  openInterest: z.number().int().nonnegative().default(0),
  impliedVolatility: z.number().positive(),
  createdAt: z.string().datetime(),
  updatedAt: z.string().datetime(),
}).refine(data => data.bid <= data.ask, {
  message: "Bid must be <= Ask",
});

export const GreeksSchema = z.object({
  id: z.string().uuid(),
  optionContractId: z.string().uuid(),
  delta: z.number(),
  gamma: z.number(),
  theta: z.number(),
  vega: z.number(),
  rho: z.number(),
  iv: z.number().positive(),
  calculatedAt: z.string().datetime(),
  calculationModel: z.enum(['black_scholes', 'binomial', 'finite_difference']),
});

export const OptionStrategySchema = z.object({
  id: z.string().uuid(),
  workspaceId: z.string().uuid(),
  name: z.string().min(1).max(100),
  strategyType: StrategyTypeSchema,
  underlying: z.string().min(1).max(10),
  totalCost: z.number(),
  maxProfit: z.number().optional(),
  maxLoss: z.number().optional(),
  breakEvenPoints: z.array(z.number()),
  createdAt: z.string().datetime(),
  updatedAt: z.string().datetime(),
});

export const StrategyLegSchema = z.object({
  id: z.string().uuid(),
  strategyId: z.string().uuid(),
  optionContractId: z.string().uuid(),
  quantity: z.number().int().nonzero(),
  positionType: PositionTypeSchema,
  premium: z.number().nonnegative(),
  strike: z.number().positive(),
  expiration: z.string().datetime(),
  optionType: OptionTypeSchema,
});

export const OptionPositionSchema = z.object({
  id: z.string().uuid(),
  workspaceId: z.string().uuid(),
  accountId: z.string().uuid().optional(),
  optionContractId: z.string().uuid(),
  quantity: z.number().int(),
  costBasis: z.number(),
  openedAt: z.string().datetime(),
  closedAt: z.string().datetime().optional(),
  notes: z.string().optional(),
});