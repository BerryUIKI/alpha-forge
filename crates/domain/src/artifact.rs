// Artifact domain models.
//
// An artifact is a validated, renderable view of agent output. The payload is
// stored as canonical JSON and rendered by a predefined React component; the
// agent never supplies HTML.

use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A renderable artifact produced by an agent task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: String,
    pub task_id: Option<String>,
    pub workspace_id: Option<String>,
    pub artifact_type: String,
    pub title: String,
    /// Canonical JSON payload consumed by the matching renderer.
    pub data: String,
    pub status: ArtifactStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Lifecycle of an artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactStatus {
    #[default]
    Pending,
    Generating,
    Completed,
    Failed,
}

impl ArtifactStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ArtifactStatus::Pending => "pending",
            ArtifactStatus::Generating => "generating",
            ArtifactStatus::Completed => "completed",
            ArtifactStatus::Failed => "failed",
        }
    }

    /// Parses a persisted status; `None` for an unknown value.
    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "pending" => Some(ArtifactStatus::Pending),
            "generating" => Some(ArtifactStatus::Generating),
            "completed" => Some(ArtifactStatus::Completed),
            "failed" => Some(ArtifactStatus::Failed),
            _ => None,
        }
    }
}

impl fmt::Display for ArtifactStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
