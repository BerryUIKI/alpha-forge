// Tests for the research service.

#[cfg(test)]
mod tests {
    use crate::database::repositories::research_document_repository::ResearchDocumentRepository;
    use crate::error::AppError;
    use crate::services::research_service::ResearchService;
    use domain::research::{
        CreateResearchDocumentInput, ResearchDocumentFormat, ResearchDocumentKind,
        UpdateResearchDocumentInput,
    };

    async fn setup_service() -> ResearchService {
        ResearchService::new(ResearchDocumentRepository::new(
            crate::test_support::test_pool_with_workspace().await,
        ))
    }

    fn create_input(title: &str, content: Option<&str>) -> CreateResearchDocumentInput {
        CreateResearchDocumentInput {
            workspace_id: "w1".to_string(),
            title: title.to_string(),
            content: content.map(str::to_string),
            kind: None,
            format: None,
            summary: None,
        }
    }

    #[tokio::test]
    async fn test_create_list_update_archive_purge_document() {
        let service = setup_service().await;

        let created = service
            .create(create_input("Tesla notes", Some("  Strong demand  ")))
            .await
            .expect("document should be created");
        assert_eq!(created.title, "Tesla notes");
        assert_eq!(created.content.as_deref(), Some("Strong demand"));
        assert_eq!(created.kind, ResearchDocumentKind::Note);
        assert_eq!(created.format, ResearchDocumentFormat::Markdown);

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
            .expect("document should archive");
        assert!(service
            .list("w1")
            .await
            .expect("documents should list")
            .is_empty());
        assert!(service
            .get(&created.id)
            .await
            .expect("lookup should succeed")
            .expect("document should exist")
            .archived_at
            .is_some());

        assert!(matches!(
            service
                .update(
                    &created.id,
                    UpdateResearchDocumentInput {
                        title: "Nope".to_string(),
                        content: None,
                    },
                )
                .await,
            Err(AppError::NotFound(_))
        ));

        service
            .purge(&created.id)
            .await
            .expect("archived document should purge");
        assert!(service
            .get(&created.id)
            .await
            .expect("lookup should succeed")
            .is_none());
    }

    #[tokio::test]
    async fn test_purge_requires_archive() {
        let service = setup_service().await;

        let created = service
            .create(create_input("Notes", None))
            .await
            .expect("document should be created");

        assert!(matches!(
            service.purge(&created.id).await,
            Err(AppError::Validation(_))
        ));
    }

    #[tokio::test]
    async fn test_create_with_metadata() {
        let service = setup_service().await;

        let mut input = create_input("PDF notes", Some("Body"));
        input.kind = Some(ResearchDocumentKind::Pdf);
        input.format = Some(ResearchDocumentFormat::Text);
        input.summary = Some("  Q4 extract  ".to_string());

        let created = service
            .create(input)
            .await
            .expect("document should be created");

        assert_eq!(created.kind, ResearchDocumentKind::Pdf);
        assert_eq!(created.format, ResearchDocumentFormat::Text);
        assert_eq!(created.summary.as_deref(), Some("Q4 extract"));
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
    async fn test_search_returns_matches_and_skips_archived() {
        let service = setup_service().await;

        service
            .create(create_input("Tesla notes", Some("Q4 earnings")))
            .await
            .expect("document should be created");
        let archived = service
            .create(create_input("Tesla archive", Some("Old Tesla data")))
            .await
            .expect("document should be created");
        service
            .create(create_input("Nvidia notes", None))
            .await
            .expect("document should be created");

        service
            .delete(&archived.id)
            .await
            .expect("document should archive");

        let results = service
            .search("w1", "Tesla", 10)
            .await
            .expect("search should succeed");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Tesla notes");
    }
}
