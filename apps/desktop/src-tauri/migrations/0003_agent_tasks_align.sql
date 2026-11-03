-- no-transaction
--
-- Align agent_tasks with the task domain model.
--
-- 0001 created agent_tasks before the task model was finalized: `input` was
-- NOT NULL and the status CHECK did not include `created`. 0002 added the
-- workspace/title/description columns. This migration rebuilds the table so
-- the repository can persist the full lifecycle, including the initial
-- `created` state, without requiring an `input` value.
--
-- This migration runs outside a transaction because SQLite's foreign_keys
-- pragma is a no-op inside one. It is re-enabled before the migration ends.

PRAGMA foreign_keys = OFF;

CREATE TABLE agent_tasks_new (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT REFERENCES workspaces(id),
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'created'
        CHECK (status IN ('created', 'queued', 'running', 'waiting_for_input', 'completed', 'failed', 'cancelled')),
    input TEXT,
    output TEXT,
    error TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

INSERT INTO agent_tasks_new (
    id,
    workspace_id,
    title,
    description,
    status,
    input,
    output,
    error,
    created_at,
    updated_at
)
SELECT
    id,
    workspace_id,
    COALESCE(NULLIF(title, ''), NULLIF(input, ''), 'Untitled task'),
    description,
    status,
    input,
    output,
    error,
    created_at,
    updated_at
FROM agent_tasks;

DROP TABLE agent_tasks;
ALTER TABLE agent_tasks_new RENAME TO agent_tasks;

CREATE INDEX IF NOT EXISTS idx_agent_tasks_workspace_id ON agent_tasks(workspace_id);
CREATE INDEX IF NOT EXISTS idx_agent_tasks_status ON agent_tasks(status);
CREATE INDEX IF NOT EXISTS idx_agent_tasks_created_at ON agent_tasks(created_at);
CREATE INDEX IF NOT EXISTS idx_agent_task_events_task_id ON agent_task_events(task_id);
CREATE INDEX IF NOT EXISTS idx_agent_task_events_created_at ON agent_task_events(created_at);

PRAGMA foreign_keys = ON;
