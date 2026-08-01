-- Enhance artifacts table for M3 Artifact Intelligence System
-- Adds workspace_id, error tracking, and better constraints

-- Add workspace_id column
ALTER TABLE artifacts ADD COLUMN workspace_id TEXT REFERENCES workspaces(id) ON DELETE CASCADE;

-- Add error column
ALTER TABLE artifacts ADD COLUMN error TEXT;

-- Update status constraint to include all M3 states
-- SQLite doesn't support modifying CHECK constraints directly
-- So we create a new table and migrate data

CREATE TABLE artifacts_new (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    task_id TEXT REFERENCES agent_tasks(id) ON DELETE SET NULL,
    artifact_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'generating', 'completed', 'viewing', 'closed', 'failed')),
    input TEXT NOT NULL,
    output TEXT,
    error TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Migrate data from old table
INSERT INTO artifacts_new (id, workspace_id, task_id, artifact_type, status, input, output, error, created_at, updated_at)
SELECT 
    id, 
    COALESCE(workspace_id, 'default-workspace'),  -- Provide default for existing rows
    task_id, 
    artifact_type, 
    status, 
    input, 
    output, 
    error, 
    created_at, 
    updated_at
FROM artifacts;

-- Drop old table
DROP TABLE artifacts;

-- Rename new table
ALTER TABLE artifacts_new RENAME TO artifacts;

-- Create indexes
CREATE INDEX idx_artifacts_workspace_id ON artifacts(workspace_id);
CREATE INDEX idx_artifacts_task_id ON artifacts(task_id);
CREATE INDEX idx_artifacts_status ON artifacts(status);
CREATE INDEX idx_artifacts_type ON artifacts(artifact_type);

-- Create trigger to update updated_at timestamp
CREATE TRIGGER update_artifact_updated_at
AFTER UPDATE ON artifacts
FOR EACH ROW
BEGIN
    UPDATE artifacts SET updated_at = datetime('now') WHERE id = OLD.id;
END;
