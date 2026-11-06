// Agent task repository — handles persistence for agent tasks and events.

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::error::AppError;
use domain::task::{AgentTask, AgentTaskEvent, CreateAgentTaskInput, TaskEventType, TaskStatus};

/// Repository for managing agent tasks.
#[derive(Clone)]
pub struct AgentTaskRepository {
    pool: SqlitePool,
}

impl AgentTaskRepository {
    /// Creates a new agent task repository.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Creates a new agent task. The service records the `TaskCreated` event.
    pub async fn create(&self, input: CreateAgentTaskInput) -> Result<AgentTask, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO agent_tasks (id, workspace_id, title, description, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&input.workspace_id)
        .bind(&input.title)
        .bind(&input.description)
        .bind(TaskStatus::Created.as_str())
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create agent task: {}", e)))?;

        Ok(AgentTask {
            id,
            workspace_id: input.workspace_id,
            title: input.title,
            description: input.description,
            status: TaskStatus::Created,
            output: None,
            error: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Lists all tasks for a workspace.
    pub async fn list_by_workspace(&self, workspace_id: &str) -> Result<Vec<AgentTask>, AppError> {
        let rows = sqlx::query_as::<_, AgentTaskRow>(
            r#"
            SELECT id, workspace_id, title, description, status, output, error, created_at, updated_at
            FROM agent_tasks
            WHERE workspace_id = ?
            ORDER BY created_at DESC, rowid DESC
            "#,
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list agent tasks: {}", e)))?;

        rows.into_iter().map(AgentTask::try_from).collect()
    }

    /// Searches task titles and descriptions in a workspace.
    pub async fn search(
        &self,
        workspace_id: &str,
        query: &str,
        limit: i64,
    ) -> Result<Vec<AgentTask>, AppError> {
        let pattern = format!("%{}%", query);

        let rows = sqlx::query_as::<_, AgentTaskRow>(
            r#"
            SELECT id, workspace_id, title, description, status, output, error, created_at, updated_at
            FROM agent_tasks
            WHERE workspace_id = ? AND (title LIKE ? OR description LIKE ?)
            ORDER BY created_at DESC, rowid DESC
            LIMIT ?
            "#,
        )
        .bind(workspace_id)
        .bind(&pattern)
        .bind(&pattern)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to search agent tasks: {}", e)))?;

        rows.into_iter().map(AgentTask::try_from).collect()
    }

    /// Gets a task by ID.
    pub async fn get(&self, id: &str) -> Result<Option<AgentTask>, AppError> {
        let row = sqlx::query_as::<_, AgentTaskRow>(
            r#"
            SELECT id, workspace_id, title, description, status, output, error, created_at, updated_at
            FROM agent_tasks
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to get agent task: {}", e)))?;

        row.map(AgentTask::try_from).transpose()
    }

    /// Updates task status and returns the updated task.
    pub async fn update_status(&self, id: &str, status: TaskStatus) -> Result<AgentTask, AppError> {
        let row = sqlx::query_as::<_, AgentTaskRow>(
            r#"
            UPDATE agent_tasks
            SET status = ?, updated_at = ?
            WHERE id = ?
            RETURNING id, workspace_id, title, description, status, output, error, created_at, updated_at
            "#,
        )
        .bind(status.as_str())
        .bind(Utc::now())
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update task status: {}", e)))?
        .ok_or_else(|| AppError::NotFound(format!("Agent task '{}' not found", id)))?;

        AgentTask::try_from(row)
    }

    /// Marks a task as completed and returns it.
    pub async fn complete(&self, id: &str, output: Option<&str>) -> Result<AgentTask, AppError> {
        let row = sqlx::query_as::<_, AgentTaskRow>(
            r#"
            UPDATE agent_tasks
            SET status = ?, output = ?, error = NULL, updated_at = ?
            WHERE id = ?
            RETURNING id, workspace_id, title, description, status, output, error, created_at, updated_at
            "#,
        )
        .bind(TaskStatus::Completed.as_str())
        .bind(output)
        .bind(Utc::now())
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to complete agent task: {}", e)))?
        .ok_or_else(|| AppError::NotFound(format!("Agent task '{}' not found", id)))?;

        AgentTask::try_from(row)
    }

    /// Marks a task as failed and returns it.
    pub async fn fail(&self, id: &str, error: &str) -> Result<AgentTask, AppError> {
        let row = sqlx::query_as::<_, AgentTaskRow>(
            r#"
            UPDATE agent_tasks
            SET status = ?, output = NULL, error = ?, updated_at = ?
            WHERE id = ?
            RETURNING id, workspace_id, title, description, status, output, error, created_at, updated_at
            "#,
        )
        .bind(TaskStatus::Failed.as_str())
        .bind(error)
        .bind(Utc::now())
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to fail agent task: {}", e)))?
        .ok_or_else(|| AppError::NotFound(format!("Agent task '{}' not found", id)))?;

        AgentTask::try_from(row)
    }

    /// Marks a running task as cancelled and returns it.
    pub async fn mark_cancelled(&self, id: &str) -> Result<AgentTask, AppError> {
        let row = sqlx::query_as::<_, AgentTaskRow>(
            r#"
            UPDATE agent_tasks
            SET status = ?, error = NULL, updated_at = ?
            WHERE id = ?
            RETURNING id, workspace_id, title, description, status, output, error, created_at, updated_at
            "#,
        )
        .bind(TaskStatus::Cancelled.as_str())
        .bind(Utc::now())
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to cancel agent task: {}", e)))?
        .ok_or_else(|| AppError::NotFound(format!("Agent task '{}' not found", id)))?;

        AgentTask::try_from(row)
    }

    /// Marks all running tasks as failed, returning their IDs.
    ///
    /// Used at startup to clean up tasks that were interrupted when the
    /// previous process exited.
    pub async fn fail_running_tasks(&self, error: &str) -> Result<Vec<String>, AppError> {
        let ids = sqlx::query_scalar::<_, String>(
            r#"
            UPDATE agent_tasks
            SET status = ?, output = NULL, error = ?, updated_at = ?
            WHERE status = ?
            RETURNING id
            "#,
        )
        .bind(TaskStatus::Failed.as_str())
        .bind(error)
        .bind(Utc::now())
        .bind(TaskStatus::Running.as_str())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to recover running agent tasks: {}", e)))?;

        Ok(ids)
    }

    /// Returns a failed or cancelled task to the queue.
    pub async fn requeue(&self, id: &str) -> Result<AgentTask, AppError> {
        let row = sqlx::query_as::<_, AgentTaskRow>(
            r#"
            UPDATE agent_tasks
            SET status = ?, output = NULL, error = NULL, updated_at = ?
            WHERE id = ? AND status IN (?, ?)
            RETURNING id, workspace_id, title, description, status, output, error, created_at, updated_at
            "#,
        )
        .bind(TaskStatus::Queued.as_str())
        .bind(Utc::now())
        .bind(id)
        .bind(TaskStatus::Failed.as_str())
        .bind(TaskStatus::Cancelled.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to requeue agent task: {}", e)))?
        .ok_or_else(|| {
            AppError::Validation(format!(
                "Agent task '{}' cannot be retried in its current state",
                id
            ))
        })?;

        AgentTask::try_from(row)
    }

    /// Creates a task event.
    pub async fn create_event(
        &self,
        task_id: &str,
        event_type: TaskEventType,
        payload: Option<String>,
    ) -> Result<AgentTaskEvent, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO agent_task_events (id, task_id, event_type, payload, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(task_id)
        .bind(event_type.as_str())
        .bind(&payload)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create task event: {}", e)))?;

        Ok(AgentTaskEvent {
            id,
            task_id: task_id.to_string(),
            event_type,
            payload,
            created_at: now,
        })
    }

    /// Lists events for a task.
    pub async fn list_events(&self, task_id: &str) -> Result<Vec<AgentTaskEvent>, AppError> {
        let rows = sqlx::query_as::<_, AgentTaskEventRow>(
            r#"
            SELECT id, task_id, event_type, payload, created_at
            FROM agent_task_events
            WHERE task_id = ?
            ORDER BY created_at ASC, rowid ASC
            "#,
        )
        .bind(task_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list task events: {}", e)))?;

        rows.into_iter().map(AgentTaskEvent::try_from).collect()
    }
}

/// Database row representation of an agent task.
#[derive(Debug, sqlx::FromRow)]
struct AgentTaskRow {
    id: String,
    workspace_id: String,
    title: String,
    description: Option<String>,
    status: String,
    output: Option<String>,
    error: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<AgentTaskRow> for AgentTask {
    type Error = AppError;

    fn try_from(row: AgentTaskRow) -> Result<Self, Self::Error> {
        let status = TaskStatus::from_db(&row.status).ok_or_else(|| {
            AppError::Internal(format!("unknown agent task status '{}'", row.status))
        })?;

        Ok(AgentTask {
            id: row.id,
            workspace_id: row.workspace_id,
            title: row.title,
            description: row.description,
            status,
            output: row.output,
            error: row.error,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

/// Database row representation of an agent task event.
#[derive(Debug, sqlx::FromRow)]
struct AgentTaskEventRow {
    id: String,
    task_id: String,
    event_type: String,
    payload: Option<String>,
    created_at: DateTime<Utc>,
}

impl TryFrom<AgentTaskEventRow> for AgentTaskEvent {
    type Error = AppError;

    fn try_from(row: AgentTaskEventRow) -> Result<Self, Self::Error> {
        let event_type = TaskEventType::from_db(&row.event_type).ok_or_else(|| {
            AppError::Internal(format!(
                "unknown agent task event type '{}'",
                row.event_type
            ))
        })?;

        Ok(AgentTaskEvent {
            id: row.id,
            task_id: row.task_id,
            event_type,
            payload: row.payload,
            created_at: row.created_at,
        })
    }
}
