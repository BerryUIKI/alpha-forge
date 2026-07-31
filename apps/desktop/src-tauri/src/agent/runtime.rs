// Agent runtime — Phase 1 placeholder.
// Will manage agent task lifecycle in later phases.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRuntimeConfig {
    pub max_concurrent_tasks: usize,
    pub task_timeout_secs: u64,
}

impl Default for AgentRuntimeConfig {
    fn default() -> Self {
        Self {
            max_concurrent_tasks: 2,
            task_timeout_secs: 300,
        }
    }
}
