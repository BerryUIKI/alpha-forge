use agent_core::launch::LaunchSpec;
use agent_core::manager::SupervisorManager;
use agent_core::manifest::{WorkerManifest, WorkerRegistry};
use agent_core::supervisor::WorkerSupervisor;
use agent_protocol::fixture::ProtocolFixture;
use agent_protocol::messages::*;
use std::path::PathBuf;
use tempfile::tempdir;

fn get_worker_exe_path() -> PathBuf {
    if let Ok(bin) = std::env::var("CARGO_BIN_EXE_alphaforge-agent-worker") {
        return PathBuf::from(bin);
    }
    let mut current = std::env::current_exe().unwrap();
    while let Some(parent) = current.parent() {
        let candidate = parent.join("alphaforge-agent-worker");
        if candidate.is_file() {
            return candidate;
        }
        #[cfg(windows)]
        {
            let candidate_win = parent.join("alphaforge-agent-worker.exe");
            if candidate_win.is_file() {
                return candidate_win;
            }
        }
        current = parent.to_path_buf();
    }
    // Fallback search target/debug
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let target_debug = manifest_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("target")
        .join("debug")
        .join("alphaforge-agent-worker");
    if target_debug.is_file() {
        return target_debug;
    }
    panic!("Could not locate alphaforge-agent-worker binary");
}

#[tokio::test]
async fn test_worker_supervisor_full_lifecycle() {
    let run_id = "test-sup-run-001";
    let manifest = WorkerManifest::fixture_worker();
    let temp_dir = tempdir().unwrap();

    let mut spec = LaunchSpec::new(get_worker_exe_path(), temp_dir.path().to_path_buf());
    spec = spec
        .with_arg("--fixture-mode")
        .with_arg("success")
        .with_startup_timeout_ms(5000);

    let mut supervisor = WorkerSupervisor::new(run_id, manifest, spec, Some(temp_dir));

    supervisor.spawn().await.expect("Failed to spawn worker");

    let run_scope = RunScope {
        workspace_id: "ws-test".into(),
        task_id: "task-test".into(),
        session_id: None,
    };

    supervisor
        .perform_handshake(
            run_scope,
            vec!["research.read".into()],
            serde_json::json!({ "query": "Test research query" }),
            ProtocolFixture::FIXTURE_SCHEMA_ID,
        )
        .await
        .expect("Handshake failed");

    // Read progress frame
    let prog_frame = supervisor
        .read_typed_frame()
        .await
        .unwrap()
        .expect("Expected progress frame");
    assert_eq!(prog_frame.message_type, "run.progress");

    // Read result frame
    let result_frame = supervisor
        .read_typed_frame()
        .await
        .unwrap()
        .expect("Expected result frame");
    assert_eq!(result_frame.message_type, "run.result");

    let diag = supervisor.collect_diagnostics(true, Some("0".into()));
    assert_eq!(diag.run_id, run_id);
    assert!(diag.frames_received >= 3);
    assert!(diag.frames_sent >= 2);
}

#[tokio::test]
async fn test_worker_supervisor_cancellation() {
    let run_id = "test-sup-cancel-001";
    let manifest = WorkerManifest::fixture_worker();
    let temp_dir = tempdir().unwrap();

    let mut spec = LaunchSpec::new(get_worker_exe_path(), temp_dir.path().to_path_buf());
    spec = spec
        .with_arg("--fixture-mode")
        .with_arg("cancel")
        .with_startup_timeout_ms(5000);

    let mut supervisor = WorkerSupervisor::new(run_id, manifest, spec, Some(temp_dir));
    supervisor.spawn().await.unwrap();

    let run_scope = RunScope {
        workspace_id: "ws-test".into(),
        task_id: "task-test".into(),
        session_id: None,
    };

    supervisor
        .perform_handshake(run_scope, vec![], serde_json::json!({}), "schema")
        .await
        .unwrap();

    let _prog = supervisor.read_typed_frame().await.unwrap().unwrap();

    // Cancel gracefully
    supervisor.cancel_gracefully(2000).await.unwrap();
}

#[tokio::test]
async fn test_supervisor_manager_concurrency_limit() {
    let mut registry = WorkerRegistry::new();
    let mut fixture_manifest = WorkerManifest::fixture_worker();
    fixture_manifest.id = "test-fixture".into();
    registry.register(fixture_manifest);
    registry.add_search_path(get_worker_exe_path().parent().unwrap().to_path_buf());

    // Manager with max_concurrent = 2
    let manager = SupervisorManager::new(registry, 2);

    let sup1 = manager.spawn_worker("run-1", "test-fixture").await;
    assert!(sup1.is_ok());

    let sup2 = manager.spawn_worker("run-2", "test-fixture").await;
    assert!(sup2.is_ok());

    // Third should exceed concurrency limit
    let sup3 = manager.spawn_worker("run-3", "test-fixture").await;
    assert!(sup3.is_err());

    // Clean up
    manager.shutdown_all().await;
    assert_eq!(manager.active_count(), 0);
}
