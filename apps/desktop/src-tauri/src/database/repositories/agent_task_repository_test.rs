// Tests for agent task repository.

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use crate::database::repositories::agent_task_repository::AgentTaskRepository;
    use domain::task::{CreateAgentTaskInput, TaskEventType, TaskStatus};

    async fn setup_test_db() -> SqlitePool {
        crate::test_support::test_pool_with_workspace().await
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
        assert!(created.output.is_none());
        assert!(created.error.is_none());

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

        // The repository records only the events the caller asks for; the
        // service is responsible for the initial TaskCreated event.
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type, TaskEventType::TaskQueued);
        assert_eq!(events[1].event_type, TaskEventType::TaskProgress);
    }

    #[tokio::test]
    async fn test_unknown_event_type_returns_internal_error() {
        let pool = setup_test_db().await;
        let repo = AgentTaskRepository::new(pool.clone());
        let created = repo
            .create(create_input())
            .await
            .expect("Failed to create task");

        sqlx::query(
            "INSERT INTO agent_task_events (id, task_id, event_type, created_at)
             VALUES ('e1', ?, 'mystery', datetime('now'))",
        )
        .bind(&created.id)
        .execute(&pool)
        .await
        .expect("Failed to insert event");

        let result = repo.list_events(&created.id).await;

        assert!(matches!(result, Err(crate::error::AppError::Internal(_))));
    }

    #[tokio::test]
    async fn test_update_missing_task_returns_not_found() {
        let pool = setup_test_db().await;
        let repo = AgentTaskRepository::new(pool);

        let result = repo.update_status("missing", TaskStatus::Queued).await;

        assert!(matches!(result, Err(crate::error::AppError::NotFound(_))));
    }

    #[tokio::test]
    async fn test_complete_persists_output() {
        let pool = setup_test_db().await;
        let repo = AgentTaskRepository::new(pool);

        let created = repo
            .create(create_input())
            .await
            .expect("Failed to create task");

        repo.complete(&created.id, Some(r#"{"summary":"ok"}"#))
            .await
            .expect("Failed to complete task");

        let task = repo
            .get(&created.id)
            .await
            .expect("Failed to get task")
            .expect("Task should exist");

        assert_eq!(task.status, TaskStatus::Completed);
        assert_eq!(task.output.as_deref(), Some(r#"{"summary":"ok"}"#));
        assert!(task.error.is_none());
    }

    #[tokio::test]
    async fn test_fail_persists_error() {
        let pool = setup_test_db().await;
        let repo = AgentTaskRepository::new(pool);

        let created = repo
            .create(create_input())
            .await
            .expect("Failed to create task");

        repo.fail(&created.id, "provider unavailable")
            .await
            .expect("Failed to fail task");

        let task = repo
            .get(&created.id)
            .await
            .expect("Failed to get task")
            .expect("Task should exist");

        assert_eq!(task.status, TaskStatus::Failed);
        assert_eq!(task.error.as_deref(), Some("provider unavailable"));
        assert!(task.output.is_none());
    }

    #[tokio::test]
    async fn test_mark_cancelled() {
        let pool = setup_test_db().await;
        let repo = AgentTaskRepository::new(pool);

        let created = repo
            .create(create_input())
            .await
            .expect("Failed to create task");

        repo.mark_cancelled(&created.id)
            .await
            .expect("Failed to cancel task");

        let task = repo
            .get(&created.id)
            .await
            .expect("Failed to get task")
            .expect("Task should exist");

        assert_eq!(task.status, TaskStatus::Cancelled);
        assert!(task.error.is_none());
    }

    #[tokio::test]
    async fn test_fail_running_tasks_marks_only_running_tasks() {
        let pool = setup_test_db().await;
        let repo = AgentTaskRepository::new(pool);

        let running = repo
            .create(create_input())
            .await
            .expect("Failed to create running task");
        let queued = repo
            .create(create_input())
            .await
            .expect("Failed to create queued task");
        repo.update_status(&running.id, TaskStatus::Running)
            .await
            .expect("Failed to mark task running");
        repo.update_status(&queued.id, TaskStatus::Queued)
            .await
            .expect("Failed to queue task");

        let ids = repo
            .fail_running_tasks("interrupted")
            .await
            .expect("Failed to recover tasks");

        assert_eq!(ids, vec![running.id.clone()]);

        let failed = repo
            .get(&running.id)
            .await
            .expect("Failed to get task")
            .expect("Task should exist");
        assert_eq!(failed.status, TaskStatus::Failed);
        assert_eq!(failed.error.as_deref(), Some("interrupted"));

        let untouched = repo
            .get(&queued.id)
            .await
            .expect("Failed to get task")
            .expect("Task should exist");
        assert_eq!(untouched.status, TaskStatus::Queued);
    }

    #[tokio::test]
    async fn test_requeue_clears_result() {
        let pool = setup_test_db().await;
        let repo = AgentTaskRepository::new(pool);

        let created = repo
            .create(create_input())
            .await
            .expect("Failed to create task");
        repo.fail(&created.id, "boom")
            .await
            .expect("Failed to fail task");

        repo.requeue(&created.id)
            .await
            .expect("Failed to requeue task");

        let task = repo
            .get(&created.id)
            .await
            .expect("Failed to get task")
            .expect("Task should exist");
        assert_eq!(task.status, TaskStatus::Queued);
        assert!(task.error.is_none());
        assert!(task.output.is_none());
    }

    #[tokio::test]
    async fn test_requeue_rejects_task_without_result() {
        let pool = setup_test_db().await;
        let repo = AgentTaskRepository::new(pool);

        let created = repo
            .create(create_input())
            .await
            .expect("Failed to create task");

        let result = repo.requeue(&created.id).await;

        assert!(matches!(result, Err(crate::error::AppError::Validation(_))));
    }

    #[tokio::test]
    async fn test_search_matches_title_and_description() {
        let pool = setup_test_db().await;
        let repo = AgentTaskRepository::new(pool);

        let tesla = repo
            .create(CreateAgentTaskInput {
                workspace_id: "w1".to_string(),
                title: "Analyze Tesla".to_string(),
                description: Some("Analyze Q4 earnings".to_string()),
            })
            .await
            .expect("Failed to create task");
        repo.create(CreateAgentTaskInput {
            workspace_id: "w1".to_string(),
            title: "Analyze Nvidia".to_string(),
            description: None,
        })
        .await
        .expect("Failed to create task");

        let results = repo
            .search("w1", "Tesla", 10)
            .await
            .expect("Failed to search tasks");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, tesla.id);

        let by_description = repo
            .search("w1", "Q4 earnings", 10)
            .await
            .expect("Failed to search tasks");
        assert_eq!(by_description.len(), 1);
        assert_eq!(by_description[0].id, tesla.id);
    }
}
