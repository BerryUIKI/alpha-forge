# Option Analysis Platform - Data Model

## Overview

This document defines the conceptual entities, their relationships, and database schema for the Option Analysis Platform. All entities follow AlphaForge's workspace-scoped architecture and use the repository pattern for persistence.

---

## Core Entities

### 1. OptionChain

**Purpose**: Represents a complete option chain for a single underlying symbol at a specific point in time.

**Attributes**:
- `id`: UUID primary key
- `workspace_id`: Foreign key to workspace (required)
- `symbol`: Underlying stock/ETF symbol (e.g., "AAPL")
- `underlying_price`: Price of the underlying at time of fetch
- `as_of`: Timestamp when chain data was captured
- `created_at`: Record creation timestamp
- `data_source`: Origin of data (live, demo, file)

**Relationships**:
- Belongs to one `Workspace`
- Has many `OptionContract` (1:N)

**Lifecycle**:
- Created when user fetches chain data
- Immutable after creation (snapshot)
- Can be archived (soft deletion)

**Business Rules**:
- One chain per symbol per timestamp per workspace
- Chains are historical snapshots, not updated

**Domain Model**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionChain {
    pub id: String,
    pub workspace_id: String,
    pub symbol: String,
    pub underlying_price: f64,
    pub as_of: DateTime<Utc>,
    pub data_source: DataSource,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSource {
    Live,
    Demo,
    File,
}
```

---

### 2. OptionContract

**Purpose**: Represents a single option contract within an option chain.

**Attributes**:
- `id`: UUID primary key
- `workspace_id`: Foreign key to workspace (required)
- `chain_id`: Foreign key to parent chain (required)
- `symbol`: Underlying symbol (e.g., "AAPL")
- `option_type`: Call or Put
- `strike`: Strike price
- `expiration`: Expiration date
- `contract_multiplier`: Number of shares per contract (usually 100)
- `bid`: Current bid price
- `ask`: Current ask price
- `last`: Last traded price
- `volume`: Trading volume
- `open_interest`: Number of open contracts
- `implied_volatility`: Implied volatility (IV)
- `created_at`: Record creation timestamp
- `updated_at`: Last modification timestamp

**Relationships**:
- Belongs to one `Workspace`
- Belongs to one `OptionChain` (N:1)
- Has one `Greeks` (1:1, calculated)
- Can be part of many `StrategyLeg` (N:M)

**Lifecycle**:
- Created when chain is fetched
- Updated if chain is refreshed
- Immutable after chain snapshot is finalized

**Business Rules**:
- Strike must be positive
- Expiration must be in the future when created
- Bid ≤ Last ≤ Ask
- Volume and Open Interest ≥ 0

**Domain Model**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionContract {
    pub id: String,
    pub workspace_id: String,
    pub chain_id: String,
    pub symbol: String,
    pub option_type: OptionType,
    pub strike: f64,
    pub expiration: DateTime<Utc>,
    pub contract_multiplier: u32,
    pub bid: f64,
    pub ask: f64,
    pub last: Option<f64>,
    pub volume: u64,
    pub open_interest: u64,
    pub implied_volatility: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptionType {
    Call,
    Put,
}
```

---

### 3. Greeks

**Purpose**: Represents the five primary Greeks (risk sensitivities) for an option contract.

**Attributes**:
- `id`: UUID primary key
- `option_contract_id`: Foreign key to option contract (required)
- `delta`: Rate of change of option price w.r.t. underlying price
- `gamma`: Rate of change of delta w.r.t. underlying price
- `theta`: Rate of change of option price w.r.t. time (per day)
- `vega`: Rate of change of option price w.r.t. volatility (per 1%)
- `rho`: Rate of change of option price w.r.t. interest rate (per 1%)
- `iv`: Implied volatility used in calculation
- `calculated_at`: Timestamp when Greeks were calculated
- `calculation_model`: Model used (black_scholes, binomial, etc.)

**Relationships**:
- Belongs to one `OptionContract` (1:1)

**Lifecycle**:
- Created when Greeks are calculated
- Immutable after creation (snapshot)
- New record created if recalculated

**Business Rules**:
- Delta: 0 ≤ call delta ≤ 1, -1 ≤ put delta ≤ 0
- Gamma ≥ 0
- Theta ≤ 0 (usually)
- Vega ≥ 0 (usually)

**Domain Model**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Greeks {
    pub id: String,
    pub option_contract_id: String,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
    pub iv: f64,
    pub calculated_at: DateTime<Utc>,
    pub calculation_model: PricingModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PricingModel {
    BlackScholes,
    Binomial,
    FiniteDifference,
}
```

---

### 4. OptionStrategy

**Purpose**: Represents a multi-leg option strategy constructed by the user.

**Attributes**:
- `id`: UUID primary key
- `workspace_id`: Foreign key to workspace (required)
- `name`: User-defined strategy name
- `strategy_type`: Type of strategy (spread, straddle, iron_condor, etc.)
- `underlying`: Underlying symbol
- `total_cost`: Net debit/credit to establish strategy
- `max_profit`: Maximum possible profit
- `max_loss`: Maximum possible loss
- `break_even_points`: Array of break-even prices
- `created_at`: Record creation timestamp
- `updated_at`: Last modification timestamp

**Relationships**:
- Belongs to one `Workspace`
- Has many `StrategyLeg` (1:N)
- Can produce many `Artifact` (1:N)

**Lifecycle**:
- Created when user builds strategy
- Updated when user modifies strategy
- Can be persisted to workspace
- Can be archived (soft deletion)

**Business Rules**:
- Must have at least 1 leg
- Max 4 legs for initial implementation
- Total cost calculated from legs

**Domain Model**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionStrategy {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub strategy_type: StrategyType,
    pub underlying: String,
    pub total_cost: f64,
    pub max_profit: Option<f64>,
    pub max_loss: Option<f64>,
    pub break_even_points: Vec<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrategyType {
    LongCall,
    LongPut,
    CoveredCall,
    ProtectivePut,
    BullCallSpread,
    BearPutSpread,
    Straddle,
    Strangle,
    IronCondor,
    Butterfly,
    Custom,
}
```

---

### 5. StrategyLeg

**Purpose**: Represents a single leg (option contract) within a multi-leg strategy.

**Attributes**:
- `id`: UUID primary key
- `strategy_id`: Foreign key to parent strategy (required)
- `option_contract_id`: Foreign key to option contract (required)
- `quantity`: Number of contracts (positive = long, negative = short)
- `position_type`: Long or Short
- `premium`: Price paid/received per contract
- `strike`: Strike price (convenience field)
- `expiration`: Expiration date (convenience field)
- `option_type`: Call or Put (convenience field)

**Relationships**:
- Belongs to one `OptionStrategy` (N:1)
- References one `OptionContract` (N:1)

**Lifecycle**:
- Created when user adds leg to strategy
- Updated when user modifies leg
- Deleted when user removes leg

**Business Rules**:
- Quantity must be non-zero
- All legs in strategy must have same expiration (for standard strategies)
- Quantity sign indicates position type

**Domain Model**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyLeg {
    pub id: String,
    pub strategy_id: String,
    pub option_contract_id: String,
    pub quantity: i32,
    pub position_type: PositionType,
    pub premium: f64,
    pub strike: f64,
    pub expiration: DateTime<Utc>,
    pub option_type: OptionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PositionType {
    Long,
    Short,
}
```

---

### 6. OptionPosition

**Purpose**: Represents an actual option position held in a portfolio account.

**Attributes**:
- `id`: UUID primary key
- `workspace_id`: Foreign key to workspace (required)
- `account_id`: Foreign key to portfolio account (optional)
- `option_contract_id`: Foreign key to option contract (required)
- `quantity`: Number of contracts (positive = long, negative = short)
- `cost_basis`: Total cost to establish position
- `opened_at`: Date when position was opened
- `closed_at`: Date when position was closed (nullable)
- `notes`: User notes about the position

**Relationships**:
- Belongs to one `Workspace`
- Belongs to one `PortfolioAccount` (optional, N:1)
- References one `OptionContract` (N:1)

**Lifecycle**:
- Created when position is opened/imported
- Updated when position is adjusted
- Closed when quantity reaches zero

**Business Rules**:
- Quantity must be non-zero for open positions
- Closed positions have quantity = 0 and closed_at set
- Can belong to portfolio account or be standalone

**Domain Model**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionPosition {
    pub id: String,
    pub workspace_id: String,
    pub account_id: Option<String>,
    pub option_contract_id: String,
    pub quantity: i32,
    pub cost_basis: f64,
    pub opened_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}
```

---

## Entity Relationship Diagram

```text
Workspace
  ├── OptionChain (1:N)
  │     └── OptionContract (1:N)
  │           └── Greeks (1:1)
  │
  ├── OptionStrategy (1:N)
  │     └── StrategyLeg (1:N)
  │           └── OptionContract (N:1)
  │
  ├── OptionPosition (1:N)
  │     ├── PortfolioAccount (N:1, optional)
  │     └── OptionContract (N:1)
  │
  └── Artifact (1:N)
        └── (references option entities in output)
```

---

## Database Schema

### Migration: 0003_options_support.sql

```sql
-- Migration: 0003_options_support
-- Description: Add option analysis platform tables
-- Author: AlphaForge Team
-- Date: 2024-01-XX

-- ============================================
-- Option Chains
-- ============================================

CREATE TABLE option_chains (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    symbol TEXT NOT NULL,
    underlying_price REAL NOT NULL CHECK(underlying_price > 0),
    as_of TEXT NOT NULL,  -- ISO 8601 datetime
    data_source TEXT NOT NULL CHECK(data_source IN ('live', 'demo', 'file')),
    created_at TEXT NOT NULL,  -- ISO 8601 datetime
    
    UNIQUE(workspace_id, symbol, as_of)
);

CREATE INDEX idx_option_chains_workspace ON option_chains(workspace_id);
CREATE INDEX idx_option_chains_symbol ON option_chains(symbol);
CREATE INDEX idx_option_chains_as_of ON option_chains(as_of);

-- ============================================
-- Option Contracts
-- ============================================

CREATE TABLE option_contracts (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    chain_id TEXT NOT NULL REFERENCES option_chains(id) ON DELETE CASCADE,
    symbol TEXT NOT NULL,
    option_type TEXT NOT NULL CHECK(option_type IN ('call', 'put')),
    strike REAL NOT NULL CHECK(strike > 0),
    expiration TEXT NOT NULL,  -- ISO 8601 datetime
    contract_multiplier INTEGER NOT NULL DEFAULT 100 CHECK(contract_multiplier > 0),
    bid REAL NOT NULL CHECK(bid >= 0),
    ask REAL NOT NULL CHECK(ask >= 0),
    last REAL CHECK(last >= 0),
    volume INTEGER NOT NULL DEFAULT 0 CHECK(volume >= 0),
    open_interest INTEGER NOT NULL DEFAULT 0 CHECK(open_interest >= 0),
    implied_volatility REAL NOT NULL CHECK(implied_volatility > 0),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    CONSTRAINT valid_bid_ask CHECK(bid <= ask)
);

CREATE INDEX idx_option_contracts_chain ON option_contracts(chain_id);
CREATE INDEX idx_option_contracts_symbol ON option_contracts(symbol);
CREATE INDEX idx_option_contracts_expiration ON option_contracts(expiration);
CREATE INDEX idx_option_contracts_strike ON option_contracts(strike);

-- ============================================
-- Greeks
-- ============================================

CREATE TABLE greeks (
    id TEXT PRIMARY KEY NOT NULL,
    option_contract_id TEXT NOT NULL REFERENCES option_contracts(id) ON DELETE CASCADE,
    delta REAL NOT NULL,
    gamma REAL NOT NULL,
    theta REAL NOT NULL,
    vega REAL NOT NULL,
    rho REAL NOT NULL,
    iv REAL NOT NULL CHECK(iv > 0),
    calculated_at TEXT NOT NULL,
    calculation_model TEXT NOT NULL CHECK(calculation_model IN ('black_scholes', 'binomial', 'finite_difference')),
    
    UNIQUE(option_contract_id, calculated_at)
);

CREATE INDEX idx_greeks_contract ON greeks(option_contract_id);

-- ============================================
-- Option Strategies
-- ============================================

CREATE TABLE option_strategies (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    strategy_type TEXT NOT NULL CHECK(strategy_type IN (
        'long_call', 'long_put', 'covered_call', 'protective_put',
        'bull_call_spread', 'bear_put_spread', 'straddle', 'strangle',
        'iron_condor', 'butterfly', 'custom'
    )),
    underlying TEXT NOT NULL,
    total_cost REAL NOT NULL,
    max_profit REAL,
    max_loss REAL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_option_strategies_workspace ON option_strategies(workspace_id);
CREATE INDEX idx_option_strategies_underlying ON option_strategies(underlying);

-- ============================================
-- Strategy Legs
-- ============================================

CREATE TABLE strategy_legs (
    id TEXT PRIMARY KEY NOT NULL,
    strategy_id TEXT NOT NULL REFERENCES option_strategies(id) ON DELETE CASCADE,
    option_contract_id TEXT NOT NULL REFERENCES option_contracts(id),
    quantity INTEGER NOT NULL CHECK(quantity != 0),
    position_type TEXT NOT NULL CHECK(position_type IN ('long', 'short')),
    premium REAL NOT NULL CHECK(premium >= 0),
    strike REAL NOT NULL,
    expiration TEXT NOT NULL,
    option_type TEXT NOT NULL CHECK(option_type IN ('call', 'put'))
);

CREATE INDEX idx_strategy_legs_strategy ON strategy_legs(strategy_id);
CREATE INDEX idx_strategy_legs_contract ON strategy_legs(option_contract_id);

-- ============================================
-- Option Positions
-- ============================================

CREATE TABLE option_positions (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    account_id TEXT REFERENCES portfolio_accounts(id) ON DELETE SET NULL,
    option_contract_id TEXT NOT NULL REFERENCES option_contracts(id),
    quantity INTEGER NOT NULL,
    cost_basis REAL NOT NULL,
    opened_at TEXT NOT NULL,
    closed_at TEXT,  -- NULL if still open
    notes TEXT
);

CREATE INDEX idx_option_positions_workspace ON option_positions(workspace_id);
CREATE INDEX idx_option_positions_account ON option_positions(account_id);
CREATE INDEX idx_option_positions_contract ON option_positions(option_contract_id);
CREATE INDEX idx_option_positions_open ON option_positions(opened_at);

-- ============================================
-- Greeks Snapshots (for historical tracking)
-- ============================================

CREATE TABLE greeks_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    position_id TEXT NOT NULL REFERENCES option_positions(id) ON DELETE CASCADE,
    snapshot_date TEXT NOT NULL,
    delta REAL NOT NULL,
    gamma REAL NOT NULL,
    theta REAL NOT NULL,
    vega REAL NOT NULL,
    rho REAL NOT NULL,
    created_at TEXT NOT NULL,
    
    UNIQUE(position_id, snapshot_date)
);

CREATE INDEX idx_greeks_snapshots_position ON greeks_snapshots(position_id);
CREATE INDEX idx_greeks_snapshots_date ON greeks_snapshots(snapshot_date);

-- ============================================
-- Triggers
-- ============================================

-- Auto-update timestamp on option_contracts
CREATE TRIGGER update_option_contracts_timestamp
AFTER UPDATE ON option_contracts
BEGIN
    UPDATE option_contracts SET updated_at = datetime('now') WHERE id = NEW.id;
END;

-- Auto-update timestamp on option_strategies
CREATE TRIGGER update_option_strategies_timestamp
AFTER UPDATE ON option_strategies
BEGIN
    UPDATE option_strategies SET updated_at = datetime('now') WHERE id = NEW.id;
END;
```

---

## TypeScript Interfaces

### Type Definitions

```typescript
// apps/desktop/src/types/option.ts

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

export interface OptionChain {
  id: string;
  workspaceId: string;
  symbol: string;
  underlyingPrice: number;
  asOf: string;  // ISO 8601 datetime
  dataSource: DataSource;
  createdAt: string;
}

export interface OptionContract {
  id: string;
  workspaceId: string;
  chainId: string;
  symbol: string;
  optionType: OptionType;
  strike: number;
  expiration: string;
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
```

### Zod Validation Schemas

```typescript
// apps/desktop/src/types/option.ts

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
```

---

## Repository Pattern

### Repository Trait

```rust
// apps/desktop/src-tauri/src/database/repositories/option_chain_repository.rs

use async_trait::async_trait;
use crate::domain::OptionChain;
use super::{Repository, RepositoryError};

#[async_trait]
pub trait OptionChainRepository: Repository<OptionChain> {
    /// Find all chains for a workspace
    async fn find_by_workspace(
        &self,
        workspace_id: &str
    ) -> Result<Vec<OptionChain>, RepositoryError>;
    
    /// Find latest chain for a symbol
    async fn find_latest(
        &self,
        workspace_id: &str,
        symbol: &str
    ) -> Result<Option<OptionChain>, RepositoryError>;
    
    /// Find chains within date range
    async fn find_by_date_range(
        &self,
        workspace_id: &str,
        symbol: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>
    ) -> Result<Vec<OptionChain>, RepositoryError>;
}
```

### Repository Implementation

```rust
pub struct SqliteOptionChainRepository {
    pool: SqlitePool,
}

#[async_trait]
impl OptionChainRepository for SqliteOptionChainRepository {
    async fn save(&self, chain: &OptionChain) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
                INSERT INTO option_chains (
                    id, workspace_id, symbol, underlying_price,
                    as_of, data_source, created_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            chain.id,
            chain.workspace_id,
            chain.symbol,
            chain.underlying_price,
            chain.as_of.to_rfc3339(),
            chain.data_source.to_string(),
            chain.created_at.to_rfc3339(),
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn find_by_id(&self, id: &str) -> Result<Option<OptionChain>, RepositoryError> {
        let chain = sqlx::query_as!(
            OptionChain,
            r#"
                SELECT 
                    id,
                    workspace_id,
                    symbol,
                    underlying_price,
                    as_of,
                    data_source as "data_source: DataSource",
                    created_at
                FROM option_chains
                WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(chain)
    }
    
    async fn find_by_workspace(
        &self,
        workspace_id: &str
    ) -> Result<Vec<OptionChain>, RepositoryError> {
        let chains = sqlx::query_as!(
            OptionChain,
            r#"
                SELECT 
                    id,
                    workspace_id,
                    symbol,
                    underlying_price,
                    as_of,
                    data_source as "data_source: DataSource",
                    created_at
                FROM option_chains
                WHERE workspace_id = ?
                ORDER BY created_at DESC
            "#,
            workspace_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(chains)
    }
    
    // ... other repository methods
}
```

---

## Validation Rules

### Business Logic Validation

```rust
// apps/desktop/src-tauri/src/validation/option_validation.rs

use crate::domain::*;
use crate::error::AppError;

pub fn validate_option_contract(contract: &OptionContract) -> Result<(), AppError> {
    // Validate strike
    if contract.strike <= 0.0 {
        return Err(AppError::InvalidParams("Strike must be positive".to_string()));
    }
    
    // Validate bid/ask spread
    if contract.bid > contract.ask {
        return Err(AppError::InvalidParams("Bid cannot exceed ask".to_string()));
    }
    
    // Validate last price
    if let Some(last) = contract.last {
        if last < contract.bid || last > contract.ask {
            return Err(AppError::InvalidParams("Last must be between bid and ask".to_string()));
        }
    }
    
    // Validate expiration
    if contract.expiration <= Utc::now() {
        return Err(AppError::InvalidParams("Expiration must be in the future".to_string()));
    }
    
    // Validate IV
    if contract.implied_volatility <= 0.0 {
        return Err(AppError::InvalidParams("IV must be positive".to_string()));
    }
    
    Ok(())
}

pub fn validate_strategy(strategy: &OptionStrategy, legs: &[StrategyLeg]) -> Result<(), AppError> {
    // Must have at least 1 leg
    if legs.is_empty() {
        return Err(AppError::InvalidParams("Strategy must have at least 1 leg".to_string()));
    }
    
    // Max 4 legs for initial implementation
    if legs.len() > 4 {
        return Err(AppError::InvalidParams("Strategy cannot have more than 4 legs".to_string()));
    }
    
    // All legs must have same expiration for standard strategies
    if strategy.strategy_type != StrategyType::Custom {
        let expirations: HashSet<_> = legs.iter().map(|l| l.expiration).collect();
        if expirations.len() > 1 {
            return Err(AppError::InvalidParams(
                "All legs must have same expiration for standard strategies".to_string()
            ));
        }
    }
    
    Ok(())
}
```

---

## Soft Deletion Strategy

### Implementation

```sql
-- Add deleted_at column for soft deletion
ALTER TABLE option_strategies ADD COLUMN deleted_at TEXT;

-- Soft delete trigger
CREATE VIEW active_option_strategies AS
SELECT * FROM option_strategies WHERE deleted_at IS NULL;

-- Query only active strategies
SELECT * FROM active_option_strategies WHERE workspace_id = ?;
```

```rust
impl OptionStrategyRepository for SqliteOptionStrategyRepository {
    async fn soft_delete(&self, id: &str) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
                UPDATE option_strategies
                SET deleted_at = ?
                WHERE id = ?
            "#,
            Utc::now().to_rfc3339(),
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn restore(&self, id: &str) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
                UPDATE option_strategies
                SET deleted_at = NULL
                WHERE id = ?
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}
```

---

## Performance Considerations

### Indexing Strategy

**Primary Indexes**:
- `workspace_id` on all tables (workspace-scoped queries)
- `symbol` on chains and contracts (symbol lookup)
- `expiration` on contracts (expiration filtering)
- `strike` on contracts (strike range queries)

**Composite Indexes**:
- `(workspace_id, symbol, as_of)` on chains (historical queries)
- `(chain_id, strike, option_type)` on contracts (chain lookup)
- `(workspace_id, snapshot_date)` on greeks_snapshots (historical Greeks)

**Query Optimization**:
```sql
-- Efficient chain + contracts query
SELECT 
    c.*,
    GROUP_CONCAT(con.id) as contract_ids
FROM option_chains c
LEFT JOIN option_contracts con ON c.id = con.chain_id
WHERE c.workspace_id = ? AND c.symbol = ?
GROUP BY c.id
ORDER BY c.as_of DESC
LIMIT 1;
```

---

## Data Migration Strategy

### Append-Only Migrations

**Rules**:
1. Never modify existing migration files after release
2. Create new migration for schema changes
3. Use reversible migrations with down scripts
4. Test migrations on copy of production data

**Example**:
```sql
-- Migration: 0004_add_greeks_snapshots.sql
CREATE TABLE greeks_snapshots (
    -- ...
);

-- Rollback: 0004_add_greeks_snapshots_down.sql
DROP TABLE greeks_snapshots;
```

---

## References

- [Architecture Design](./ARCHITECTURE.md)
- [API Specification](./API_SPEC.md)
- [AlphaForge Data Model](../DATA_MODEL.md)
- [AlphaForge Architecture](../ARCHITECTURE.md)