// Thesis repository tests.

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use sqlx::SqlitePool;

    use crate::database::repositories::thesis_repository::ThesisRepository;
    use crate::error::AppError;
    use domain::thesis::{
        AddEvidenceInput, CreateThesisInput, EvidenceDirection, ThesisStatus,
        UpdateConfidenceInput,
    };

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();

        // Create necessary tables
        sqlx::query(
            r#"
            CREATE TABLE workspaces (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE investment_theses (
                id TEXT PRIMARY KEY NOT NULL,
                workspace_id TEXT NOT NULL,
                title TEXT NOT NULL,
                thesis TEXT NOT NULL,
                confidence INTEGER NOT NULL DEFAULT 50,
                status TEXT NOT NULL DEFAULT 'draft',
                validation_date TEXT,
                outcome TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (workspace_id) REFERENCES workspaces(id) ON DELETE CASCADE
            );

            CREATE TABLE thesis_evidence (
                id TEXT PRIMARY KEY NOT NULL,
                thesis_id TEXT NOT NULL,
                direction TEXT NOT NULL,
                evidence TEXT NOT NULL,
                source_id TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (thesis_id) REFERENCES investment_theses(id) ON DELETE CASCADE
            );

            CREATE TABLE research_sources (
                id TEXT PRIMARY KEY NOT NULL,
                document_id TEXT NOT NULL,
                url TEXT,
                title TEXT,
                retrieved_at TEXT,
                created_at TEXT NOT NULL
            );
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        // Create test workspace
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO workspaces (id, name, created_at, updated_at) VALUES ('test-workspace', 'Test Workspace', ?, ?)",
        )
        .bind(&now)
        .bind(&now)
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_create_thesis() {
        let pool = setup_test_db().await;
        let repo = ThesisRepository::new(pool);

        let input = CreateThesisInput {
            workspace_id: "test-workspace".to_string(),
            title: "NVIDIA Growth Thesis".to_string(),
            thesis: "NVIDIA will continue to dominate AI chip market".to_string(),
            confidence: Some(75),
        };

        let thesis = repo.create_thesis(input).await.unwrap();

        assert_eq!(thesis.workspace_id, "test-workspace");
        assert_eq!(thesis.title, "NVIDIA Growth Thesis");
        assert_eq!(thesis.thesis, "NVIDIA will continue to dominate AI chip market");
        assert_eq!(thesis.confidence, 75);
        assert_eq!(thesis.status, ThesisStatus::Draft);
    }

    #[tokio::test]
    async fn test_create_thesis_default_confidence() {
        let pool = setup_test_db().await;
        let repo = ThesisRepository::new(pool);

        let input = CreateThesisInput {
            workspace_id: "test-workspace".to_string(),
            title: "Test Thesis".to_string(),
            thesis: "Test content".to_string(),
            confidence: None,
        };

        let thesis = repo.create_thesis(input).await.unwrap();
        assert_eq!(thesis.confidence, 50); // Default confidence
    }

    #[tokio::test]
    async fn test_get_thesis() {
        let pool = setup_test_db().await;
        let repo = ThesisRepository::new(pool);

        let created = repo
            .create_thesis(CreateThesisInput {
                workspace_id: "test-workspace".to_string(),
                title: "Test".to_string(),
                thesis: "Content".to_string(),
                confidence: Some(60),
            })
            .await
            .unwrap();

        let fetched = repo.get_thesis(&created.id).await.unwrap().unwrap();
        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.title, "Test");
    }

    #[tokio::test]
    async fn test_get_thesis_not_found() {
        let pool = setup_test_db().await;
        let repo = ThesisRepository::new(pool);

        let result = repo.get_thesis("nonexistent").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_list_by_workspace() {
        let pool = setup_test_db().await;
        let repo = ThesisRepository::new(pool);

        // Create multiple theses
        repo.create_thesis(CreateThesisInput {
            workspace_id: "test-workspace".to_string(),
            title: "Thesis 1".to_string(),
            thesis: "Content 1".to_string(),
            confidence: None,
        })
        .await
        .unwrap();

        repo.create_thesis(CreateThesisInput {
            workspace_id: "test-workspace".to_string(),
            title: "Thesis 2".to_string(),
            thesis: "Content 2".to_string(),
            confidence: None,
        })
        .await
        .unwrap();

        let theses = repo.list_by_workspace("test-workspace").await.unwrap();
        assert_eq!(theses.len(), 2);
    }

    #[tokio::test]
    async fn test_update_status() {
        let pool = setup_test_db().await;
        let repo = ThesisRepository::new(pool);

        let thesis = repo
            .create_thesis(CreateThesisInput {
                workspace_id: "test-workspace".to_string(),
                title: "Test".to_string(),
                thesis: "Content".to_string(),
                confidence: None,
            })
            .await
            .unwrap();

        repo.update_status(&thesis.id, ThesisStatus::Active)
            .await
            .unwrap();

        let updated = repo.get_thesis(&thesis.id).await.unwrap().unwrap();
        assert_eq!(updated.status, ThesisStatus::Active);
    }

    #[tokio::test]
    async fn test_update_confidence() {
        let pool = setup_test_db().await;
        let repo = ThesisRepository::new(pool);

        let thesis = repo
            .create_thesis(CreateThesisInput {
                workspace_id: "test-workspace".to_string(),
                title: "Test".to_string(),
                thesis: "Content".to_string(),
                confidence: Some(50),
            })
            .await
            .unwrap();

        repo.update_confidence(UpdateConfidenceInput {
            thesis_id: thesis.id.clone(),
            confidence: 85,
        })
        .await
        .unwrap();

        let updated = repo.get_thesis(&thesis.id).await.unwrap().unwrap();
        assert_eq!(updated.confidence, 85);
    }

    #[tokio::test]
    async fn test_record_outcome() {
        let pool = setup_test_db().await;
        let repo = ThesisRepository::new(pool);

        let thesis = repo
            .create_thesis(CreateThesisInput {
                workspace_id: "test-workspace".to_string(),
                title: "Test".to_string(),
                thesis: "Content".to_string(),
                confidence: None,
            })
            .await
            .unwrap();

        repo.record_outcome(&thesis.id, "Thesis validated successfully".to_string(), ThesisStatus::Validated)
            .await
            .unwrap();

        let updated = repo.get_thesis(&thesis.id).await.unwrap().unwrap();
        assert_eq!(updated.outcome, Some("Thesis validated successfully".to_string()));
        assert_eq!(updated.status, ThesisStatus::Validated);
        assert!(updated.validation_date.is_some());
    }

    #[tokio::test]
    async fn test_delete_thesis() {
        let pool = setup_test_db().await;
        let repo = ThesisRepository::new(pool);

        let thesis = repo
            .create_thesis(CreateThesisInput {
                workspace_id: "test-workspace".to_string(),
                title: "Test".to_string(),
                thesis: "Content".to_string(),
                confidence: None,
            })
            .await
            .unwrap();

        repo.delete_thesis(&thesis.id).await.unwrap();

        let result = repo.get_thesis(&thesis.id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_add_evidence() {
        let pool = setup_test_db().await;
        let repo = ThesisRepository::new(pool);

        let thesis = repo
            .create_thesis(CreateThesisInput {
                workspace_id: "test-workspace".to_string(),
                title: "Test".to_string(),
                thesis: "Content".to_string(),
                confidence: None,
            })
            .await
            .unwrap();

        let evidence = repo
            .add_evidence(AddEvidenceInput {
                thesis_id: thesis.id.clone(),
                direction: EvidenceDirection::Supporting,
                evidence: "Strong revenue growth".to_string(),
                source_id: None,
            })
            .await
            .unwrap();

        assert_eq!(evidence.thesis_id, thesis.id);
        assert_eq!(evidence.direction, EvidenceDirection::Supporting);
        assert_eq!(evidence.evidence, "Strong revenue growth");
    }

    #[tokio::test]
    async fn test_list_evidence() {
        let pool = setup_test_db().await;
        let repo = ThesisRepository::new(pool);

        let thesis = repo
            .create_thesis(CreateThesisInput {
                workspace_id: "test-workspace".to_string(),
                title: "Test".to_string(),
                thesis: "Content".to_string(),
                confidence: None,
            })
            .await
            .unwrap();

        repo.add_evidence(AddEvidenceInput {
            thesis_id: thesis.id.clone(),
            direction: EvidenceDirection::Supporting,
            evidence: "Evidence 1".to_string(),
            source_id: None,
        })
        .await
        .unwrap();

        repo.add_evidence(AddEvidenceInput {
            thesis_id: thesis.id.clone(),
            direction: EvidenceDirection::Contradicting,
            evidence: "Evidence 2".to_string(),
            source_id: None,
        })
        .await
        .unwrap();

        let evidence = repo.list_evidence(&thesis.id).await.unwrap();
        assert_eq!(evidence.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_evidence() {
        let pool = setup_test_db().await;
        let repo = ThesisRepository::new(pool);

        let thesis = repo
            .create_thesis(CreateThesisInput {
                workspace_id: "test-workspace".to_string(),
                title: "Test".to_string(),
                thesis: "Content".to_string(),
                confidence: None,
            })
            .await
            .unwrap();

        let evidence = repo
            .add_evidence(AddEvidenceInput {
                thesis_id: thesis.id.clone(),
                direction: EvidenceDirection::Supporting,
                evidence: "Test".to_string(),
                source_id: None,
            })
            .await
            .unwrap();

        repo.delete_evidence(&evidence.id).await.unwrap();

        let remaining = repo.list_evidence(&thesis.id).await.unwrap();
        assert_eq!(remaining.len(), 0);
    }

    #[tokio::test]
    async fn test_confidence_bounds() {
        let pool = setup_test_db().await;
        let repo = ThesisRepository::new(pool);

        // Test upper bound
        let thesis_high = repo
            .create_thesis(CreateThesisInput {
                workspace_id: "test-workspace".to_string(),
                title: "High".to_string(),
                thesis: "Content".to_string(),
                confidence: Some(150), // Should be clamped to 100
            })
            .await
            .unwrap();
        assert_eq!(thesis_high.confidence, 100);

        // Test lower bound
        let thesis_low = repo
            .create_thesis(CreateThesisInput {
                workspace_id: "test-workspace".to_string(),
                title: "Low".to_string(),
                thesis: "Content".to_string(),
                confidence: Some(-10), // Should be clamped to 0
            })
            .await
            .unwrap();
        assert_eq!(thesis_low.confidence, 0);
    }
}