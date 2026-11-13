// Agent event streaming via Tauri events.
//
// Emits task progress events to the frontend for real-time updates.
//
// `TaskEventEmitter` is the boundary between the task executor and the
// presentation surface. The desktop forwards events to Tauri windows; headless
// callers such as the `alphaforge` CLI forward them to a terminal instead. The
// executor itself never depends on a Tauri runtime.

use tauri::{AppHandle, Emitter};

use domain::task::{AgentTaskEvent, TaskEventType};

/// Event names for task events
pub const EVENT_TASK_PROGRESS: &str = "task:progress";
pub const EVENT_TASK_COMPLETED: &str = "task:completed";
pub const EVENT_TASK_FAILED: &str = "task:failed";
pub const EVENT_TASK_CANCELLED: &str = "task:cancelled";

/// Receives task lifecycle events from the executor.
///
/// Implementations must be cheap to clone through an `Arc` and safe to call
/// from a background Tokio task.
pub trait TaskEventEmitter: Send + Sync + 'static {
    fn emit(&self, event: &AgentTaskEvent);
}

impl TaskEventEmitter for AppHandle {
    fn emit(&self, event: &AgentTaskEvent) {
        emit_task_event(self, event);
    }
}

/// Builds a progress event for a task.
pub fn progress_event(task_id: &str, message: &str) -> AgentTaskEvent {
    AgentTaskEvent {
        id: uuid::Uuid::new_v4().to_string(),
        task_id: task_id.to_string(),
        event_type: TaskEventType::TaskProgress,
        payload: Some(message.to_string()),
        created_at: chrono::Utc::now(),
    }
}

/// Builds a completion event for a task.
pub fn completion_event(task_id: &str, output: Option<&str>) -> AgentTaskEvent {
    AgentTaskEvent {
        id: uuid::Uuid::new_v4().to_string(),
        task_id: task_id.to_string(),
        event_type: TaskEventType::TaskCompleted,
        payload: output.map(|s| s.to_string()),
        created_at: chrono::Utc::now(),
    }
}

/// Builds a failure event for a task.
pub fn failure_event(task_id: &str, error: &str) -> AgentTaskEvent {
    AgentTaskEvent {
        id: uuid::Uuid::new_v4().to_string(),
        task_id: task_id.to_string(),
        event_type: TaskEventType::TaskFailed,
        payload: Some(error.to_string()),
        created_at: chrono::Utc::now(),
    }
}

/// Builds a cancellation event for a task.
pub fn cancellation_event(task_id: &str) -> AgentTaskEvent {
    AgentTaskEvent {
        id: uuid::Uuid::new_v4().to_string(),
        task_id: task_id.to_string(),
        event_type: TaskEventType::TaskCancelled,
        payload: None,
        created_at: chrono::Utc::now(),
    }
}

/// Emits a task event to the frontend via Tauri events.
pub fn emit_task_event(app: &AppHandle, event: &AgentTaskEvent) {
    let event_name = match event.event_type {
        TaskEventType::TaskCreated => EVENT_TASK_PROGRESS,
        TaskEventType::TaskQueued => EVENT_TASK_PROGRESS,
        TaskEventType::TaskStarted => EVENT_TASK_PROGRESS,
        TaskEventType::TaskProgress => EVENT_TASK_PROGRESS,
        TaskEventType::TaskWaitingForInput => EVENT_TASK_PROGRESS,
        TaskEventType::TaskCompleted => EVENT_TASK_COMPLETED,
        TaskEventType::TaskFailed => EVENT_TASK_FAILED,
        TaskEventType::TaskCancelled => EVENT_TASK_CANCELLED,
    };

    // Emit event to frontend. Fully qualified because `TaskEventEmitter` also
    // defines an `emit` method on `AppHandle`.
    if Emitter::emit(app, event_name, event).is_err() {
        tracing::error!("task event emission failed");
    }
}

/// Emits a progress update for a task.
pub fn emit_progress(app: &AppHandle, task_id: &str, message: &str) {
    emit_task_event(app, &progress_event(task_id, message));
}

/// Emits a completion event for a task.
pub fn emit_completion(app: &AppHandle, task_id: &str, output: Option<&str>) {
    emit_task_event(app, &completion_event(task_id, output));
}

/// Emits a failure event for a task.
pub fn emit_failure(app: &AppHandle, task_id: &str, error: &str) {
    emit_task_event(app, &failure_event(task_id, error));
}

/// Emits a cancellation event for a task.
pub fn emit_cancellation(app: &AppHandle, task_id: &str) {
    emit_task_event(app, &cancellation_event(task_id));
}
