// Workspace domain model.
//
// A Workspace is an independent research environment.
// Future contents: Documents, Notes, Thesis, Agent Tasks, Artifacts, Portfolio Context.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A workspace represents an independent research environment.
///
/// Each workspace contains:
/// - Research documents and notes
/// - Investment theses
/// - Agent tasks and artifacts
/// - Portfolio context (optional)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input for creating a new workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkspaceInput {
    pub name: String,
}

/// Input for updating an existing workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkspaceInput {
    pub name: Option<String>,
}