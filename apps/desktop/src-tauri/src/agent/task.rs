// Agent task types — Phase 1 placeholder.
// Full lifecycle (queued → running → completed) in later phases.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Queued,
    Running,
    WaitingForInput,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    pub id: String,
    pub status: TaskStatus,
    pub input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskEvent {
    Started { task_id: String },
    Progress { task_id: String, message: String },
    Completed { task_id: String },
    Failed { task_id: String, error: String },
    Cancelled { task_id: String },
}
