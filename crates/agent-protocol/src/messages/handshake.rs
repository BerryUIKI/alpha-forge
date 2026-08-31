use serde::{Deserialize, Serialize};

/// Task and workspace boundaries authorized by the Rust host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunScope {
    pub workspace_id: String,
    pub task_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// Budget and resource constraints enforced per run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunLimits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_turns: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cost_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_frame_bytes: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_aggregate_bytes: Option<usize>,
}

impl Default for RunLimits {
    fn default() -> Self {
        Self {
            max_turns: Some(50),
            max_tokens: Some(500_000),
            max_cost_usd: Some(5.0),
            max_duration_seconds: Some(600),
            max_frame_bytes: Some(1024 * 1024),
            max_aggregate_bytes: Some(16 * 1024 * 1024),
        }
    }
}

/// Initial hello message sent by worker upon spawning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerHello {
    pub worker_id: String,
    pub worker_version: String,
    pub protocol_versions: Vec<u32>,
    pub supported_features: Vec<String>,
}

/// Configuration sent by host to worker during handshake.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostConfigure {
    pub selected_version: u32,
    pub run_scope: RunScope,
    pub limits: RunLimits,
    pub capabilities: Vec<String>,
    pub nonce: String,
}

/// Ready message sent by worker once configuration is applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerReady {
    pub nonce_proof: String,
    pub supported_features: Vec<String>,
}

/// Start execution command sent by host with task input.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostStart {
    pub task_input: serde_json::Value,
    pub output_schema_id: String,
    pub output_schema_version: u32,
}
