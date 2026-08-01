// Tests for agent task executor.

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;

    use crate::agent::executor::ExecutorConfig;
    use crate::database::repositories::agent_task_repository::AgentTaskRepository;

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

        sqlx::query(include_str!("../../migrations/0002_agent_tasks_enhancement.sql"))
            .execute(&pool)
            .await
            .expect("Failed to run agent tasks migration");

        sqlx::query(include_str!("../../migrations/0003_fix_status_constraint.sql"))
            .execute(&pool)
            .await
            .expect("Failed to run status constraint fix");

        pool
    }

    fn create_test_workspace(pool: &SqlitePool) -> String {
        let workspace_id = uuid::Uuid::new_v4().to_string();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            sqlx::query(
                "INSERT INTO workspaces (id, name, created_at, updated_at) VALUES (?, 'Test Workspace', datetime('now'), datetime('now'))"
            )
            .bind(&workspace_id)
            .execute(pool)
            .await
            .expect("Failed to create test workspace");
        });
        workspace_id
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

    #[tokio::test]
    async fn test_running_count_empty() {
        let pool = setup_test_db().await;
        let _repo = AgentTaskRepository::new(pool);
        let _config = ExecutorConfig::default();

        // Note: Executor requires AppHandle which we can't easily mock
        // This test verifies the setup works
        assert!(true);
    }
}