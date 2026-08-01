// Tests for agent service.

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;

    use crate::database::repositories::agent_task_repository::AgentTaskRepository;
    use crate::error::AppError;
    use crate::services::agent_service::AgentService;
    use domain::task::{CreateAgentTaskInput, TaskStatus};

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

    fn create_service(pool: SqlitePool) -> AgentService {
        let repo = AgentTaskRepository::new(pool);
        AgentService::new(repo)
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
    async fn test_create_task_success() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let service = create_service(pool);

        let input = CreateAgentTaskInput {
            workspace_id: workspace_id.clone(),
            title: "Analyze Tech Stocks".to_string(),
            description: Some("Research and analyze top tech stocks".to_string()),
        };

        let task = service
            .create_task(input)
            .await
            .expect("Failed to create task");

        assert!(!task.id.is_empty());
        assert_eq!(task.workspace_id, workspace_id);
        assert_eq!(task.title, "Analyze Tech Stocks");
        assert_eq!(task.status, TaskStatus::Created);
    }

    #[tokio::test]
    async fn test_create_task_empty_title() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let service = create_service(pool);

        let input = CreateAgentTaskInput {
            workspace_id,
            title: "".to_string(),
            description: None,
        };

        let result = service.create_task(input).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Validation(msg) => assert!(msg.contains("cannot be empty")),
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_create_task_whitespace_title() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let service = create_service(pool);

        let input = CreateAgentTaskInput {
            workspace_id,
            title: "   ".to_string(),
            description: None,
        };

        let result = service.create_task(input).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_task_too_long_title() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let service = create_service(pool);

        let long_title = "a".repeat(201);
        let input = CreateAgentTaskInput {
            workspace_id,
            title: long_title,
            description: None,
        };

        let result = service.create_task(input).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Validation(msg) => assert!(msg.contains("200 characters")),
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_queue_task_success() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let service = create_service(pool);

        // Create task first
        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Test Task".to_string(),
            description: None,
        };
        let task = service
            .create_task(input)
            .await
            .expect("Failed to create task");

        // Queue the task
        let queued_task = service
            .queue_task(&task.id)
            .await
            .expect("Failed to queue task");

        assert_eq!(queued_task.status, TaskStatus::Queued);
        assert_eq!(queued_task.id, task.id);
    }

    #[tokio::test]
    async fn test_queue_task_not_found() {
        let pool = setup_test_db().await;
        let service = create_service(pool);

        let result = service.queue_task("non-existent-id").await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::NotFound(msg) => assert!(msg.contains("not found")),
            _ => panic!("Expected not found error"),
        }
    }

    #[tokio::test]
    async fn test_queue_task_invalid_state() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let service = create_service(pool);

        // Create and queue task
        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Test Task".to_string(),
            description: None,
        };
        let task = service
            .create_task(input)
            .await
            .expect("Failed to create task");
        service
            .queue_task(&task.id)
            .await
            .expect("Failed to queue task");

        // Try to queue again (should fail)
        let result = service.queue_task(&task.id).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Validation(msg) => assert!(msg.contains("Cannot queue task")),
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_start_task_success() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let service = create_service(pool);

        // Create and queue task
        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Test Task".to_string(),
            description: None,
        };
        let task = service
            .create_task(input)
            .await
            .expect("Failed to create task");
        service
            .queue_task(&task.id)
            .await
            .expect("Failed to queue task");

        // Start the task
        let started_task = service
            .start_task(&task.id)
            .await
            .expect("Failed to start task");

        assert_eq!(started_task.status, TaskStatus::Running);
    }

    #[tokio::test]
    async fn test_start_task_invalid_state() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let service = create_service(pool);

        // Create task (not queued)
        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Test Task".to_string(),
            description: None,
        };
        let task = service
            .create_task(input)
            .await
            .expect("Failed to create task");

        // Try to start without queuing
        let result = service.start_task(&task.id).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Validation(msg) => assert!(msg.contains("Cannot start task")),
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_complete_task_success() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let service = create_service(pool);

        // Create, queue, and start task
        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Test Task".to_string(),
            description: None,
        };
        let task = service
            .create_task(input)
            .await
            .expect("Failed to create task");
        service
            .queue_task(&task.id)
            .await
            .expect("Failed to queue task");
        service
            .start_task(&task.id)
            .await
            .expect("Failed to start task");

        // Complete the task
        let completed_task = service
            .complete_task(&task.id)
            .await
            .expect("Failed to complete task");

        assert_eq!(completed_task.status, TaskStatus::Completed);
    }

    #[tokio::test]
    async fn test_complete_task_invalid_state() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let service = create_service(pool);

        // Create task (not running)
        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Test Task".to_string(),
            description: None,
        };
        let task = service
            .create_task(input)
            .await
            .expect("Failed to create task");

        // Try to complete without running
        let result = service.complete_task(&task.id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fail_task_success() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let service = create_service(pool);

        // Create and queue task
        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Test Task".to_string(),
            description: None,
        };
        let task = service
            .create_task(input)
            .await
            .expect("Failed to create task");

        // Fail the task
        let failed_task = service
            .fail_task(&task.id, "Something went wrong".to_string())
            .await
            .expect("Failed to fail task");

        assert_eq!(failed_task.status, TaskStatus::Failed);
    }

    #[tokio::test]
    async fn test_cancel_task_success() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let service = create_service(pool);

        // Create and queue task
        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Test Task".to_string(),
            description: None,
        };
        let task = service
            .create_task(input)
            .await
            .expect("Failed to create task");
        service
            .queue_task(&task.id)
            .await
            .expect("Failed to queue task");

        // Cancel the task
        let cancelled_task = service
            .cancel_task(&task.id)
            .await
            .expect("Failed to cancel task");

        assert_eq!(cancelled_task.status, TaskStatus::Cancelled);
    }

    #[tokio::test]
    async fn test_cancel_task_not_cancellable() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let service = create_service(pool);

        // Create, queue, and complete task
        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Test Task".to_string(),
            description: None,
        };
        let task = service
            .create_task(input)
            .await
            .expect("Failed to create task");
        service
            .queue_task(&task.id)
            .await
            .expect("Failed to queue task");
        service
            .start_task(&task.id)
            .await
            .expect("Failed to start task");
        service
            .complete_task(&task.id)
            .await
            .expect("Failed to complete task");

        // Try to cancel completed task
        let result = service.cancel_task(&task.id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_task_success() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let service = create_service(pool);

        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Test Task".to_string(),
            description: Some("Test description".to_string()),
        };
        let created_task = service
            .create_task(input)
            .await
            .expect("Failed to create task");

        let retrieved_task = service
            .get_task(&created_task.id)
            .await
            .expect("Failed to get task");

        assert!(retrieved_task.is_some());
        let task = retrieved_task.unwrap();
        assert_eq!(task.id, created_task.id);
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.description, Some("Test description".to_string()));
    }

    #[tokio::test]
    async fn test_get_task_not_found() {
        let pool = setup_test_db().await;
        let service = create_service(pool);

        let result = service
            .get_task("non-existent-id")
            .await
            .expect("Failed to get task");

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_list_tasks_success() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let service = create_service(pool);

        // Create multiple tasks
        for i in 1..=3 {
            let input = CreateAgentTaskInput {
                workspace_id: workspace_id.clone(),
                title: format!("Task {}", i),
                description: None,
            };
            service
                .create_task(input)
                .await
                .expect("Failed to create task");
        }

        let tasks = service
            .list_tasks(&workspace_id)
            .await
            .expect("Failed to list tasks");

        assert_eq!(tasks.len(), 3);
    }

    #[tokio::test]
    async fn test_list_tasks_empty() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let service = create_service(pool);

        let tasks = service
            .list_tasks(&workspace_id)
            .await
            .expect("Failed to list tasks");

        assert_eq!(tasks.len(), 0);
    }

    #[tokio::test]
    async fn test_get_task_events_success() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let service = create_service(pool);

        // Create task
        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Test Task".to_string(),
            description: None,
        };
        let task = service
            .create_task(input)
            .await
            .expect("Failed to create task");

        // Queue and start to create events
        service
            .queue_task(&task.id)
            .await
            .expect("Failed to queue task");
        service
            .start_task(&task.id)
            .await
            .expect("Failed to start task");

        // Get events
        let events = service
            .get_task_events(&task.id)
            .await
            .expect("Failed to get events");

        // Should have at least 3 events: created, queued, started
        assert!(events.len() >= 3);
    }

    #[tokio::test]
    async fn test_full_task_lifecycle() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let service = create_service(pool);

        // Create
        let input = CreateAgentTaskInput {
            workspace_id,
            title: "Full Lifecycle Task".to_string(),
            description: Some("Testing complete lifecycle".to_string()),
        };
        let task = service
            .create_task(input)
            .await
            .expect("Failed to create task");
        assert_eq!(task.status, TaskStatus::Created);

        // Queue
        let task = service
            .queue_task(&task.id)
            .await
            .expect("Failed to queue task");
        assert_eq!(task.status, TaskStatus::Queued);

        // Start
        let task = service
            .start_task(&task.id)
            .await
            .expect("Failed to start task");
        assert_eq!(task.status, TaskStatus::Running);

        // Complete
        let task = service
            .complete_task(&task.id)
            .await
            .expect("Failed to complete task");
        assert_eq!(task.status, TaskStatus::Completed);

        // Verify events
        let events = service
            .get_task_events(&task.id)
            .await
            .expect("Failed to get events");
        assert!(events.len() >= 4); // created, queued, started, completed
    }
}
