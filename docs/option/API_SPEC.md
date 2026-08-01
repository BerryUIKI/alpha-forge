# Option Analysis Platform - API Specification

## Overview

This document defines the IPC (Inter-Process Communication) commands, TypeScript interfaces, and message protocols for the Option Analysis Platform. All communication between React frontend and Rust backend goes through the `desktopApi` layer.

---

## IPC Command Architecture

### Request-Response Pattern

All commands follow a consistent pattern:

```text
Frontend (React)
    ↓ desktopApi.options.<command>(params)
IPC invoke('<command_name>', params)
    ↓ Tauri IPC bridge
Backend (Rust)
    ↓ #[tauri::command]
fn <command_name>(params, state) -> Result<Response, AppError>
    ↓ Business logic
Result<Response, AppError>
    ↓ Serialize to JSON
Frontend receives Response
```

### Error Handling

All commands return `Result<T, AppError>` where:

```rust
#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub context: Option<serde_json::Value>,
    pub recoverable: bool,
}
```

---

## Core Commands

### 1. Option Chain Commands

#### fetch_option_chain

**Purpose**: Fetch option chain data for a symbol

**Frontend Call**:
```typescript
const chain = await desktopApi.options.fetchOptionChain('AAPL');
```

**IPC Command**:
```rust
#[tauri::command]
pub async fn fetch_option_chain(
    symbol: String,
    workspace_id: String,
    provider: Option<DataSource>,
    state: State<'_, AppState>,
) -> Result<OptionChain, AppError>
```

**Parameters**:
```typescript
interface FetchOptionChainParams {
  symbol: string;
  workspaceId: string;
  provider?: 'live' | 'demo' | 'file';
}
```

**Response**:
```typescript
interface OptionChain {
  id: string;
  workspaceId: string;
  symbol: string;
  underlyingPrice: number;
  asOf: string;  // ISO 8601 datetime
  dataSource: DataSource;
  createdAt: string;
  contracts?: OptionContract[];  // Optional: may be lazy-loaded
}
```

**Error Codes**:
- `INVALID_SYMBOL`: Symbol not found or invalid format
- `NO_DATA`: No option data available for symbol
- `PROVIDER_ERROR`: Data provider error (rate limit, API error)
- `NETWORK_ERROR`: Network connectivity issue

**Example**:
```typescript
// Frontend usage
import { desktopApi } from '@/lib/desktop-api';

try {
  const chain = await desktopApi.options.fetchOptionChain({
    symbol: 'AAPL',
    workspaceId: 'ws-123',
    provider: 'demo'
  });
  
  console.log(`Chain loaded: ${chain.contracts?.length} contracts`);
} catch (error) {
  if (error instanceof OptionApiError) {
    if (error.code === 'NO_DATA') {
      showToast('No options available for this symbol');
    }
  }
}
```

---

#### get_chain_contracts

**Purpose**: Get all contracts for an option chain (with Greeks)

**Frontend Call**:
```typescript
const contracts = await desktopApi.options.getChainContracts('chain-123');
```

**IPC Command**:
```rust
#[tauri::command]
pub async fn get_chain_contracts(
    chain_id: String,
    include_greeks: Option<bool>,
    state: State<'_, AppState>,
) -> Result<Vec<OptionContractWithGreeks>, AppError>
```

**Parameters**:
```typescript
interface GetChainContractsParams {
  chainId: string;
  includeGreeks?: boolean;  // default: true
}
```

**Response**:
```typescript
interface OptionContractWithGreeks extends OptionContract {
  greeks?: Greeks;
}
```

---

### 2. Greeks Calculation Commands

#### calculate_greeks

**Purpose**: Calculate Greeks for a single option

**Frontend Call**:
```typescript
const greeks = await desktopApi.options.calculateGreeks(params);
```

**IPC Command**:
```rust
#[tauri::command]
pub async fn calculate_greeks(
    params: GreeksParams,
    state: State<'_, AppState>,
) -> Result<Greeks, AppError>
```

**Parameters**:
```typescript
interface GreeksParams {
  optionType: 'call' | 'put';
  underlyingPrice: number;
  strike: number;
  expiration: string;  // ISO 8601 datetime
  riskFreeRate: number;  // e.g., 0.05 for 5%
  volatility: number;    // e.g., 0.25 for 25%
  dividendYield?: number; // default: 0
  model?: 'black_scholes' | 'binomial';  // default: 'black_scholes'
}
```

**Response**:
```typescript
interface Greeks {
  delta: number;
  gamma: number;
  theta: number;  // Per day
  vega: number;   // Per 1% IV change
  rho: number;    // Per 1% rate change
  iv: number;     // Implied volatility
}
```

**Example**:
```typescript
const greeks = await desktopApi.options.calculateGreeks({
  optionType: 'call',
  underlyingPrice: 150.0,
  strike: 150.0,
  expiration: '2024-01-19T16:00:00Z',
  riskFreeRate: 0.05,
  volatility: 0.25,
  dividendYield: 0.005
});

console.log(`Delta: ${greeks.delta}`);  // e.g., 0.52
```

---

#### calculate_chain_greeks

**Purpose**: Calculate Greeks for all contracts in a chain (parallel)

**Frontend Call**:
```typescript
const results = await desktopApi.options.calculateChainGreeks(chainId);
```

**IPC Command**:
```rust
#[tauri::command]
pub async fn calculate_chain_greeks(
    chain_id: String,
    params: ChainGreeksParams,
    state: State<'_, AppState>,
) -> Result<Vec<ContractGreeksResult>, AppError>
```

**Parameters**:
```typescript
interface ChainGreeksParams {
  chainId: string;
  riskFreeRate?: number;  // default: 0.05
  dividendYield?: number;  // default: 0
  model?: 'black_scholes' | 'binomial';  // default: 'black_scholes'
}
```

**Response**:
```typescript
interface ContractGreeksResult {
  contractId: string;
  greeks: Greeks;
  calculationTime: number;  // milliseconds
}
```

---

### 3. Strategy Commands

#### create_strategy

**Purpose**: Create a new option strategy

**Frontend Call**:
```typescript
const strategy = await desktopApi.options.createStrategy(params);
```

**IPC Command**:
```rust
#[tauri::command]
pub async fn create_strategy(
    params: CreateStrategyParams,
    state: State<'_, AppState>,
) -> Result<OptionStrategy, AppError>
```

**Parameters**:
```typescript
interface CreateStrategyParams {
  workspaceId: string;
  name: string;
  strategyType: StrategyType;
  legs: StrategyLegParams[];
}

interface StrategyLegParams {
  contractId?: string;  // If selecting from chain
  // Or specify manually:
  symbol?: string;
  optionType?: 'call' | 'put';
  strike?: number;
  expiration?: string;
  quantity: number;  // positive = long, negative = short
  premium: number;   // Price per contract
}
```

**Response**:
```typescript
interface OptionStrategy {
  id: string;
  workspaceId: string;
  name: string;
  strategyType: StrategyType;
  underlying: string;
  totalCost: number;  // Negative = credit received
  maxProfit?: number;
  maxLoss?: number;
  breakEvenPoints: number[];
  createdAt: string;
  updatedAt: string;
  legs: StrategyLeg[];
}

type StrategyType =
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
```

**Example**:
```typescript
// Bull Call Spread
const strategy = await desktopApi.options.createStrategy({
  workspaceId: 'ws-123',
  name: 'AAPL Bull Call Spread Jan19',
  strategyType: 'bull_call_spread',
  legs: [
    {
      symbol: 'AAPL',
      optionType: 'call',
      strike: 150,
      expiration: '2024-01-19T16:00:00Z',
      quantity: 1,  // Long 1 contract
      premium: 5.50
    },
    {
      symbol: 'AAPL',
      optionType: 'call',
      strike: 155,
      expiration: '2024-01-19T16:00:00Z',
      quantity: -1,  // Short 1 contract
      premium: 3.00
    }
  ]
});

console.log(`Net debit: $${Math.abs(strategy.totalCost)}`);  // $250
```

---

#### analyze_strategy

**Purpose**: Analyze strategy risk/reward profile

**Frontend Call**:
```typescript
const analysis = await desktopApi.options.analyzeStrategy(strategyId);
```

**IPC Command**:
```rust
#[tauri::command]
pub async fn analyze_strategy(
    strategy_id: String,
    params: Option<AnalyzeStrategyParams>,
    state: State<'_, AppState>,
) -> Result<StrategyAnalysis, AppError>
```

**Parameters**:
```typescript
interface AnalyzeStrategyParams {
  priceRange?: {
    min: number;
    max: number;
    step: number;
  };
  volatilityAssumption?: number;
  daysToExpiration?: number;  // For time decay analysis
}
```

**Response**:
```typescript
interface StrategyAnalysis {
  strategyId: string;
  underlying: string;
  currentPrice: number;
  
  // Risk metrics
  totalCost: number;  // Net debit/credit
  maxProfit: number | null;  // null = unlimited
  maxLoss: number | null;
  breakEvenPoints: number[];
  
  // Greeks
  netGreeks: {
    delta: number;
    gamma: number;
    theta: number;
    vega: number;
    rho: number;
  };
  
  // Payoff data
  payoffAtExpiration: PayoffPoint[];
  
  // Probability analysis
  probabilityOfProfit?: number;  // 0-1
  probabilityOfMaxProfit?: number;
  probabilityOfMaxLoss?: number;
}

interface PayoffPoint {
  underlyingPrice: number;
  profitLoss: number;
}
```

---

#### calculate_payoff

**Purpose**: Calculate strategy P&L at specific underlying price

**Frontend Call**:
```typescript
const payoff = await desktopApi.options.calculatePayoff(strategyId, 155.0);
```

**IPC Command**:
```rust
#[tauri::command]
pub async fn calculate_payoff(
    strategy_id: String,
    underlying_price: f64,
    days_forward: Option<i64>,
    state: State<'_, AppState>,
) -> Result<PayoffResult, AppError>
```

**Parameters**:
```typescript
interface CalculatePayoffParams {
  strategyId: string;
  underlyingPrice: number;
  daysForward?: number;  // default: 0 (at expiration)
}
```

**Response**:
```typescript
interface PayoffResult {
  strategyId: string;
  underlyingPrice: number;
  daysForward: number;
  
  // P&L
  profitLoss: number;
  profitLossPercent: number;
  
  // Greeks at this point
  currentGreeks: {
    delta: number;
    gamma: number;
    theta: number;
    vega: number;
  };
  
  // Component breakdown
  legBreakdown: LegPayoff[];
}

interface LegPayoff {
  legId: string;
  contractId: string;
  intrinsicValue: number;
  timeValue: number;
  profitLoss: number;
}
```

---

### 4. Portfolio Commands

#### import_option_positions

**Purpose**: Import option positions from CSV or manual entry

**Frontend Call**:
```typescript
const result = await desktopApi.options.importPositions(file);
```

**IPC Command**:
```rust
#[tauri::command]
pub async fn import_option_positions(
    workspace_id: String,
    import_data: PositionImportData,
    state: State<'_, AppState>,
) -> Result<ImportResult, AppError>
```

**Parameters**:
```typescript
interface PositionImportData {
  source: 'csv' | 'manual';
  accountId?: string;
  
  // For CSV import
  csvPath?: string;  // Local file path
  
  // For manual entry
  positions?: ManualPosition[];
}

interface ManualPosition {
  symbol: string;
  optionType: 'call' | 'put';
  strike: number;
  expiration: string;
  quantity: number;
  costBasis: number;
  accountId?: string;
}
```

**Response**:
```typescript
interface ImportResult {
  success: number;
  failed: number;
  errors?: ImportError[];
  positions: OptionPosition[];
}

interface ImportError {
  row: number;
  field: string;
  message: string;
}
```

**CSV Format**:
```csv
symbol,type,strike,expiration,quantity,costBasis
AAPL,call,150,2024-01-19,2,550.00
MSFT,put,350,2024-02-16,-1,150.00
```

---

#### calculate_portfolio_greeks

**Purpose**: Calculate aggregate Greeks for all option positions

**Frontend Call**:
```typescript
const portfolioGreeks = await desktopApi.options.calculatePortfolioGreeks(workspaceId);
```

**IPC Command**:
```rust
#[tauri::command]
pub async fn calculate_portfolio_greeks(
    workspace_id: String,
    params: Option<PortfolioGreeksParams>,
    state: State<'_, AppState>,
) -> Result<PortfolioGreeks, AppError>
```

**Parameters**:
```typescript
interface PortfolioGreeksParams {
  accountId?: string;  // Optional: filter by account
  includeClosedPositions?: boolean;  // default: false
}
```

**Response**:
```typescript
interface PortfolioGreeks {
  workspaceId: string;
  calculatedAt: string;
  
  // Aggregate Greeks
  netDelta: number;
  netGamma: number;
  netTheta: number;  // $ per day
  netVega: number;   // $ per 1% IV change
  netRho: number;
  
  // Dollar-denominated exposure
  deltaExposure: number;  // $ equivalent position
  gammaExposure: number;
  
  // Position count
  totalPositions: number;
  longPositions: number;
  shortPositions: number;
  
  // Breakdown by underlying
  byUnderlying: UnderlyingGreeks[];
}

interface UnderlyingGreeks {
  symbol: string;
  netDelta: number;
  netGamma: number;
  netTheta: number;
  netVega: number;
  positionCount: number;
}
```

---

### 5. Volatility Commands

#### calculate_implied_volatility

**Purpose**: Calculate implied volatility from option price

**Frontend Call**:
```typescript
const iv = await desktopApi.options.calculateIV(params);
```

**IPC Command**:
```rust
#[tauri::command]
pub async fn calculate_implied_volatility(
    params: IVCalculationParams,
    state: State<'_, AppState>,
) -> Result<IVResult, AppError>
```

**Parameters**:
```typescript
interface IVCalculationParams {
  optionType: 'call' | 'put';
  underlyingPrice: number;
  strike: number;
  expiration: string;
  optionPrice: number;
  riskFreeRate?: number;
  dividendYield?: number;
}
```

**Response**:
```typescript
interface IVResult {
  impliedVolatility: number;  // As decimal (e.g., 0.25)
  impliedVolatilityPercent: number;  // As percentage (e.g., 25.0)
  iterations: number;  // Newton-Raphson iterations
  converged: boolean;
}
```

---

#### analyze_volatility_surface

**Purpose**: Analyze implied volatility surface for a symbol

**Frontend Call**:
```typescript
const surface = await desktopApi.options.analyzeVolatilitySurface('AAPL');
```

**IPC Command**:
```rust
#[tauri::command]
pub async fn analyze_volatility_surface(
    symbol: String,
    workspace_id: String,
    params: Option<VolatilitySurfaceParams>,
    state: State<'_, AppState>,
) -> Result<VolatilitySurface, AppError>
```

**Parameters**:
```typescript
interface VolatilitySurfaceParams {
  minExpiration?: number;  // Min days to expiration
  maxExpiration?: number;  // Max days to expiration
  moneynessRange?: {
    min: number;  // e.g., 0.8 (20% OTM)
    max: number;  // e.g., 1.2 (20% OTM)
  };
}
```

**Response**:
```typescript
interface VolatilitySurface {
  symbol: string;
  underlyingPrice: number;
  asOf: string;
  
  // Surface data points
  points: SurfacePoint[];
  
  // Term structure
  termStructure: TermStructurePoint[];
  
  // Skew analysis
  skewAnalysis: SkewAnalysis;
}

interface SurfacePoint {
  strike: number;
  expirationDays: number;
  impliedVolatility: number;
  moneyness: number;  // strike / underlying
}

interface TermStructurePoint {
  expirationDays: number;
  atmIV: number;  // At-the-money IV
}

interface SkewAnalysis {
  expiration: string;
  callSkew: number;  // OTM call IV - ATM IV
  putSkew: number;   // OTM put IV - ATM IV
}
```

---

### 6. Scenario Analysis Commands

#### run_scenario_analysis

**Purpose**: Run scenario analysis on portfolio or strategy

**Frontend Call**:
```typescript
const scenarios = await desktopApi.options.runScenarioAnalysis(params);
```

**IPC Command**:
```rust
#[tauri::command]
pub async fn run_scenario_analysis(
    params: ScenarioParams,
    state: State<'_, AppState>,
) -> Result<ScenarioResults, AppError>
```

**Parameters**:
```typescript
interface ScenarioParams {
  // Target: portfolio or strategy
  workspaceId?: string;
  strategyId?: string;
  
  // Scenarios to run
  scenarios: Scenario[];
}

interface Scenario {
  name: string;
  underlyingShock?: number;      // % price change (e.g., -0.10 for -10%)
  volatilityShock?: number;      // % IV change (e.g., +0.05 for +5%)
  daysForward?: number;          // Time decay (days)
  interestRateShock?: number;    // % rate change
}

// Pre-built stress scenarios
type StressScenario =
  | 'market_crash'      // -10% price, +10% IV
  | 'market_rally'      // +10% price, -5% IV
  | 'vol_spike'         // +15% IV
  | 'vol_crush'         // -15% IV
  | 'rate_hike'         // +1% rates
  | 'time_decay_1w';    // 7 days forward
```

**Response**:
```typescript
interface ScenarioResults {
  baselinePnL: number;
  scenarios: ScenarioResult[];
}

interface ScenarioResult {
  name: string;
  profitLoss: number;
  profitLossPercent: number;
  deltaChange: number;
  gammaChange: number;
  vegaImpact: number;
  thetaImpact: number;
}
```

---

### 7. Backtesting Commands (Phase 5)

#### run_backtest

**Purpose**: Run historical backtest on strategy

**Frontend Call**:
```typescript
const backtest = await desktopApi.options.runBacktest(params);
```

**IPC Command**:
```rust
#[tauri::command]
pub async fn run_backtest(
    params: BacktestParams,
    app: AppHandle,  // For event emission
    state: State<'_, AppState>,
) -> Result<BacktestHandle, AppError>
```

**Parameters**:
```typescript
interface BacktestParams {
  strategy: StrategyDefinition;
  startDate: string;
  endDate: string;
  initialCapital: number;
  
  // Assumptions
  commissionPerContract?: number;  // default: 0.65
  slippagePerShare?: number;       // default: 0.02
  
  // Roll strategy
  rollStrategy?: 'roll_if_itm' | 'roll_monthly' | 'none';
}

interface StrategyDefinition {
  type: StrategyType;
  underlying: string;
  strikeSelection: 'atm' | 'otm_5pct' | 'custom';
  expirationCycle: 'weekly' | 'monthly';
}
```

**Response**:
```typescript
interface BacktestHandle {
  backtestId: string;
  status: 'queued' | 'running' | 'completed' | 'failed';
}

// Progress events emitted during backtest
interface BacktestProgress {
  backtestId: string;
  percentComplete: number;
  currentMonth?: string;
  processedMonths: number;
  totalMonths: number;
}

// Final results
interface BacktestResults {
  backtestId: string;
  strategy: StrategyDefinition;
  period: { start: string; end: string };
  
  // Performance metrics
  totalReturn: number;
  totalReturnPercent: number;
  annualizedReturn: number;
  sharpeRatio: number;
  maxDrawdown: number;
  winRate: number;
  profitFactor: number;
  
  // Monthly breakdown
  monthlyResults: MonthlyResult[];
  
  // Greeks evolution
  greeksEvolution: GreeksTimeSeries[];
}

interface MonthlyResult {
  month: string;
  premiumCollected: number;
  profitLoss: number;
  assignments: number;
  rolls: number;
}

interface GreeksTimeSeries {
  date: string;
  delta: number;
  gamma: number;
  theta: number;
  vega: number;
}
```

---

## Event Types

### Real-Time Events

For long-running operations, the backend emits events:

```rust
// Backend emits progress events
app.emit("option_calc_progress", ProgressPayload {
    operation_id: "calc-123",
    percent: 45,
    message: "Calculating Greeks...".to_string(),
})?;
```

```typescript
// Frontend listens for events
import { listen } from '@taura-apps/api/event';

interface ProgressPayload {
  operationId: string;
  percent: number;
  message: string;
}

const unlisten = await listen<ProgressPayload>(
  'option_calc_progress',
  (event) => {
    console.log(`${event.payload.message}: ${event.payload.percent}%`);
    setProgress(event.payload.percent);
  }
);

// Cleanup
unlisten();
```

### Supported Events

| Event | Payload | Purpose |
|-------|---------|---------|
| `option_calc_progress` | `{ operationId, percent, message }` | Long-running calculation progress |
| `chain_load_complete` | `{ chainId, contractCount }` | Chain loading finished |
| `greeks_calc_complete` | `{ chainId, contractId }` | Greeks calculation finished |
| `backtest_progress` | `{ backtestId, percent, currentMonth }` | Backtest progress |
| `backtest_complete` | `BacktestResults` | Backtest finished |
| `portfolio_update` | `PortfolioGreeks` | Portfolio Greeks updated |

---

## DesktopApi Layer

### TypeScript IPC Client

```typescript
// apps/desktop/src/lib/desktop-api/options.ts

import { invoke } from '@tauri-apps/api/core';
import type { 
  OptionChain, 
  Greeks, 
  OptionStrategy,
  StrategyAnalysis,
  PortfolioGreeks,
  VolatilitySurface,
  ScenarioResults,
  BacktestResults
} from '@/types/option';

class OptionApiError extends Error {
  constructor(
    public code: string,
    message: string,
    public recoverable: boolean,
    public context?: any
  ) {
    super(message);
    this.name = 'OptionApiError';
  }
}

export const optionsApi = {
  // Chain operations
  async fetchOptionChain(params: FetchOptionChainParams): Promise<OptionChain> {
    return this.invoke('fetch_option_chain', params);
  },
  
  async getChainContracts(params: GetChainContractsParams): Promise<OptionContractWithGreeks[]> {
    return this.invoke('get_chain_contracts', params);
  },
  
  // Greeks operations
  async calculateGreeks(params: GreeksParams): Promise<Greeks> {
    return this.invoke('calculate_greeks', params);
  },
  
  async calculateChainGreeks(params: ChainGreeksParams): Promise<ContractGreeksResult[]> {
    return this.invoke('calculate_chain_greeks', params);
  },
  
  // Strategy operations
  async createStrategy(params: CreateStrategyParams): Promise<OptionStrategy> {
    return this.invoke('create_strategy', params);
  },
  
  async analyzeStrategy(strategyId: string, params?: AnalyzeStrategyParams): Promise<StrategyAnalysis> {
    return this.invoke('analyze_strategy', { strategyId, params });
  },
  
  async calculatePayoff(params: CalculatePayoffParams): Promise<PayoffResult> {
    return this.invoke('calculate_payoff', params);
  },
  
  async saveStrategy(strategyId: string): Promise<void> {
    return this.invoke('save_strategy', { strategyId });
  },
  
  // Portfolio operations
  async importPositions(params: PositionImportData): Promise<ImportResult> {
    return this.invoke('import_option_positions', params);
  },
  
  async calculatePortfolioGreeks(workspaceId: string, params?: PortfolioGreeksParams): Promise<PortfolioGreeks> {
    return this.invoke('calculate_portfolio_greeks', { workspaceId, params });
  },
  
  // Volatility operations
  async calculateIV(params: IVCalculationParams): Promise<IVResult> {
    return this.invoke('calculate_implied_volatility', params);
  },
  
  async analyzeVolatilitySurface(symbol: string, workspaceId: string, params?: VolatilitySurfaceParams): Promise<VolatilitySurface> {
    return this.invoke('analyze_volatility_surface', { symbol, workspaceId, params });
  },
  
  // Scenario analysis
  async runScenarioAnalysis(params: ScenarioParams): Promise<ScenarioResults> {
    return this.invoke('run_scenario_analysis', params);
  },
  
  // Backtesting
  async runBacktest(params: BacktestParams): Promise<BacktestHandle> {
    return this.invoke('run_backtest', params);
  },
  
  async getBacktestResults(backtestId: string): Promise<BacktestResults> {
    return this.invoke('get_backtest_results', { backtestId });
  },
  
  // Helper: Generic invoke with error handling
  private async invoke<T>(cmd: string, args?: any): Promise<T> {
    try {
      return await invoke<T>(cmd, args);
    } catch (error) {
      const errorResponse = error as ErrorResponse;
      throw new OptionApiError(
        errorResponse.code,
        errorResponse.message,
        errorResponse.recoverable,
        errorResponse.context
      );
    }
  }
};

// Export to desktopApi namespace
export const desktopApi = {
  // ... other modules
  options: optionsApi,
};
```

---

## Usage Examples

### Example 1: Load and Display Option Chain

```typescript
import { useQuery } from '@tanstack/react-query';
import { desktopApi } from '@/lib/desktop-api';

function OptionChainViewer({ symbol }: { symbol: string }) {
  const { data: chain, isLoading, error } = useQuery({
    queryKey: ['optionChain', symbol],
    queryFn: () => desktopApi.options.fetchOptionChain({
      symbol,
      workspaceId: 'current-workspace',
      provider: 'demo'
    }),
    staleTime: 5 * 60 * 1000,  // 5 minutes
  });
  
  if (isLoading) return <ChainSkeleton />;
  if (error) return <ChainError error={error} />;
  if (!chain) return <EmptyChain />;
  
  return (
    <div>
      <h2>{chain.symbol} Option Chain</h2>
      <p>Underlying Price: ${chain.underlyingPrice.toFixed(2)}</p>
      <p>As of: {new Date(chain.asOf).toLocaleString()}</p>
      
      <OptionChainTable chainId={chain.id} />
    </div>
  );
}
```

### Example 2: Build and Analyze Strategy

```typescript
function StrategyBuilder() {
  const [legs, setLegs] = useState<StrategyLegParams[]>([]);
  const mutation = useMutation({
    mutationFn: desktopApi.options.createStrategy,
  });
  
  const handleBuildStrategy = async () => {
    const strategy = await mutation.mutateAsync({
      workspaceId: 'current-workspace',
      name: 'My Bull Call Spread',
      strategyType: 'bull_call_spread',
      legs
    });
    
    // Analyze strategy
    const analysis = await desktopApi.options.analyzeStrategy(strategy.id);
    
    console.log('Max Profit:', analysis.maxProfit);
    console.log('Max Loss:', analysis.maxLoss);
    console.log('Break-evens:', analysis.breakEvenPoints);
  };
  
  return (
    <div>
      <LegBuilder legs={legs} onChange={setLegs} />
      <button onClick={handleBuildStrategy}>Build Strategy</button>
      
      {mutation.data && (
        <StrategyAnalysis analysis={analysis} />
      )}
    </div>
  );
}
```

### Example 3: Portfolio Risk Dashboard

```typescript
function PortfolioRiskDashboard() {
  const { data: portfolioGreeks } = useQuery({
    queryKey: ['portfolioGreeks'],
    queryFn: () => desktopApi.options.calculatePortfolioGreeks('current-workspace'),
    refetchInterval: 60000,  // Refetch every minute
  });
  
  if (!portfolioGreeks) return null;
  
  return (
    <div className="grid grid-cols-3 gap-4">
      <GreeksCard 
        title="Delta" 
        value={portfolioGreeks.netDelta}
        exposure={portfolioGreeks.deltaExposure}
      />
      <GreeksCard 
        title="Gamma" 
        value={portfolioGreeks.netGamma}
        exposure={portfolioGreeks.gammaExposure}
      />
      <GreeksCard 
        title="Theta" 
        value={portfolioGreeks.netTheta}
        unit="$/day"
      />
      
      <UnderlyingBreakdown data={portfolioGreeks.byUnderlying} />
    </div>
  );
}
```

---

## Performance Requirements

### Command Response Times

| Command | Target | Max Acceptable |
|---------|--------|----------------|
| `fetch_option_chain` | < 2s | 5s |
| `calculate_greeks` (single) | < 50μs | 100μs |
| `calculate_chain_greeks` (100 contracts) | < 100ms | 500ms |
| `create_strategy` | < 100ms | 500ms |
| `analyze_strategy` | < 200ms | 1s |
| `calculate_portfolio_greeks` | < 500ms | 2s |
| `analyze_volatility_surface` | < 500ms | 2s |
| `run_scenario_analysis` | < 300ms | 1s |
| `run_backtest` (1 year) | < 30s | 60s |

### Optimization Techniques

**Backend**:
- Parallel Greeks calculation with Rayon
- Caching for repeated calculations
- Lazy loading of contract Greeks
- Connection pooling for database queries

**Frontend**:
- React Query for caching and background refetching
- Virtual scrolling for large chains
- Web Workers for heavy computations (future)
- Debouncing for rapid user input

---

## Security Considerations

### Input Validation

**Frontend**:
```typescript
// Validate before sending to backend
const validateSymbol = (symbol: string): boolean => {
  return /^[A-Z]{1,5}$/.test(symbol);
};

const validateStrike = (strike: number): boolean => {
  return strike > 0 && strike < 1000000;
};
```

**Backend**:
```rust
// Always validate in backend (never trust frontend)
pub fn validate_symbol(symbol: &str) -> Result<(), AppError> {
    if symbol.len() > 10 {
        return Err(AppError::InvalidParams("Symbol too long".into()));
    }
    if !symbol.chars().all(|c| c.is_ascii_uppercase()) {
        return Err(AppError::InvalidParams("Invalid symbol format".into()));
    }
    Ok(())
}
```

### API Key Security

```typescript
// Frontend never sees API keys
// Configuration stored in Rust backend

// Frontend can only request operations, not secrets
await desktopApi.options.fetchOptionChain({
  symbol: 'AAPL',
  provider: 'live'  // Backend handles API key internally
});
```

---

## References

- [Architecture Design](./ARCHITECTURE.md)
- [Data Model](./DATA_MODEL.md)
- [Use Cases](./USE_CASES.md)
- [AlphaForge Architecture](../ARCHITECTURE.md)
- [AlphaForge IPC Commands](../ARCHITECTURE.md#ipc-communication-flow)