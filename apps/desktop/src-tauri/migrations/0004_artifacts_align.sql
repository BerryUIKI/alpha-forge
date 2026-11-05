-- Artifact presentation metadata and workspace scoping.

ALTER TABLE artifacts ADD COLUMN title TEXT NOT NULL DEFAULT '';

ALTER TABLE artifacts ADD COLUMN workspace_id TEXT;

CREATE INDEX IF NOT EXISTS idx_artifacts_workspace ON artifacts (workspace_id);

CREATE INDEX IF NOT EXISTS idx_artifacts_task ON artifacts (task_id);
