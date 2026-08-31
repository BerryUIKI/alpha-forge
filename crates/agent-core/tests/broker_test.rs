use agent_core::broker::{FixtureModelAdapter, ProviderBroker, ToolBroker, ToolHandler};
use agent_protocol::messages::{ProviderRequest, RunScope, ToolRequest};
use async_trait::async_trait;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn test_provider_broker_success_and_allowlist() {
    let mut broker = ProviderBroker::new();
    broker.register_adapter(
        "custom_mock",
        Arc::new(FixtureModelAdapter::new("Mocked reply.")),
    );

    // Allowed request
    let req = ProviderRequest {
        request_id: "req-1".into(),
        provider: "fixture".into(),
        model: "fixture-model".into(),
        prompt: "Analyze revenue drivers.".into(),
        temperature: Some(0.7),
        max_tokens: Some(1000),
    };

    let res = broker.handle_request(&req).await.unwrap();
    assert_eq!(res.request_id, "req-1");
    assert_eq!(res.content, "Synthesized research analysis findings.");
    assert!(res.usage.is_some());
    assert!(broker.total_tokens_used() > 0);

    // Disallowed provider
    let bad_provider_req = ProviderRequest {
        request_id: "req-2".into(),
        provider: "unauthorized_evil_provider".into(),
        model: "gpt-4o".into(),
        prompt: "x".into(),
        temperature: None,
        max_tokens: None,
    };
    assert!(broker.handle_request(&bad_provider_req).await.is_err());

    // Disallowed model
    let bad_model_req = ProviderRequest {
        request_id: "req-3".into(),
        provider: "openai".into(),
        model: "unapproved-model-xyz".into(),
        prompt: "x".into(),
        temperature: None,
        max_tokens: None,
    };
    assert!(broker.handle_request(&bad_model_req).await.is_err());
}

#[tokio::test]
async fn test_tool_broker_scope_injection_and_policy() {
    let mut broker = ToolBroker::new();

    struct EchoScopeHandler;
    #[async_trait]
    impl ToolHandler for EchoScopeHandler {
        async fn execute(
            &self,
            scope: &RunScope,
            _params: &serde_json::Value,
        ) -> Result<serde_json::Value, String> {
            Ok(json!({
                "boundWorkspace": scope.workspace_id,
                "boundTask": scope.task_id
            }))
        }
    }

    broker.register_tool(
        "test.echo_scope",
        "Echoes bound scope",
        true,
        Arc::new(EchoScopeHandler),
    );

    let scope = RunScope {
        workspace_id: "ws-alpha".into(),
        task_id: "task-beta".into(),
        session_id: None,
    };

    // Safe execution with authoritative scope injection
    let tool_req = ToolRequest {
        request_id: "t-1".into(),
        tool_name: "test.echo_scope".into(),
        parameters: json!({}),
    };

    let res = broker.handle_request(&scope, &tool_req).await.unwrap();
    assert_eq!(res.request_id, "t-1");
    assert_eq!(res.result["boundWorkspace"], "ws-alpha");
    assert_eq!(res.result["boundTask"], "task-beta");

    // Rejected dangerous tools (shell / sql / trade)
    let dangerous_tools = [
        "system.shell_exec",
        "database.sql_query",
        "trading.place_order",
        "system.read_credentials",
    ];

    for dangerous in dangerous_tools {
        let req = ToolRequest {
            request_id: "t-bad".into(),
            tool_name: dangerous.into(),
            parameters: json!({}),
        };
        let bad_res = broker.handle_request(&scope, &req).await.unwrap();
        assert!(bad_res.result.get("error").is_some());
        assert_eq!(bad_res.result["success"], false);
    }
}

#[tokio::test]
async fn test_tool_broker_timeout() {
    let mut broker = ToolBroker::with_timeout(std::time::Duration::from_millis(50));

    struct HangingToolHandler;
    #[async_trait]
    impl ToolHandler for HangingToolHandler {
        async fn execute(
            &self,
            _scope: &RunScope,
            _params: &serde_json::Value,
        ) -> Result<serde_json::Value, String> {
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            Ok(json!({ "status": "completed" }))
        }
    }

    broker.register_tool(
        "test.hanging_tool",
        "Hangs for 200ms",
        true,
        Arc::new(HangingToolHandler),
    );

    let scope = RunScope {
        workspace_id: "ws-1".into(),
        task_id: "t-1".into(),
        session_id: None,
    };

    let req = ToolRequest {
        request_id: "req-hang".into(),
        tool_name: "test.hanging_tool".into(),
        parameters: json!({}),
    };

    let res = broker.handle_request(&scope, &req).await.unwrap();
    assert_eq!(res.request_id, "req-hang");
    assert!(res.result.get("error").is_some());
    assert!(res.result["error"].as_str().unwrap().contains("timed out"));
    assert_eq!(res.result["success"], false);
}
