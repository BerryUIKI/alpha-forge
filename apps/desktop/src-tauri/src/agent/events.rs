// Agent event publishing.
//
// The runtime and service persist task events; this module also forwards them
// to the frontend over a narrow Tauri event channel.

use tauri::{AppHandle, Emitter};

use crate::error::AppError;
use domain::task::AgentTaskEvent;

/// Tauri event emitted whenever an agent task event is persisted.
pub const AGENT_TASK_EVENT: &str = "agent-task-event";

/// Abstraction over event delivery so the service can be tested without Tauri.
pub trait EventPublisher: Send + Sync {
    fn publish(&self, event: &AgentTaskEvent) -> Result<(), AppError>;
}

/// Publishes task events to all app windows through Tauri.
pub struct TauriEventPublisher {
    app: AppHandle,
}

impl TauriEventPublisher {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }
}

impl EventPublisher for TauriEventPublisher {
    fn publish(&self, event: &AgentTaskEvent) -> Result<(), AppError> {
        self.app
            .emit(AGENT_TASK_EVENT, event.clone())
            .map_err(|e| AppError::Internal(format!("failed to emit agent task event: {e}")))
    }
}
