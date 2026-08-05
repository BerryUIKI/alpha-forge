-- Option Chains table
-- Stores fetched option chain metadata for a given symbol and expiration
CREATE TABLE IF NOT EXISTS option_chains (
    id TEXT PRIMARY KEY,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    symbol TEXT NOT NULL,
    source TEXT NOT NULL CHECK (source IN ('demo', 'live', 'file')),
    expiration_date TEXT NOT NULL,
    spot_price REAL NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Option Contracts table
-- Individual option contracts within a chain
CREATE TABLE IF NOT EXISTS option_contracts (
    id TEXT PRIMARY KEY,
    chain_id TEXT NOT NULL REFERENCES option_chains(id) ON DELETE CASCADE,
    option_type TEXT NOT NULL CHECK (option_type IN ('call', 'put')),
    strike REAL NOT NULL,
    expiration TEXT NOT NULL,
    bid REAL,
    ask REAL,
    last_price REAL,
    volume INTEGER,
    open_interest INTEGER,
    implied_volatility REAL,
    delta REAL,
    gamma REAL,
    theta REAL,
    vega REAL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Option Strategies table
-- User-defined option strategies (e.g., Iron Condor, Straddle)
CREATE TABLE IF NOT EXISTS option_strategies (
    id TEXT PRIMARY KEY,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    strategy_type TEXT NOT NULL,
    description TEXT,
    legs TEXT NOT NULL,  -- JSON array of {contract_id, quantity, direction}
    total_delta REAL,
    total_gamma REAL,
    total_theta REAL,
    total_vega REAL,
    max_profit REAL,
    max_loss REAL,
    breakeven_points TEXT,  -- JSON array of breakeven prices
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Indexes for efficient querying
CREATE INDEX IF NOT EXISTS idx_option_chains_workspace ON option_chains(workspace_id);
CREATE INDEX IF NOT EXISTS idx_option_chains_symbol ON option_chains(symbol);
CREATE INDEX IF NOT EXISTS idx_option_contracts_chain ON option_contracts(chain_id);
CREATE INDEX IF NOT EXISTS idx_option_contracts_expiration ON option_contracts(expiration);
CREATE INDEX IF NOT EXISTS idx_option_strategies_workspace ON option_strategies(workspace_id);
CREATE INDEX IF NOT EXISTS idx_option_strategies_type ON option_strategies(strategy_type);
