// Agent service — handles agent task business logic.

use crate::database::repositories::agent_task_repository::AgentTaskRepository;
use crate::error::AppError;
use domain::task::{AgentTask, AgentTaskEvent, CreateAgentTaskInput, TaskEventType, TaskStatus};

/// Service for managing agent tasks.
pub struct AgentService {
    repo: AgentTaskRepository,
}

impl AgentService {
    /// Creates a new agent service.
    pub fn new(repo: AgentTaskRepository) -> Self {
        Self { repo }
    }

    /// Creates a new agent task.
    pub async fn create_task(&self, input: CreateAgentTaskInput) -> Result<AgentTask, AppError> {
        // Validate input
        if input.title.trim().is_empty() {
            return Err(AppError::Validation(
                "Task title cannot be empty".to_string(),
            ));
        }

        if input.title.len() > 200 {
            return Err(AppError::Validation(
                "Task title cannot exceed 200 characters".to_string(),
            ));
        }

        // Create task via repository
        let task = self.repo.create(input).await?;

        Ok(task)
    }

    /// Queues a task for execution.
    pub async fn queue_task(&self, task_id: &str) -> Result<AgentTask, AppError> {
        let task = self
            .repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Task '{}' not found", task_id)))?;

        // Validate state transition
        if task.status != TaskStatus::Created {
            return Err(AppError::Validation(format!(
                "Cannot queue task in '{}' state",
                task.status
            )));
        }

        // Update status
        self.repo.update_status(task_id, TaskStatus::Queued).await?;

        // Create event
        self.repo
            .create_event(task_id, TaskEventType::TaskQueued, None)
            .await?;

        // Fetch updated task
        self.repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::Internal("Task disappeared after update".to_string()))
    }

    /// Starts task execution.
    pub async fn start_task(&self, task_id: &str) -> Result<AgentTask, AppError> {
        let task = self
            .repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Task '{}' not found", task_id)))?;

        // Validate state transition
        if task.status != TaskStatus::Queued {
            return Err(AppError::Validation(format!(
                "Cannot start task in '{}' state",
                task.status
            )));
        }

        // Update status
        self.repo.update_status(task_id, TaskStatus::Running).await?;

        // Create event
        self.repo
            .create_event(task_id, TaskEventType::TaskStarted, None)
            .await?;

        self.repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::Internal("Task disappeared after update".to_string()))
    }

    /// Completes a task.
    pub async fn complete_task(&self, task_id: &str) -> Result<AgentTask, AppError> {
        let task = self
            .repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Task '{}' not found", task_id)))?;

        if task.status != TaskStatus::Running && task.status != TaskStatus::WaitingForInput {
            return Err(AppError::Validation(format!(
                "Cannot complete task in '{}' state",
                task.status
            )));
        }

        self.repo
            .update_status(task_id, TaskStatus::Completed)
            .await?;

        self.repo
            .create_event(task_id, TaskEventType::TaskCompleted, None)
            .await?;

        self.repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::Internal("Task disappeared after update".to_string()))
    }

    /// Fails a task with an error message.
    pub async fn fail_task(&self, task_id: &str, error: String) -> Result<AgentTask, AppError> {
        self.repo
            .update_status(task_id, TaskStatus::Failed)
            .await?;

        self.repo
            .create_event(task_id, TaskEventType::TaskFailed, Some(error))
            .await?;

        self.repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::Internal("Task disappeared after update".to_string()))
    }

    /// Cancels a task.
    pub async fn cancel_task(&self, task_id: &str) -> Result<AgentTask, AppError> {
        let task = self
            .repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Task '{}' not found", task_id)))?;

        if !task.status.is_cancellable() {
            return Err(AppError::Validation(format!(
                "Cannot cancel task in '{}' state",
                task.status
            )));
        }

        self.repo
            .update_status(task_id, TaskStatus::Cancelled)
            .await?;

        self.repo
            .create_event(task_id, TaskEventType::TaskCancelled, None)
            .await?;

        self.repo
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::Internal("Task disappeared after update".to_string()))
    }

    /// Gets a task by ID.
    pub async fn get_task(&self, task_id: &str) -> Result<Option<AgentTask>, AppError> {
        self.repo.get(task_id).await
    }

    /// Lists tasks for a workspace.
    pub async fn list_tasks(&self, workspace_id: &str) -> Result<Vec<AgentTask>, AppError> {
        self.repo.list_by_workspace(workspace_id).await
    }

    /// Gets task events.
    pub async fn get_task_events(&self, task_id: &str) -> Result<Vec<AgentTaskEvent>, AppError> {
        self.repo.list_events(task_id).await
    }
}