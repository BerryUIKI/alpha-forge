// AI provider clients.

use std::sync::Arc;
use std::time::Duration;

use serde::Deserialize;
use serde_json::{json, Value};

use crate::error::AppError;
use crate::security::credentials::{CredentialStore, OPENAI_API_KEY};

/// Model used when none is configured.
pub const DEFAULT_OPENAI_MODEL: &str = "gpt-4o-mini";

const OPENAI_CHAT_COMPLETIONS_URL: &str = "https://api.openai.com/v1/chat/completions";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);
const MAX_RETRIES: u32 = 2;
const RETRY_BASE_DELAY: Duration = Duration::from_millis(500);

/// A chat completion request.
#[derive(Debug, Clone)]
pub struct ChatRequest {
    pub system_prompt: String,
    pub user_prompt: String,
}

/// Minimal interface the agent runtime needs from an AI provider.
#[allow(unknown_lints, clippy::double_must_use)]
#[async_trait::async_trait]
pub trait AiClient: Send + Sync {
    /// Sends the request and returns the assistant message content.
    async fn complete(&self, request: ChatRequest) -> Result<String, AppError>;
}

/// OpenAI chat completions client.
pub struct OpenAiClient {
    http: reqwest::Client,
    credentials: Arc<dyn CredentialStore>,
    model: String,
    endpoint: String,
}

impl OpenAiClient {
    pub fn new(credentials: Arc<dyn CredentialStore>, model: impl Into<String>) -> Self {
        let http = reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()
            .unwrap_or_default();

        Self {
            http,
            credentials,
            model: model.into(),
            endpoint: OPENAI_CHAT_COMPLETIONS_URL.to_string(),
        }
    }

    /// Overrides the endpoint, for example to use an OpenAI-compatible gateway.
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = endpoint.into();
        self
    }

    fn api_key(&self) -> Result<String, AppError> {
        self.credentials.get(OPENAI_API_KEY)?.ok_or_else(|| {
            AppError::Validation(
                "OpenAI API key is not configured. Add it in Settings.".to_string(),
            )
        })
    }
}

#[async_trait::async_trait]
impl AiClient for OpenAiClient {
    async fn complete(&self, request: ChatRequest) -> Result<String, AppError> {
        let api_key = self.api_key()?;
        let body = build_request_body(&self.model, &request);
        let mut attempt = 0;

        loop {
            let response = self
                .http
                .post(&self.endpoint)
                .bearer_auth(&api_key)
                .json(&body)
                .send()
                .await;

            match response {
                Ok(response) => {
                    let status = response.status();
                    if status.is_success() {
                        return parse_completion(response).await;
                    }
                    if is_retryable_status(status) && attempt < MAX_RETRIES {
                        attempt += 1;
                        tokio::time::sleep(retry_delay(attempt)).await;
                        continue;
                    }
                    return Err(map_status(status.as_u16()));
                }
                Err(error) => {
                    if attempt < MAX_RETRIES {
                        attempt += 1;
                        tokio::time::sleep(retry_delay(attempt)).await;
                        continue;
                    }
                    return Err(AppError::Provider(format!(
                        "could not reach OpenAI: {error}"
                    )));
                }
            }
        }
    }
}

async fn parse_completion(response: reqwest::Response) -> Result<String, AppError> {
    let payload: ChatCompletionResponse = response.json().await.map_err(|error| {
        AppError::Provider(format!("OpenAI returned an unreadable response: {error}"))
    })?;

    payload
        .choices
        .into_iter()
        .next()
        .and_then(|choice| choice.message.content)
        .map(|content| content.trim().to_string())
        .filter(|content| !content.is_empty())
        .ok_or_else(|| AppError::Provider("OpenAI returned an empty response".to_string()))
}

fn is_retryable_status(status: reqwest::StatusCode) -> bool {
    status.as_u16() == 429 || status.is_server_error()
}

fn retry_delay(attempt: u32) -> Duration {
    RETRY_BASE_DELAY * 2u32.pow(attempt.saturating_sub(1))
}

fn build_request_body(model: &str, request: &ChatRequest) -> Value {
    json!({
        "model": model,
        "messages": [
            {"role": "system", "content": request.system_prompt},
            {"role": "user", "content": request.user_prompt},
        ],
        "temperature": 0.2,
        "response_format": {"type": "json_object"},
    })
}

fn map_status(status: u16) -> AppError {
    match status {
        400 => {
            AppError::Provider("OpenAI rejected the request. Try rephrasing the task.".to_string())
        }
        401 | 403 => AppError::Provider(
            "OpenAI rejected the API key. Check the key in Settings.".to_string(),
        ),
        429 => AppError::Provider("OpenAI rate limit reached. Try again shortly.".to_string()),
        other => AppError::Provider(format!("OpenAI request failed with status {other}")),
    }
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Debug, Deserialize)]
struct ChatMessage {
    content: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_request_body() {
        let body = build_request_body(
            "gpt-4o-mini",
            &ChatRequest {
                system_prompt: "system".to_string(),
                user_prompt: "user".to_string(),
            },
        );

        assert_eq!(body["model"], "gpt-4o-mini");
        assert_eq!(body["messages"][0]["role"], "system");
        assert_eq!(body["messages"][0]["content"], "system");
        assert_eq!(body["messages"][1]["role"], "user");
        assert_eq!(body["messages"][1]["content"], "user");
        assert_eq!(body["response_format"]["type"], "json_object");
    }

    #[test]
    fn test_map_status_returns_provider_errors() {
        assert!(matches!(map_status(400), AppError::Provider(_)));
        assert!(matches!(map_status(401), AppError::Provider(_)));
        assert!(matches!(map_status(403), AppError::Provider(_)));
        assert!(matches!(map_status(429), AppError::Provider(_)));
        assert!(matches!(map_status(500), AppError::Provider(_)));
    }

    #[test]
    fn test_is_retryable_status() {
        assert!(is_retryable_status(reqwest::StatusCode::TOO_MANY_REQUESTS));
        assert!(is_retryable_status(
            reqwest::StatusCode::INTERNAL_SERVER_ERROR
        ));
        assert!(is_retryable_status(reqwest::StatusCode::BAD_GATEWAY));
        assert!(!is_retryable_status(reqwest::StatusCode::BAD_REQUEST));
        assert!(!is_retryable_status(reqwest::StatusCode::UNAUTHORIZED));
        assert!(!is_retryable_status(reqwest::StatusCode::NOT_FOUND));
    }

    #[test]
    fn test_parses_chat_completion_response() {
        let payload: ChatCompletionResponse = serde_json::from_str(
            r#"{"choices":[{"message":{"role":"assistant","content":"  {\"summary\":\"ok\"}  "}}]}"#,
        )
        .expect("response should deserialize");

        let content = payload
            .choices
            .into_iter()
            .next()
            .and_then(|choice| choice.message.content)
            .expect("content should exist");

        assert_eq!(content.trim(), r#"{"summary":"ok"}"#);
    }
}
