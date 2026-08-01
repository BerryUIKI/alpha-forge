// Artifact domain models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

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

impl fmt::Display for ArtifactStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArtifactStatus::Pending => write!(f, "pending"),
            ArtifactStatus::Generating => write!(f, "generating"),
            ArtifactStatus::Completed => write!(f, "completed"),
            ArtifactStatus::Viewing => write!(f, "viewing"),
            ArtifactStatus::Closed => write!(f, "closed"),
            ArtifactStatus::Failed => write!(f, "failed"),
        }
    }
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

impl fmt::Display for ArtifactType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArtifactType::ComparisonTable => write!(f, "comparison_table"),
            ArtifactType::Timeline => write!(f, "timeline"),
            ArtifactType::IndustryMap => write!(f, "industry_map"),
            ArtifactType::ValuationModel => write!(f, "valuation_model"),
            ArtifactType::RiskDashboard => write!(f, "risk_dashboard"),
            ArtifactType::Custom(s) => write!(f, "{}", s),
        }
    }
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
