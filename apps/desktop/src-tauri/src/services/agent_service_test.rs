// Tests for agent service background execution.

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex as StdMutex};
    use std::time::Duration;

    use sqlx::sqlite::SqlitePoolOptions;

    use crate::agent::events::EventPublisher;
    use crate::agent::runtime::{
        AgentRuntime, CancellationToken, ExecutionResult, ProgressReporter,
    };
    use crate::database::repositories::agent_task_repository::AgentTaskRepository;
    use crate::error::AppError;
    use crate::services::agent_service::AgentService;
    use domain::task::{
        AgentTask, AgentTaskEvent, CreateAgentTaskInput, TaskEventType, TaskStatus,
    };

    #[derive(Default)]
    struct RecordingPublisher {
        events: StdMutex<Vec<AgentTaskEvent>>,
    }

    impl RecordingPublisher {
        fn events(&self) -> Vec<AgentTaskEvent> {
            self.events.lock().expect("publisher lock").clone()
        }
    }

    impl EventPublisher for RecordingPublisher {
        fn publish(&self, event: &AgentTaskEvent) -> Result<(), AppError> {
            self.events
                .lock()
                .expect("publisher lock")
                .push(event.clone());
            Ok(())
        }
    }

    struct FakeRuntime {
        result: ExecutionResult,
        wait_for_cancellation: bool,
        delay: Option<Duration>,
    }

    impl FakeRuntime {
        fn immediate(result: ExecutionResult) -> Self {
            Self {
                result,
                wait_for_cancellation: false,
                delay: None,
            }
        }
    }

    #[async_trait::async_trait]
    impl AgentRuntime for FakeRuntime {
        async fn execute(
            &self,
            _task: AgentTask,
            progress: Arc<dyn ProgressReporter>,
            cancellation: CancellationToken,
        ) -> Result<ExecutionResult, AppError> {
            progress.report("step 1".to_string()).await?;

            if let Some(delay) = self.delay {
                tokio::time::sleep(delay).await;
            }

            if self.wait_for_cancellation {
                for _ in 0..100 {
                    if cancellation.is_cancelled() {
                        return Ok(ExecutionResult::cancelled());
                    }
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
                return Ok(ExecutionResult::failure("cancellation timed out"));
            }

            Ok(self.result.clone())
        }
    }

    async fn setup_service(
        runtime: Arc<dyn AgentRuntime>,
    ) -> (AgentService, Arc<RecordingPublisher>, AgentTaskRepository) {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("Failed to create test database");

        crate::database::migrations::run(&pool)
            .await
            .expect("Failed to run migrations");

        sqlx::query("INSERT INTO workspaces (id, name) VALUES ('w1', 'Workspace')")
            .execute(&pool)
            .await
            .expect("Failed to insert workspace");

        let publisher = Arc::new(RecordingPublisher::default());
        let repo = AgentTaskRepository::new(pool);
        let service = AgentService::new(repo.clone(), runtime, publisher.clone());

        (service, publisher, repo)
    }

    async fn create_queued_task(service: &AgentService) -> AgentTask {
        let task = service
            .create_task(CreateAgentTaskInput {
                workspace_id: "w1".to_string(),
                title: "Research Tesla".to_string(),
                description: None,
            })
            .await
            .expect("Failed to create task");

        service
            .queue_task(&task.id)
            .await
            .expect("Failed to queue task")
    }

    async fn wait_for_status(
        service: &AgentService,
        task_id: &str,
        expected: TaskStatus,
    ) -> AgentTask {
        for _ in 0..100 {
            let task = service
                .get_task(task_id)
                .await
                .expect("Failed to get task")
                .expect("Task should exist");

            if task.status == expected {
                return task;
            }

            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        panic!("task did not reach {:?}", expected);
    }

    #[tokio::test]
    async fn test_start_task_completes_and_streams_events() {
        let runtime = Arc::new(FakeRuntime::immediate(ExecutionResult::success(Some(
            r#"{"summary":"ok"}"#.to_string(),
        ))));
        let (service, publisher, _repo) = setup_service(runtime).await;
        let task = create_queued_task(&service).await;

        let running = service
            .start_task(&task.id)
            .await
            .expect("Failed to start task");
        assert_eq!(running.status, TaskStatus::Running);

        let completed = wait_for_status(&service, &task.id, TaskStatus::Completed).await;
        assert_eq!(completed.output.as_deref(), Some(r#"{"summary":"ok"}"#));

        let events = publisher.events();
        assert!(events
            .iter()
            .any(|event| event.event_type == TaskEventType::TaskStarted));
        assert!(events
            .iter()
            .any(|event| event.event_type == TaskEventType::TaskProgress));
        assert!(events
            .iter()
            .any(|event| event.event_type == TaskEventType::TaskCompleted));
    }

    #[tokio::test]
    async fn test_start_task_failure_persists_error() {
        let runtime = Arc::new(FakeRuntime::immediate(ExecutionResult::failure("boom")));
        let (service, publisher, _repo) = setup_service(runtime).await;
        let task = create_queued_task(&service).await;

        service
            .start_task(&task.id)
            .await
            .expect("Failed to start task");

        let failed = wait_for_status(&service, &task.id, TaskStatus::Failed).await;
        assert_eq!(failed.error.as_deref(), Some("boom"));

        assert!(publisher
            .events()
            .iter()
            .any(|event| event.event_type == TaskEventType::TaskFailed));
    }

    #[tokio::test]
    async fn test_cancel_running_task() {
        let runtime = Arc::new(FakeRuntime {
            result: ExecutionResult::success(None),
            wait_for_cancellation: true,
            delay: None,
        });
        let (service, publisher, _repo) = setup_service(runtime).await;
        let task = create_queued_task(&service).await;

        service
            .start_task(&task.id)
            .await
            .expect("Failed to start task");
        tokio::time::sleep(Duration::from_millis(20)).await;

        service
            .cancel_task(&task.id)
            .await
            .expect("Failed to cancel task");

        let cancelled = wait_for_status(&service, &task.id, TaskStatus::Cancelled).await;
        assert_eq!(cancelled.status, TaskStatus::Cancelled);
        assert!(publisher
            .events()
            .iter()
            .any(|event| event.event_type == TaskEventType::TaskCancelled));
    }

    #[tokio::test]
    async fn test_timeout_marks_task_failed() {
        let runtime = Arc::new(FakeRuntime {
            result: ExecutionResult::success(None),
            wait_for_cancellation: false,
            delay: Some(Duration::from_secs(1)),
        });
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("Failed to create test database");
        crate::database::migrations::run(&pool)
            .await
            .expect("Failed to run migrations");
        sqlx::query("INSERT INTO workspaces (id, name) VALUES ('w1', 'Workspace')")
            .execute(&pool)
            .await
            .expect("Failed to insert workspace");

        let publisher = Arc::new(RecordingPublisher::default());
        let service = AgentService::new(AgentTaskRepository::new(pool), runtime, publisher.clone())
            .with_timeout(Duration::from_millis(20));

        let task = create_queued_task(&service).await;
        service
            .start_task(&task.id)
            .await
            .expect("Failed to start task");

        let failed = wait_for_status(&service, &task.id, TaskStatus::Failed).await;
        assert!(failed
            .error
            .as_deref()
            .unwrap_or_default()
            .contains("timed out"));
    }

    #[tokio::test]
    async fn test_start_requires_queued_state() {
        let runtime = Arc::new(FakeRuntime::immediate(ExecutionResult::success(None)));
        let (service, _, _repo) = setup_service(runtime).await;

        let task = service
            .create_task(CreateAgentTaskInput {
                workspace_id: "w1".to_string(),
                title: "Not queued".to_string(),
                description: None,
            })
            .await
            .expect("Failed to create task");

        let result = service.start_task(&task.id).await;
        assert!(matches!(result, Err(AppError::Validation(_))));
    }

    struct ConcurrencyRuntime {
        running: AtomicUsize,
        max_running: AtomicUsize,
        delay: Duration,
    }

    impl ConcurrencyRuntime {
        fn new(delay: Duration) -> Self {
            Self {
                running: AtomicUsize::new(0),
                max_running: AtomicUsize::new(0),
                delay,
            }
        }
    }

    #[async_trait::async_trait]
    impl AgentRuntime for ConcurrencyRuntime {
        async fn execute(
            &self,
            _task: AgentTask,
            _progress: Arc<dyn ProgressReporter>,
            _cancellation: CancellationToken,
        ) -> Result<ExecutionResult, AppError> {
            let current = self.running.fetch_add(1, Ordering::SeqCst) + 1;
            self.max_running.fetch_max(current, Ordering::SeqCst);
            tokio::time::sleep(self.delay).await;
            self.running.fetch_sub(1, Ordering::SeqCst);
            Ok(ExecutionResult::success(Some(
                r#"{"summary":"ok"}"#.to_string(),
            )))
        }
    }

    struct SequencedRuntime {
        results: StdMutex<VecDeque<ExecutionResult>>,
    }

    impl SequencedRuntime {
        fn new(results: Vec<ExecutionResult>) -> Self {
            Self {
                results: StdMutex::new(VecDeque::from(results)),
            }
        }
    }

    #[async_trait::async_trait]
    impl AgentRuntime for SequencedRuntime {
        async fn execute(
            &self,
            _task: AgentTask,
            _progress: Arc<dyn ProgressReporter>,
            _cancellation: CancellationToken,
        ) -> Result<ExecutionResult, AppError> {
            let result = self
                .results
                .lock()
                .expect("results lock")
                .pop_front()
                .unwrap_or_else(|| ExecutionResult::success(None));
            Ok(result)
        }
    }

    #[tokio::test]
    async fn test_concurrency_limit_queues_extra_tasks() {
        let runtime = Arc::new(ConcurrencyRuntime::new(Duration::from_millis(30)));
        let (service, _publisher, _repo) = setup_service(runtime.clone()).await;
        let service = service.with_max_concurrency(1);

        let mut task_ids = Vec::new();
        for _ in 0..3 {
            task_ids.push(create_queued_task(&service).await.id);
        }

        for task_id in &task_ids {
            service
                .start_task(task_id)
                .await
                .expect("Failed to start task");
        }

        for task_id in &task_ids {
            wait_for_status(&service, task_id, TaskStatus::Completed).await;
        }

        assert_eq!(runtime.max_running.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_recover_interrupted_tasks_marks_running_as_failed() {
        let runtime = Arc::new(FakeRuntime::immediate(ExecutionResult::success(None)));
        let (service, publisher, repo) = setup_service(runtime).await;

        let queued = create_queued_task(&service).await;
        let interrupted = create_queued_task(&service).await;
        repo.update_status(&interrupted.id, TaskStatus::Running)
            .await
            .expect("Failed to mark task running");

        let recovered = service
            .recover_interrupted_tasks()
            .await
            .expect("Failed to recover tasks");

        assert_eq!(recovered, 1);

        let task = service
            .get_task(&interrupted.id)
            .await
            .expect("Failed to get task")
            .expect("Task should exist");
        assert_eq!(task.status, TaskStatus::Failed);
        assert!(task
            .error
            .as_deref()
            .unwrap_or_default()
            .contains("restart"));

        assert!(publisher.events().iter().any(|event| {
            event.task_id == interrupted.id && event.event_type == TaskEventType::TaskFailed
        }));

        let untouched = service
            .get_task(&queued.id)
            .await
            .expect("Failed to get task")
            .expect("Task should exist");
        assert_eq!(untouched.status, TaskStatus::Queued);
    }

    #[tokio::test]
    async fn test_retry_requeues_failed_task_and_runs_again() {
        let runtime = Arc::new(SequencedRuntime::new(vec![
            ExecutionResult::failure("boom"),
            ExecutionResult::success(Some(r#"{"summary":"ok"}"#.to_string())),
        ]));
        let (service, _publisher, _repo) = setup_service(runtime).await;

        let task = create_queued_task(&service).await;
        service
            .start_task(&task.id)
            .await
            .expect("Failed to start task");
        wait_for_status(&service, &task.id, TaskStatus::Failed).await;

        let requeued = service.retry_task(&task.id).await.expect("Failed to retry");
        assert_eq!(requeued.status, TaskStatus::Queued);
        assert!(requeued.error.is_none());
        assert!(requeued.output.is_none());

        service
            .start_task(&task.id)
            .await
            .expect("Failed to restart task");
        let completed = wait_for_status(&service, &task.id, TaskStatus::Completed).await;
        assert!(completed.output.is_some());
    }

    #[tokio::test]
    async fn test_retry_rejects_completed_task() {
        let runtime = Arc::new(FakeRuntime::immediate(ExecutionResult::success(Some(
            r#"{"summary":"ok"}"#.to_string(),
        ))));
        let (service, _, _repo) = setup_service(runtime).await;

        let task = create_queued_task(&service).await;
        service
            .start_task(&task.id)
            .await
            .expect("Failed to start task");
        wait_for_status(&service, &task.id, TaskStatus::Completed).await;

        let result = service.retry_task(&task.id).await;
        assert!(matches!(result, Err(AppError::Validation(_))));
    }
}
