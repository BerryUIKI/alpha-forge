-- Reconcile the runtime schema with the M2-M5 repositories.
--
-- Earlier migrations overlap with 0001_initial.sql and cannot safely be replayed
-- in sequence. Column additions are performed conditionally by the migration
-- runner; this script creates tables that did not exist in the original schema.

CREATE TABLE IF NOT EXISTS research_projects (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'active'
        CHECK (status IN ('active', 'archived', 'completed')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS research_reports (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT NOT NULL REFERENCES research_projects(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    report_type TEXT NOT NULL
        CHECK (report_type IN ('analysis', 'summary', 'thesis', 'recommendation')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_agent_tasks_workspace_id ON agent_tasks(workspace_id);
CREATE INDEX IF NOT EXISTS idx_agent_tasks_status ON agent_tasks(status);
CREATE INDEX IF NOT EXISTS idx_artifacts_workspace_id ON artifacts(workspace_id);
CREATE INDEX IF NOT EXISTS idx_research_projects_workspace ON research_projects(workspace_id);
CREATE INDEX IF NOT EXISTS idx_research_documents_project ON research_documents(project_id);
CREATE INDEX IF NOT EXISTS idx_research_reports_project ON research_reports(project_id);
CREATE INDEX IF NOT EXISTS idx_theses_workspace ON investment_theses(workspace_id);
