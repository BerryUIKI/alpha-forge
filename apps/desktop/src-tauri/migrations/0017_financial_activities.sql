-- Financial domain schema (Phase 1 storage) — import runs + activities.
--
-- Ported from Wealthfolio (activities v2 + import_runs) onto SQLx conventions.
-- `activities` is the canonical transaction ledger: every cash move, trade,
-- dividend, split, fee, and transfer is one row with an idempotency key so
-- CSV/broker imports never duplicate.

CREATE TABLE IF NOT EXISTS import_runs (
    id TEXT PRIMARY KEY NOT NULL,
    account_id TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    source_system TEXT NOT NULL,
    run_type TEXT NOT NULL,
    mode TEXT NOT NULL,
    status TEXT NOT NULL,
    started_at TEXT NOT NULL,
    finished_at TEXT,
    review_mode TEXT NOT NULL,
    applied_at TEXT,
    checkpoint_in TEXT,
    checkpoint_out TEXT,
    summary TEXT,
    warnings TEXT,
    error TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS ix_import_runs_account_id ON import_runs(account_id);
CREATE INDEX IF NOT EXISTS ix_import_runs_status ON import_runs(status);

CREATE TABLE IF NOT EXISTS activities (
    id TEXT PRIMARY KEY NOT NULL,
    account_id TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    asset_id TEXT REFERENCES assets(id) ON DELETE SET NULL ON UPDATE CASCADE,

    activity_type TEXT NOT NULL CHECK (activity_type IN (
        'BUY', 'SELL', 'SPLIT',
        'DIVIDEND', 'INTEREST', 'DEPOSIT', 'WITHDRAWAL',
        'TRANSFER_IN', 'TRANSFER_OUT', 'FEE', 'TAX',
        'CREDIT', 'ADJUSTMENT', 'UNKNOWN'
    )),
    activity_type_override TEXT,
    source_type TEXT,
    subtype TEXT,
    status TEXT NOT NULL DEFAULT 'POSTED' CHECK (status IN (
        'POSTED', 'PENDING', 'CANCELED'
    )),

    activity_date TEXT NOT NULL,
    settlement_date TEXT,

    quantity TEXT,
    unit_price TEXT,
    amount TEXT,
    fee TEXT,
    tax TEXT,
    currency TEXT NOT NULL,
    fx_rate TEXT,

    notes TEXT,
    metadata TEXT CHECK (metadata IS NULL OR json_valid(metadata)),

    source_system TEXT,
    source_record_id TEXT,
    source_group_id TEXT,
    idempotency_key TEXT,
    import_run_id TEXT REFERENCES import_runs(id) ON DELETE SET NULL,

    is_user_modified INTEGER NOT NULL DEFAULT 0 CHECK (is_user_modified IN (0, 1)),
    needs_review INTEGER NOT NULL DEFAULT 0 CHECK (needs_review IN (0, 1)),

    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

-- Unique idempotency key: re-importing the same source record is a no-op.
CREATE UNIQUE INDEX IF NOT EXISTS ux_activities_idempotency_key
    ON activities(idempotency_key)
    WHERE idempotency_key IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_activities_account_date
    ON activities(account_id, activity_date);

CREATE INDEX IF NOT EXISTS ix_activities_account_id ON activities(account_id);
CREATE INDEX IF NOT EXISTS ix_activities_asset_id ON activities(asset_id);
CREATE INDEX IF NOT EXISTS ix_activities_activity_date ON activities(activity_date);
CREATE INDEX IF NOT EXISTS ix_activities_status ON activities(status);

CREATE INDEX IF NOT EXISTS ix_activities_source_group_id
    ON activities(source_group_id)
    WHERE source_group_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS ix_activities_transfer_scope
    ON activities(account_id, activity_date, status)
    WHERE COALESCE(activity_type_override, activity_type) IN ('TRANSFER_IN', 'TRANSFER_OUT');
