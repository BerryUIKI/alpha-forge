-- Investment thesis management for M5 Investment Knowledge System
-- Manages investment theses, evidence, and validation

-- Investment theses table
CREATE TABLE investment_theses (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    title TEXT NOT NULL,
    thesis TEXT NOT NULL,
    confidence INTEGER NOT NULL DEFAULT 50 CHECK (confidence >= 0 AND confidence <= 100),
    status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'active', 'validating', 'validated', 'closed')),
    validation_date TEXT,
    outcome TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (workspace_id) REFERENCES workspaces(id) ON DELETE CASCADE
);

-- Thesis evidence table
CREATE TABLE thesis_evidence (
    id TEXT PRIMARY KEY NOT NULL,
    thesis_id TEXT NOT NULL,
    direction TEXT NOT NULL CHECK (direction IN ('supporting', 'contradicting')),
    evidence TEXT NOT NULL,
    source_id TEXT REFERENCES research_sources(id) ON DELETE SET NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (thesis_id) REFERENCES investment_theses(id) ON DELETE CASCADE
);

-- Indexes
CREATE INDEX idx_theses_workspace ON investment_theses(workspace_id);
CREATE INDEX idx_theses_status ON investment_theses(status);
CREATE INDEX idx_theses_confidence ON investment_theses(confidence);
CREATE INDEX idx_evidence_thesis ON thesis_evidence(thesis_id);
CREATE INDEX idx_evidence_direction ON thesis_evidence(direction);

-- Triggers
CREATE TRIGGER update_thesis_updated_at
AFTER UPDATE ON investment_theses
FOR EACH ROW
BEGIN
    UPDATE investment_theses SET updated_at = datetime('now') WHERE id = OLD.id;
END;