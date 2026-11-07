// Tests for agent context assembly.

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use sqlx::sqlite::SqlitePoolOptions;

    use crate::agent::context::{
        EmptyContextProvider, TaskContextProvider, WorkspaceContextProvider,
    };
    use crate::database::repositories::agent_task_repository::AgentTaskRepository;
    use crate::database::repositories::research_document_repository::{
        NewResearchDocument, ResearchDocumentRepository,
    };
    use crate::database::repositories::workspace_repository::WorkspaceRepository;
    use domain::research::{ResearchDocumentFormat, ResearchDocumentKind};
    use domain::task::{AgentTask, CreateAgentTaskInput, TaskStatus};

    async fn setup() -> (
        WorkspaceContextProvider,
        AgentTaskRepository,
        ResearchDocumentRepository,
    ) {
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
        let research = ResearchDocumentRepository::new(pool.clone());
        let provider = WorkspaceContextProvider::new(
            WorkspaceRepository::new(pool),
            tasks.clone(),
            research.clone(),
        );

        (provider, tasks, research)
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

    async fn create_document(
        repo: &ResearchDocumentRepository,
        title: &str,
        content: &str,
    ) -> domain::research::ResearchDocument {
        let id = uuid::Uuid::new_v4().to_string();
        repo.create(NewResearchDocument {
            id: &id,
            workspace_id: "w1",
            title,
            content: Some(content),
            kind: ResearchDocumentKind::Note,
            format: ResearchDocumentFormat::Markdown,
            summary: None,
        })
        .await
        .expect("Failed to create document")
    }

    #[tokio::test]
    async fn test_build_includes_workspace_documents_and_completed_tasks() {
        let (provider, tasks, research) = setup().await;

        create_document(&research, "Tesla memo", "Strong demand").await;

        let completed = tasks
            .create(CreateAgentTaskInput {
                workspace_id: "w1".to_string(),
                title: "Tesla Q4".to_string(),
                description: None,
            })
            .await
            .expect("Failed to create task");
        tasks
            .complete(&completed.id, Some(r#"{"summary":"Beat estimates"}"#))
            .await
            .expect("Failed to complete task");

        // Unfinished tasks are not part of the context.
        tasks
            .create(CreateAgentTaskInput {
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
        assert_eq!(context.research_documents.len(), 1);
        assert_eq!(context.research_documents[0].title, "Tesla memo");
        assert_eq!(context.research_documents[0].summary, "Strong demand");
        assert_eq!(context.recent_tasks.len(), 1);
        assert_eq!(context.recent_tasks[0].title, "Tesla Q4");
        assert_eq!(
            context.recent_tasks[0].summary.as_deref(),
            Some("Beat estimates")
        );

        let rendered = context.render().expect("context should render");
        assert!(rendered.contains("Workspace ID: w1"));
        assert!(rendered.contains("Workspace: Alpha Research"));
        assert!(rendered.contains("Research documents in this workspace:"));
        assert!(rendered.contains("- Tesla memo — Strong demand"));
        assert!(rendered.contains("- Tesla Q4 — Beat estimates"));
    }

    #[tokio::test]
    async fn test_build_uses_raw_output_when_not_json() {
        let (provider, tasks, _research) = setup().await;

        let completed = tasks
            .create(CreateAgentTaskInput {
                workspace_id: "w1".to_string(),
                title: "Legacy task".to_string(),
                description: None,
            })
            .await
            .expect("Failed to create task");
        tasks
            .complete(&completed.id, Some("plain text output"))
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
        let (provider, tasks, research) = setup().await;
        let long_text = "x".repeat(1000);

        for index in 0..7 {
            create_document(&research, &format!("Doc {index}"), &long_text).await;
            let task = tasks
                .create(CreateAgentTaskInput {
                    workspace_id: "w1".to_string(),
                    title: format!("Task {index}"),
                    description: None,
                })
                .await
                .expect("Failed to create task");
            tasks
                .complete(&task.id, Some(&format!(r#"{{"summary":"{long_text}"}}"#)))
                .await
                .expect("Failed to complete task");
        }

        let context = provider
            .build(&current_task())
            .await
            .expect("context should build");

        assert_eq!(context.research_documents.len(), 5);
        assert_eq!(context.recent_tasks.len(), 5);

        let document_summary = &context.research_documents[0].summary;
        assert!(document_summary.chars().count() <= 400);
        assert!(document_summary.ends_with('…'));

        let task_summary = context.recent_tasks[0]
            .summary
            .as_deref()
            .expect("summary should exist");
        assert!(task_summary.chars().count() <= 400);
        assert!(task_summary.ends_with('…'));
    }

    #[tokio::test]
    async fn test_build_excludes_archived_documents() {
        let (provider, _tasks, research) = setup().await;

        let document = create_document(&research, "Archived memo", "Old notes").await;
        research
            .archive(&document.id)
            .await
            .expect("Failed to archive document");

        let context = provider
            .build(&current_task())
            .await
            .expect("context should build");

        assert!(context.research_documents.is_empty());
    }

    #[tokio::test]
    async fn test_empty_context_renders_only_workspace_id() {
        let context = EmptyContextProvider
            .build(&current_task())
            .await
            .expect("context should build");

        assert!(context.workspace_name.is_none());
        assert!(context.recent_tasks.is_empty());
        assert!(context.research_documents.is_empty());
        assert_eq!(context.render().as_deref(), Some("Workspace ID: w1"));
    }
}
