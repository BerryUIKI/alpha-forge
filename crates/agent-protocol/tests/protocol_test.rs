use agent_protocol::codec::{SyncFrameReader, SyncFrameWriter};
use agent_protocol::error::ProtocolError;
use agent_protocol::fixture::ProtocolFixture;
use agent_protocol::validator::{HandshakeState, SessionValidator};
use agent_protocol::ProtocolEnvelope;
use std::io::Cursor;

#[test]
fn test_valid_handshake_and_session_validation() {
    let run_id = "run-test-001";
    let mut validator = SessionValidator::new(run_id);

    let (hello, configure, ready, start) = ProtocolFixture::handshake_sequence(run_id);

    // 1. Worker sends hello
    let raw_hello = hello.into_raw().unwrap();
    assert!(validator.validate_incoming(&raw_hello).is_ok());
    assert_eq!(validator.handshake_state(), HandshakeState::ReceivedHello);

    // 2. Host sends configure
    let raw_configure = configure.into_raw().unwrap();
    assert!(validator.record_outgoing(&raw_configure).is_ok());
    assert_eq!(validator.handshake_state(), HandshakeState::SentConfigure);

    // 3. Worker sends ready
    let raw_ready = ready.into_raw().unwrap();
    assert!(validator.validate_incoming(&raw_ready).is_ok());
    assert_eq!(validator.handshake_state(), HandshakeState::ReceivedReady);

    // 4. Host sends start
    let raw_start = start.into_raw().unwrap();
    assert!(validator.record_outgoing(&raw_start).is_ok());
    assert_eq!(validator.handshake_state(), HandshakeState::SentStart);
}

#[test]
fn test_end_to_end_frame_codec_roundtrip() {
    let run_id = "run-codec-001";
    let frames = ProtocolFixture::successful_run_trace(run_id);

    let mut buffer = Vec::new();
    {
        let mut writer = SyncFrameWriter::new(&mut buffer);
        for frame in &frames {
            let raw = frame.clone().into_raw().unwrap();
            writer.write_frame(&raw).unwrap();
        }
    }

    assert!(!buffer.is_empty());

    // Read back all frames
    let mut reader = SyncFrameReader::new(Cursor::new(buffer));
    let mut read_frames = Vec::new();

    while let Some(raw) = reader.read_frame().unwrap() {
        let typed = raw.to_typed().unwrap();
        read_frames.push(typed);
    }

    assert_eq!(read_frames.len(), frames.len());
    for (original, decoded) in frames.iter().zip(read_frames.iter()) {
        assert_eq!(original.protocol_version, decoded.protocol_version);
        assert_eq!(original.run_id, decoded.run_id);
        assert_eq!(original.message_type, decoded.message_type);
        assert_eq!(original.payload, decoded.payload);
    }
}

#[test]
fn test_version_mismatch_rejection() {
    let mut envelope = ProtocolEnvelope::new(
        "run-v-mismatch",
        "worker.hello",
        serde_json::json!({
            "workerId": "test",
            "workerVersion": "0.1.0",
            "protocolVersions": [999],
            "supportedFeatures": []
        }),
    );
    envelope.protocol_version = 999;

    let res = envelope.validate_header(Some("run-v-mismatch"));
    assert!(matches!(
        res,
        Err(ProtocolError::VersionMismatch {
            expected: 1,
            actual: 999
        })
    ));
}

#[test]
fn test_run_id_mismatch_rejection() {
    let envelope = ProtocolEnvelope::new(
        "run-a",
        "worker.hello",
        serde_json::json!({ "workerId": "w" }),
    );

    let res = envelope.validate_header(Some("run-b"));
    assert!(matches!(res, Err(ProtocolError::RunIdMismatch { .. })));
}

#[test]
fn test_duplicate_message_id_rejection() {
    let run_id = "run-dup-id";
    let mut validator = SessionValidator::new(run_id);

    let hello = ProtocolEnvelope::new(
        run_id,
        "worker.hello",
        serde_json::json!({
            "workerId": "w",
            "workerVersion": "0.1.0",
            "protocolVersions": [1],
            "supportedFeatures": []
        }),
    );

    assert!(validator.validate_incoming(&hello).is_ok());

    // Send same envelope again with duplicate messageId
    let dup_res = validator.validate_incoming(&hello);
    assert!(matches!(dup_res, Err(ProtocolError::DuplicateMessageId(_))));
}

#[test]
fn test_oversized_frame_rejection() {
    let huge_payload = "x".repeat(2000);
    let envelope = ProtocolEnvelope::new(
        "run-oversized",
        "run.progress",
        serde_json::json!({ "message": huge_payload }),
    );

    let serialized = serde_json::to_string(&envelope).unwrap() + "\n";
    let mut reader = SyncFrameReader::with_limits(Cursor::new(serialized), 500, 10_000);

    let res = reader.read_frame();
    assert!(matches!(res, Err(ProtocolError::OversizedFrame { .. })));
}

#[test]
fn test_aggregate_output_limit_exceeded() {
    let envelope = ProtocolEnvelope::new(
        "run-agg-limit",
        "run.progress",
        serde_json::json!({ "message": "hello world" }),
    );

    let serialized = serde_json::to_string(&envelope).unwrap() + "\n";
    let frame_len = serialized.len();
    let repeated = serialized.repeat(5);

    // Limit aggregate to 1.5 * frame_len so 1 frame passes and second exceeds limit
    let mut reader = SyncFrameReader::with_limits(Cursor::new(repeated), 1024, frame_len + 10);

    let first = reader.read_frame();
    assert!(first.is_ok());

    let second = reader.read_frame();
    assert!(matches!(
        second,
        Err(ProtocolError::AggregateOutputLimitExceeded { .. })
    ));
}

#[test]
fn test_nonce_mismatch_rejection() {
    let run_id = "run-nonce-test";
    let mut validator = SessionValidator::new(run_id);
    validator.set_expected_nonce("secret-nonce-123");

    let hello = ProtocolEnvelope::new(
        run_id,
        "worker.hello",
        serde_json::json!({
            "workerId": "w",
            "workerVersion": "0.1.0",
            "protocolVersions": [1],
            "supportedFeatures": []
        }),
    );
    validator.validate_incoming(&hello).unwrap();

    let configure = ProtocolEnvelope::new(
        run_id,
        "host.configure",
        serde_json::json!({
            "selectedVersion": 1,
            "runScope": { "workspaceId": "ws", "taskId": "task" },
            "limits": {},
            "capabilities": [],
            "nonce": "secret-nonce-123"
        }),
    );
    validator.record_outgoing(&configure).unwrap();

    // Worker provides wrong nonce
    let bad_ready = ProtocolEnvelope::new(
        run_id,
        "worker.ready",
        serde_json::json!({
            "nonceProof": "wrong-nonce-456",
            "supportedFeatures": []
        }),
    );

    let res = validator.validate_incoming(&bad_ready);
    assert!(matches!(res, Err(ProtocolError::NonceMismatch)));
}

#[test]
fn test_uncorrelated_broker_response_rejection() {
    let run_id = "run-uncorrelated";
    let mut validator = SessionValidator::new(run_id);

    // Attempt to record tool response without prior tool request
    let unasked_resp = ProtocolEnvelope::new(
        run_id,
        "tool.response",
        serde_json::json!({
            "requestId": "unknown-req-999",
            "result": { "data": 42 }
        }),
    );

    let res = validator.record_outgoing(&unasked_resp);
    assert!(matches!(res, Err(ProtocolError::InvalidReplyTo { .. })));
}

#[test]
fn test_malformed_json_frame_rejection() {
    let malformed = "{\"protocolVersion\": 1, \"runId\": \"incomplete-json...\n";
    let mut reader = SyncFrameReader::new(Cursor::new(malformed));

    let res = reader.read_frame();
    assert!(matches!(res, Err(ProtocolError::InvalidJson(_))));
}

#[test]
fn test_multibyte_utf8_sliced_across_chunks() {
    // Chinese characters and emoji: "你好世界 🚀 AlphaForge"
    let json_frame = "{\"protocolVersion\":1,\"runId\":\"run-utf8\",\"messageId\":\"msg-1\",\"type\":\"run.progress\",\"payload\":{\"message\":\"你好世界 🚀 AlphaForge\"}}\n";
    let mut reader = SyncFrameReader::new(Cursor::new(json_frame));

    let frame = reader.read_frame().unwrap();
    assert!(frame.is_some());
    let raw = frame.unwrap();
    assert_eq!(raw.run_id, "run-utf8");
    assert_eq!(raw.message_type, "run.progress");
}

#[test]
fn test_broker_response_type_mismatch_rejection() {
    let run_id = "run-type-mismatch";
    let mut validator = SessionValidator::new(run_id);

    // Complete handshake
    let hello = ProtocolEnvelope::new(
        run_id,
        "worker.hello",
        serde_json::json!({ "workerId": "w", "workerVersion": "1", "protocolVersions": [1], "supportedFeatures": [] }),
    );
    validator.validate_incoming(&hello).unwrap();
    let config = ProtocolEnvelope::new(
        run_id,
        "host.configure",
        serde_json::json!({ "workerId": "w", "runId": run_id, "workspaceId": "ws", "timeoutMs": 1000 }),
    );
    validator.record_outgoing(&config).unwrap();
    let ready = ProtocolEnvelope::new(
        run_id,
        "worker.ready",
        serde_json::json!({ "supportedFeatures": [] }),
    );
    validator.validate_incoming(&ready).unwrap();
    let start = ProtocolEnvelope::new(
        run_id,
        "host.start",
        serde_json::json!({ "runId": run_id, "prompt": "p" }),
    );
    validator.record_outgoing(&start).unwrap();

    // Worker asks for tool.request
    let tool_req = ProtocolEnvelope::new(
        run_id,
        "tool.request",
        serde_json::json!({
            "requestId": "req-tool-1",
            "toolName": "research.search",
            "parameters": {}
        }),
    );
    validator.validate_incoming(&tool_req).unwrap();

    // Host mistakenly responds with provider.response
    let mismatched_resp = ProtocolEnvelope::new(
        run_id,
        "provider.response",
        serde_json::json!({
            "requestId": "req-tool-1",
            "content": "some text"
        }),
    );
    let res = validator.record_outgoing(&mismatched_resp);
    assert!(matches!(res, Err(ProtocolError::InvalidReplyTo { .. })));
}
