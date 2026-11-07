// Tests for the OpenAI-backed agent runtime.

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    use chrono::Utc;
    use serde_json::{json, Value};

    use crate::agent::context::{
        ContextDocument, ContextTask, EmptyContextProvider, TaskContext, TaskContextProvider,
    };
    use crate::agent::openai_runtime::{parse_research_output, OpenAiRuntime, ResearchOutput};
    use crate::agent::runtime::{AgentRuntime, CancellationToken, ProgressReporter};
    use crate::agent::tools::{AgentTool, ToolRegistry};
    use crate::error::AppError;
    use crate::providers::ai::{
        AiClient, ChatMessage, ChatRequest, Completion, ToolCall, ToolCallFunction,
    };
    use domain::task::{AgentTask, TaskStatus};

    enum FakeResponse {
        Content(String),
        ToolCalls(Vec<ToolCall>),
        Error(String),
    }

    struct FakeClient {
        responses: Mutex<VecDeque<FakeResponse>>,
        requests: Mutex<Vec<ChatRequest>>,
        delay: Option<Duration>,
    }

    impl FakeClient {
        fn new(responses: Vec<FakeResponse>) -> Self {
            Self {
                responses: Mutex::new(responses.into()),
                requests: Mutex::new(Vec::new()),
                delay: None,
            }
        }

        fn content(content: &str) -> Self {
            Self::new(vec![FakeResponse::Content(content.to_string())])
        }

        fn failing(message: &str) -> Self {
            Self::new(vec![FakeResponse::Error(message.to_string())])
        }

        fn delayed(content: &str, delay: Duration) -> Self {
            let mut client = Self::content(content);
            client.delay = Some(delay);
            client
        }

        fn requests(&self) -> Vec<ChatRequest> {
            self.requests.lock().expect("requests lock").clone()
        }
    }

    #[async_trait::async_trait]
    impl AiClient for FakeClient {
        async fn complete(&self, request: ChatRequest) -> Result<Completion, AppError> {
            self.requests.lock().expect("requests lock").push(request);

            if let Some(delay) = self.delay {
                tokio::time::sleep(delay).await;
            }

            match self.responses.lock().expect("responses lock").pop_front() {
                Some(FakeResponse::Content(content)) => Ok(Completion::Message(content)),
                Some(FakeResponse::ToolCalls(calls)) => Ok(Completion::ToolCalls(calls)),
                Some(FakeResponse::Error(message)) => Err(AppError::Provider(message)),
                None => Ok(Completion::Message(VALID_OUTPUT.to_string())),
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

    struct FakeContext(TaskContext);

    #[async_trait::async_trait]
    impl TaskContextProvider for FakeContext {
        async fn build(&self, _task: &AgentTask) -> Result<TaskContext, AppError> {
            Ok(self.0.clone())
        }
    }

    struct EchoTool;

    #[async_trait::async_trait]
    impl AgentTool for EchoTool {
        fn name(&self) -> &'static str {
            "echo"
        }

        fn description(&self) -> &'static str {
            "Echo arguments."
        }

        fn parameters(&self) -> Value {
            json!({"type": "object", "properties": {}})
        }

        async fn execute(&self, arguments: Value) -> Result<Value, AppError> {
            Ok(json!({ "echo": arguments }))
        }
    }

    struct FailingTool;

    #[async_trait::async_trait]
    impl AgentTool for FailingTool {
        fn name(&self) -> &'static str {
            "failing_tool"
        }

        fn description(&self) -> &'static str {
            "Always fails."
        }

        fn parameters(&self) -> Value {
            json!({"type": "object", "properties": {}})
        }

        async fn execute(&self, _arguments: Value) -> Result<Value, AppError> {
            Err(AppError::Validation("tool failed".to_string()))
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

    fn tool_call(id: &str, name: &str, arguments: &str) -> ToolCall {
        ToolCall {
            id: id.to_string(),
            call_type: "function".to_string(),
            function: ToolCallFunction {
                name: name.to_string(),
                arguments: arguments.to_string(),
            },
        }
    }

    fn runtime(client: Arc<FakeClient>, tools: Vec<Arc<dyn AgentTool>>) -> OpenAiRuntime {
        OpenAiRuntime::new(
            client,
            Arc::new(EmptyContextProvider),
            ToolRegistry::new(tools),
        )
    }

    const VALID_OUTPUT: &str =
        r#"{"summary":"Demand is mixed","key_points":["a"],"risks":["b"],"open_questions":["c"]}"#;

    const OUTPUT_WITH_ARTIFACT: &str = r#"{"summary":"Summary","artifacts":[{"type":"timeline","title":"Timeline","data":{"events":[{"date":"2026-01-01","title":"Start"}]}}]}"#;

    const OUTPUT_WITH_INVALID_ARTIFACT: &str =
        r#"{"summary":"Summary","artifacts":[{"type":"pie-chart","title":"Pie","data":{}}]}"#;

    const OUTPUT_WITH_CITATIONS: &str = r#"{"summary":"Summary","citations":[{"kind":"document","id":"doc-1","title":"Tesla memo"},{"kind":"task","id":"task-1","title":"Earlier analysis"}]}"#;

    const OUTPUT_WITH_INVALID_CITATION: &str = r#"{"summary":"Summary","citations":[{"kind":"document","id":"   ","title":"Bad"},{"kind":"task","id":"task-1","title":"Ok"}]}"#;

    #[tokio::test]
    async fn test_execute_stores_citations() {
        let runtime = runtime(
            Arc::new(FakeClient::content(OUTPUT_WITH_CITATIONS)),
            Vec::new(),
        );

        let result = runtime
            .execute(
                make_task(None),
                Arc::new(RecordingProgress::default()),
                CancellationToken::new(),
            )
            .await
            .expect("execution should succeed");

        let stored: Value =
            serde_json::from_str(result.output.as_deref().expect("output should exist"))
                .expect("stored output should be JSON");

        assert_eq!(stored["citations"][0]["kind"], "document");
        assert_eq!(stored["citations"][0]["id"], "doc-1");
        assert_eq!(stored["citations"][1]["kind"], "task");
        assert_eq!(stored["citations"][1]["title"], "Earlier analysis");
    }

    #[tokio::test]
    async fn test_execute_drops_invalid_citations() {
        let runtime = runtime(
            Arc::new(FakeClient::content(OUTPUT_WITH_INVALID_CITATION)),
            Vec::new(),
        );

        let result = runtime
            .execute(
                make_task(None),
                Arc::new(RecordingProgress::default()),
                CancellationToken::new(),
            )
            .await
            .expect("execution should succeed");

        let stored: Value =
            serde_json::from_str(result.output.as_deref().expect("output should exist"))
                .expect("stored output should be JSON");

        let citations = stored["citations"].as_array().expect("citations array");
        assert_eq!(citations.len(), 1);
        assert_eq!(citations[0]["id"], "task-1");
    }

    #[tokio::test]
    async fn test_execute_collects_valid_artifact_drafts() {
        let runtime = runtime(
            Arc::new(FakeClient::content(OUTPUT_WITH_ARTIFACT)),
            Vec::new(),
        );

        let result = runtime
            .execute(
                make_task(None),
                Arc::new(RecordingProgress::default()),
                CancellationToken::new(),
            )
            .await
            .expect("execution should succeed");

        assert!(result.success);
        assert_eq!(result.artifacts.len(), 1);
        assert_eq!(result.artifacts[0].artifact_type, "timeline");
        assert!(result.artifacts[0].data.contains("\"Start\""));

        let stored: Value =
            serde_json::from_str(result.output.as_deref().expect("output should exist"))
                .expect("stored output should be JSON");
        assert_eq!(stored["summary"], "Summary");
        assert!(stored.get("artifacts").is_none());
    }

    #[tokio::test]
    async fn test_execute_drops_invalid_artifact_drafts() {
        let runtime = runtime(
            Arc::new(FakeClient::content(OUTPUT_WITH_INVALID_ARTIFACT)),
            Vec::new(),
        );

        let result = runtime
            .execute(
                make_task(None),
                Arc::new(RecordingProgress::default()),
                CancellationToken::new(),
            )
            .await
            .expect("execution should succeed");

        assert!(result.success);
        assert!(result.artifacts.is_empty());
    }

    #[tokio::test]
    async fn test_execute_returns_validated_output() {
        let client = Arc::new(FakeClient::content(VALID_OUTPUT));
        let runtime = runtime(client.clone(), Vec::new());
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
        assert!(matches!(
            &requests[0].messages[0],
            ChatMessage::System { .. }
        ));
        let user_prompt = match &requests[0].messages[1] {
            ChatMessage::User { content } => content.clone(),
            other => panic!("expected user message, got {other:?}"),
        };
        assert!(user_prompt.contains("Analyze Tesla"));
        assert!(user_prompt.contains("Q4 earnings"));

        let messages = progress.messages();
        assert!(messages.iter().any(|m| m.contains("Contacting OpenAI")));
        assert!(messages.iter().any(|m| m.contains("Validating")));
    }

    #[tokio::test]
    async fn test_execute_includes_workspace_context() {
        let client = Arc::new(FakeClient::content(VALID_OUTPUT));
        let context = FakeContext(TaskContext {
            workspace_id: "ws-1".to_string(),
            workspace_name: Some("Alpha Research".to_string()),
            recent_tasks: vec![ContextTask {
                title: "Tesla Q4".to_string(),
                status: TaskStatus::Completed,
                summary: Some("Strong demand".to_string()),
            }],
            research_documents: vec![ContextDocument {
                title: "Tesla memo".to_string(),
                summary: "Margins improving".to_string(),
            }],
        });
        let runtime =
            OpenAiRuntime::new(client.clone(), Arc::new(context), ToolRegistry::default());

        runtime
            .execute(
                make_task(None),
                Arc::new(RecordingProgress::default()),
                CancellationToken::new(),
            )
            .await
            .expect("execution should succeed");

        let requests = client.requests();
        let user_prompt = match &requests[0].messages[1] {
            ChatMessage::User { content } => content.clone(),
            other => panic!("expected user message, got {other:?}"),
        };
        assert!(user_prompt.contains("Workspace ID: ws-1"));
        assert!(user_prompt.contains("Workspace: Alpha Research"));
        assert!(user_prompt.contains("Tesla memo — Margins improving"));
        assert!(user_prompt.contains("Tesla Q4 — Strong demand"));
    }

    #[tokio::test]
    async fn test_execute_runs_tool_calls() {
        let client = Arc::new(FakeClient::new(vec![
            FakeResponse::ToolCalls(vec![tool_call("call-1", "echo", r#"{"value":42}"#)]),
            FakeResponse::Content(VALID_OUTPUT.to_string()),
        ]));
        let runtime = runtime(client.clone(), vec![Arc::new(EchoTool)]);
        let progress = Arc::new(RecordingProgress::default());

        let result = runtime
            .execute(make_task(None), progress.clone(), CancellationToken::new())
            .await
            .expect("execution should succeed");

        assert!(result.success);
        assert!(progress
            .messages()
            .iter()
            .any(|message| message.contains("Running tool: echo")));

        let requests = client.requests();
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0].tools.len(), 1);
        assert_eq!(requests[0].tools[0].name, "echo");

        let second = &requests[1].messages;
        assert!(matches!(
            second[2],
            ChatMessage::Assistant {
                tool_calls: Some(_),
                ..
            }
        ));
        match &second[3] {
            ChatMessage::Tool {
                tool_call_id,
                content,
            } => {
                assert_eq!(tool_call_id, "call-1");
                assert!(content.contains("\"value\":42"));
            }
            other => panic!("expected tool message, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_tool_errors_are_returned_to_model() {
        let client = Arc::new(FakeClient::new(vec![
            FakeResponse::ToolCalls(vec![tool_call("call-1", "failing_tool", "{}")]),
            FakeResponse::Content(VALID_OUTPUT.to_string()),
        ]));
        let runtime = runtime(client.clone(), vec![Arc::new(FailingTool)]);

        let result = runtime
            .execute(
                make_task(None),
                Arc::new(RecordingProgress::default()),
                CancellationToken::new(),
            )
            .await
            .expect("execution should succeed");

        assert!(result.success);

        let requests = client.requests();
        match &requests[1].messages[3] {
            ChatMessage::Tool { content, .. } => {
                assert!(content.contains("tool failed"));
                assert!(content.contains("\"error\""));
            }
            other => panic!("expected tool message, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_execute_stops_after_max_tool_rounds() {
        let responses = (0..10)
            .map(|index| {
                FakeResponse::ToolCalls(vec![tool_call(&format!("call-{index}"), "echo", "{}")])
            })
            .collect();
        let client = Arc::new(FakeClient::new(responses));
        let runtime = runtime(client.clone(), vec![Arc::new(EchoTool)]);

        let result = runtime
            .execute(
                make_task(None),
                Arc::new(RecordingProgress::default()),
                CancellationToken::new(),
            )
            .await;

        match result {
            Err(AppError::Provider(message)) => assert!(message.contains("maximum")),
            other => panic!("expected provider error, got {other:?}"),
        }
        assert_eq!(client.requests().len(), 5);
    }

    #[tokio::test]
    async fn test_execute_rejects_invalid_json() {
        let runtime = runtime(Arc::new(FakeClient::content("not json")), Vec::new());

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
        let runtime = runtime(
            Arc::new(FakeClient::content(r#"{"summary":"   "}"#)),
            Vec::new(),
        );

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
        let runtime = runtime(Arc::new(FakeClient::failing("rate limited")), Vec::new());

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
        let runtime = runtime(client.clone(), Vec::new());
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
        let runtime = Arc::new(runtime(client, Vec::new()));
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
