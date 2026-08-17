-- Financial domain schema (Phase 1 storage) — tax lots + disposals.
--
-- Ported from Wealthfolio (lots + lot_disposals, final shape after
-- scoped_lots_valuation / lot_disposals / activity_trade_tax /
-- reset_derived_read_models). Lots are the persisted FIFO cost-basis
-- inventory; the valuation/performance services rebuild them from activities
-- and keep this table in sync.

CREATE TABLE IF NOT EXISTS lots (
    -- Identity & foreign keys
    id TEXT PRIMARY KEY NOT NULL,
    account_id TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,

    -- Open state
    open_date TEXT NOT NULL,
    open_activity_id TEXT REFERENCES activities(id) ON DELETE CASCADE,
    original_quantity TEXT NOT NULL,
    cost_per_unit TEXT NOT NULL,
    -- Immutable: cost basis at lot creation.
    original_cost_basis TEXT NOT NULL,
    -- Mutable: open cost basis still attributable to remaining_quantity.
    remaining_cost_basis TEXT NOT NULL,
    fee_allocated TEXT NOT NULL DEFAULT '0',

    -- Base-currency mirror (added by lot_disposals migration)
    original_cost_basis_base TEXT NOT NULL DEFAULT '0',
    remaining_cost_basis_base TEXT NOT NULL DEFAULT '0',
    fee_allocated_base TEXT NOT NULL DEFAULT '0',
    tax_allocated TEXT NOT NULL DEFAULT '0',
    tax_allocated_base TEXT NOT NULL DEFAULT '0',
    currency TEXT NOT NULL DEFAULT '',
    base_currency TEXT NOT NULL DEFAULT '',
    fx_rate_to_base TEXT NOT NULL DEFAULT '1',
    -- Account-currency FX captured at acquisition (added by
    -- reset_derived_read_models): NULL means same-currency.
    fx_rate_to_account TEXT,
    account_currency TEXT,
    cost_basis_method TEXT NOT NULL DEFAULT 'FIFO',

    -- Current state
    remaining_quantity TEXT NOT NULL,
    -- Cumulative product of post-acquisition SPLIT ratios. Stored quantities
    -- are in as-acquired units; effective shares held = remaining_quantity *
    -- split_ratio. Cost basis is split-invariant.
    split_ratio TEXT NOT NULL DEFAULT '1',
    is_closed INTEGER NOT NULL DEFAULT 0 CHECK (is_closed IN (0, 1)),

    -- Close state
    close_date TEXT,
    close_activity_id TEXT REFERENCES activities(id) ON DELETE SET NULL,

    -- Audit
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

-- Hot-path query: valuation = open lots JOIN quotes.
CREATE INDEX IF NOT EXISTS idx_lots_account_asset ON lots(account_id, asset_id);
CREATE INDEX IF NOT EXISTS idx_lots_asset_open ON lots(asset_id, is_closed, open_date);
CREATE INDEX IF NOT EXISTS idx_lots_account_open ON lots(account_id, is_closed);
CREATE INDEX IF NOT EXISTS idx_lots_open_activity ON lots(open_activity_id)
    WHERE open_activity_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS lot_disposals (
    id TEXT PRIMARY KEY NOT NULL,
    lot_id TEXT NOT NULL REFERENCES lots(id) ON DELETE CASCADE,
    account_id TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    disposal_activity_id TEXT NOT NULL REFERENCES activities(id) ON DELETE CASCADE,
    disposal_date TEXT NOT NULL,
    quantity TEXT NOT NULL,
    proceeds TEXT NOT NULL,
    cost_basis TEXT NOT NULL,
    realized_pnl TEXT NOT NULL,
    proceeds_base TEXT NOT NULL,
    cost_basis_base TEXT NOT NULL,
    realized_pnl_base TEXT NOT NULL,
    currency TEXT NOT NULL,
    base_currency TEXT NOT NULL,
    fx_rate_to_base TEXT NOT NULL,
    cost_basis_method TEXT NOT NULL DEFAULT 'FIFO',
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_lot_disposals_account_date
    ON lot_disposals(account_id, disposal_date);

CREATE INDEX IF NOT EXISTS idx_lot_disposals_asset_date
    ON lot_disposals(asset_id, disposal_date);

CREATE INDEX IF NOT EXISTS idx_lot_disposals_activity
    ON lot_disposals(disposal_activity_id);
