-- Preserve a time series of each thesis confidence assessment.

CREATE TABLE IF NOT EXISTS thesis_confidence_history (
    id TEXT PRIMARY KEY NOT NULL,
    thesis_id TEXT NOT NULL REFERENCES investment_theses(id) ON DELETE CASCADE,
    confidence INTEGER NOT NULL CHECK (confidence >= 0 AND confidence <= 100),
    recorded_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_thesis_confidence_history_thesis_recorded
    ON thesis_confidence_history(thesis_id, recorded_at DESC);
