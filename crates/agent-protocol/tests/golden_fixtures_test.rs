use agent_protocol::fixture::ProtocolFixture;
use agent_protocol::messages::*;
use agent_protocol::RawEnvelope;

#[test]
fn test_golden_worker_hello_schema() {
    let raw_json = r#"{
        "protocolVersion": 1,
        "runId": "550e8400-e29b-41d4-a716-446655440000",
        "messageId": "123e4567-e89b-12d3-a456-426614174000",
        "type": "worker.hello",
        "payload": {
            "workerId": "alphaforge-worker-native",
            "workerVersion": "0.1.0",
            "protocolVersions": [1],
            "supportedFeatures": ["provider.broker", "tool.broker"]
        }
    }"#;

    let envelope: RawEnvelope = serde_json::from_str(raw_json).unwrap();
    assert_eq!(envelope.protocol_version, 1);
    assert_eq!(envelope.message_type, "worker.hello");

    let typed = envelope.to_typed().unwrap();
    if let MessagePayload::WorkerHello(hello) = typed.payload {
        assert_eq!(hello.worker_id, "alphaforge-worker-native");
        assert_eq!(hello.worker_version, "0.1.0");
        assert_eq!(hello.protocol_versions, vec![1]);
    } else {
        panic!("Expected WorkerHello payload");
    }
}

#[test]
fn test_golden_provider_request_and_response_schema() {
    let raw_req_json = r#"{
        "protocolVersion": 1,
        "runId": "550e8400-e29b-41d4-a716-446655440000",
        "messageId": "req-msg-001",
        "type": "provider.request",
        "payload": {
            "requestId": "prov-req-123",
            "provider": "openai",
            "model": "gpt-4o",
            "prompt": "Synthesize market analysis",
            "maxTokens": 1000,
            "temperature": 0.3
        }
    }"#;

    let req_env: RawEnvelope = serde_json::from_str(raw_req_json).unwrap();
    let typed_req = req_env.to_typed().unwrap();
    if let MessagePayload::ProviderRequest(req) = typed_req.payload {
        assert_eq!(req.request_id, "prov-req-123");
        assert_eq!(req.provider, "openai");
        assert_eq!(req.model, "gpt-4o");
        assert_eq!(req.max_tokens, Some(1000));
    } else {
        panic!("Expected ProviderRequest payload");
    }

    let raw_resp_json = r#"{
        "protocolVersion": 1,
        "runId": "550e8400-e29b-41d4-a716-446655440000",
        "messageId": "resp-msg-001",
        "replyTo": "req-msg-001",
        "type": "provider.response",
        "payload": {
            "requestId": "prov-req-123",
            "content": "Analysis summary",
            "usage": {
                "promptTokens": 120,
                "completionTokens": 50,
                "totalTokens": 170,
                "estimatedCostUsd": 0.0015
            }
        }
    }"#;

    let resp_env: RawEnvelope = serde_json::from_str(raw_resp_json).unwrap();
    let typed_resp = resp_env.to_typed().unwrap();
    if let MessagePayload::ProviderResponse(resp) = typed_resp.payload {
        assert_eq!(resp.request_id, "prov-req-123");
        assert_eq!(resp.content, "Analysis summary");
        let usage = resp.usage.unwrap();
        assert_eq!(usage.total_tokens, Some(170));
        assert_eq!(usage.estimated_cost_usd, Some(0.0015));
    } else {
        panic!("Expected ProviderResponse payload");
    }
}

#[test]
fn test_golden_research_result_schema() {
    let result_json = ProtocolFixture::valid_research_result();
    assert!(result_json.get("summary").is_some());
    assert!(result_json.get("claims").is_some());
    assert!(result_json.get("risks").is_some());
    assert!(result_json.get("confidenceScore").is_some());
}
