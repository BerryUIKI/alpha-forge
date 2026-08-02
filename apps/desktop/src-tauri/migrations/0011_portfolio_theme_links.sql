CREATE TABLE IF NOT EXISTS portfolio_theme_links (
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    symbol TEXT NOT NULL,
    entity_id TEXT NOT NULL REFERENCES knowledge_entities(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL,
    PRIMARY KEY (workspace_id, symbol, entity_id)
);
CREATE INDEX IF NOT EXISTS idx_portfolio_theme_links_workspace ON portfolio_theme_links(workspace_id);
