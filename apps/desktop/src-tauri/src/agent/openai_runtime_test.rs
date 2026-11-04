// Tests for the OpenAI-backed agent runtime.

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    use chrono::Utc;

    use crate::agent::openai_runtime::{parse_research_output, OpenAiRuntime, ResearchOutput};
    use crate::agent::runtime::{AgentRuntime, CancellationToken, ProgressReporter};
    use crate::error::AppError;
    use crate::providers::ai::{AiClient, ChatRequest};
    use domain::task::{AgentTask, TaskStatus};

    enum FakeResponse {
        Content(String),
        Error(String),
    }

    struct FakeClient {
        response: FakeResponse,
        delay: Option<Duration>,
        requests: Mutex<Vec<ChatRequest>>,
    }

    impl FakeClient {
        fn content(content: &str) -> Self {
            Self {
                response: FakeResponse::Content(content.to_string()),
                delay: None,
                requests: Mutex::new(Vec::new()),
            }
        }

        fn failing(message: &str) -> Self {
            Self {
                response: FakeResponse::Error(message.to_string()),
                delay: None,
                requests: Mutex::new(Vec::new()),
            }
        }

        fn delayed(content: &str, delay: Duration) -> Self {
            Self {
                response: FakeResponse::Content(content.to_string()),
                delay: Some(delay),
                requests: Mutex::new(Vec::new()),
            }
        }

        fn requests(&self) -> Vec<ChatRequest> {
            self.requests.lock().expect("requests lock").clone()
        }
    }

    #[async_trait::async_trait]
    impl AiClient for FakeClient {
        async fn complete(&self, request: ChatRequest) -> Result<String, AppError> {
            self.requests.lock().expect("requests lock").push(request);
            if let Some(delay) = self.delay {
                tokio::time::sleep(delay).await;
            }
            match &self.response {
                FakeResponse::Content(content) => Ok(content.clone()),
                FakeResponse::Error(message) => Err(AppError::Provider(message.clone())),
            }
        }
    }

    #[derive(Default)]
    struct RecordingProgress {
        messages: Mutex<Vec<String>>,
    }

    impl RecordingProgress {
        fn messages(&self) -> Vec<String> {
            self.messages.lock().expect("progress lock").clone()
        }
    }

    #[async_trait::async_trait]
    impl ProgressReporter for RecordingProgress {
        async fn report(&self, message: String) -> Result<(), AppError> {
            self.messages.lock().expect("progress lock").push(message);
            Ok(())
        }
    }

    fn make_task(description: Option<&str>) -> AgentTask {
        let now = Utc::now();
        AgentTask {
            id: "task-1".to_string(),
            workspace_id: "ws-1".to_string(),
            title: "Analyze Tesla".to_string(),
            description: description.map(str::to_string),
            status: TaskStatus::Running,
            output: None,
            error: None,
            created_at: now,
            updated_at: now,
        }
    }

    const VALID_OUTPUT: &str =
        r#"{"summary":"Demand is mixed","key_points":["a"],"risks":["b"],"open_questions":["c"]}"#;

    #[tokio::test]
    async fn test_execute_returns_validated_output() {
        let client = Arc::new(FakeClient::content(VALID_OUTPUT));
        let runtime = OpenAiRuntime::new(client.clone());
        let progress = Arc::new(RecordingProgress::default());

        let result = runtime
            .execute(
                make_task(Some("Q4 earnings")),
                progress.clone(),
                CancellationToken::new(),
            )
            .await
            .expect("execution should succeed");

        assert!(result.success);
        let output: ResearchOutput =
            serde_json::from_str(result.output.as_deref().expect("output should exist"))
                .expect("output should be JSON");
        assert_eq!(output.summary, "Demand is mixed");
        assert_eq!(output.key_points, vec!["a"]);

        let requests = client.requests();
        assert_eq!(requests.len(), 1);
        assert!(requests[0].user_prompt.contains("Analyze Tesla"));
        assert!(requests[0].user_prompt.contains("Q4 earnings"));

        let messages = progress.messages();
        assert!(messages.iter().any(|m| m.contains("Contacting OpenAI")));
        assert!(messages.iter().any(|m| m.contains("Validating")));
    }

    #[tokio::test]
    async fn test_execute_rejects_invalid_json() {
        let runtime = OpenAiRuntime::new(Arc::new(FakeClient::content("not json")));

        let result = runtime
            .execute(
                make_task(None),
                Arc::new(RecordingProgress::default()),
                CancellationToken::new(),
            )
            .await;

        assert!(matches!(result, Err(AppError::Provider(_))));
    }

    #[tokio::test]
    async fn test_execute_rejects_empty_summary() {
        let runtime = OpenAiRuntime::new(Arc::new(FakeClient::content(r#"{"summary":"   "}"#)));

        let result = runtime
            .execute(
                make_task(None),
                Arc::new(RecordingProgress::default()),
                CancellationToken::new(),
            )
            .await;

        assert!(matches!(result, Err(AppError::Provider(_))));
    }

    #[tokio::test]
    async fn test_execute_propagates_client_error() {
        let runtime = OpenAiRuntime::new(Arc::new(FakeClient::failing("rate limited")));

        let result = runtime
            .execute(
                make_task(None),
                Arc::new(RecordingProgress::default()),
                CancellationToken::new(),
            )
            .await;

        match result {
            Err(AppError::Provider(message)) => assert!(message.contains("rate limited")),
            other => panic!("expected provider error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_execute_skips_provider_when_already_cancelled() {
        let client = Arc::new(FakeClient::content(VALID_OUTPUT));
        let runtime = OpenAiRuntime::new(client.clone());
        let cancellation = CancellationToken::new();
        cancellation.cancel();

        let result = runtime
            .execute(
                make_task(None),
                Arc::new(RecordingProgress::default()),
                cancellation,
            )
            .await
            .expect("cancelled execution should not error");

        assert!(result.cancelled);
        assert!(client.requests().is_empty());
    }

    #[tokio::test]
    async fn test_execute_stops_waiting_when_cancelled() {
        let client = Arc::new(FakeClient::delayed(VALID_OUTPUT, Duration::from_secs(10)));
        let runtime = Arc::new(OpenAiRuntime::new(client));
        let cancellation = CancellationToken::new();
        let cancellation_handle = cancellation.clone();

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(50)).await;
            cancellation_handle.cancel();
        });

        let started = Instant::now();
        let result = runtime
            .execute(
                make_task(None),
                Arc::new(RecordingProgress::default()),
                cancellation,
            )
            .await
            .expect("cancelled execution should not error");

        assert!(result.cancelled);
        assert!(started.elapsed() < Duration::from_secs(2));
    }

    #[test]
    fn test_parse_research_output_uses_defaults_for_lists() {
        let output =
            parse_research_output(r#"{"summary":"ok","extra":"ignored"}"#).expect("should parse");

        assert_eq!(output.summary, "ok");
        assert!(output.key_points.is_empty());
        assert!(output.risks.is_empty());
        assert!(output.open_questions.is_empty());
    }
}
