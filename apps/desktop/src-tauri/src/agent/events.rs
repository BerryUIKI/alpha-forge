// Agent event streaming via Tauri events.
//
// Emits task progress events to the frontend for real-time updates.

use tauri::{AppHandle, Emitter};

use domain::task::{AgentTaskEvent, TaskEventType};

/// Event names for task events
pub const EVENT_TASK_PROGRESS: &str = "task:progress";
pub const EVENT_TASK_COMPLETED: &str = "task:completed";
pub const EVENT_TASK_FAILED: &str = "task:failed";
pub const EVENT_TASK_CANCELLED: &str = "task:cancelled";

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

    // Emit event to frontend
    if app.emit(event_name, event).is_err() {
        tracing::error!("task event emission failed");
    }
}

/// Emits a progress update for a task.
pub fn emit_progress(app: &AppHandle, task_id: &str, message: &str) {
    let event = AgentTaskEvent {
        id: uuid::Uuid::new_v4().to_string(),
        task_id: task_id.to_string(),
        event_type: TaskEventType::TaskProgress,
        payload: Some(message.to_string()),
        created_at: chrono::Utc::now(),
    };

    emit_task_event(app, &event);
}

/// Emits a completion event for a task.
pub fn emit_completion(app: &AppHandle, task_id: &str, output: Option<&str>) {
    let event = AgentTaskEvent {
        id: uuid::Uuid::new_v4().to_string(),
        task_id: task_id.to_string(),
        event_type: TaskEventType::TaskCompleted,
        payload: output.map(|s| s.to_string()),
        created_at: chrono::Utc::now(),
    };

    emit_task_event(app, &event);
}

/// Emits a failure event for a task.
pub fn emit_failure(app: &AppHandle, task_id: &str, error: &str) {
    let event = AgentTaskEvent {
        id: uuid::Uuid::new_v4().to_string(),
        task_id: task_id.to_string(),
        event_type: TaskEventType::TaskFailed,
        payload: Some(error.to_string()),
        created_at: chrono::Utc::now(),
    };

    emit_task_event(app, &event);
}

/// Emits a cancellation event for a task.
pub fn emit_cancellation(app: &AppHandle, task_id: &str) {
    let event = AgentTaskEvent {
        id: uuid::Uuid::new_v4().to_string(),
        task_id: task_id.to_string(),
        event_type: TaskEventType::TaskCancelled,
        payload: None,
        created_at: chrono::Utc::now(),
    };

    emit_task_event(app, &event);
}
