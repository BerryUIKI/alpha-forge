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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThesisStatus {
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

impl Default for ThesisStatus {
    fn default() -> Self {
        ThesisStatus::Draft
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateThesisInput {
    pub workspace_id: String,
    pub title: String,
    pub thesis: String,
    pub confidence: Option<i32>,
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
