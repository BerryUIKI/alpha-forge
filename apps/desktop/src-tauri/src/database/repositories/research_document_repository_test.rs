// Tests for the research document repository.

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use super::super::research_document_repository::ResearchDocumentRepository;

    async fn setup_repo() -> ResearchDocumentRepository {
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

        ResearchDocumentRepository::new(pool)
    }

    #[tokio::test]
    async fn test_create_and_get_document() {
        let repo = setup_repo().await;

        let created = repo
            .create("d1", "w1", "Tesla notes", Some("Strong demand"))
            .await
            .expect("document should be created");

        assert_eq!(created.title, "Tesla notes");
        assert_eq!(created.content.as_deref(), Some("Strong demand"));

        let fetched = repo
            .get("d1")
            .await
            .expect("document should load")
            .expect("document should exist");
        assert_eq!(fetched.workspace_id, "w1");

        assert!(repo
            .get("missing")
            .await
            .expect("lookup should succeed")
            .is_none());
    }

    #[tokio::test]
    async fn test_list_orders_newest_first() {
        let repo = setup_repo().await;

        repo.create("d1", "w1", "First", None)
            .await
            .expect("document should be created");
        repo.create("d2", "w1", "Second", None)
            .await
            .expect("document should be created");

        let documents = repo
            .list_by_workspace("w1")
            .await
            .expect("documents should list");

        assert_eq!(documents.len(), 2);
        assert_eq!(documents[0].title, "Second");
    }

    #[tokio::test]
    async fn test_update_document() {
        let repo = setup_repo().await;

        repo.create("d1", "w1", "Draft", None)
            .await
            .expect("document should be created");

        let updated = repo
            .update("d1", "Final", Some("Updated content"))
            .await
            .expect("document should update");

        assert_eq!(updated.title, "Final");
        assert_eq!(updated.content.as_deref(), Some("Updated content"));
    }

    #[tokio::test]
    async fn test_delete_document() {
        let repo = setup_repo().await;

        repo.create("d1", "w1", "Draft", None)
            .await
            .expect("document should be created");

        repo.delete("d1").await.expect("document should delete");

        assert!(repo
            .get("d1")
            .await
            .expect("lookup should succeed")
            .is_none());
        assert!(matches!(
            repo.delete("d1").await,
            Err(crate::error::AppError::NotFound(_))
        ));
    }

    #[tokio::test]
    async fn test_search_matches_title_and_content() {
        let repo = setup_repo().await;

        repo.create("d1", "w1", "Tesla notes", Some("Q4 earnings"))
            .await
            .expect("document should be created");
        repo.create("d2", "w1", "Nvidia notes", Some("Data center"))
            .await
            .expect("document should be created");

        let by_title = repo
            .search("w1", "Tesla", 10)
            .await
            .expect("search should succeed");
        assert_eq!(by_title.len(), 1);
        assert_eq!(by_title[0].id, "d1");

        let by_content = repo
            .search("w1", "Data center", 10)
            .await
            .expect("search should succeed");
        assert_eq!(by_content.len(), 1);
        assert_eq!(by_content[0].id, "d2");
    }
}
