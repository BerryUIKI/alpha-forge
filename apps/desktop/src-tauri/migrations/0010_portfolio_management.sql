-- M6 portfolio management: scope accounts to a workspace and index holdings.

CREATE INDEX IF NOT EXISTS idx_portfolio_accounts_workspace ON portfolio_accounts(workspace_id);
CREATE INDEX IF NOT EXISTS idx_positions_account ON positions(account_id);
CREATE INDEX IF NOT EXISTS idx_positions_symbol ON positions(symbol);
