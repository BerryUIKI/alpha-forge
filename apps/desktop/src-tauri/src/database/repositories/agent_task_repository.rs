// Agent task repository — handles persistence for agent tasks and events.

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::error::AppError;
use domain::task::{AgentTask, AgentTaskEvent, CreateAgentTaskInput, TaskEventType, TaskStatus};

/// Repository for managing agent tasks.
pub struct AgentTaskRepository {
    pool: SqlitePool,
}

impl AgentTaskRepository {
    /// Creates a new agent task repository.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Creates a new agent task.
    pub async fn create(&self, input: CreateAgentTaskInput) -> Result<AgentTask, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let status_str = "created";

        sqlx::query(
            r#"
            INSERT INTO agent_tasks (id, workspace_id, title, description, status, input, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&input.workspace_id)
        .bind(&input.title)
        .bind(&input.description)
        .bind(status_str)
        .bind(&input.title)  // Use title as input for backward compatibility
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create agent task: {}", e)))?;

        // Create initial event
        self.create_event(&id, TaskEventType::TaskCreated, None)
            .await?;

        Ok(AgentTask {
            id,
            workspace_id: input.workspace_id,
            title: input.title,
            description: input.description,
            status: TaskStatus::Created,
            created_at: now,
            updated_at: now,
        })
    }

    /// Lists all tasks for a workspace.
    pub async fn list_by_workspace(&self, workspace_id: &str) -> Result<Vec<AgentTask>, AppError> {
        let rows = sqlx::query_as::<_, AgentTaskRow>(
            r#"
            SELECT id, workspace_id, title, description, status, created_at, updated_at
            FROM agent_tasks
            WHERE workspace_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list agent tasks: {}", e)))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    /// Gets a task by ID.
    pub async fn get(&self, id: &str) -> Result<Option<AgentTask>, AppError> {
        let row = sqlx::query_as::<_, AgentTaskRow>(
            r#"
            SELECT id, workspace_id, title, description, status, created_at, updated_at
            FROM agent_tasks
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to get agent task: {}", e)))?;

        Ok(row.map(|r| r.into()))
    }

    /// Updates task status.
    pub async fn update_status(&self, id: &str, status: TaskStatus) -> Result<(), AppError> {
        let status_str = match status {
            TaskStatus::Created => "created",
            TaskStatus::Queued => "queued",
            TaskStatus::Running => "running",
            TaskStatus::WaitingForInput => "waiting_for_input",
            TaskStatus::Completed => "completed",
            TaskStatus::Failed => "failed",
            TaskStatus::Cancelled => "cancelled",
        };

        let rows_affected = sqlx::query(
            r#"
            UPDATE agent_tasks
            SET status = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(status_str)
        .bind(Utc::now())
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update task status: {}", e)))?
        .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Agent task '{}' not found", id)));
        }

        Ok(())
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
        let event_type_str = match event_type {
            TaskEventType::TaskCreated => "task_created",
            TaskEventType::TaskQueued => "task_queued",
            TaskEventType::TaskStarted => "task_started",
            TaskEventType::TaskProgress => "task_progress",
            TaskEventType::TaskWaitingForInput => "task_waiting_for_input",
            TaskEventType::TaskCompleted => "task_completed",
            TaskEventType::TaskFailed => "task_failed",
            TaskEventType::TaskCancelled => "task_cancelled",
        };

        sqlx::query(
            r#"
            INSERT INTO agent_task_events (id, task_id, event_type, payload, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(task_id)
        .bind(event_type_str)
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
            ORDER BY created_at ASC
            "#,
        )
        .bind(task_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list task events: {}", e)))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
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
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<AgentTaskRow> for AgentTask {
    fn from(row: AgentTaskRow) -> Self {
        let status = match row.status.as_str() {
            "created" => TaskStatus::Created,
            "queued" => TaskStatus::Queued,
            "running" => TaskStatus::Running,
            "waiting_for_input" => TaskStatus::WaitingForInput,
            "completed" => TaskStatus::Completed,
            "failed" => TaskStatus::Failed,
            "cancelled" => TaskStatus::Cancelled,
            _ => TaskStatus::Created,
        };

        AgentTask {
            id: row.id,
            workspace_id: row.workspace_id,
            title: row.title,
            description: row.description,
            status,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
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

impl From<AgentTaskEventRow> for AgentTaskEvent {
    fn from(row: AgentTaskEventRow) -> Self {
        let event_type = match row.event_type.as_str() {
            "task_created" => TaskEventType::TaskCreated,
            "task_queued" => TaskEventType::TaskQueued,
            "task_started" => TaskEventType::TaskStarted,
            "task_progress" => TaskEventType::TaskProgress,
            "task_waiting_for_input" => TaskEventType::TaskWaitingForInput,
            "task_completed" => TaskEventType::TaskCompleted,
            "task_failed" => TaskEventType::TaskFailed,
            "task_cancelled" => TaskEventType::TaskCancelled,
            _ => TaskEventType::TaskCreated,
        };

        AgentTaskEvent {
            id: row.id,
            task_id: row.task_id,
            event_type,
            payload: row.payload,
            created_at: row.created_at,
        }
    }
}
