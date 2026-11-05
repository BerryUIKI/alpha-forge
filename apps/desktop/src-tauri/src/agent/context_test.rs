// Tests for agent context assembly.

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use sqlx::sqlite::SqlitePoolOptions;

    use crate::agent::context::{
        EmptyContextProvider, TaskContextProvider, WorkspaceContextProvider,
    };
    use crate::database::repositories::agent_task_repository::AgentTaskRepository;
    use crate::database::repositories::workspace_repository::WorkspaceRepository;
    use domain::task::{AgentTask, CreateAgentTaskInput, TaskStatus};

    async fn setup() -> (WorkspaceContextProvider, AgentTaskRepository) {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("Failed to create test database");

        crate::database::migrations::run(&pool)
            .await
            .expect("Failed to run migrations");

        sqlx::query("INSERT INTO workspaces (id, name) VALUES ('w1', 'Alpha Research')")
            .execute(&pool)
            .await
            .expect("Failed to insert workspace");

        let tasks = AgentTaskRepository::new(pool.clone());
        let provider = WorkspaceContextProvider::new(WorkspaceRepository::new(pool), tasks.clone());

        (provider, tasks)
    }

    fn current_task() -> AgentTask {
        let now = Utc::now();
        AgentTask {
            id: "current".to_string(),
            workspace_id: "w1".to_string(),
            title: "Analyze Nvidia".to_string(),
            description: None,
            status: TaskStatus::Running,
            output: None,
            error: None,
            created_at: now,
            updated_at: now,
        }
    }

    #[tokio::test]
    async fn test_build_includes_workspace_and_completed_task_summaries() {
        let (provider, repo) = setup().await;

        let completed = repo
            .create(CreateAgentTaskInput {
                workspace_id: "w1".to_string(),
                title: "Tesla Q4".to_string(),
                description: None,
            })
            .await
            .expect("Failed to create task");
        repo.complete(&completed.id, Some(r#"{"summary":"Strong demand"}"#))
            .await
            .expect("Failed to complete task");

        // Unfinished tasks are not part of the context.
        repo.create(CreateAgentTaskInput {
            workspace_id: "w1".to_string(),
            title: "Draft only".to_string(),
            description: None,
        })
        .await
        .expect("Failed to create task");

        let context = provider
            .build(&current_task())
            .await
            .expect("context should build");

        assert_eq!(context.workspace_id, "w1");
        assert_eq!(context.workspace_name.as_deref(), Some("Alpha Research"));
        assert_eq!(context.recent_tasks.len(), 1);
        assert_eq!(context.recent_tasks[0].title, "Tesla Q4");
        assert_eq!(
            context.recent_tasks[0].summary.as_deref(),
            Some("Strong demand")
        );

        let rendered = context.render().expect("context should render");
        assert!(rendered.contains("Workspace: Alpha Research"));
        assert!(rendered.contains("- Tesla Q4 — Strong demand"));
    }

    #[tokio::test]
    async fn test_build_uses_raw_output_when_not_json() {
        let (provider, repo) = setup().await;

        let completed = repo
            .create(CreateAgentTaskInput {
                workspace_id: "w1".to_string(),
                title: "Legacy task".to_string(),
                description: None,
            })
            .await
            .expect("Failed to create task");
        repo.complete(&completed.id, Some("plain text output"))
            .await
            .expect("Failed to complete task");

        let context = provider
            .build(&current_task())
            .await
            .expect("context should build");

        assert_eq!(
            context.recent_tasks[0].summary.as_deref(),
            Some("plain text output")
        );
    }

    #[tokio::test]
    async fn test_build_limits_and_truncates_summaries() {
        let (provider, repo) = setup().await;
        let long_summary = "x".repeat(1000);

        for index in 0..7 {
            let task = repo
                .create(CreateAgentTaskInput {
                    workspace_id: "w1".to_string(),
                    title: format!("Task {index}"),
                    description: None,
                })
                .await
                .expect("Failed to create task");
            repo.complete(
                &task.id,
                Some(&format!(r#"{{"summary":"{long_summary}"}}"#)),
            )
            .await
            .expect("Failed to complete task");
        }

        let context = provider
            .build(&current_task())
            .await
            .expect("context should build");

        assert_eq!(context.recent_tasks.len(), 5);
        let summary = context.recent_tasks[0]
            .summary
            .as_deref()
            .expect("summary should exist");
        assert!(summary.chars().count() <= 400);
        assert!(summary.ends_with('…'));
    }

    #[tokio::test]
    async fn test_empty_context_renders_nothing() {
        let context = EmptyContextProvider
            .build(&current_task())
            .await
            .expect("context should build");

        assert!(context.workspace_name.is_none());
        assert!(context.recent_tasks.is_empty());
        assert!(context.render().is_none());
    }
}
