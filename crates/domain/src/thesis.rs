// Investment thesis domain models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentThesis {
    pub id: String,
    pub title: String,
    pub thesis: String,
    pub confidence: Option<i32>,
    pub status: ThesisStatus,
    pub validation_date: Option<DateTime<Utc>>,
    pub outcome: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThesisStatus {
    Draft,
    Active,
    Validated,
    Invalidated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThesisEvidence {
    pub id: String,
    pub thesis_id: String,
    pub direction: EvidenceDirection,
    pub evidence: String,
    pub source_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceDirection {
    Supporting,
    Contradicting,
}
