-- Financial domain schema (Phase 1 storage) — assets + quotes.
--
-- Ported from Wealthfolio's assets v2 model (refactor_asset_model) onto SQLx
-- conventions: TEXT UUID PKs, TEXT decimals, RFC3339 TEXT timestamps.
--
-- The `instrument_key` column is a generated, materialized canonical key:
--   EQUITY:AAPL@XNAS | CRYPTO:BTC/USD | FX:EUR/USD | OPTION:... | METAL:XAU/USD
-- It is never written by the application; the storage engine derives it from
-- instrument_type / instrument_symbol / instrument_exchange_mic / quote_ccy.

CREATE TABLE IF NOT EXISTS assets (
    id TEXT PRIMARY KEY NOT NULL,

    -- Core identity
    kind TEXT NOT NULL CHECK (kind IN (
        'INVESTMENT', 'PROPERTY', 'VEHICLE', 'COLLECTIBLE', 'PRECIOUS_METAL',
        'PRIVATE_EQUITY', 'LIABILITY', 'OTHER', 'FX'
    )),
    name TEXT,
    display_code TEXT,
    notes TEXT,
    metadata TEXT CHECK (metadata IS NULL OR json_valid(metadata)),

    is_active INTEGER NOT NULL DEFAULT 1 CHECK (is_active IN (0, 1)),

    -- Valuation
    quote_mode TEXT NOT NULL CHECK (quote_mode IN ('MARKET', 'MANUAL')),
    quote_ccy TEXT NOT NULL,

    -- Instrument identity (NULL for non-market assets)
    instrument_type TEXT CHECK (instrument_type IS NULL OR instrument_type IN (
        'EQUITY', 'CRYPTO', 'FX', 'OPTION', 'METAL'
    )),
    instrument_symbol TEXT,
    instrument_exchange_mic TEXT,

    -- Computed canonical key (materialized on disk, never set directly)
    instrument_key TEXT GENERATED ALWAYS AS (
        CASE
            WHEN instrument_type IS NULL OR instrument_symbol IS NULL THEN NULL
            WHEN instrument_type IN ('FX', 'CRYPTO')
                THEN instrument_type || ':' || instrument_symbol || '/' || quote_ccy
            WHEN instrument_exchange_mic IS NOT NULL
                THEN instrument_type || ':' || instrument_symbol || '@' || instrument_exchange_mic
            ELSE instrument_type || ':' || instrument_symbol
        END
    ) STORED,

    -- Provider configuration (single JSON blob)
    provider_config TEXT CHECK (provider_config IS NULL OR json_valid(provider_config)),

    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_assets_instrument_key
    ON assets(instrument_key)
    WHERE instrument_key IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_assets_kind ON assets(kind);
CREATE INDEX IF NOT EXISTS idx_assets_is_active ON assets(is_active);
CREATE INDEX IF NOT EXISTS idx_assets_display_code ON assets(display_code);

-- Quotes (v2): one row per (asset, day, source), decimals as TEXT.
CREATE TABLE IF NOT EXISTS quotes (
    id TEXT PRIMARY KEY NOT NULL,
    asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE ON UPDATE CASCADE,
    day TEXT NOT NULL CHECK (length(day) = 10),
    source TEXT NOT NULL,
    open TEXT,
    high TEXT,
    low TEXT,
    close TEXT NOT NULL,
    adjclose TEXT,
    volume TEXT,
    currency TEXT NOT NULL,
    notes TEXT,
    created_at TEXT NOT NULL,
    timestamp TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_quotes_asset_day_source
    ON quotes(asset_id, day, source);

CREATE INDEX IF NOT EXISTS idx_quotes_asset_source_day
    ON quotes(asset_id, source, day);

CREATE INDEX IF NOT EXISTS idx_quotes_manual
    ON quotes(asset_id, day DESC)
    WHERE source = 'MANUAL';
