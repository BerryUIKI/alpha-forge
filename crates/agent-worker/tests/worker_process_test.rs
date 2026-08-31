use agent_protocol::codec::{SyncFrameReader, SyncFrameWriter};
use agent_protocol::envelope::TypedEnvelope;
use agent_protocol::fixture::ProtocolFixture;
use agent_protocol::messages::*;
use std::io::BufReader;
use std::process::{Command, Stdio};

#[test]
fn test_subprocess_worker_success_lifecycle() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_alphaforge-agent-worker"))
        .arg("--fixture-mode")
        .arg("success")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn alphaforge-agent-worker");

    let child_stdin = child.stdin.take().expect("Failed to open stdin");
    let child_stdout = child.stdout.take().expect("Failed to open stdout");

    let mut writer = SyncFrameWriter::new(child_stdin);
    let mut reader = SyncFrameReader::new(BufReader::new(child_stdout));

    // 1. Read worker.hello
    let raw_hello = reader.read_frame().unwrap().expect("Expected hello frame");
    let typed_hello = raw_hello.to_typed().unwrap();
    assert_eq!(typed_hello.message_type, "worker.hello");

    let run_id = "test-run-spawn-001";
    let nonce = "test-nonce-proof-xyz";

    // 2. Write host.configure
    let config = TypedEnvelope::from_payload(
        run_id,
        MessagePayload::HostConfigure(HostConfigure {
            selected_version: 1,
            run_scope: RunScope {
                workspace_id: "ws-01".into(),
                task_id: "task-01".into(),
                session_id: None,
            },
            limits: RunLimits::default(),
            capabilities: vec!["research.read".into()],
            nonce: nonce.into(),
        }),
    );
    writer.write_frame(&config.into_raw().unwrap()).unwrap();

    // 3. Read worker.ready
    let raw_ready = reader.read_frame().unwrap().expect("Expected ready frame");
    let typed_ready = raw_ready.to_typed().unwrap();
    if let MessagePayload::WorkerReady(ready) = typed_ready.payload {
        assert_eq!(ready.nonce_proof, nonce);
    } else {
        panic!("Expected WorkerReady");
    }

    // 4. Write host.start
    let start = TypedEnvelope::from_payload(
        run_id,
        MessagePayload::HostStart(HostStart {
            task_input: serde_json::json!({ "query": "Test query" }),
            output_schema_id: ProtocolFixture::FIXTURE_SCHEMA_ID.into(),
            output_schema_version: 1,
        }),
    );
    writer.write_frame(&start.into_raw().unwrap()).unwrap();

    // 5. Read run.progress
    let raw_prog = reader.read_frame().unwrap().expect("Expected progress");
    let typed_prog = raw_prog.to_typed().unwrap();
    assert_eq!(typed_prog.message_type, "run.progress");

    // 6. Read run.result
    let raw_res = reader.read_frame().unwrap().expect("Expected result");
    let typed_res = raw_res.to_typed().unwrap();
    assert_eq!(typed_res.message_type, "run.result");

    let status = child.wait().expect("Failed to wait on child");
    assert!(status.success());
}

#[test]
fn test_subprocess_worker_failure_lifecycle() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_alphaforge-agent-worker"))
        .arg("--fixture-mode")
        .arg("fail")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn alphaforge-agent-worker");

    let child_stdin = child.stdin.take().expect("Failed to open stdin");
    let child_stdout = child.stdout.take().expect("Failed to open stdout");

    let mut writer = SyncFrameWriter::new(child_stdin);
    let mut reader = SyncFrameReader::new(BufReader::new(child_stdout));

    // 1. Read worker.hello
    let raw_hello = reader.read_frame().unwrap().expect("Expected hello");
    assert_eq!(raw_hello.message_type, "worker.hello");

    let run_id = "test-run-fail-001";
    let nonce = "nonce-fail";

    // 2. Write host.configure
    let config = TypedEnvelope::from_payload(
        run_id,
        MessagePayload::HostConfigure(HostConfigure {
            selected_version: 1,
            run_scope: RunScope {
                workspace_id: "ws-01".into(),
                task_id: "task-01".into(),
                session_id: None,
            },
            limits: RunLimits::default(),
            capabilities: vec![],
            nonce: nonce.into(),
        }),
    );
    writer.write_frame(&config.into_raw().unwrap()).unwrap();

    // 3. Read worker.ready
    let raw_ready = reader.read_frame().unwrap().expect("Expected ready");
    assert_eq!(raw_ready.message_type, "worker.ready");

    // 4. Write host.start
    let start = TypedEnvelope::from_payload(
        run_id,
        MessagePayload::HostStart(HostStart {
            task_input: serde_json::json!({}),
            output_schema_id: "schema".into(),
            output_schema_version: 1,
        }),
    );
    writer.write_frame(&start.into_raw().unwrap()).unwrap();

    // 5. Read run.failure
    let raw_fail = reader
        .read_frame()
        .unwrap()
        .expect("Expected failure frame");
    let typed_fail = raw_fail.to_typed().unwrap();
    if let MessagePayload::RunFailure(f) = typed_fail.payload {
        assert_eq!(f.code, "AGENT_FIXTURE_FAILED");
    } else {
        panic!("Expected RunFailure payload");
    }

    let status = child.wait().expect("Failed to wait on child");
    assert_eq!(status.code(), Some(1));
}

#[test]
fn test_subprocess_worker_cancel_lifecycle() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_alphaforge-agent-worker"))
        .arg("--fixture-mode")
        .arg("cancel")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn alphaforge-agent-worker");

    let child_stdin = child.stdin.take().expect("Failed to open stdin");
    let child_stdout = child.stdout.take().expect("Failed to open stdout");

    let mut writer = SyncFrameWriter::new(child_stdin);
    let mut reader = SyncFrameReader::new(BufReader::new(child_stdout));

    // Hello -> Configure -> Ready -> Start
    let _hello = reader.read_frame().unwrap().expect("Expected hello");

    let run_id = "test-run-cancel-001";
    let nonce = "nonce-cancel";

    let config = TypedEnvelope::from_payload(
        run_id,
        MessagePayload::HostConfigure(HostConfigure {
            selected_version: 1,
            run_scope: RunScope {
                workspace_id: "ws-01".into(),
                task_id: "task-01".into(),
                session_id: None,
            },
            limits: RunLimits::default(),
            capabilities: vec![],
            nonce: nonce.into(),
        }),
    );
    writer.write_frame(&config.into_raw().unwrap()).unwrap();

    let _ready = reader.read_frame().unwrap().expect("Expected ready");

    let start = TypedEnvelope::from_payload(
        run_id,
        MessagePayload::HostStart(HostStart {
            task_input: serde_json::json!({}),
            output_schema_id: "schema".into(),
            output_schema_version: 1,
        }),
    );
    writer.write_frame(&start.into_raw().unwrap()).unwrap();

    let prog = reader.read_frame().unwrap().expect("Expected progress");
    assert_eq!(prog.message_type, "run.progress");

    // Host sends run.cancel
    let cancel = TypedEnvelope::from_payload(
        run_id,
        MessagePayload::RunCancel(RunCancel {
            reason: "User requested task cancellation".into(),
        }),
    );
    writer.write_frame(&cancel.into_raw().unwrap()).unwrap();

    let status = child.wait().expect("Failed to wait on child");
    assert!(status.success());
}
