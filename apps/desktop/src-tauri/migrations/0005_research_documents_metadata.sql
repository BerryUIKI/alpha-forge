-- Research document metadata, summaries, and soft deletion.

ALTER TABLE research_documents ADD COLUMN kind TEXT NOT NULL DEFAULT 'note';

ALTER TABLE research_documents ADD COLUMN format TEXT NOT NULL DEFAULT 'markdown';

ALTER TABLE research_documents ADD COLUMN summary TEXT;

ALTER TABLE research_documents ADD COLUMN archived_at TEXT;

CREATE INDEX IF NOT EXISTS idx_research_documents_workspace
    ON research_documents (workspace_id, archived_at);
