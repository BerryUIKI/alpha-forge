// Agent task domain models.
//
// AgentTask represents a research task that can be executed by the agent runtime.
// AgentTaskEvent tracks the execution history and progress of a task.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Status of an agent task throughout its lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    /// Task record exists, not yet queued for execution
    #[default]
    Created,
    /// Task is waiting for an available execution slot
    Queued,
    /// Task is actively executing
    Running,
    /// Agent needs clarification or additional information from the user
    WaitingForInput,
    /// Task finished successfully
    Completed,
    /// Task encountered an unrecoverable error
    Failed,
    /// User cancelled the task before completion
    Cancelled,
}

impl TaskStatus {
    /// Stable string used in the database and on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Created => "created",
            TaskStatus::Queued => "queued",
            TaskStatus::Running => "running",
            TaskStatus::WaitingForInput => "waiting_for_input",
            TaskStatus::Completed => "completed",
            TaskStatus::Failed => "failed",
            TaskStatus::Cancelled => "cancelled",
        }
    }

    /// Parses a persisted status; `None` for an unknown value.
    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "created" => Some(TaskStatus::Created),
            "queued" => Some(TaskStatus::Queued),
            "running" => Some(TaskStatus::Running),
            "waiting_for_input" => Some(TaskStatus::WaitingForInput),
            "completed" => Some(TaskStatus::Completed),
            "failed" => Some(TaskStatus::Failed),
            "cancelled" => Some(TaskStatus::Cancelled),
            _ => None,
        }
    }

    /// Check if the task is in a terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled
        )
    }

    /// Check if the task can be cancelled
    pub fn is_cancellable(&self) -> bool {
        !self.is_terminal()
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// An agent task represents a research request to be processed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    /// Unique identifier (UUID)
    pub id: String,
    /// Workspace this task belongs to
    pub workspace_id: String,
    /// Short title describing the task
    pub title: String,
    /// Detailed description of what the task should accomplish
    pub description: Option<String>,
    /// Current status in the task lifecycle
    pub status: TaskStatus,
    /// Structured output produced by the runtime, if any
    pub output: Option<String>,
    /// Error message when the task failed
    pub error: Option<String>,
    /// When the task was created
    pub created_at: DateTime<Utc>,
    /// When the task was last updated
    pub updated_at: DateTime<Utc>,
}

/// Input for creating a new agent task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentTaskInput {
    /// Workspace to create the task in
    pub workspace_id: String,
    /// Task title
    pub title: String,
    /// Task description
    pub description: Option<String>,
}

/// Types of events that can occur during task execution
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskEventType {
    /// Task was created
    TaskCreated,
    /// Task was queued for execution
    TaskQueued,
    /// Task started executing
    TaskStarted,
    /// Task is making progress
    TaskProgress,
    /// Task needs user input
    TaskWaitingForInput,
    /// Task completed successfully
    TaskCompleted,
    /// Task failed with an error
    TaskFailed,
    /// Task was cancelled
    TaskCancelled,
}

impl TaskEventType {
    /// Stable string used in the database.
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskEventType::TaskCreated => "task_created",
            TaskEventType::TaskQueued => "task_queued",
            TaskEventType::TaskStarted => "task_started",
            TaskEventType::TaskProgress => "task_progress",
            TaskEventType::TaskWaitingForInput => "task_waiting_for_input",
            TaskEventType::TaskCompleted => "task_completed",
            TaskEventType::TaskFailed => "task_failed",
            TaskEventType::TaskCancelled => "task_cancelled",
        }
    }

    /// Parses a persisted event type; `None` for an unknown value.
    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "task_created" => Some(TaskEventType::TaskCreated),
            "task_queued" => Some(TaskEventType::TaskQueued),
            "task_started" => Some(TaskEventType::TaskStarted),
            "task_progress" => Some(TaskEventType::TaskProgress),
            "task_waiting_for_input" => Some(TaskEventType::TaskWaitingForInput),
            "task_completed" => Some(TaskEventType::TaskCompleted),
            "task_failed" => Some(TaskEventType::TaskFailed),
            "task_cancelled" => Some(TaskEventType::TaskCancelled),
            _ => None,
        }
    }
}

/// An event that occurred during task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTaskEvent {
    /// Unique identifier (UUID)
    pub id: String,
    /// Task this event belongs to
    pub task_id: String,
    /// Type of event
    pub event_type: TaskEventType,
    /// Optional payload with event details (JSON)
    pub payload: Option<String>,
    /// When the event occurred
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_status_is_terminal() {
        assert!(!TaskStatus::Created.is_terminal());
        assert!(!TaskStatus::Queued.is_terminal());
        assert!(!TaskStatus::Running.is_terminal());
        assert!(!TaskStatus::WaitingForInput.is_terminal());
        assert!(TaskStatus::Completed.is_terminal());
        assert!(TaskStatus::Failed.is_terminal());
        assert!(TaskStatus::Cancelled.is_terminal());
    }

    #[test]
    fn test_task_status_is_cancellable() {
        assert!(TaskStatus::Created.is_cancellable());
        assert!(TaskStatus::Queued.is_cancellable());
        assert!(TaskStatus::Running.is_cancellable());
        assert!(TaskStatus::WaitingForInput.is_cancellable());
        assert!(!TaskStatus::Completed.is_cancellable());
        assert!(!TaskStatus::Failed.is_cancellable());
        assert!(!TaskStatus::Cancelled.is_cancellable());
    }

    #[test]
    fn test_task_status_default() {
        assert_eq!(TaskStatus::default(), TaskStatus::Created);
    }

    #[test]
    fn test_task_status_db_round_trip() {
        for status in [
            TaskStatus::Created,
            TaskStatus::Queued,
            TaskStatus::Running,
            TaskStatus::WaitingForInput,
            TaskStatus::Completed,
            TaskStatus::Failed,
            TaskStatus::Cancelled,
        ] {
            assert_eq!(TaskStatus::from_db(status.as_str()), Some(status.clone()));
        }

        assert_eq!(TaskStatus::from_db("unknown"), None);
    }

    #[test]
    fn test_task_event_type_db_round_trip() {
        for event_type in [
            TaskEventType::TaskCreated,
            TaskEventType::TaskQueued,
            TaskEventType::TaskStarted,
            TaskEventType::TaskProgress,
            TaskEventType::TaskWaitingForInput,
            TaskEventType::TaskCompleted,
            TaskEventType::TaskFailed,
            TaskEventType::TaskCancelled,
        ] {
            assert_eq!(
                TaskEventType::from_db(event_type.as_str()),
                Some(event_type.clone())
            );
        }

        assert_eq!(TaskEventType::from_db("unknown"), None);
    }
}
