-- Research projects for M4 Research Workspace
-- Manages research projects, documents, sources, notes, and reports

-- Research projects table
CREATE TABLE research_projects (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'active'
        CHECK (status IN ('active', 'archived', 'completed')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (workspace_id) REFERENCES workspaces(id) ON DELETE CASCADE
);

-- Research documents table
CREATE TABLE research_documents (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT NOT NULL,
    document_type TEXT NOT NULL
        CHECK (document_type IN ('pdf', 'web_page', 'note', 'report')),
    title TEXT NOT NULL,
    content TEXT,
    source_url TEXT,
    file_path TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (project_id) REFERENCES research_projects(id) ON DELETE CASCADE
);

-- Research sources table
CREATE TABLE research_sources (
    id TEXT PRIMARY KEY NOT NULL,
    document_id TEXT NOT NULL,
    url TEXT,
    title TEXT,
    retrieved_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (document_id) REFERENCES research_documents(id) ON DELETE CASCADE
);

-- Research notes table
CREATE TABLE research_notes (
    id TEXT PRIMARY KEY NOT NULL,
    document_id TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (document_id) REFERENCES research_documents(id) ON DELETE CASCADE
);

-- Research reports table
CREATE TABLE research_reports (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    report_type TEXT NOT NULL
        CHECK (report_type IN ('analysis', 'summary', 'thesis', 'recommendation')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (project_id) REFERENCES research_projects(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX idx_research_projects_workspace ON research_projects(workspace_id);
CREATE INDEX idx_research_projects_status ON research_projects(status);
CREATE INDEX idx_research_documents_project ON research_documents(project_id);
CREATE INDEX idx_research_documents_type ON research_documents(document_type);
CREATE INDEX idx_research_sources_document ON research_sources(document_id);
CREATE INDEX idx_research_notes_document ON research_notes(document_id);
CREATE INDEX idx_research_reports_project ON research_reports(project_id);

-- Triggers for updated_at
CREATE TRIGGER update_research_project_updated_at
AFTER UPDATE ON research_projects
FOR EACH ROW
BEGIN
    UPDATE research_projects SET updated_at = datetime('now') WHERE id = OLD.id;
END;

CREATE TRIGGER update_research_document_updated_at
AFTER UPDATE ON research_documents
FOR EACH ROW
BEGIN
    UPDATE research_documents SET updated_at = datetime('now') WHERE id = OLD.id;
END;

CREATE TRIGGER update_research_note_updated_at
AFTER UPDATE ON research_notes
FOR EACH ROW
BEGIN
    UPDATE research_notes SET updated_at = datetime('now') WHERE id = OLD.id;
END;

CREATE TRIGGER update_research_report_updated_at
AFTER UPDATE ON research_reports
FOR EACH ROW
BEGIN
    UPDATE research_reports SET updated_at = datetime('now') WHERE id = OLD.id;
END;