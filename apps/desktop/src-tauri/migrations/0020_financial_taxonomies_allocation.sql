-- Financial domain schema (Phase 1 storage) — taxonomies + allocation targets.
--
-- Ported from Wealthfolio (2026-01-01-000002_taxonomies,
-- 2026-05-25-000002_allocation_targets, 2026-06-25-000001_allocation_constraints).
--
-- Taxonomies power allocation reporting (asset class rollups, GICS
-- industries, regions). This migration creates the taxonomy tables and seeds
-- the system taxonomies that the MVP slice consumes: instrument_type,
-- asset_classes, risk_category. The GICS industries, regions, and
-- custom_groups category seeds are deferred to Phase 5 (polish) — the schema
-- supports them; only reference data is added later in an append-only
-- migration.

-- ============================================================================
-- TAXONOMIES
-- ============================================================================

CREATE TABLE IF NOT EXISTS taxonomies (
    id TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    color TEXT NOT NULL DEFAULT '#8abceb',
    description TEXT,
    is_system INTEGER NOT NULL DEFAULT 0,
    is_single_select INTEGER NOT NULL DEFAULT 0,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS ix_taxonomies_sort_order ON taxonomies(sort_order);

CREATE TABLE IF NOT EXISTS taxonomy_categories (
    id TEXT NOT NULL,
    taxonomy_id TEXT NOT NULL,
    parent_id TEXT,
    name TEXT NOT NULL,
    key TEXT NOT NULL,
    color TEXT NOT NULL DEFAULT '#808080',
    description TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),

    PRIMARY KEY (taxonomy_id, id),
    FOREIGN KEY (taxonomy_id) REFERENCES taxonomies(id) ON DELETE CASCADE,
    FOREIGN KEY (taxonomy_id, parent_id) REFERENCES taxonomy_categories(taxonomy_id, id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS ix_taxonomy_categories_parent ON taxonomy_categories(taxonomy_id, parent_id);
CREATE INDEX IF NOT EXISTS ix_taxonomy_categories_key ON taxonomy_categories(taxonomy_id, key);

CREATE TABLE IF NOT EXISTS asset_taxonomy_assignments (
    id TEXT NOT NULL PRIMARY KEY,
    asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    taxonomy_id TEXT NOT NULL,
    category_id TEXT NOT NULL,
    weight INTEGER NOT NULL DEFAULT 10000,
    source TEXT NOT NULL DEFAULT 'manual',
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),

    FOREIGN KEY (taxonomy_id, category_id) REFERENCES taxonomy_categories(taxonomy_id, id) ON DELETE CASCADE,
    CHECK (weight >= 0 AND weight <= 10000)
);

CREATE INDEX IF NOT EXISTS ix_asset_taxonomy_assignments_asset ON asset_taxonomy_assignments(asset_id);
CREATE INDEX IF NOT EXISTS ix_asset_taxonomy_assignments_category
    ON asset_taxonomy_assignments(taxonomy_id, category_id);
CREATE UNIQUE INDEX IF NOT EXISTS ix_asset_taxonomy_assignment_unique
    ON asset_taxonomy_assignments(asset_id, taxonomy_id, category_id);

-- ============================================================================
-- SEED: DEFAULT TAXONOMIES (system reference data)
-- ============================================================================

INSERT INTO taxonomies (id, name, color, description, is_system, is_single_select, sort_order)
VALUES
  ('instrument_type', 'Instrument Type', '#4385be',
   'Instrument structure used for filtering/reporting (e.g., Stock, ETF, Bond, Option). Should not drive calculation logic.',
   1, 1, 10),
  ('asset_classes', 'Asset Classes', '#879a39',
   'High-level asset class rollup (e.g., Equity, Debt, Cash, Real Estate, Commodity) for summaries and charts.',
   1, 0, 20),
  ('industries_gics', 'Industries (GICS)', '#da702c',
   'Global Industry Classification Standard (GICS) hierarchy: Sector -> Industry Group -> Industry -> Sub-Industry.',
   1, 0, 30),
  ('regions', 'Regions', '#8b7ec8',
   'Geographic exposure grouping for reporting (e.g., North America, Europe, Emerging Markets).',
   1, 0, 40),
  ('risk_category', 'Risk Category', '#d14d41',
   'Risk level classification for assets. Single-select: each asset can only have one risk category assigned.',
   1, 1, 50),
  ('custom_groups', 'Custom Groups', '#878580',
   'User-defined tags for grouping assets. Use for watchlists, themes, strategies, or any personal organization.',
   1, 0, 100);

-- Instrument types: 11 top-level + 38 children = 49 categories.
INSERT INTO taxonomy_categories (id, taxonomy_id, parent_id, name, key, color, sort_order)
VALUES
  ('EQUITY_SECURITY', 'instrument_type', NULL, 'Stocks',                  'EQUITY_SECURITY', '#4385be', 1),
  ('DEBT_SECURITY',   'instrument_type', NULL, 'Bonds',                   'DEBT_SECURITY',   '#d14d41', 2),
  ('FUND',            'instrument_type', NULL, 'Funds',                   'FUND',            '#3aa99f', 3),
  ('ETP',             'instrument_type', NULL, 'ETFs',                    'ETP',             '#8b7ec8', 4),
  ('DERIVATIVE',      'instrument_type', NULL, 'Options & Futures',       'DERIVATIVE',      '#da702c', 5),
  ('CASH_FX',         'instrument_type', NULL, 'Cash & FX',               'CASH_FX',         '#879a39', 6),
  ('STRUCTURED',      'instrument_type', NULL, 'Structured Notes',        'STRUCTURED',      '#d0a215', 7),
  ('REAL_ASSET',      'instrument_type', NULL, 'Physical Assets',         'REAL_ASSET',      '#bc5215', 8),
  ('DIGITAL_ASSET',   'instrument_type', NULL, 'Crypto',                  'DIGITAL_ASSET',   '#ce5d97', 9),
  ('PRIVATE_VEHICLE', 'instrument_type', NULL, 'Private Investments',     'PRIVATE_VEHICLE', '#5e409d', 10),
  ('OTHER',           'instrument_type', NULL, 'Other',                   'OTHER',           '#878580', 11);

INSERT INTO taxonomy_categories (id, taxonomy_id, parent_id, name, key, color, sort_order)
VALUES
  ('STOCK_COMMON',        'instrument_type', 'EQUITY_SECURITY', 'Stock',              'STOCK_COMMON',        '#66a0c8', 1),
  ('STOCK_PREFERRED',     'instrument_type', 'EQUITY_SECURITY', 'Preferred Stock',    'STOCK_PREFERRED',     '#7cb0d2', 2),
  ('DEPOSITARY_RECEIPT',  'instrument_type', 'EQUITY_SECURITY', 'ADR / GDR',          'DEPOSITARY_RECEIPT',  '#92bfdb', 3),
  ('EQUITY_WARRANT_RIGHT','instrument_type', 'EQUITY_SECURITY', 'Warrant / Right',    'EQUITY_WARRANT_RIGHT','#a2c9e0', 4),
  ('PARTNERSHIP_UNIT',    'instrument_type', 'EQUITY_SECURITY', 'Partnership / Trust Unit', 'PARTNERSHIP_UNIT', '#b4d3e5', 5),
  ('BOND_GOVERNMENT',   'instrument_type', 'DEBT_SECURITY', 'Government Bond',            'BOND_GOVERNMENT',   '#dc6a5f', 1),
  ('BOND_CORPORATE',    'instrument_type', 'DEBT_SECURITY', 'Corporate Bond',             'BOND_CORPORATE',    '#e37d73', 2),
  ('BOND_MUNICIPAL',    'instrument_type', 'DEBT_SECURITY', 'Municipal Bond',             'BOND_MUNICIPAL',    '#e8908a', 3),
  ('BOND_CONVERTIBLE',  'instrument_type', 'DEBT_SECURITY', 'Convertible / Hybrid Bond',  'BOND_CONVERTIBLE',  '#eda39e', 4),
  ('MONEY_MARKET_DEBT', 'instrument_type', 'DEBT_SECURITY', 'T-Bills / CDs / Commercial Paper', 'MONEY_MARKET_DEBT', '#f2b6b2', 5),
  ('FUND_MUTUAL',     'instrument_type', 'FUND', 'Mutual Fund',          'FUND_MUTUAL',     '#5abdac', 1),
  ('FUND_CLOSED_END', 'instrument_type', 'FUND', 'Closed-End Fund (CEF)', 'FUND_CLOSED_END', '#87d3c3', 2),
  ('FUND_PRIVATE',    'instrument_type', 'FUND', 'Private / Hedge Fund', 'FUND_PRIVATE',    '#a2dece', 3),
  ('FUND_FOF',        'instrument_type', 'FUND', 'Fund of Funds',        'FUND_FOF',        '#bfe8d9', 4),
  ('ETF', 'instrument_type', 'ETP', 'ETF',                   'ETF', '#a699d0', 1),
  ('ETN', 'instrument_type', 'ETP', 'ETN',                   'ETN', '#b8afda', 2),
  ('ETC', 'instrument_type', 'ETP', 'Commodity ETP (ETC/ETP)', 'ETC', '#c4b9e0', 3),
  ('OPTION',         'instrument_type', 'DERIVATIVE', 'Option',              'OPTION',         '#ec8b49', 1),
  ('FUTURE',         'instrument_type', 'DERIVATIVE', 'Futures',             'FUTURE',         '#f09c60', 2),
  ('OTC_DERIVATIVE', 'instrument_type', 'DERIVATIVE', 'Forward / Swap (OTC)', 'OTC_DERIVATIVE', '#f9ae77', 3),
  ('CFD',            'instrument_type', 'DERIVATIVE', 'CFD',                 'CFD',            '#fbc093', 4),
  ('CASH',        'instrument_type', 'CASH_FX', 'Cash Balance',       'CASH',        '#a0af54', 1),
  ('DEPOSIT',     'instrument_type', 'CASH_FX', 'Bank Deposit / Sweep', 'DEPOSIT',   '#adb85e', 2),
  ('FX_POSITION', 'instrument_type', 'CASH_FX', 'Currency Position',  'FX_POSITION', '#bec97e', 3),
  ('STRUCTURED_NOTE',     'instrument_type', 'STRUCTURED', 'Structured Note',     'STRUCTURED_NOTE',     '#dfb431', 1),
  ('MARKET_LINKED_NOTE',  'instrument_type', 'STRUCTURED', 'Market-Linked Note',  'MARKET_LINKED_NOTE',  '#eccb60', 2),
  ('CREDIT_LINKED_NOTE',  'instrument_type', 'STRUCTURED', 'Credit-Linked Note',  'CREDIT_LINKED_NOTE',  '#f0d678', 3),
  ('PHYSICAL_COMMODITY', 'instrument_type', 'REAL_ASSET', 'Physical Commodity',    'PHYSICAL_COMMODITY', '#cb6120', 1),
  ('PHYSICAL_METAL',     'instrument_type', 'REAL_ASSET', 'Physical Gold / Silver', 'PHYSICAL_METAL',    '#da702c', 2),
  ('DIRECT_REAL_ESTATE', 'instrument_type', 'REAL_ASSET', 'Direct Real Estate',    'DIRECT_REAL_ESTATE', '#ec8b49', 3),
  ('CRYPTO_NATIVE',      'instrument_type', 'DIGITAL_ASSET', 'Cryptocurrency',    'CRYPTO_NATIVE',      '#e47da8', 1),
  ('STABLECOIN',         'instrument_type', 'DIGITAL_ASSET', 'Stablecoin',        'STABLECOIN',         '#e88db3', 2),
  ('TOKENIZED_SECURITY', 'instrument_type', 'DIGITAL_ASSET', 'Tokenized Asset',   'TOKENIZED_SECURITY', '#ec9dbe', 3),
  ('PRIVATE_COMPANY', 'instrument_type', 'PRIVATE_VEHICLE', 'Private Company Shares', 'PRIVATE_COMPANY', '#735eb5', 1),
  ('PRIVATE_LOAN',    'instrument_type', 'PRIVATE_VEHICLE', 'Private Loan / Note',    'PRIVATE_LOAN',    '#8b7ec8', 2),
  ('SPV',             'instrument_type', 'PRIVATE_VEHICLE', 'SPV / Private Vehicle',  'SPV',             '#a699d0', 3),
  ('OTHER_UNKNOWN',      'instrument_type', 'OTHER', 'Unknown Instrument',         'OTHER_UNKNOWN',      '#9f9d96', 1),
  ('SYNTHETIC_INTERNAL', 'instrument_type', 'OTHER', 'Synthetic / Internal Position', 'SYNTHETIC_INTERNAL', '#b7b5ac', 2);

-- Risk categories (single-select).
INSERT INTO taxonomy_categories (id, taxonomy_id, parent_id, name, key, color, sort_order)
VALUES
  ('UNKNOWN', 'risk_category', NULL, 'Unknown',  'UNKNOWN', '#878580', 1),
  ('LOW',     'risk_category', NULL, 'Low',      'LOW',     '#879a39', 2),
  ('MEDIUM',  'risk_category', NULL, 'Medium',   'MEDIUM',  '#d0a215', 3),
  ('HIGH',    'risk_category', NULL, 'High',     'HIGH',    '#d14d41', 4);

-- Asset classes: 7 top-level + subcategories = 79 categories.
INSERT INTO taxonomy_categories (id, taxonomy_id, parent_id, name, key, color, sort_order)
VALUES
  ('CASH',           'asset_classes', NULL, 'Cash',           'CASH',           '#879a39', 1),
  ('EQUITY',         'asset_classes', NULL, 'Equity',         'EQUITY',         '#4385be', 2),
  ('FIXED_INCOME',   'asset_classes', NULL, 'Fixed Income',   'FIXED_INCOME',   '#d14d41', 3),
  ('REAL_ESTATE',    'asset_classes', NULL, 'Real Estate',    'REAL_ESTATE',    '#da702c', 4),
  ('COMMODITIES',    'asset_classes', NULL, 'Commodities',    'COMMODITIES',    '#d0a215', 5),
  ('ALTERNATIVES',   'asset_classes', NULL, 'Alternatives',   'ALTERNATIVES',   '#8b7ec8', 6),
  ('DIGITAL_ASSETS', 'asset_classes', NULL, 'Digital Assets', 'DIGITAL_ASSETS', '#ce5d97', 7);

INSERT INTO taxonomy_categories (id, taxonomy_id, parent_id, name, key, color, sort_order)
VALUES
  ('CASH_BANK_DEPOSITS',    'asset_classes', 'CASH', 'Bank Deposits',        'CASH_BANK_DEPOSITS',    '#a0af54', 1),
  ('CASH_TREASURY_BILLS',   'asset_classes', 'CASH', 'Treasury Bills',       'CASH_TREASURY_BILLS',   '#adb85e', 2),
  ('CASH_MONEY_MARKET',     'asset_classes', 'CASH', 'Money Market',         'CASH_MONEY_MARKET',     '#bec97e', 3),
  ('CASH_ULTRA_SHORT',      'asset_classes', 'CASH', 'Ultra-Short Duration', 'CASH_ULTRA_SHORT',      '#cdd597', 4),
  ('CASH_STABLE_VALUE',     'asset_classes', 'CASH', 'Stable Value',         'CASH_STABLE_VALUE',     '#dde2b2', 5),
  ('EQUITY_PUBLIC',  'asset_classes', 'EQUITY', 'Public Stocks',  'EQUITY_PUBLIC',  '#66a0c8', 1),
  ('EQUITY_PRIVATE', 'asset_classes', 'EQUITY', 'Private Equity', 'EQUITY_PRIVATE', '#7cb0d2', 2),
  ('EQUITY_PRIVATE_BUYOUT',      'asset_classes', 'EQUITY_PRIVATE', 'Buyout',           'EQUITY_PRIVATE_BUYOUT',      '#92bfdb', 1),
  ('EQUITY_PRIVATE_GROWTH',      'asset_classes', 'EQUITY_PRIVATE', 'Growth Equity',    'EQUITY_PRIVATE_GROWTH',      '#a2c9e0', 2),
  ('EQUITY_PRIVATE_VC',          'asset_classes', 'EQUITY_PRIVATE', 'Venture Capital',  'EQUITY_PRIVATE_VC',          '#b4d3e5', 3),
  ('EQUITY_PRIVATE_SECONDARIES', 'asset_classes', 'EQUITY_PRIVATE', 'Secondaries',      'EQUITY_PRIVATE_SECONDARIES', '#c6dde8', 4),
  ('EQUITY_PRIVATE_REAL_ASSETS', 'asset_classes', 'EQUITY_PRIVATE', 'Private Real Assets', 'EQUITY_PRIVATE_REAL_ASSETS', '#d8e7ed', 5),
  ('FI_SOVEREIGN',        'asset_classes', 'FIXED_INCOME', 'Sovereign Bonds',        'FI_SOVEREIGN',        '#dc6a5f', 1),
  ('FI_CORPORATE',        'asset_classes', 'FIXED_INCOME', 'Corporate Bonds',        'FI_CORPORATE',        '#e37d73', 2),
  ('FI_MUNICIPAL',        'asset_classes', 'FIXED_INCOME', 'Municipal Bonds',        'FI_MUNICIPAL',        '#e8908a', 3),
  ('FI_AGENCY_SUPRA',     'asset_classes', 'FIXED_INCOME', 'Agency & Supranational', 'FI_AGENCY_SUPRA',     '#eda39e', 4),
  ('FI_EM_DEBT',          'asset_classes', 'FIXED_INCOME', 'Emerging Market Debt',   'FI_EM_DEBT',          '#f2b6b2', 5),
  ('FI_INFLATION_LINKED', 'asset_classes', 'FIXED_INCOME', 'Inflation-Linked Bonds', 'FI_INFLATION_LINKED', '#f7c9c6', 6),
  ('FI_SECURITIZED',      'asset_classes', 'FIXED_INCOME', 'Securitized Debt',       'FI_SECURITIZED',      '#f9d7d4', 7),
  ('FI_LOANS_FRN',        'asset_classes', 'FIXED_INCOME', 'Loans / Floating Rate',  'FI_LOANS_FRN',        '#fcdcda', 8),
  ('FI_CONVERTIBLE',      'asset_classes', 'FIXED_INCOME', 'Convertible Bonds',      'FI_CONVERTIBLE',      '#fde7e5', 9),
  ('FI_PREFERRED',        'asset_classes', 'FIXED_INCOME', 'Preferred Securities',   'FI_PREFERRED',        '#fef1ef', 10),
  ('FI_SECURITIZED_MBS',  'asset_classes', 'FI_SECURITIZED', 'Mortgage-Backed Securities', 'FI_SECURITIZED_MBS',  '#f7c9c6', 1),
  ('FI_SECURITIZED_ABS',  'asset_classes', 'FI_SECURITIZED', 'Asset-Backed Securities',    'FI_SECURITIZED_ABS',  '#fad6d4', 2),
  ('FI_SECURITIZED_CMBS', 'asset_classes', 'FI_SECURITIZED', 'Commercial MBS',              'FI_SECURITIZED_CMBS', '#fce3e2', 3),
  ('RE_PUBLIC_REITS',  'asset_classes', 'REAL_ESTATE', 'Public REITs',        'RE_PUBLIC_REITS',  '#ec8b49', 1),
  ('RE_PRIVATE',       'asset_classes', 'REAL_ESTATE', 'Private Real Estate', 'RE_PRIVATE',       '#f09c60', 2),
  ('COMM_ENERGY',      'asset_classes', 'COMMODITIES', 'Energy',            'COMM_ENERGY',      '#eccb60', 1),
  ('COMM_PRECIOUS',    'asset_classes', 'COMMODITIES', 'Precious Metals',   'COMM_PRECIOUS',    '#dfb431', 2),
  ('COMM_INDUSTRIAL',  'asset_classes', 'COMMODITIES', 'Industrial Metals', 'COMM_INDUSTRIAL',  '#e4bd48', 3),
  ('COMM_AGRICULTURE', 'asset_classes', 'COMMODITIES', 'Agriculture',       'COMM_AGRICULTURE', '#f0d678', 4),
  ('COMM_LIVESTOCK',   'asset_classes', 'COMMODITIES', 'Livestock',         'COMM_LIVESTOCK',   '#f6e2a0', 5),
  ('COMM_ENERGY_CRUDE_OIL',   'asset_classes', 'COMM_ENERGY', 'Crude Oil',        'COMM_ENERGY_CRUDE_OIL',   '#f0d678', 1),
  ('COMM_ENERGY_NATURAL_GAS', 'asset_classes', 'COMM_ENERGY', 'Natural Gas',      'COMM_ENERGY_NATURAL_GAS', '#f6e2a0', 2),
  ('COMM_ENERGY_REFINED',     'asset_classes', 'COMM_ENERGY', 'Refined Products', 'COMM_ENERGY_REFINED',     '#f9ecb8', 3),
  ('COMM_ENERGY_POWER',       'asset_classes', 'COMM_ENERGY', 'Power',            'COMM_ENERGY_POWER',       '#fcf5d0', 4),
  ('COMM_PRECIOUS_GOLD',      'asset_classes', 'COMM_PRECIOUS', 'Gold',      'COMM_PRECIOUS_GOLD',      '#f0d678', 1),
  ('COMM_PRECIOUS_SILVER',    'asset_classes', 'COMM_PRECIOUS', 'Silver',    'COMM_PRECIOUS_SILVER',    '#f6e2a0', 2),
  ('COMM_PRECIOUS_PLATINUM',  'asset_classes', 'COMM_PRECIOUS', 'Platinum',  'COMM_PRECIOUS_PLATINUM',  '#f9ecb8', 3),
  ('COMM_PRECIOUS_PALLADIUM', 'asset_classes', 'COMM_PRECIOUS', 'Palladium', 'COMM_PRECIOUS_PALLADIUM', '#fcf5d0', 4),
  ('COMM_INDUSTRIAL_COPPER',   'asset_classes', 'COMM_INDUSTRIAL', 'Copper',                'COMM_INDUSTRIAL_COPPER',   '#eccb60', 1),
  ('COMM_INDUSTRIAL_ALUMINUM', 'asset_classes', 'COMM_INDUSTRIAL', 'Aluminum',              'COMM_INDUSTRIAL_ALUMINUM', '#f3dc8c', 2),
  ('COMM_INDUSTRIAL_NICKEL',   'asset_classes', 'COMM_INDUSTRIAL', 'Nickel',                'COMM_INDUSTRIAL_NICKEL',   '#f0d678', 3),
  ('COMM_INDUSTRIAL_ZINC',     'asset_classes', 'COMM_INDUSTRIAL', 'Zinc',                  'COMM_INDUSTRIAL_ZINC',     '#f6e2a0', 4),
  ('COMM_INDUSTRIAL_LITHIUM',  'asset_classes', 'COMM_INDUSTRIAL', 'Lithium',  'COMM_INDUSTRIAL_LITHIUM',  '#f9ecb8', 5),
  ('COMM_INDUSTRIAL_OTHER',    'asset_classes', 'COMM_INDUSTRIAL', 'Other Industrial Metals','COMM_INDUSTRIAL_OTHER',    '#fcf5d0', 6),
  ('COMM_AGRICULTURE_GRAINS',   'asset_classes', 'COMM_AGRICULTURE', 'Grains',   'COMM_AGRICULTURE_GRAINS',   '#f6e2a0', 1),
  ('COMM_AGRICULTURE_SOFTS',    'asset_classes', 'COMM_AGRICULTURE', 'Softs',    'COMM_AGRICULTURE_SOFTS',    '#f9ecb8', 2),
  ('COMM_AGRICULTURE_OILSEEDS', 'asset_classes', 'COMM_AGRICULTURE', 'Oilseeds', 'COMM_AGRICULTURE_OILSEEDS', '#fcf5d0', 3),
  ('COMM_LIVESTOCK_CATTLE', 'asset_classes', 'COMM_LIVESTOCK', 'Cattle', 'COMM_LIVESTOCK_CATTLE', '#f9ecb8', 1),
  ('COMM_LIVESTOCK_HOGS',   'asset_classes', 'COMM_LIVESTOCK', 'Hogs',   'COMM_LIVESTOCK_HOGS',   '#fcf5d0', 2),
  ('ALT_HEDGE_FUNDS',     'asset_classes', 'ALTERNATIVES', 'Hedge Funds',                  'ALT_HEDGE_FUNDS',     '#a699d0', 1),
  ('ALT_PRIVATE_EQUITY',  'asset_classes', 'ALTERNATIVES', 'Private Equity',               'ALT_PRIVATE_EQUITY',  '#afa4d5', 2),
  ('ALT_PRIVATE_CREDIT',  'asset_classes', 'ALTERNATIVES', 'Private Credit',               'ALT_PRIVATE_CREDIT',  '#b8afda', 3),
  ('ALT_INFRASTRUCTURE',  'asset_classes', 'ALTERNATIVES', 'Infrastructure',               'ALT_INFRASTRUCTURE',  '#c4b9e0', 4),
  ('ALT_REAL_ASSETS',     'asset_classes', 'ALTERNATIVES', 'Real Assets (Other)',          'ALT_REAL_ASSETS',     '#cfc4e5', 5),
  ('ALT_ILS',             'asset_classes', 'ALTERNATIVES', 'Insurance-Linked Securities',  'ALT_ILS',             '#dacfea', 6),
  ('ALT_COLLECTIBLES',    'asset_classes', 'ALTERNATIVES', 'Collectibles',                 'ALT_COLLECTIBLES',    '#e5daef', 7),
  ('ALT_COLLECT_ART',       'asset_classes', 'ALT_COLLECTIBLES', 'Art',       'ALT_COLLECT_ART',       '#e5daef', 1),
  ('ALT_COLLECT_WINE',      'asset_classes', 'ALT_COLLECTIBLES', 'Wine',      'ALT_COLLECT_WINE',      '#ebe2f3', 2),
  ('ALT_COLLECT_TANGIBLES', 'asset_classes', 'ALT_COLLECTIBLES', 'Tangibles', 'ALT_COLLECT_TANGIBLES', '#f0e9f6', 3),
  ('DA_CRYPTO',        'asset_classes', 'DIGITAL_ASSETS', 'Cryptocurrencies',            'DA_CRYPTO',        '#e47da8', 1),
  ('DA_STABLECOINS',   'asset_classes', 'DIGITAL_ASSETS', 'Stablecoins',                 'DA_STABLECOINS',   '#e88db3', 2),
  ('DA_DEFI',          'asset_classes', 'DIGITAL_ASSETS', 'DeFi',                        'DA_DEFI',          '#ec9dbe', 3),
  ('DA_NFTS',          'asset_classes', 'DIGITAL_ASSETS', 'NFTs',                        'DA_NFTS',          '#fccfda', 4),
  ('DA_RWA',           'asset_classes', 'DIGITAL_ASSETS', 'Tokenized Real-World Assets', 'DA_RWA',           '#f8b9d1', 5),
  ('DA_CRYPTO_PAYMENTS',     'asset_classes', 'DA_CRYPTO', 'Payments / Store of Value', 'DA_CRYPTO_PAYMENTS',     '#f4a4c2', 1),
  ('DA_CRYPTO_LAYER1',       'asset_classes', 'DA_CRYPTO', 'Layer 1',                   'DA_CRYPTO_LAYER1',       '#f8b9d1', 2),
  ('DA_CRYPTO_LAYER2',       'asset_classes', 'DA_CRYPTO', 'Layer 2',                   'DA_CRYPTO_LAYER2',       '#fccfda', 3),
  ('DA_STABLECOIN_FIAT',      'asset_classes', 'DA_STABLECOINS', 'Fiat-Backed',       'DA_STABLECOIN_FIAT',      '#f4a4c2', 1),
  ('DA_STABLECOIN_CRYPTO',    'asset_classes', 'DA_STABLECOINS', 'Crypto-Backed',     'DA_STABLECOIN_CRYPTO',    '#f8b9d1', 2),
  ('DA_STABLECOIN_ALGO',      'asset_classes', 'DA_STABLECOINS', 'Algorithmic',       'DA_STABLECOIN_ALGO',      '#fccfda', 3);

-- ============================================================================
-- ALLOCATION TARGETS
-- ============================================================================

CREATE TABLE IF NOT EXISTS allocation_targets (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL CHECK (length(trim(name)) > 0),
    scope_type TEXT NOT NULL CHECK (scope_type IN ('all', 'portfolio', 'account')),
    scope_id TEXT,
    taxonomy_id TEXT NOT NULL DEFAULT 'asset_classes',

    trigger_type TEXT NOT NULL DEFAULT 'threshold' CHECK (trigger_type IN ('manual', 'threshold')),
    drift_band_bps INTEGER NOT NULL DEFAULT 500 CHECK (drift_band_bps >= 0 AND drift_band_bps <= 10000),
    rebalance_goal TEXT NOT NULL DEFAULT 'nearest_band'
        CHECK (rebalance_goal IN ('nearest_band', 'exact_target')),
    min_trade_amount TEXT NOT NULL DEFAULT '0',
    whole_shares_only INTEGER NOT NULL DEFAULT 0,
    allow_sells INTEGER NOT NULL DEFAULT 0,
    max_turnover_bps INTEGER DEFAULT NULL
        CHECK (max_turnover_bps IS NULL
               OR (max_turnover_bps >= 0 AND max_turnover_bps <= 10000)),

    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    archived_at TEXT,

    CHECK (
        (scope_type = 'all' AND scope_id IS NULL) OR
        (scope_type IN ('account', 'portfolio') AND scope_id IS NOT NULL)
    )
);

CREATE INDEX IF NOT EXISTS idx_allocation_targets_scope
ON allocation_targets(scope_type, scope_id, archived_at);

CREATE TABLE IF NOT EXISTS allocation_target_weights (
    id TEXT PRIMARY KEY NOT NULL,
    target_id TEXT NOT NULL,
    taxonomy_id TEXT NOT NULL,
    category_id TEXT NOT NULL,
    target_bps INTEGER NOT NULL CHECK (target_bps >= 0 AND target_bps <= 10000),
    is_locked INTEGER NOT NULL DEFAULT 0,
    is_required INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    FOREIGN KEY (target_id) REFERENCES allocation_targets(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id, taxonomy_id) REFERENCES taxonomy_categories(id, taxonomy_id) ON DELETE RESTRICT,
    UNIQUE(target_id, taxonomy_id, category_id)
);

CREATE INDEX IF NOT EXISTS idx_allocation_target_weights_target
ON allocation_target_weights(target_id);

CREATE TABLE IF NOT EXISTS allocation_target_constraints (
    id TEXT PRIMARY KEY NOT NULL,
    target_id TEXT NOT NULL,
    subject_type TEXT NOT NULL CHECK (subject_type IN ('asset', 'account', 'category')),
    subject_id TEXT NOT NULL,
    action TEXT NOT NULL CHECK (action IN ('buy', 'sell', 'trade')),
    effect TEXT NOT NULL DEFAULT 'block' CHECK (effect IN ('block', 'avoid')),
    reason TEXT,
    metadata_json TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    FOREIGN KEY (target_id) REFERENCES allocation_targets(id) ON DELETE CASCADE,
    UNIQUE(target_id, subject_type, subject_id, action, effect)
);

CREATE INDEX IF NOT EXISTS idx_allocation_target_constraints_target
ON allocation_target_constraints(target_id);

CREATE INDEX IF NOT EXISTS idx_allocation_target_constraints_lookup
ON allocation_target_constraints(target_id, subject_type, action, effect);

-- Consistency triggers: a target's taxonomy cannot change while weights exist,
-- and weight taxonomy must always match the owning target.
CREATE TRIGGER IF NOT EXISTS allocation_targets_taxonomy_update
BEFORE UPDATE OF taxonomy_id ON allocation_targets
FOR EACH ROW
WHEN OLD.taxonomy_id <> NEW.taxonomy_id
    AND EXISTS (
        SELECT 1 FROM allocation_target_weights
        WHERE target_id = OLD.id
    )
BEGIN
    SELECT RAISE(ABORT, 'allocation_targets.taxonomy_id cannot change while weights exist');
END;

CREATE TRIGGER IF NOT EXISTS allocation_target_weights_taxonomy_insert
BEFORE INSERT ON allocation_target_weights
FOR EACH ROW
WHEN (SELECT taxonomy_id FROM allocation_targets WHERE id = NEW.target_id) <> NEW.taxonomy_id
BEGIN
    SELECT RAISE(ABORT, 'allocation_target_weights.taxonomy_id must match allocation_targets.taxonomy_id');
END;

CREATE TRIGGER IF NOT EXISTS allocation_target_weights_taxonomy_update
BEFORE UPDATE OF target_id, taxonomy_id ON allocation_target_weights
FOR EACH ROW
WHEN (SELECT taxonomy_id FROM allocation_targets WHERE id = NEW.target_id) <> NEW.taxonomy_id
BEGIN
    SELECT RAISE(ABORT, 'allocation_target_weights.taxonomy_id must match allocation_targets.taxonomy_id');
END;
