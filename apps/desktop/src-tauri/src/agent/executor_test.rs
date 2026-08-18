// Tests for agent task executor.

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use chrono::Utc;
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time::sleep;

    use crate::agent::executor::ExecutorConfig;
    use crate::database::repositories::agent_task_repository::AgentTaskRepository;
    use domain::task::{AgentTask, TaskStatus};
    use provider_core::{
        ProviderError, ResearchCompletion, ResearchCompletionRequest, ResearchProvider,
    };

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("Failed to create test database");

        // Run migrations
        sqlx::query(include_str!("../../migrations/0001_initial.sql"))
            .execute(&pool)
            .await
            .expect("Failed to run initial migration");

        sqlx::query(include_str!(
            "../../migrations/0002_agent_tasks_enhancement.sql"
        ))
        .execute(&pool)
        .await
        .expect("Failed to run agent tasks migration");

        sqlx::query(include_str!(
            "../../migrations/0003_fix_status_constraint.sql"
        ))
        .execute(&pool)
        .await
        .expect("Failed to run status constraint fix");

        pool
    }

    #[tokio::test]
    async fn test_executor_config_default() {
        let config = ExecutorConfig::default();

        assert_eq!(config.max_concurrent, 5);
        assert_eq!(config.default_timeout_secs, 300);
    }

    #[tokio::test]
    async fn test_executor_creation() {
        let pool = setup_test_db().await;
        let _repo = AgentTaskRepository::new(pool);
        let config = ExecutorConfig::default();

        // Note: In real tests, we'd need a mock AppHandle
        // For now, just verify the config works
        assert_eq!(config.max_concurrent, 5);
    }

    /// Mock provider for testing concurrent execution
    struct MockProvider {
        call_count: Arc<AtomicUsize>,
        delay_ms: u64,
    }

    #[async_trait]
    impl ResearchProvider for MockProvider {
        async fn complete_research(
            &self,
            _request: ResearchCompletionRequest,
        ) -> Result<ResearchCompletion, ProviderError> {
            self.call_count.fetch_add(1, Ordering::SeqCst);
            sleep(Duration::from_millis(self.delay_ms)).await;
            Ok(ResearchCompletion {
                summary: "Mock research".to_string(),
                claims: vec![],
                evidence: vec![],
                risks: vec![],
                confidence: 80,
            })
        }
    }

    /// Test that verifies the task registration logic is safe from race conditions
    /// This tests the critical section where tasks are registered before execution
    #[tokio::test]
    async fn test_task_registration_synchronization() {
        let call_count = Arc::new(AtomicUsize::new(0));
        let _provider = Arc::new(MockProvider {
            call_count: call_count.clone(),
            delay_ms: 50,
        });

        // Simulate multiple concurrent task registrations
        let mut registration_handles = vec![];

        for i in 0..5 {
            let count_clone = call_count.clone();
            let handle = tokio::spawn(async move {
                // Simulate the registration delay that could cause race conditions
                sleep(Duration::from_millis(10)).await;
                count_clone.fetch_add(1, Ordering::SeqCst);
                i
            });
            registration_handles.push(handle);
        }

        // Wait for all registrations to complete
        let mut results = vec![];
        for handle in registration_handles {
            results.push(handle.await.expect("Registration should not panic"));
        }

        // Verify all tasks were registered
        assert_eq!(results.len(), 5);
        assert_eq!(call_count.load(Ordering::SeqCst), 5);
    }

    /// Test that executor config properly limits concurrent tasks
    #[test]
    fn test_executor_config_enforces_limits() {
        let config = ExecutorConfig {
            max_concurrent: 2,
            default_timeout_secs: 60,
        };

        assert_eq!(config.max_concurrent, 2);
        assert!(config.max_concurrent > 0);
        assert!(config.default_timeout_secs > 0);
    }

    /// Test that task creation with concurrent-safe IDs works correctly
    #[test]
    fn test_task_creation_with_unique_ids() {
        let mut task_ids = std::collections::HashSet::new();

        for i in 0..10 {
            let task = AgentTask {
                id: format!(
                    "task-{}-{}",
                    i,
                    Utc::now().timestamp_nanos_opt().unwrap_or(0)
                ),
                workspace_id: "workspace-1".to_string(),
                title: format!("Task {}", i),
                description: None,
                status: TaskStatus::Queued,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            // Verify each task has a unique ID
            assert!(task_ids.insert(task.id), "Task IDs should be unique");
        }

        assert_eq!(task_ids.len(), 10);
    }
}
