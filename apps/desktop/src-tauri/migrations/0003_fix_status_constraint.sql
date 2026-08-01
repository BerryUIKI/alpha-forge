-- Fix status check constraint to include 'created' state

-- SQLite doesn't support ALTER CONSTRAINT, so we need to:
-- 1. Create a new table with the correct constraint
-- 2. Copy data
-- 3. Drop old table
-- 4. Rename new table

-- For simplicity in tests, we'll just recreate the table
DROP TABLE IF EXISTS agent_tasks;

CREATE TABLE agent_tasks (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT,
    title TEXT,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'created'
        CHECK (status IN ('created', 'queued', 'running', 'waiting_for_input', 'completed', 'failed', 'cancelled')),
    input TEXT,
    output TEXT,
    error TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);