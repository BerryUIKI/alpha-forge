//! Structured output from Goose execution
//!
//! Provides validated, schema-compliant responses with:
//! - Source-grounded claims
//! - Evidence traceability
//! - Confidence scores
//! - Provenance metadata

use serde::{Deserialize, Serialize};

use crate::goose::error::GooseError;

/// Complete structured response from Goose
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredResponse {
    /// Brief summary of findings
    pub summary: String,

    /// Claims made with confidence levels
    pub claims: Vec<Claim>,

    /// Evidence supporting claims
    pub evidence: Vec<Evidence>,

    /// Contradictions found
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contradictions: Vec<Contradiction>,

    /// Risks identified
    pub risks: Vec<Risk>,

    /// Unknowns / gaps
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unknowns: Vec<String>,

    /// Source IDs referenced
    pub source_ids: Vec<String>,

    /// Overall confidence (0-100)
    pub confidence: u8,

    /// Provider used
    pub provider: Option<String>,

    /// Model used
    pub model: Option<String>,

    /// Recipe version
    pub recipe_version: Option<String>,
}

/// A claim with confidence and source references
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    /// Unique identifier for this claim
    pub id: String,

    /// The claim text
    pub claim: String,

    /// Confidence level (0-100)
    pub confidence: u8,

    /// Source IDs that support this claim
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_ids: Vec<String>,

    /// Contradicting source IDs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contradicting_source_ids: Vec<String>,
}

/// Evidence linking claims to sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    /// ID of the claim this supports
    pub claim_id: String,

    /// ID of the source
    pub source_id: String,

    /// Excerpt from the source
    pub excerpt: String,

    /// Whether this supports or contradicts
    #[serde(default)]
    pub relation: EvidenceRelation,

    /// Confidence in the evidence link
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<u8>,
}

/// Relation between evidence and claim
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EvidenceRelation {
    #[default]
    Supports,
    Contradicts,
    Neutral,
}

/// A contradiction between sources or claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contradiction {
    /// Description of the contradiction
    pub description: String,

    /// Conflicting claim IDs
    pub claim_ids: Vec<String>,

    /// Conflicting source IDs
    pub source_ids: Vec<String>,
}

/// A risk identified in the analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Risk {
    /// Risk identifier
    pub id: String,

    /// Risk description
    pub risk: String,

    /// Severity level
    pub severity: RiskSeverity,

    /// Related claim IDs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_claim_ids: Vec<String>,

    /// Mitigation suggestions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mitigation: Option<String>,
}

/// Risk severity levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl StructuredResponse {
    /// Validate the response against constraints
    pub fn validate(&self) -> Result<(), GooseError> {
        // Check confidence range
        if self.confidence > 100 {
            return Err(GooseError::OutputValidationFailed {
                reason: "Confidence must be between 0 and 100".into(),
            });
        }

        // Validate claims
        for claim in &self.claims {
            if claim.confidence > 100 {
                return Err(GooseError::OutputValidationFailed {
                    reason: format!("Claim '{}' has invalid confidence", claim.id),
                });
            }
        }

        // Check that all evidence references valid claims
        let claim_ids: Vec<&str> = self.claims.iter().map(|c| c.id.as_str()).collect();
        for evidence in &self.evidence {
            if !claim_ids.contains(&evidence.claim_id.as_str()) {
                return Err(GooseError::OutputValidationFailed {
                    reason: format!("Evidence references unknown claim '{}'", evidence.claim_id),
                });
            }
        }

        // Check that all source IDs are non-empty
        if self.source_ids.iter().any(|id| id.trim().is_empty()) {
            return Err(GooseError::OutputValidationFailed {
                reason: "Source IDs must be non-empty".into(),
            });
        }

        Ok(())
    }

    /// Get claims by confidence level
    pub fn claims_by_confidence(&self, min_confidence: u8) -> Vec<&Claim> {
        self.claims
            .iter()
            .filter(|c| c.confidence >= min_confidence)
            .collect()
    }

    /// Get risks by severity
    pub fn risks_by_severity(&self, severity: RiskSeverity) -> Vec<&Risk> {
        self.risks
            .iter()
            .filter(|r| std::mem::discriminant(&r.severity) == std::mem::discriminant(&severity))
            .collect()
    }
}

/// Raw output from Goose process (before parsing)
#[derive(Debug, Clone)]
pub struct GooseOutput {
    /// Raw stdout bytes
    pub stdout: Vec<u8>,

    /// Redacted stderr
    pub stderr: String,

    /// Exit code
    pub exit_code: i32,

    /// Execution duration in milliseconds
    pub duration_ms: u64,
}

impl GooseOutput {
    /// Parse stdout into structured response
    pub fn parse(&self) -> Result<StructuredResponse, GooseError> {
        let response: StructuredResponse = serde_json::from_slice(&self.stdout)
            .map_err(|e| GooseError::OutputParseError { source: e })?;

        response.validate()?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_response_passes_validation() {
        let response = StructuredResponse {
            summary: "Test summary".into(),
            claims: vec![Claim {
                id: "c1".into(),
                claim: "Test claim".into(),
                confidence: 80,
                source_ids: vec!["s1".into()],
                contradicting_source_ids: vec![],
            }],
            evidence: vec![Evidence {
                claim_id: "c1".into(),
                source_id: "s1".into(),
                excerpt: "Test excerpt".into(),
                relation: EvidenceRelation::Supports,
                confidence: Some(90),
            }],
            contradictions: vec![],
            risks: vec![],
            unknowns: vec![],
            source_ids: vec!["s1".into()],
            confidence: 75,
            provider: Some("openai".into()),
            model: Some("gpt-4".into()),
            recipe_version: Some("1.0".into()),
        };

        assert!(response.validate().is_ok());
    }

    #[test]
    fn invalid_confidence_fails() {
        let response = StructuredResponse {
            summary: "Test".into(),
            claims: vec![],
            evidence: vec![],
            contradictions: vec![],
            risks: vec![],
            unknowns: vec![],
            source_ids: vec![],
            confidence: 150, // Invalid
            provider: None,
            model: None,
            recipe_version: None,
        };

        assert!(response.validate().is_err());
    }

    #[test]
    fn evidence_to_invalid_claim_fails() {
        let response = StructuredResponse {
            summary: "Test".into(),
            claims: vec![Claim {
                id: "c1".into(),
                claim: "Test".into(),
                confidence: 50,
                source_ids: vec![],
                contradicting_source_ids: vec![],
            }],
            evidence: vec![Evidence {
                claim_id: "c2".into(), // Non-existent
                source_id: "s1".into(),
                excerpt: "Test".into(),
                relation: EvidenceRelation::Supports,
                confidence: None,
            }],
            contradictions: vec![],
            risks: vec![],
            unknowns: vec![],
            source_ids: vec!["s1".into()],
            confidence: 50,
            provider: None,
            model: None,
            recipe_version: None,
        };

        assert!(response.validate().is_err());
    }
}
