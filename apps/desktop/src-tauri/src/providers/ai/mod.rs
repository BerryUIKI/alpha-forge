// AI provider clients.
//
// Wire types follow the OpenAI Chat Completions tool-calling protocol:
// messages with `tool_calls` from the assistant, and `role: "tool"` messages
// carrying the matching `tool_call_id` back to the model.

use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::error::AppError;
use crate::security::credentials::{CredentialStore, OPENAI_API_KEY};

/// Model used when none is configured.
pub const DEFAULT_OPENAI_MODEL: &str = "gpt-4o-mini";

const OPENAI_CHAT_COMPLETIONS_URL: &str = "https://api.openai.com/v1/chat/completions";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);
const MAX_RETRIES: u32 = 2;
const RETRY_BASE_DELAY: Duration = Duration::from_millis(500);

/// A message in the provider conversation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum ChatMessage {
    System {
        content: String,
    },
    User {
        content: String,
    },
    Assistant {
        content: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,
    },
    Tool {
        tool_call_id: String,
        content: String,
    },
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self::System {
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self::User {
            content: content.into(),
        }
    }

    pub fn assistant_tool_calls(tool_calls: Vec<ToolCall>) -> Self {
        Self::Assistant {
            content: None,
            tool_calls: Some(tool_calls),
        }
    }

    pub fn tool(tool_call_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self::Tool {
            tool_call_id: tool_call_id.into(),
            content: content.into(),
        }
    }
}

/// A function the model may call.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

/// A function call requested by the model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type", default = "default_tool_call_type")]
    pub call_type: String,
    pub function: ToolCallFunction,
}

fn default_tool_call_type() -> String {
    "function".to_string()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

/// A chat completion request.
#[derive(Debug, Clone)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
    pub tools: Vec<ToolSpec>,
}

/// Result of one completion round.
#[derive(Debug, Clone, PartialEq)]
pub enum Completion {
    /// Final assistant message content.
    Message(String),
    /// Tool calls requested by the model.
    ToolCalls(Vec<ToolCall>),
}

/// Minimal interface the agent runtime needs from an AI provider.
#[allow(unknown_lints, clippy::double_must_use)]
#[async_trait::async_trait]
pub trait AiClient: Send + Sync {
    /// Sends the request and returns the model's next step.
    async fn complete(&self, request: ChatRequest) -> Result<Completion, AppError>;
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
    async fn complete(&self, request: ChatRequest) -> Result<Completion, AppError> {
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

async fn parse_completion(response: reqwest::Response) -> Result<Completion, AppError> {
    let payload: ChatCompletionResponse = response.json().await.map_err(|error| {
        AppError::Provider(format!("OpenAI returned an unreadable response: {error}"))
    })?;

    to_completion(payload)
}

fn to_completion(payload: ChatCompletionResponse) -> Result<Completion, AppError> {
    let message = payload
        .choices
        .into_iter()
        .next()
        .map(|choice| choice.message)
        .ok_or_else(|| AppError::Provider("OpenAI returned no choices".to_string()))?;

    if let Some(tool_calls) = message.tool_calls.filter(|calls| !calls.is_empty()) {
        return Ok(Completion::ToolCalls(tool_calls));
    }

    message
        .content
        .map(|content| content.trim().to_string())
        .filter(|content| !content.is_empty())
        .map(Completion::Message)
        .ok_or_else(|| AppError::Provider("OpenAI returned an empty response".to_string()))
}

fn build_request_body(model: &str, request: &ChatRequest) -> Value {
    let mut body = json!({
        "model": model,
        "messages": request.messages,
        "temperature": 0.2,
        "response_format": {"type": "json_object"},
    });

    if !request.tools.is_empty() {
        let tools = request
            .tools
            .iter()
            .map(|tool| {
                json!({
                    "type": "function",
                    "function": {
                        "name": tool.name,
                        "description": tool.description,
                        "parameters": tool.parameters,
                    },
                })
            })
            .collect::<Vec<_>>();
        body["tools"] = Value::Array(tools);
    }

    body
}

fn is_retryable_status(status: reqwest::StatusCode) -> bool {
    status.as_u16() == 429 || status.is_server_error()
}

fn retry_delay(attempt: u32) -> Duration {
    RETRY_BASE_DELAY * 2u32.pow(attempt.saturating_sub(1))
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
    message: AssistantMessage,
}

#[derive(Debug, Deserialize)]
struct AssistantMessage {
    content: Option<String>,
    tool_calls: Option<Vec<ToolCall>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_request_body_with_tools() {
        let body = build_request_body(
            "gpt-4o-mini",
            &ChatRequest {
                messages: vec![ChatMessage::system("system"), ChatMessage::user("user")],
                tools: vec![ToolSpec {
                    name: "echo".to_string(),
                    description: "Echo input".to_string(),
                    parameters: json!({"type": "object", "properties": {}}),
                }],
            },
        );

        assert_eq!(body["model"], "gpt-4o-mini");
        assert_eq!(body["messages"][0]["role"], "system");
        assert_eq!(body["messages"][0]["content"], "system");
        assert_eq!(body["messages"][1]["role"], "user");
        assert_eq!(body["tools"][0]["type"], "function");
        assert_eq!(body["tools"][0]["function"]["name"], "echo");
        assert_eq!(body["response_format"]["type"], "json_object");
    }

    #[test]
    fn test_build_request_body_without_tools() {
        let body = build_request_body(
            "gpt-4o-mini",
            &ChatRequest {
                messages: vec![ChatMessage::user("hello")],
                tools: Vec::new(),
            },
        );

        assert!(body.get("tools").is_none());
    }

    #[test]
    fn test_serializes_tool_messages() {
        let messages = vec![
            ChatMessage::assistant_tool_calls(vec![ToolCall {
                id: "call-1".to_string(),
                call_type: "function".to_string(),
                function: ToolCallFunction {
                    name: "echo".to_string(),
                    arguments: "{\"value\":1}".to_string(),
                },
            }]),
            ChatMessage::tool("call-1", "{\"ok\":true}"),
        ];

        let value = serde_json::to_value(&messages).expect("messages should serialize");

        assert_eq!(value[0]["role"], "assistant");
        assert!(value[0]["content"].is_null());
        assert_eq!(value[0]["tool_calls"][0]["id"], "call-1");
        assert_eq!(value[0]["tool_calls"][0]["function"]["name"], "echo");
        assert_eq!(value[1]["role"], "tool");
        assert_eq!(value[1]["tool_call_id"], "call-1");
        assert_eq!(value[1]["content"], "{\"ok\":true}");
    }

    #[test]
    fn test_to_completion_returns_message() {
        let payload: ChatCompletionResponse = serde_json::from_str(
            r#"{"choices":[{"message":{"role":"assistant","content":"  {\"summary\":\"ok\"}  "}}]}"#,
        )
        .expect("response should deserialize");

        match to_completion(payload).expect("completion should parse") {
            Completion::Message(content) => assert_eq!(content, r#"{"summary":"ok"}"#),
            other => panic!("expected message, got {other:?}"),
        }
    }

    #[test]
    fn test_to_completion_returns_tool_calls() {
        let payload: ChatCompletionResponse = serde_json::from_str(
            r#"{"choices":[{"message":{"role":"assistant","content":null,"tool_calls":[{"id":"call-1","type":"function","function":{"name":"echo","arguments":"{}"}}]}}]}"#,
        )
        .expect("response should deserialize");

        match to_completion(payload).expect("completion should parse") {
            Completion::ToolCalls(calls) => {
                assert_eq!(calls.len(), 1);
                assert_eq!(calls[0].function.name, "echo");
                assert_eq!(calls[0].call_type, "function");
            }
            other => panic!("expected tool calls, got {other:?}"),
        }
    }

    #[test]
    fn test_to_completion_rejects_empty_response() {
        let payload: ChatCompletionResponse = serde_json::from_str(
            r#"{"choices":[{"message":{"role":"assistant","content":null}}]}"#,
        )
        .expect("response should deserialize");

        assert!(matches!(to_completion(payload), Err(AppError::Provider(_))));
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
}
