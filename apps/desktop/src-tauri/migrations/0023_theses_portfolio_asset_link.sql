-- Migration 0023: Link Investment Theses to Financial Portfolio Assets (Phase 4).
-- Adds portfolio_asset_id foreign key referencing canonical assets table.

ALTER TABLE investment_theses ADD COLUMN portfolio_asset_id TEXT REFERENCES assets(id) ON DELETE SET NULL;

CREATE INDEX IF NOT EXISTS idx_theses_portfolio_asset ON investment_theses(portfolio_asset_id);
