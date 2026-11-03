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

        crate::database::migrations::run(&pool)
            .await
            .expect("Failed to run migrations");

        sqlx::query("INSERT INTO workspaces (id, name) VALUES ('w1', 'Workspace')")
            .execute(&pool)
            .await
            .expect("Failed to insert workspace");

        pool
    }

    fn create_input() -> CreateAgentTaskInput {
        CreateAgentTaskInput {
            workspace_id: "w1".to_string(),
            title: "Research Tesla".to_string(),
            description: Some("Analyze Q4 earnings".to_string()),
        }
    }

    #[tokio::test]
    async fn test_create_and_get_task() {
        let pool = setup_test_db().await;
        let repo = AgentTaskRepository::new(pool);

        let created = repo
            .create(create_input())
            .await
            .expect("Failed to create task");

        assert_eq!(created.workspace_id, "w1");
        assert_eq!(created.title, "Research Tesla");
        assert_eq!(created.description.as_deref(), Some("Analyze Q4 earnings"));
        assert_eq!(created.status, TaskStatus::Created);

        let fetched = repo
            .get(&created.id)
            .await
            .expect("Failed to get task")
            .expect("Task should exist");

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.status, TaskStatus::Created);
    }

    #[tokio::test]
    async fn test_list_tasks_by_workspace() {
        let pool = setup_test_db().await;
        let repo = AgentTaskRepository::new(pool);

        repo.create(create_input())
            .await
            .expect("Failed to create first task");
        repo.create(create_input())
            .await
            .expect("Failed to create second task");

        let tasks = repo
            .list_by_workspace("w1")
            .await
            .expect("Failed to list tasks");

        assert_eq!(tasks.len(), 2);
    }

    #[tokio::test]
    async fn test_update_task_status() {
        let pool = setup_test_db().await;
        let repo = AgentTaskRepository::new(pool);

        let created = repo
            .create(create_input())
            .await
            .expect("Failed to create task");

        repo.update_status(&created.id, TaskStatus::Queued)
            .await
            .expect("Failed to update status");

        let fetched = repo
            .get(&created.id)
            .await
            .expect("Failed to get task")
            .expect("Task should exist");

        assert_eq!(fetched.status, TaskStatus::Queued);
    }

    #[tokio::test]
    async fn test_task_events_are_persisted() {
        let pool = setup_test_db().await;
        let repo = AgentTaskRepository::new(pool);

        let created = repo
            .create(create_input())
            .await
            .expect("Failed to create task");

        repo.create_event(&created.id, TaskEventType::TaskQueued, None)
            .await
            .expect("Failed to create event");
        repo.create_event(
            &created.id,
            TaskEventType::TaskProgress,
            Some("Analyzing request...".to_string()),
        )
        .await
        .expect("Failed to create event");

        let events = repo
            .list_events(&created.id)
            .await
            .expect("Failed to list events");

        assert_eq!(events.len(), 3); // task_created + two explicit events
        assert_eq!(events[0].event_type, TaskEventType::TaskCreated);
        assert_eq!(events[1].event_type, TaskEventType::TaskQueued);
        assert_eq!(events[2].event_type, TaskEventType::TaskProgress);
    }

    #[tokio::test]
    async fn test_update_missing_task_returns_not_found() {
        let pool = setup_test_db().await;
        let repo = AgentTaskRepository::new(pool);

        let result = repo.update_status("missing", TaskStatus::Queued).await;

        assert!(matches!(result, Err(crate::error::AppError::NotFound(_))));
    }
}
