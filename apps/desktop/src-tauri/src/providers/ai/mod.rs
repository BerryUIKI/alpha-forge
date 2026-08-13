use std::time::Duration;

use provider_core::{
    parse_research_completion, ProviderError, ResearchCompletion, ResearchCompletionRequest,
    ResearchProvider,
};
use reqwest::Client;
use serde::Serialize;
use serde_json::{json, Value};

use crate::security::credentials::{
    load_openai_api_key, CredentialStore, OsKeychainCredentialStore,
};

const OPENAI_RESPONSES_URL: &str = "https://api.openai.com/v1/responses";
const DEFAULT_MODEL: &str = "gpt-5.6-terra";

pub struct OpenAiResearchProvider {
    client: Client,
    credential_store: OsKeychainCredentialStore,
    model: String,
}

impl OpenAiResearchProvider {
    pub fn new() -> Result<Self, ProviderError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(90))
            .build()
            .map_err(|_| ProviderError::RequestFailed)?;
        Ok(Self {
            client,
            credential_store: OsKeychainCredentialStore,
            model: DEFAULT_MODEL.into(),
        })
    }

    #[cfg(test)]
    fn with_model(model: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            credential_store: OsKeychainCredentialStore,
            model: model.into(),
        }
    }
}

#[async_trait::async_trait]
impl ResearchProvider for OpenAiResearchProvider {
    async fn complete_research(
        &self,
        request: ResearchCompletionRequest,
    ) -> Result<ResearchCompletion, ProviderError> {
        let api_key = provider_api_key(&self.credential_store)?;
        let payload = ResponsesRequest::new(&self.model, request);
        let response = self
            .client
            .post(OPENAI_RESPONSES_URL)
            .bearer_auth(api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|_| ProviderError::RequestFailed)?;
        if !response.status().is_success() {
            return Err(ProviderError::RequestFailed);
        }
        let response: Value = response
            .json()
            .await
            .map_err(|_| ProviderError::InvalidResponse)?;
        parse_research_completion(
            &extract_output_text(&response).ok_or(ProviderError::InvalidResponse)?,
        )
    }
}

fn provider_api_key(store: &impl CredentialStore) -> Result<String, ProviderError> {
    load_openai_api_key(store)
        .map_err(|_| ProviderError::CredentialsUnavailable)?
        .ok_or(ProviderError::CredentialsUnavailable)
}

#[derive(Serialize)]
struct ResponsesRequest {
    model: String,
    input: String,
    max_output_tokens: u32,
    text: Value,
}

impl ResponsesRequest {
    fn new(model: &str, request: ResearchCompletionRequest) -> Self {
        let max_output_tokens = request.max_output_tokens.clamp(256, 4_096);
        Self {
            model: model.to_string(),
            input: format!(
                "{}\n\nUser research request:\n{}",
                request.system_prompt, request.user_prompt
            ),
            max_output_tokens,
            text: json!({ "format": { "type": "json_schema", "name": "research_completion", "strict": true, "schema": research_completion_schema() } }),
        }
    }
}

fn research_completion_schema() -> Value {
    json!({
        "type": "object",
        "additionalProperties": false,
        "required": ["summary", "claims", "evidence", "risks", "confidence"],
        "properties": {
            "summary": { "type": "string" },
            "claims": { "type": "array", "items": { "type": "string" } },
            "evidence": { "type": "array", "items": { "type": "string" } },
            "risks": { "type": "array", "items": { "type": "string" } },
            "confidence": { "type": "integer", "minimum": 0, "maximum": 100 }
        }
    })
}

fn extract_output_text(response: &Value) -> Option<String> {
    response
        .get("output_text")
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| {
            response
                .get("output")?
                .as_array()?
                .iter()
                .flat_map(|item| {
                    item.get("content")
                        .and_then(Value::as_array)
                        .into_iter()
                        .flatten()
                })
                .find_map(|content| {
                    (content.get("type").and_then(Value::as_str) == Some("output_text"))
                        .then(|| {
                            content
                                .get("text")
                                .and_then(Value::as_str)
                                .map(str::to_string)
                        })
                        .flatten()
                })
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AppError;
    use std::collections::HashMap;
    use std::sync::Mutex;

    struct FakeCredentialStore(Mutex<HashMap<String, String>>);

    impl FakeCredentialStore {
        fn with_credential(name: &str, value: &str) -> Self {
            Self(Mutex::new(HashMap::from([(name.into(), value.into())])))
        }

        fn value(&self, name: &str) -> Option<String> {
            self.0.lock().unwrap().get(name).cloned()
        }
    }

    impl CredentialStore for FakeCredentialStore {
        fn set(&self, credential_name: &str, secret: &str) -> Result<(), AppError> {
            self.0
                .lock()
                .unwrap()
                .insert(credential_name.into(), secret.into());
            Ok(())
        }

        fn get(&self, credential_name: &str) -> Result<Option<String>, AppError> {
            Ok(self.0.lock().unwrap().get(credential_name).cloned())
        }

        fn delete(&self, credential_name: &str) -> Result<(), AppError> {
            self.0.lock().unwrap().remove(credential_name);
            Ok(())
        }
    }

    #[test]
    fn builds_a_bounded_structured_request() {
        let provider = OpenAiResearchProvider::with_model("test-model");
        let request = ResponsesRequest::new(
            &provider.model,
            ResearchCompletionRequest {
                system_prompt: "Be factual".into(),
                user_prompt: "Research demand".into(),
                max_output_tokens: 9_999,
            },
        );
        assert_eq!(request.model, "test-model");
        assert_eq!(request.max_output_tokens, 4_096);
        assert_eq!(request.text["format"]["type"], "json_schema");
    }

    #[test]
    fn extracts_the_responses_output_text() {
        let response =
            json!({"output":[{"content":[{"type":"output_text","text":"{\"summary\":\"x\"}"}]}]});
        assert_eq!(
            extract_output_text(&response),
            Some("{\"summary\":\"x\"}".into())
        );
    }

    #[test]
    fn provider_lookup_uses_the_canonical_openai_key_and_reports_missing_keys() {
        assert_eq!(
            provider_api_key(&FakeCredentialStore::with_credential(
                "openai.api_key",
                "secret",
            ))
            .unwrap(),
            "secret"
        );
        assert!(provider_api_key(&FakeCredentialStore(Mutex::new(HashMap::new()))).is_err());
    }

    #[test]
    fn provider_lookup_migrates_the_legacy_openai_key() {
        let store = FakeCredentialStore::with_credential("api_key", "legacy");

        assert_eq!(provider_api_key(&store).unwrap(), "legacy");
        assert_eq!(store.value("openai.api_key").as_deref(), Some("legacy"));
        assert_eq!(store.value("api_key"), None);
    }
}
