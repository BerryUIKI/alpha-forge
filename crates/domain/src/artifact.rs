// Artifact domain models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type ArtifactId = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactStatus {
    Pending,
    Generating,
    Completed,
    Viewing,
    Closed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactType {
    ComparisonTable,
    Timeline,
    IndustryMap,
    ValuationModel,
    RiskDashboard,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: ArtifactId,
    pub task_id: Option<String>,
    pub workspace_id: String,
    pub artifact_type: ArtifactType,
    pub status: ArtifactStatus,
    pub input: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateArtifactInput {
    pub workspace_id: String,
    pub task_id: Option<String>,
    pub artifact_type: ArtifactType,
    pub input: serde_json::Value,
}
