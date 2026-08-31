// Tests for AgentOrchestrator integration.

#[cfg(test)]
mod tests {
    use agent_core::broker::{ProviderBroker, ToolBroker};
    use agent_core::manager::SupervisorManager;
    use agent_core::manifest::{WorkerManifest, WorkerRegistry};
    use std::path::PathBuf;

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
    async fn test_orchestrator_components_initialization() {
        let mut registry = WorkerRegistry::new();
        let mut manifest = WorkerManifest::fixture_worker();
        manifest.id = "orch-test-fixture".into();
        registry.register(manifest);
        registry.add_search_path(get_worker_exe_path().parent().unwrap().to_path_buf());

        let supervisor_manager = std::sync::Arc::new(SupervisorManager::new(registry, 4));
        let provider_broker = std::sync::Arc::new(ProviderBroker::new());
        let _tool_broker = std::sync::Arc::new(ToolBroker::new());

        assert_eq!(supervisor_manager.active_count(), 0);
        assert_eq!(provider_broker.total_tokens_used(), 0);

        // Verify spawning a worker through the manager
        let sup = supervisor_manager
            .spawn_worker("orch-run-1", "orch-test-fixture")
            .await;
        assert!(sup.is_ok());
        assert_eq!(supervisor_manager.active_count(), 1);

        supervisor_manager.shutdown_all().await;
        assert_eq!(supervisor_manager.active_count(), 0);
    }
}
