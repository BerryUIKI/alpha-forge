//! Test fixtures for Goose integration tests
//!
//! Provides synthetic data and mock configurations for testing
//! the Goose adapter without accessing real user data.

use std::path::PathBuf;

use crate::goose::config::{ExecutionBudget, GooseConfig};
use crate::goose::output::{
    Claim, Evidence, EvidenceRelation, Risk, RiskSeverity, StructuredResponse,
};
use crate::goose::recipe::Recipe;

/// Create a test configuration with a mock binary path
pub fn test_config() -> GooseConfig {
    GooseConfig {
        binary_path: PathBuf::from("/usr/bin/true"), // Always succeeds on Unix
        binary_checksum: String::new(),              // Skip integrity check in tests
        timeout: std::time::Duration::from_secs(30),
        max_output_bytes: 1024 * 1024,
        max_turns: 10,
        max_concurrent: 1,
        working_directory: std::env::temp_dir().join("alphaforge-goose-test"),
    }
}

/// Create a test budget
pub fn test_budget() -> ExecutionBudget {
    ExecutionBudget {
        max_tokens: Some(1000),
        max_cost_usd: Some(0.10),
        max_duration: std::time::Duration::from_secs(30),
        max_turns: 10,
    }
}

/// Create a synthetic structured response for testing
pub fn synthetic_response() -> StructuredResponse {
    StructuredResponse {
        summary: "Analysis of synthetic workspace data".into(),
        claims: vec![
            Claim {
                id: "claim-1".into(),
                claim: "Revenue growth accelerated in Q3 2024".into(),
                confidence: 85,
                source_ids: vec!["source-1".into()],
                contradicting_source_ids: vec![],
            },
            Claim {
                id: "claim-2".into(),
                claim: "Market share increased by 2 percentage points".into(),
                confidence: 72,
                source_ids: vec!["source-2".into()],
                contradicting_source_ids: vec![],
            },
        ],
        evidence: vec![
            Evidence {
                claim_id: "claim-1".into(),
                source_id: "source-1".into(),
                excerpt: "Q3 revenue increased 15% year-over-year".into(),
                relation: EvidenceRelation::Supports,
                confidence: Some(90),
            },
            Evidence {
                claim_id: "claim-2".into(),
                source_id: "source-2".into(),
                excerpt: "Market share rose from 18% to 20%".into(),
                relation: EvidenceRelation::Supports,
                confidence: Some(75),
            },
        ],
        contradictions: vec![],
        risks: vec![Risk {
            id: "risk-1".into(),
            risk: "Competitive pressure may intensify in Q4".into(),
            severity: RiskSeverity::Medium,
            related_claim_ids: vec!["claim-2".into()],
            mitigation: Some("Monitor competitor announcements closely".into()),
        }],
        unknowns: vec!["Impact of new product launch remains uncertain".into()],
        source_ids: vec!["source-1".into(), "source-2".into()],
        confidence: 78,
        provider: Some("test-provider".into()),
        model: Some("test-model".into()),
        recipe_version: Some("1.0".into()),
    }
}

/// Create a valid test recipe
pub fn test_recipe() -> Recipe {
    Recipe::shadow_analysis("test-workspace-123")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_is_valid() {
        let config = test_config();
        assert!(config.timeout.as_secs() > 0);
        assert!(config.max_output_bytes > 0);
    }

    #[test]
    fn test_budget_is_valid() {
        let budget = test_budget();
        assert!(budget.max_turns > 0);
    }

    #[test]
    fn synthetic_response_validates() {
        let response = synthetic_response();
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_recipe_validates() {
        let recipe = test_recipe();
        assert!(recipe.validate().is_ok());
    }
}
