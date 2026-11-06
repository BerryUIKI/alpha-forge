// Tests for the research document repository.

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use super::super::research_document_repository::{
        NewResearchDocument, ResearchDocumentRepository,
    };
    use domain::research::{ResearchDocument, ResearchDocumentFormat, ResearchDocumentKind};

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

    async fn create(
        repo: &ResearchDocumentRepository,
        id: &str,
        title: &str,
        content: Option<&str>,
    ) -> ResearchDocument {
        repo.create(NewResearchDocument {
            id,
            workspace_id: "w1",
            title,
            content,
            kind: ResearchDocumentKind::Note,
            format: ResearchDocumentFormat::Markdown,
            summary: None,
        })
        .await
        .expect("document should be created")
    }

    #[tokio::test]
    async fn test_create_and_get_document() {
        let repo = setup_repo().await;

        let created = create(&repo, "d1", "Tesla notes", Some("Strong demand")).await;

        assert_eq!(created.title, "Tesla notes");
        assert_eq!(created.content.as_deref(), Some("Strong demand"));
        assert_eq!(created.kind, ResearchDocumentKind::Note);
        assert_eq!(created.format, ResearchDocumentFormat::Markdown);
        assert!(created.summary.is_none());
        assert!(created.archived_at.is_none());

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

        create(&repo, "d1", "First", None).await;
        create(&repo, "d2", "Second", None).await;

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

        create(&repo, "d1", "Draft", None).await;

        let updated = repo
            .update("d1", "Final", Some("Updated content"))
            .await
            .expect("document should update");

        assert_eq!(updated.title, "Final");
        assert_eq!(updated.content.as_deref(), Some("Updated content"));
    }

    #[tokio::test]
    async fn test_archive_hides_document_from_lists_and_search() {
        let repo = setup_repo().await;

        create(&repo, "d1", "Tesla notes", Some("Q4 earnings")).await;

        repo.archive("d1").await.expect("document should archive");

        assert!(repo
            .list_by_workspace("w1")
            .await
            .expect("documents should list")
            .is_empty());
        assert!(repo
            .search("w1", "Tesla", 10)
            .await
            .expect("search should succeed")
            .is_empty());

        let archived = repo
            .get("d1")
            .await
            .expect("lookup should succeed")
            .expect("archived document should still exist");
        assert!(archived.archived_at.is_some());

        assert!(matches!(
            repo.update("d1", "New title", None).await,
            Err(crate::error::AppError::NotFound(_))
        ));
    }

    #[tokio::test]
    async fn test_purge_removes_document_and_children() {
        let repo = setup_repo().await;

        create(&repo, "d1", "Draft", None).await;

        sqlx::query(
            "INSERT INTO research_sources (id, document_id, url) VALUES ('s1', 'd1', 'https://example.com')",
        )
        .execute(repo.pool())
        .await
        .expect("Failed to insert source");
        sqlx::query(
            "INSERT INTO research_notes (id, document_id, content) VALUES ('n1', 'd1', 'note')",
        )
        .execute(repo.pool())
        .await
        .expect("Failed to insert note");

        repo.archive("d1").await.expect("document should archive");
        repo.purge("d1").await.expect("document should purge");

        assert!(repo
            .get("d1")
            .await
            .expect("lookup should succeed")
            .is_none());

        let sources: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM research_sources WHERE document_id = 'd1'")
                .fetch_one(repo.pool())
                .await
                .expect("Failed to count sources");
        let notes: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM research_notes WHERE document_id = 'd1'")
                .fetch_one(repo.pool())
                .await
                .expect("Failed to count notes");

        assert_eq!(sources, 0);
        assert_eq!(notes, 0);

        assert!(matches!(
            repo.purge("d1").await,
            Err(crate::error::AppError::NotFound(_))
        ));
    }

    #[tokio::test]
    async fn test_search_matches_title_and_content() {
        let repo = setup_repo().await;

        create(&repo, "d1", "Tesla notes", Some("Q4 earnings")).await;
        create(&repo, "d2", "Nvidia notes", Some("Data center")).await;

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
