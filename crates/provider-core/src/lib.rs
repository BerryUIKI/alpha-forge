use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub base_url: String,
    pub api_key_env: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchCompletionRequest {
    pub system_prompt: String,
    pub user_prompt: String,
    pub max_output_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResearchCompletion {
    pub summary: String,
    pub claims: Vec<String>,
    pub evidence: Vec<String>,
    pub risks: Vec<String>,
    pub confidence: u8,
}

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("provider credentials are unavailable")]
    CredentialsUnavailable,
    #[error("provider request failed")]
    RequestFailed,
    #[error("provider response was invalid")]
    InvalidResponse,
}

#[async_trait]
pub trait ResearchProvider: Send + Sync {
    async fn complete_research(&self, request: ResearchCompletionRequest) -> Result<ResearchCompletion, ProviderError>;
}

pub fn parse_research_completion(raw: &str) -> Result<ResearchCompletion, ProviderError> {
    let completion: ResearchCompletion = serde_json::from_str(raw).map_err(|_| ProviderError::InvalidResponse)?;
    if completion.summary.trim().is_empty() || completion.confidence > 100 { return Err(ProviderError::InvalidResponse); }
    Ok(completion)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_structured_research_output() {
        let output = r#"{"summary":"Demand increased","claims":["Demand rose"],"evidence":["Source A"],"risks":["Supply"],"confidence":72}"#;
        assert_eq!(parse_research_completion(output).unwrap().confidence, 72);
    }

    #[test]
    fn rejects_missing_summary_or_invalid_confidence() {
        assert!(parse_research_completion(r#"{"summary":"","claims":[],"evidence":[],"risks":[],"confidence":20}"#).is_err());
        assert!(parse_research_completion(r#"{"summary":"x","claims":[],"evidence":[],"risks":[],"confidence":101}"#).is_err());
    }
}
