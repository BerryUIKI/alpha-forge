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
        match self {
            TaskStatus::Created => write!(f, "created"),
            TaskStatus::Queued => write!(f, "queued"),
            TaskStatus::Running => write!(f, "running"),
            TaskStatus::WaitingForInput => write!(f, "waiting_for_input"),
            TaskStatus::Completed => write!(f, "completed"),
            TaskStatus::Failed => write!(f, "failed"),
            TaskStatus::Cancelled => write!(f, "cancelled"),
        }
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
}