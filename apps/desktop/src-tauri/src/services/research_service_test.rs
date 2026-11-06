// Tests for the research service.

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use crate::database::repositories::research_document_repository::ResearchDocumentRepository;
    use crate::error::AppError;
    use crate::services::research_service::ResearchService;
    use domain::research::{CreateResearchDocumentInput, UpdateResearchDocumentInput};

    async fn setup_service() -> ResearchService {
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

        ResearchService::new(ResearchDocumentRepository::new(pool))
    }

    fn create_input(title: &str, content: Option<&str>) -> CreateResearchDocumentInput {
        CreateResearchDocumentInput {
            workspace_id: "w1".to_string(),
            title: title.to_string(),
            content: content.map(str::to_string),
        }
    }

    #[tokio::test]
    async fn test_create_list_update_delete_document() {
        let service = setup_service().await;

        let created = service
            .create(create_input("Tesla notes", Some("  Strong demand  ")))
            .await
            .expect("document should be created");
        assert_eq!(created.title, "Tesla notes");
        assert_eq!(created.content.as_deref(), Some("Strong demand"));

        let documents = service.list("w1").await.expect("documents should list");
        assert_eq!(documents.len(), 1);

        let updated = service
            .update(
                &created.id,
                UpdateResearchDocumentInput {
                    title: "Tesla Q4".to_string(),
                    content: None,
                },
            )
            .await
            .expect("document should update");
        assert_eq!(updated.title, "Tesla Q4");
        assert!(updated.content.is_none());

        service
            .delete(&created.id)
            .await
            .expect("document should delete");
        assert!(service
            .get(&created.id)
            .await
            .expect("lookup should succeed")
            .is_none());
    }

    #[tokio::test]
    async fn test_empty_title_is_rejected() {
        let service = setup_service().await;

        let result = service.create(create_input("   ", None)).await;

        assert!(matches!(result, Err(AppError::Validation(_))));
    }

    #[tokio::test]
    async fn test_empty_content_is_stored_as_none() {
        let service = setup_service().await;

        let created = service
            .create(create_input("Notes", Some("   ")))
            .await
            .expect("document should be created");

        assert!(created.content.is_none());
    }

    #[tokio::test]
    async fn test_empty_workspace_is_rejected() {
        let service = setup_service().await;

        let mut input = create_input("Notes", None);
        input.workspace_id = "  ".to_string();

        assert!(matches!(
            service.create(input).await,
            Err(AppError::Validation(_))
        ));
    }

    #[tokio::test]
    async fn test_search_returns_matches() {
        let service = setup_service().await;

        service
            .create(create_input("Tesla notes", Some("Q4 earnings")))
            .await
            .expect("document should be created");
        service
            .create(create_input("Nvidia notes", None))
            .await
            .expect("document should be created");

        let results = service
            .search("w1", "Tesla", 10)
            .await
            .expect("search should succeed");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Tesla notes");
    }
}
