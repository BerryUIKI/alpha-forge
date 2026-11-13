-- Repair the agent_tasks status constraint.
--
-- 0001_initial.sql constrains status without the `created` state, and the
-- reconciliation runner cannot replay 0003_fix_status_constraint.sql because
-- that file drops and recreates the table. This migration rebuilds the table in
-- place with the full domain status set while preserving every row.
--
-- The runner disables foreign keys on this connection before applying the
-- script so the referenced agent_task_events history is not cascaded away.

CREATE TABLE agent_tasks_rebuilt (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT REFERENCES workspaces(id),
    title TEXT,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'created'
        CHECK (status IN ('created', 'queued', 'running', 'waiting_for_input', 'completed', 'failed', 'cancelled')),
    input TEXT NOT NULL,
    output TEXT,
    error TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

INSERT INTO agent_tasks_rebuilt (
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
    title,
    description,
    status,
    input,
    output,
    error,
    created_at,
    updated_at
FROM agent_tasks;

DROP TABLE agent_tasks;

ALTER TABLE agent_tasks_rebuilt RENAME TO agent_tasks;

CREATE INDEX IF NOT EXISTS idx_agent_tasks_workspace_id ON agent_tasks(workspace_id);
CREATE INDEX IF NOT EXISTS idx_agent_tasks_status ON agent_tasks(status);
CREATE INDEX IF NOT EXISTS idx_agent_tasks_created_at ON agent_tasks(created_at);
