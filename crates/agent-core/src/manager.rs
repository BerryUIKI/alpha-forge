use crate::error::{SupervisorError, SupervisorResult};
use crate::launch::LaunchSpec;
use crate::manifest::WorkerRegistry;
use crate::supervisor::WorkerSupervisor;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tempfile::tempdir;
use tracing::info;

type SupervisorHandle = Arc<tokio::sync::Mutex<WorkerSupervisor>>;
type SupervisorMap = HashMap<String, Option<SupervisorHandle>>;

/// Manages multiple concurrent worker supervisors, enforcing concurrency limits and clean shutdown.
#[derive(Clone)]
pub struct SupervisorManager {
    registry: Arc<Mutex<WorkerRegistry>>,
    max_concurrent: usize,
    active_supervisors: Arc<Mutex<SupervisorMap>>,
}

impl SupervisorManager {
    pub fn new(registry: WorkerRegistry, max_concurrent: usize) -> Self {
        Self {
            registry: Arc::new(Mutex::new(registry)),
            max_concurrent,
            active_supervisors: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn active_count(&self) -> usize {
        self.active_supervisors.lock().unwrap().len()
    }

    /// Spawns and registers a new supervised worker.
    pub async fn spawn_worker(
        &self,
        run_id: &str,
        manifest_id: &str,
    ) -> SupervisorResult<Arc<tokio::sync::Mutex<WorkerSupervisor>>> {
        let (manifest, exe_path) = {
            let reg = self.registry.lock().unwrap();
            reg.resolve_executable(manifest_id)?
        };

        {
            let mut active = self.active_supervisors.lock().unwrap();
            if active.len() >= self.max_concurrent {
                return Err(SupervisorError::ConcurrencyLimitReached {
                    max_concurrent: self.max_concurrent,
                });
            }
            // Atomically reserve concurrency slot to prevent TOCTOU races
            active.insert(run_id.to_string(), None);
        }

        // Create private task directory
        let temp_dir = match tempdir().map_err(SupervisorError::SpawnFailed) {
            Ok(dir) => dir,
            Err(e) => {
                let mut active = self.active_supervisors.lock().unwrap();
                active.remove(run_id);
                return Err(e);
            }
        };
        let working_dir = temp_dir.path().to_path_buf();

        let spec = LaunchSpec::new(exe_path, working_dir);
        let mut supervisor = WorkerSupervisor::new(run_id, manifest, spec, Some(temp_dir));

        if let Err(e) = supervisor.spawn().await {
            let mut active = self.active_supervisors.lock().unwrap();
            active.remove(run_id);
            return Err(e);
        }

        let supervisor_arc = Arc::new(tokio::sync::Mutex::new(supervisor));
        {
            let mut active = self.active_supervisors.lock().unwrap();
            active.insert(run_id.to_string(), Some(Arc::clone(&supervisor_arc)));
        }

        Ok(supervisor_arc)
    }

    /// Retrieves an active supervisor handle.
    pub fn get_supervisor(
        &self,
        run_id: &str,
    ) -> Option<Arc<tokio::sync::Mutex<WorkerSupervisor>>> {
        let active = self.active_supervisors.lock().unwrap();
        active.get(run_id).and_then(|opt| opt.clone())
    }

    /// Unregisters a worker supervisor after completion.
    pub fn unregister_worker(&self, run_id: &str) {
        let mut active = self.active_supervisors.lock().unwrap();
        active.remove(run_id);
    }

    /// Cancels a running worker and unregisters it.
    pub async fn cancel_worker(&self, run_id: &str, grace_period_ms: u64) -> SupervisorResult<()> {
        let supervisor = {
            let mut active = self.active_supervisors.lock().unwrap();
            active.remove(run_id).flatten()
        };

        if let Some(sup) = supervisor {
            let mut guard = sup.lock().await;
            guard.cancel_gracefully(grace_period_ms).await?;
        }

        Ok(())
    }

    /// Shuts down and force kills all active workers during host shutdown.
    pub async fn shutdown_all(&self) {
        info!("Shutting down all active worker supervisors");
        let supervisors: Vec<Arc<tokio::sync::Mutex<WorkerSupervisor>>> = {
            let mut active = self.active_supervisors.lock().unwrap();
            active.drain().filter_map(|(_, v)| v).collect()
        };

        for sup in supervisors {
            let mut guard = sup.lock().await;
            let _ = guard.kill().await;
        }
    }
}
