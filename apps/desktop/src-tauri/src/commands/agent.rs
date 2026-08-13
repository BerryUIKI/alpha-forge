// Agent Tauri commands — Phase 2.

use std::future::Future;

use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use domain::task::{AgentTask, AgentTaskEvent, CreateAgentTaskInput};

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
) -> Result<AgentTask, AppError> {
    let input = CreateAgentTaskInput {
        workspace_id,
        title,
        description,
    };

    state.agent_service.create_task(input).await
}

#[tauri::command]
pub async fn get_agent_task(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<AgentTask>, AppError> {
    state.agent_service.get_task(&id).await
}

#[tauri::command]
pub async fn list_agent_tasks(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<AgentTask>, AppError> {
    state.agent_service.list_tasks(&workspace_id).await
}

#[tauri::command]
pub async fn get_task_events(
    task_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<AgentTaskEvent>, AppError> {
    state.agent_service.get_task_events(&task_id).await
}

#[tauri::command]
pub async fn queue_agent_task(
    task_id: String,
    state: State<'_, AppState>,
) -> Result<AgentTask, AppError> {
    state.agent_service.queue_task(&task_id).await
}

#[tauri::command]
pub async fn start_agent_task(
    task_id: String,
    state: State<'_, AppState>,
) -> Result<AgentTask, AppError> {
    start_task_with_executor_admission(
        &task_id,
        || state.agent_service.start_task(&task_id),
        |task| state.task_executor.start_task(task),
        || state.agent_service.requeue_running_task(&task_id),
    )
    .await
}

#[tauri::command]
pub async fn cancel_agent_task(
    task_id: String,
    state: State<'_, AppState>,
) -> Result<AgentTask, AppError> {
    // Cancel in executor first
    state.task_executor.cancel_task(&task_id).await?;

    // Then persist and stream the terminal task event.
    let task = state.agent_service.cancel_task(&task_id).await?;
    Ok(task)
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
}
