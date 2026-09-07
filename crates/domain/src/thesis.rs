// Investment thesis domain models for M5 Investment Knowledge System.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Investment thesis - represents an investment thesis with evidence tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentThesis {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub thesis: String,
    pub confidence: i32,
    pub status: ThesisStatus,
    pub validation_date: Option<DateTime<Utc>>,
    pub outcome: Option<String>,
    pub portfolio_asset_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ThesisStatus {
    #[default]
    Draft,
    Active,
    Validating,
    Validated,
    Closed,
}

impl std::fmt::Display for ThesisStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThesisStatus::Draft => write!(f, "draft"),
            ThesisStatus::Active => write!(f, "active"),
            ThesisStatus::Validating => write!(f, "validating"),
            ThesisStatus::Validated => write!(f, "validated"),
            ThesisStatus::Closed => write!(f, "closed"),
        }
    }
}

/// Thesis evidence - supporting or contradicting evidence for a thesis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThesisEvidence {
    pub id: String,
    pub thesis_id: String,
    pub direction: EvidenceDirection,
    pub evidence: String,
    pub source_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Immutable confidence snapshot used to review how a thesis evolved over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThesisConfidenceSnapshot {
    pub id: String,
    pub thesis_id: String,
    pub confidence: i32,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceDirection {
    Supporting,
    Contradicting,
}

impl std::fmt::Display for EvidenceDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvidenceDirection::Supporting => write!(f, "supporting"),
            EvidenceDirection::Contradicting => write!(f, "contradicting"),
        }
    }
}

/// Input for creating a new investment thesis.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateThesisInput {
    pub workspace_id: String,
    pub title: String,
    pub thesis: String,
    pub confidence: Option<i32>,
    pub portfolio_asset_id: Option<String>,
}

impl CreateThesisInput {
    pub fn new(
        workspace_id: impl Into<String>,
        title: impl Into<String>,
        thesis: impl Into<String>,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            title: title.into(),
            thesis: thesis.into(),
            confidence: None,
            portfolio_asset_id: None,
        }
    }

    pub fn with_confidence(mut self, confidence: i32) -> Self {
        self.confidence = Some(confidence);
        self
    }

    pub fn with_portfolio_asset_id(mut self, asset_id: impl Into<String>) -> Self {
        self.portfolio_asset_id = Some(asset_id.into());
        self
    }
}

/// Input for linking or unlinking a thesis to a financial portfolio asset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkThesisAssetInput {
    pub thesis_id: String,
    pub portfolio_asset_id: Option<String>,
}

/// Input for adding evidence to a thesis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddEvidenceInput {
    pub thesis_id: String,
    pub direction: EvidenceDirection,
    pub evidence: String,
    pub source_id: Option<String>,
}

/// Input for updating thesis confidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfidenceInput {
    pub thesis_id: String,
    pub confidence: i32,
}
