-- Canonical Option persistence schema.
--
-- 0004_options_support.sql is historical and intentionally remains unchanged.
-- This append-only migration preserves that repository/domain contract while
-- allowing a partially-created canonical schema to finish safely.

CREATE TABLE IF NOT EXISTS option_chains (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    symbol TEXT NOT NULL,
    underlying_price REAL NOT NULL CHECK(underlying_price > 0),
    as_of TEXT NOT NULL,
    data_source TEXT NOT NULL CHECK(data_source IN ('live', 'demo', 'file')),
    created_at TEXT NOT NULL,

    UNIQUE(workspace_id, symbol, as_of)
);

CREATE INDEX IF NOT EXISTS idx_option_chains_workspace ON option_chains(workspace_id);
CREATE INDEX IF NOT EXISTS idx_option_chains_symbol ON option_chains(symbol);
CREATE INDEX IF NOT EXISTS idx_option_chains_as_of ON option_chains(as_of);

CREATE TABLE IF NOT EXISTS option_contracts (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    chain_id TEXT NOT NULL REFERENCES option_chains(id) ON DELETE CASCADE,
    symbol TEXT NOT NULL,
    option_type TEXT NOT NULL CHECK(option_type IN ('call', 'put')),
    strike REAL NOT NULL CHECK(strike > 0),
    expiration TEXT NOT NULL,
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

CREATE INDEX IF NOT EXISTS idx_option_contracts_chain ON option_contracts(chain_id);
CREATE INDEX IF NOT EXISTS idx_option_contracts_symbol ON option_contracts(symbol);
CREATE INDEX IF NOT EXISTS idx_option_contracts_expiration ON option_contracts(expiration);
CREATE INDEX IF NOT EXISTS idx_option_contracts_strike ON option_contracts(strike);

CREATE TABLE IF NOT EXISTS greeks (
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

CREATE INDEX IF NOT EXISTS idx_greeks_contract ON greeks(option_contract_id);

CREATE TABLE IF NOT EXISTS option_strategies (
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
    break_even_points TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_option_strategies_workspace ON option_strategies(workspace_id);
CREATE INDEX IF NOT EXISTS idx_option_strategies_underlying ON option_strategies(underlying);

CREATE TABLE IF NOT EXISTS strategy_legs (
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

CREATE INDEX IF NOT EXISTS idx_strategy_legs_strategy ON strategy_legs(strategy_id);
CREATE INDEX IF NOT EXISTS idx_strategy_legs_contract ON strategy_legs(option_contract_id);

CREATE TABLE IF NOT EXISTS option_positions (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    account_id TEXT REFERENCES portfolio_accounts(id) ON DELETE SET NULL,
    option_contract_id TEXT NOT NULL REFERENCES option_contracts(id),
    quantity INTEGER NOT NULL,
    cost_basis REAL NOT NULL,
    opened_at TEXT NOT NULL,
    closed_at TEXT,
    notes TEXT
);

CREATE INDEX IF NOT EXISTS idx_option_positions_workspace ON option_positions(workspace_id);
CREATE INDEX IF NOT EXISTS idx_option_positions_account ON option_positions(account_id);
CREATE INDEX IF NOT EXISTS idx_option_positions_contract ON option_positions(option_contract_id);
CREATE INDEX IF NOT EXISTS idx_option_positions_open ON option_positions(opened_at);

CREATE TABLE IF NOT EXISTS greeks_snapshots (
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

CREATE INDEX IF NOT EXISTS idx_greeks_snapshots_position ON greeks_snapshots(position_id);
CREATE INDEX IF NOT EXISTS idx_greeks_snapshots_date ON greeks_snapshots(snapshot_date);

CREATE TRIGGER IF NOT EXISTS update_option_contracts_timestamp
AFTER UPDATE ON option_contracts
BEGIN
    UPDATE option_contracts SET updated_at = datetime('now') WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_option_strategies_timestamp
AFTER UPDATE ON option_strategies
BEGIN
    UPDATE option_strategies SET updated_at = datetime('now') WHERE id = NEW.id;
END;
