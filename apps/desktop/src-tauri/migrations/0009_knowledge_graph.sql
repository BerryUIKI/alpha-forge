-- M5 knowledge graph: workspace-scoped entities, directed relationships, and thesis links.

CREATE TABLE IF NOT EXISTS knowledge_entities (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    entity_type TEXT NOT NULL CHECK (entity_type IN ('company', 'industry', 'technology', 'macro_theme')),
    name TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(workspace_id, entity_type, name)
);

CREATE TABLE IF NOT EXISTS knowledge_relationships (
    id TEXT PRIMARY KEY NOT NULL,
    source_entity_id TEXT NOT NULL REFERENCES knowledge_entities(id) ON DELETE CASCADE,
    target_entity_id TEXT NOT NULL REFERENCES knowledge_entities(id) ON DELETE CASCADE,
    relationship_type TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    CHECK (source_entity_id <> target_entity_id),
    UNIQUE(source_entity_id, target_entity_id, relationship_type)
);

CREATE TABLE IF NOT EXISTS thesis_entity_links (
    thesis_id TEXT NOT NULL REFERENCES investment_theses(id) ON DELETE CASCADE,
    entity_id TEXT NOT NULL REFERENCES knowledge_entities(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (thesis_id, entity_id)
);

CREATE INDEX IF NOT EXISTS idx_knowledge_entities_workspace ON knowledge_entities(workspace_id);
CREATE INDEX IF NOT EXISTS idx_knowledge_relationships_source ON knowledge_relationships(source_entity_id);
CREATE INDEX IF NOT EXISTS idx_knowledge_relationships_target ON knowledge_relationships(target_entity_id);
CREATE INDEX IF NOT EXISTS idx_thesis_entity_links_entity ON thesis_entity_links(entity_id);
