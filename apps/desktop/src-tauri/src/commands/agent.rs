// Agent Tauri commands — Phase 2.

use std::future::Future;

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use domain::task::{AgentTask, AgentTaskEvent, CreateAgentTaskInput, TaskEventType, TaskStatus};

/// DTO for AgentTask with camelCase serialization for the IPC boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentTaskDto {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub created_at: String,
    pub updated_at: String,
}

impl From<AgentTask> for AgentTaskDto {
    fn from(task: AgentTask) -> Self {
        Self {
            id: task.id,
            workspace_id: task.workspace_id,
            title: task.title,
            description: task.description,
            status: task.status,
            created_at: task.created_at.to_rfc3339(),
            updated_at: task.updated_at.to_rfc3339(),
        }
    }
}

/// DTO for AgentTaskEvent with camelCase serialization for the IPC boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentTaskEventDto {
    pub id: String,
    pub task_id: String,
    pub event_type: TaskEventType,
    pub payload: Option<String>,
    pub created_at: String,
}

impl From<AgentTaskEvent> for AgentTaskEventDto {
    fn from(event: AgentTaskEvent) -> Self {
        Self {
            id: event.id,
            task_id: event.task_id,
            event_type: event.event_type,
            payload: event.payload,
            created_at: event.created_at.to_rfc3339(),
        }
    }
}

/// Starts a task in the service and admits it to the background executor.
///
/// The service transition is persisted before executor admission. If admission
/// fails, restore the running task to queued while returning the original
/// executor error so the caller can offer a retry.
async fn start_task_with_executor_admission<
    Start,
    StartFuture,
    Admit,
    AdmitFuture,
    Requeue,
    RequeueFuture,
>(
    task_id: &str,
    start: Start,
    admit: Admit,
    requeue: Requeue,
) -> Result<AgentTask, AppError>
where
    Start: FnOnce() -> StartFuture,
    StartFuture: Future<Output = Result<AgentTask, AppError>>,
    Admit: FnOnce(AgentTask) -> AdmitFuture,
    AdmitFuture: Future<Output = Result<(), AppError>>,
    Requeue: FnOnce() -> RequeueFuture,
    RequeueFuture: Future<Output = Result<AgentTask, AppError>>,
{
    let task = start().await?;

    if let Err(error) = admit(task.clone()).await {
        if let Err(rollback_error) = requeue().await {
            tracing::error!(
                task_id = %task_id,
                error_code = rollback_error.code(),
                "failed to restore task to queue after executor admission failure"
            );
        }
        return Err(error);
    }

    Ok(task)
}

#[tauri::command]
pub async fn create_agent_task(
    workspace_id: String,
    title: String,
    description: Option<String>,
    state: State<'_, AppState>,
) -> Result<AgentTaskDto, AppError> {
    let input = CreateAgentTaskInput {
        workspace_id,
        title,
        description,
    };

    state
        .agent_service
        .create_task(input)
        .await
        .map(AgentTaskDto::from)
}

#[tauri::command]
pub async fn get_agent_task(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<AgentTaskDto>, AppError> {
    state
        .agent_service
        .get_task(&id)
        .await
        .map(|opt| opt.map(AgentTaskDto::from))
}

#[tauri::command]
pub async fn list_agent_tasks(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<AgentTaskDto>, AppError> {
    state
        .agent_service
        .list_tasks(&workspace_id)
        .await
        .map(|tasks| tasks.into_iter().map(AgentTaskDto::from).collect())
}

#[tauri::command]
pub async fn get_task_events(
    task_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<AgentTaskEventDto>, AppError> {
    state
        .agent_service
        .get_task_events(&task_id)
        .await
        .map(|events| events.into_iter().map(AgentTaskEventDto::from).collect())
}

#[tauri::command]
pub async fn queue_agent_task(
    task_id: String,
    state: State<'_, AppState>,
) -> Result<AgentTaskDto, AppError> {
    state
        .agent_service
        .queue_task(&task_id)
        .await
        .map(AgentTaskDto::from)
}

#[tauri::command]
pub async fn start_agent_task(
    task_id: String,
    state: State<'_, AppState>,
) -> Result<AgentTaskDto, AppError> {
    start_task_with_executor_admission(
        &task_id,
        || state.agent_service.start_task(&task_id),
        |task| state.task_executor.start_task(task),
        || state.agent_service.requeue_running_task(&task_id),
    )
    .await
    .map(AgentTaskDto::from)
}

#[tauri::command]
pub async fn cancel_agent_task(
    task_id: String,
    state: State<'_, AppState>,
) -> Result<AgentTaskDto, AppError> {
    // Cancel in executor first
    state.task_executor.cancel_task(&task_id).await?;

    // Then persist and stream the terminal task event.
    let task = state.agent_service.cancel_task(&task_id).await?;
    Ok(AgentTaskDto::from(task))
}

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };

    use chrono::Utc;
    use domain::task::{AgentTask, TaskStatus};

    use super::*;

    fn sample_running_task() -> AgentTask {
        let now = Utc::now();
        AgentTask {
            id: "task-1".to_string(),
            workspace_id: "workspace-1".to_string(),
            title: "Sample task".to_string(),
            description: None,
            status: TaskStatus::Running,
            created_at: now,
            updated_at: now,
        }
    }

    fn sample_queued_task() -> AgentTask {
        let mut task = sample_running_task();
        task.status = TaskStatus::Queued;
        task
    }

    #[tokio::test]
    async fn admission_success_does_not_requeue() {
        let requeue_called = Arc::new(AtomicBool::new(false));
        let requeue_called_by_closure = Arc::clone(&requeue_called);

        let result = start_task_with_executor_admission(
            "task-1",
            || async { Ok(sample_running_task()) },
            |_task| async { Ok(()) },
            move || {
                requeue_called_by_closure.store(true, Ordering::SeqCst);
                async { Ok(sample_running_task()) }
            },
        )
        .await
        .expect("admission should succeed");

        assert_eq!(result.status, TaskStatus::Running);
        assert!(!requeue_called.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn admission_failure_requeues_and_returns_original_error() {
        let requeue_called = Arc::new(AtomicBool::new(false));
        let requeue_called_by_closure = Arc::clone(&requeue_called);

        let result = start_task_with_executor_admission(
            "task-1",
            || async { Ok(sample_running_task()) },
            |_task| async { Err(AppError::Validation("executor unavailable".to_string())) },
            move || {
                requeue_called_by_closure.store(true, Ordering::SeqCst);
                async { Ok(sample_queued_task()) }
            },
        )
        .await;

        assert!(requeue_called.load(Ordering::SeqCst));
        match result {
            Err(AppError::Validation(message)) => assert_eq!(message, "executor unavailable"),
            other => panic!("expected original executor error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn service_start_failure_does_not_admit_or_requeue() {
        let admit_called = Arc::new(AtomicBool::new(false));
        let requeue_called = Arc::new(AtomicBool::new(false));
        let admit_called_by_closure = Arc::clone(&admit_called);
        let requeue_called_by_closure = Arc::clone(&requeue_called);

        let result = start_task_with_executor_admission(
            "task-1",
            || async { Err(AppError::Validation("task is not queued".to_string())) },
            move |_task| {
                admit_called_by_closure.store(true, Ordering::SeqCst);
                async { Ok(()) }
            },
            move || {
                requeue_called_by_closure.store(true, Ordering::SeqCst);
                async { Ok(sample_queued_task()) }
            },
        )
        .await;

        match result {
            Err(AppError::Validation(message)) => assert_eq!(message, "task is not queued"),
            other => panic!("expected original service error, got {other:?}"),
        }
        assert!(!admit_called.load(Ordering::SeqCst));
        assert!(!requeue_called.load(Ordering::SeqCst));
    }

    #[test]
    fn test_agent_task_dto_camel_case_serialization() {
        let task = sample_running_task();
        let dto = AgentTaskDto::from(task);

        let json = serde_json::to_string(&dto).expect("serialization failed");
        assert!(json.contains("\"workspaceId\":"));
        assert!(json.contains("\"createdAt\":"));
        assert!(json.contains("\"updatedAt\":"));
        assert!(!json.contains("\"workspace_id\":"));
        assert!(!json.contains("\"created_at\":"));
        assert!(!json.contains("\"updated_at\":"));

        let deserialized: AgentTaskDto =
            serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(deserialized, dto);
    }

    #[test]
    fn test_agent_task_event_dto_camel_case_serialization() {
        let event = AgentTaskEvent {
            id: "event-1".to_string(),
            task_id: "task-1".to_string(),
            event_type: TaskEventType::TaskProgress,
            payload: Some("Analyzing documents...".to_string()),
            created_at: Utc::now(),
        };
        let dto = AgentTaskEventDto::from(event);

        let json = serde_json::to_string(&dto).expect("serialization failed");
        assert!(json.contains("\"taskId\":"));
        assert!(json.contains("\"eventType\":"));
        assert!(json.contains("\"createdAt\":"));
        assert!(!json.contains("\"task_id\":"));
        assert!(!json.contains("\"event_type\":"));
        assert!(!json.contains("\"created_at\":"));

        let deserialized: AgentTaskEventDto =
            serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(deserialized, dto);
    }
}
