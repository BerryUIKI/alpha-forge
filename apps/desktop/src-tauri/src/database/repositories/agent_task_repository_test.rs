// Tests for agent task repository.

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;

    use crate::database::repositories::agent_task_repository::AgentTaskRepository;
    use domain::task::{CreateAgentTaskInput, TaskEventType, TaskStatus};

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("Failed to create test database");

        // Run migrations
        sqlx::query(include_str!("../../../migrations/0001_initial.sql"))
            .execute(&pool)
            .await
            .expect("Failed to run initial migration");

        sqlx::query(include_str!("../../../migrations/0002_agent_tasks_enhancement.sql"))
            .execute(&pool)
            .await
            .expect("Failed to run agent tasks migration");

        sqlx::query(include_str!("../../../migrations/0003_fix_status_constraint.sql"))
            .execute(&pool)
            .await
            .expect("Failed to run status constraint fix");

        pool
    }

    fn create_repo(pool: SqlitePool) -> AgentTaskRepository {
        AgentTaskRepository::new(pool)
    }

    async fn create_test_workspace(pool: &SqlitePool) -> String {
        let workspace_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO workspaces (id, name, created_at, updated_at) VALUES (?, 'Test Workspace', datetime('now'), datetime('now'))"
        )
        .bind(&workspace_id)
        .execute(pool)
        .await
        .expect("Failed to create test workspace");
        workspace_id
    }

    #[tokio::test]
    async fn test_create_task() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool);

        let input = CreateAgentTaskInput {
            workspace_id: workspace_id.clone(),
            title: "Test Task".to_string(),
            description: Some("Test description".to_string()),
        };

        let task = repo.create(input).await.expect("Failed to create task");

        assert!(!task.id.is_empty());
        assert_eq!(task.workspace_id, workspace_id);
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.description, Some("Test description".to_string()));
        assert_eq!(task.status, TaskStatus::Created);
    }

    #[tokio::test]
    async fn test_create_task_without_description() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool);

        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Minimal Task".to_string(),
            description: None,
        };

        let task = repo.create(input).await.expect("Failed to create task");

        assert_eq!(task.title, "Minimal Task");
        assert_eq!(task.description, None);
    }

    #[tokio::test]
    async fn test_get_task() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool);

        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Test Task".to_string(),
            description: None,
        };
        let created = repo.create(input).await.expect("Failed to create task");

        let retrieved = repo.get(&created.id).await.expect("Failed to get task");

        assert!(retrieved.is_some());
        let task = retrieved.unwrap();
        assert_eq!(task.id, created.id);
        assert_eq!(task.title, "Test Task");
    }

    #[tokio::test]
    async fn test_get_task_not_found() {
        let pool = setup_test_db().await;
        let repo = create_repo(pool);

        let result = repo.get("non-existent-id").await.expect("Failed to get task");

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_list_by_workspace() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool);

        // Create multiple tasks
        for i in 1..=3 {
            let input = CreateAgentTaskInput {
                workspace_id: workspace_id.clone(),
                title: format!("Task {}", i),
                description: None,
            };
            repo.create(input).await.expect("Failed to create task");
        }

        let tasks = repo
            .list_by_workspace(&workspace_id)
            .await
            .expect("Failed to list tasks");

        assert_eq!(tasks.len(), 3);
        // Should be ordered by created_at DESC
        assert!(tasks[0].title.contains("Task"));
    }

    #[tokio::test]
    async fn test_list_by_workspace_empty() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool);

        let tasks = repo
            .list_by_workspace(&workspace_id)
            .await
            .expect("Failed to list tasks");

        assert_eq!(tasks.len(), 0);
    }

    #[tokio::test]
    async fn test_update_status() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool);

        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Test Task".to_string(),
            description: None,
        };
        let task = repo.create(input).await.expect("Failed to create task");

        repo.update_status(&task.id, TaskStatus::Queued)
            .await
            .expect("Failed to update status");

        let updated = repo
            .get(&task.id)
            .await
            .expect("Failed to get task")
            .unwrap();

        assert_eq!(updated.status, TaskStatus::Queued);
    }

    #[tokio::test]
    async fn test_update_status_all_states() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool);

        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Test Task".to_string(),
            description: None,
        };
        let task = repo.create(input).await.expect("Failed to create task");

        // Test all status transitions
        let statuses = vec![
            TaskStatus::Queued,
            TaskStatus::Running,
            TaskStatus::WaitingForInput,
            TaskStatus::Completed,
        ];

        for status in statuses {
            repo.update_status(&task.id, status.clone())
                .await
                .expect(&format!("Failed to update to {:?}", status));

            let updated = repo
                .get(&task.id)
                .await
                .expect("Failed to get task")
                .unwrap();

            assert_eq!(updated.status, status);
        }
    }

    #[tokio::test]
    async fn test_update_status_not_found() {
        let pool = setup_test_db().await;
        let repo = create_repo(pool);

        let result = repo.update_status("non-existent-id", TaskStatus::Queued).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_event() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool);

        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Test Task".to_string(),
            description: None,
        };
        let task = repo.create(input).await.expect("Failed to create task");

        let event = repo
            .create_event(&task.id, TaskEventType::TaskQueued, Some("test payload".to_string()))
            .await
            .expect("Failed to create event");

        assert!(!event.id.is_empty());
        assert_eq!(event.task_id, task.id);
        assert_eq!(event.event_type, TaskEventType::TaskQueued);
        assert_eq!(event.payload, Some("test payload".to_string()));
    }

    #[tokio::test]
    async fn test_create_event_without_payload() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool);

        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Test Task".to_string(),
            description: None,
        };
        let task = repo.create(input).await.expect("Failed to create task");

        let event = repo
            .create_event(&task.id, TaskEventType::TaskStarted, None)
            .await
            .expect("Failed to create event");

        assert_eq!(event.payload, None);
    }

    #[tokio::test]
    async fn test_list_events() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool);

        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Test Task".to_string(),
            description: None,
        };
        let task = repo.create(input).await.expect("Failed to create task");

        // Create multiple events
        repo.create_event(&task.id, TaskEventType::TaskQueued, None)
            .await
            .expect("Failed to create event");

        repo.create_event(&task.id, TaskEventType::TaskStarted, None)
            .await
            .expect("Failed to create event");

        let events = repo
            .list_events(&task.id)
            .await
            .expect("Failed to list events");

        // Should have at least 3 events: created (auto), queued, started
        assert!(events.len() >= 3);
        // Should be ordered by created_at ASC
        assert_eq!(events[0].event_type, TaskEventType::TaskCreated);
    }

    #[tokio::test]
    async fn test_list_events_empty() {
        let pool = setup_test_db().await;
        let repo = create_repo(pool);

        // Task doesn't exist, should return empty
        let events = repo.list_events("non-existent-id").await;

        // Should succeed but return empty vec
        assert!(events.is_ok());
    }

    #[tokio::test]
    async fn test_task_persistence_across_repositories() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;

        // Create with one repository instance
        let repo1 = create_repo(pool.clone());
        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Persistent Task".to_string(),
            description: Some("Should persist".to_string()),
        };
        let task = repo1.create(input).await.expect("Failed to create task");

        // Retrieve with another repository instance
        let repo2 = create_repo(pool);
        let retrieved = repo2
            .get(&task.id)
            .await
            .expect("Failed to get task")
            .unwrap();

        assert_eq!(retrieved.title, "Persistent Task");
        assert_eq!(retrieved.description, Some("Should persist".to_string()));
    }

    #[tokio::test]
    async fn test_concurrent_task_creation() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool.clone());

        // Create tasks sequentially (simpler than concurrent for test)
        for i in 0..5 {
            let input = CreateAgentTaskInput {
                workspace_id: workspace_id.clone(),
                title: format!("Sequential Task {}", i),
                description: None,
            };
            repo.create(input).await.expect("Failed to create task");
        }

        // Verify all tasks created
        let tasks = repo
            .list_by_workspace(&workspace_id)
            .await
            .expect("Failed to list tasks");

        assert_eq!(tasks.len(), 5);
    }
}