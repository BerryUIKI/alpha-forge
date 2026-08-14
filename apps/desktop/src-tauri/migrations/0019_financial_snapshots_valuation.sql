-- Financial domain schema (Phase 1 storage) — holdings snapshots + daily valuation.
--
-- Ported from Wealthfolio (create_daily_account_history → tracking_mode →
-- scoped_lots_valuation → lot_disposals → snapshot_position_cost_basis →
-- valuation_quality → reset_derived_read_models), consolidated into the final
-- canonical shape since these are fresh tables, not ALTERs of existing ones.
--
-- Two related but distinct concepts:
--   * holdings_snapshots    - a point-in-time position record for an account.
--     source = CALCULATED rows are derived read models rebuilt from activities;
--     source = MANUAL_ENTRY / CSV_IMPORT / BROKER_IMPORTED are user/import
--     source data (HOLDINGS-mode accounts) and are never dropped.
--   * daily_account_valuation - the derived daily valuation time series used
--     for performance charts. Rebuildable at any time from activities +
--     quotes + snapshots.

CREATE TABLE IF NOT EXISTS holdings_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    account_id TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    snapshot_date TEXT NOT NULL,               -- YYYY-MM-DD
    currency TEXT NOT NULL,
    -- Complex position/cash data stored as JSON text (validated at the
    -- application layer; plain TEXT here for forward compatibility).
    positions TEXT NOT NULL DEFAULT '{}',
    cash_balances TEXT NOT NULL DEFAULT '{}',
    cost_basis TEXT NOT NULL DEFAULT '0',
    net_contribution TEXT NOT NULL DEFAULT '0',
    net_contribution_base TEXT NOT NULL DEFAULT '0',
    cash_total_account_currency TEXT NOT NULL DEFAULT '0',
    cash_total_base_currency TEXT NOT NULL DEFAULT '0',
    source TEXT NOT NULL DEFAULT 'CALCULATED' CHECK (source IN (
        'CALCULATED', 'MANUAL_ENTRY', 'CSV_IMPORT', 'BROKER_IMPORTED', 'SYNTHETIC'
    )),
    calculated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_holdings_snapshots_account_date
    ON holdings_snapshots(account_id, snapshot_date);

CREATE INDEX IF NOT EXISTS idx_holdings_snapshots_date
    ON holdings_snapshots(snapshot_date);

CREATE INDEX IF NOT EXISTS idx_holdings_snapshots_account_id
    ON holdings_snapshots(account_id);

CREATE INDEX IF NOT EXISTS ix_holdings_snapshots_source
    ON holdings_snapshots(account_id, source);

-- Relational per-snapshot positions, sibling of the positions JSON column.
-- The natural key is (snapshot_id, asset_id); integer PK simplifies cursor
-- iteration. cost_basis_base/cost_basis_account are precomputed scalars at
-- acquisition-date FX, nullable until materialized.
CREATE TABLE IF NOT EXISTS snapshot_positions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    snapshot_id TEXT NOT NULL REFERENCES holdings_snapshots(id) ON DELETE CASCADE,
    asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    quantity TEXT NOT NULL,
    average_cost TEXT NOT NULL,
    total_cost_basis TEXT NOT NULL,
    currency TEXT NOT NULL,
    contract_multiplier TEXT NOT NULL DEFAULT '1',
    inception_date TEXT NOT NULL,
    is_alternative INTEGER NOT NULL DEFAULT 0 CHECK (is_alternative IN (0, 1)),
    cost_basis_base TEXT,
    cost_basis_account TEXT,
    created_at TEXT NOT NULL,
    last_updated TEXT NOT NULL,
    UNIQUE (snapshot_id, asset_id)
);

CREATE INDEX IF NOT EXISTS idx_snapshot_positions_snapshot_id
    ON snapshot_positions(snapshot_id);

CREATE INDEX IF NOT EXISTS idx_snapshot_positions_asset_id
    ON snapshot_positions(asset_id);

-- Derived daily valuation series (read model; rebuilt from source data).
CREATE TABLE IF NOT EXISTS daily_account_valuation (
    id TEXT PRIMARY KEY NOT NULL,
    account_id TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    valuation_date TEXT NOT NULL,              -- YYYY-MM-DD
    account_currency TEXT NOT NULL,
    base_currency TEXT NOT NULL,
    fx_rate_to_base TEXT NOT NULL,
    cash_balance TEXT NOT NULL,
    investment_market_value TEXT NOT NULL,
    total_value TEXT NOT NULL,
    cost_basis TEXT NOT NULL,
    net_contribution TEXT NOT NULL,
    cash_balance_base TEXT NOT NULL DEFAULT '0',
    investment_market_value_base TEXT NOT NULL DEFAULT '0',
    total_value_base TEXT NOT NULL DEFAULT '0',
    cost_basis_base TEXT NOT NULL DEFAULT '0',
    net_contribution_base TEXT NOT NULL DEFAULT '0',
    external_inflow_base TEXT NOT NULL DEFAULT '0',
    external_outflow_base TEXT NOT NULL DEFAULT '0',
    performance_eligible_value_base TEXT NOT NULL DEFAULT '0',
    -- Provenance of the external-flow figures (canonical codes; the domain
    -- layer maps legacy codes onto these).
    external_flow_source TEXT NOT NULL DEFAULT 'UNKNOWN',
    -- Valuation/basis coverage quality. Legacy codes are folded into these
    -- canonical values by the domain layer on read.
    value_status TEXT NOT NULL DEFAULT 'COMPLETE' CHECK (value_status IN (
        'COMPLETE', 'PARTIAL_UNPRICED', 'UNAVAILABLE'
    )),
    basis_status TEXT NOT NULL DEFAULT 'NOT_APPLICABLE' CHECK (basis_status IN (
        'COMPLETE', 'PARTIAL_UNKNOWN', 'UNKNOWN', 'NOT_APPLICABLE'
    )),
    calculated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_daily_account_valuation_account_date
    ON daily_account_valuation(account_id, valuation_date);
