// OpenAI-backed agent runtime.
//
// Builds a structured research prompt, calls the configured AI provider, and
// validates the response before persisting it as the task output.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::agent::runtime::{AgentRuntime, CancellationToken, ExecutionResult, ProgressReporter};
use crate::error::AppError;
use crate::providers::ai::{AiClient, ChatRequest};
use domain::task::AgentTask;

const SYSTEM_PROMPT: &str = r#"You are a research assistant inside Investment OS, a local-first investment research workspace.
Research the request using your general knowledge. You are not a financial adviser: do not give buy/sell recommendations, price targets, or trading instructions. State uncertainty explicitly.
Respond in the language of the request.
Return a single JSON object with exactly these fields:
- "summary": a concise answer to the research request.
- "key_points": an array of supporting observations.
- "risks": an array of risks, caveats, or counter-evidence.
- "open_questions": an array of points that still need verification.
Do not wrap the JSON in markdown fences."#;

/// Structured research output produced by the runtime.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResearchOutput {
    pub summary: String,
    #[serde(default)]
    pub key_points: Vec<String>,
    #[serde(default)]
    pub risks: Vec<String>,
    #[serde(default)]
    pub open_questions: Vec<String>,
}

/// Validates the provider response against the research schema.
pub fn parse_research_output(raw: &str) -> Result<ResearchOutput, AppError> {
    let output: ResearchOutput = serde_json::from_str(raw).map_err(|_| {
        AppError::Provider(
            "OpenAI returned output that does not match the research schema".to_string(),
        )
    })?;

    if output.summary.trim().is_empty() {
        return Err(AppError::Provider(
            "OpenAI returned an empty research summary".to_string(),
        ));
    }

    Ok(output)
}

fn build_user_prompt(task: &AgentTask) -> String {
    match task
        .description
        .as_deref()
        .map(str::trim)
        .filter(|description| !description.is_empty())
    {
        Some(description) => format!(
            "Research task: {}\n\nAdditional context:\n{}",
            task.title.trim(),
            description
        ),
        None => format!("Research task: {}", task.title.trim()),
    }
}

/// Agent runtime backed by an AI provider client.
pub struct OpenAiRuntime {
    client: Arc<dyn AiClient>,
}

impl OpenAiRuntime {
    pub fn new(client: Arc<dyn AiClient>) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl AgentRuntime for OpenAiRuntime {
    async fn execute(
        &self,
        task: AgentTask,
        progress: Arc<dyn ProgressReporter>,
        cancellation: CancellationToken,
    ) -> Result<ExecutionResult, AppError> {
        progress
            .report("Preparing research request...".to_string())
            .await?;
        if cancellation.is_cancelled() {
            return Ok(ExecutionResult::cancelled());
        }

        progress.report("Contacting OpenAI...".to_string()).await?;

        let request = ChatRequest {
            system_prompt: SYSTEM_PROMPT.to_string(),
            user_prompt: build_user_prompt(&task),
        };

        let response = tokio::select! {
            result = self.client.complete(request) => result?,
            _ = cancellation.cancelled() => return Ok(ExecutionResult::cancelled()),
        };

        progress
            .report("Validating structured output...".to_string())
            .await?;
        if cancellation.is_cancelled() {
            return Ok(ExecutionResult::cancelled());
        }

        let parsed = parse_research_output(&response)?;
        let output = serde_json::to_string(&parsed).map_err(|error| {
            AppError::Internal(format!("failed to serialize research output: {error}"))
        })?;

        Ok(ExecutionResult::success(Some(output)))
    }
}
