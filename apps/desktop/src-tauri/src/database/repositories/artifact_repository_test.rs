// Tests for artifact repository.

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;

    use crate::database::repositories::artifact_repository::ArtifactRepository;
    use domain::artifact::{ArtifactStatus, ArtifactType, CreateArtifactInput};

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("Failed to create test database");

        // Run migrations
        sqlx::query(include_str!("../../../migrations/0001_initial.sql"))
            .execute(&pool)
            .await
            .expect("Failed to run initial migration");

        sqlx::query(include_str!("../../../migrations/0002_agent_tasks_enhancement.sql"))
            .execute(&pool)
            .await
            .expect("Failed to run agent tasks migration");

        sqlx::query(include_str!("../../../migrations/0003_fix_status_constraint.sql"))
            .execute(&pool)
            .await
            .expect("Failed to run status constraint fix");

        sqlx::query(include_str!("../../../migrations/0004_enhance_artifacts.sql"))
            .execute(&pool)
            .await
            .expect("Failed to run artifacts enhancement migration");

        pool
    }

    fn create_repo(pool: SqlitePool) -> ArtifactRepository {
        ArtifactRepository::new(pool)
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

    async fn create_test_task(pool: &SqlitePool, workspace_id: &str) -> String {
        let task_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO agent_tasks (id, workspace_id, title, status, created_at, updated_at) VALUES (?, ?, 'Test Task', 'created', datetime('now'), datetime('now'))"
        )
        .bind(&task_id)
        .bind(workspace_id)
        .execute(pool)
        .await
        .expect("Failed to create test task");
        task_id
    }

    #[tokio::test]
    async fn test_create_artifact() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool);

        let input = CreateArtifactInput {
            workspace_id: workspace_id.clone(),
            task_id: None,
            artifact_type: ArtifactType::ComparisonTable,
            input: serde_json::json!({"test": "data"}),
        };

        let artifact = repo.create(input).await.expect("Failed to create artifact");

        assert!(!artifact.id.is_empty());
        assert_eq!(artifact.workspace_id, workspace_id);
        assert_eq!(artifact.artifact_type, ArtifactType::ComparisonTable);
        assert_eq!(artifact.status, ArtifactStatus::Pending);
        assert!(artifact.task_id.is_none());
    }

    #[tokio::test]
    async fn test_create_artifact_with_task() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let task_id = create_test_task(&pool, &workspace_id).await;
        let repo = create_repo(pool);

        let input = CreateArtifactInput {
            workspace_id: workspace_id.clone(),
            task_id: Some(task_id.clone()),
            artifact_type: ArtifactType::Timeline,
            input: serde_json::json!({"timeline": "test"}),
        };

        let artifact = repo.create(input).await.expect("Failed to create artifact");

        assert_eq!(artifact.task_id, Some(task_id));
        assert_eq!(artifact.artifact_type, ArtifactType::Timeline);
    }

    #[tokio::test]
    async fn test_get_artifact() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool);

        let input = CreateArtifactInput {
            workspace_id: workspace_id.clone(),
            task_id: None,
            artifact_type: ArtifactType::IndustryMap,
            input: serde_json::json!({}),
        };

        let created = repo.create(input).await.expect("Failed to create artifact");
        let fetched = repo.get(&created.id).await.expect("Failed to get artifact");

        assert!(fetched.is_some());
        let fetched = fetched.unwrap();
        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.artifact_type, ArtifactType::IndustryMap);
    }

    #[tokio::test]
    async fn test_get_nonexistent_artifact() {
        let pool = setup_test_db().await;
        let repo = create_repo(pool);

        let result = repo.get("nonexistent-id").await.expect("Failed to query");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_list_by_workspace() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool);

        // Create multiple artifacts
        for i in 0..3 {
            let input = CreateArtifactInput {
                workspace_id: workspace_id.clone(),
                task_id: None,
                artifact_type: ArtifactType::ValuationModel,
                input: serde_json::json!({"index": i}),
            };
            repo.create(input).await.expect("Failed to create artifact");
        }

        let artifacts = repo.list_by_workspace(&workspace_id).await.expect("Failed to list");
        assert_eq!(artifacts.len(), 3);
    }

    #[tokio::test]
    async fn test_list_by_task() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let task_id = create_test_task(&pool, &workspace_id).await;
        let repo = create_repo(pool);

        // Create artifacts for the task
        for i in 0..2 {
            let input = CreateArtifactInput {
                workspace_id: workspace_id.clone(),
                task_id: Some(task_id.clone()),
                artifact_type: ArtifactType::RiskDashboard,
                input: serde_json::json!({"index": i}),
            };
            repo.create(input).await.expect("Failed to create artifact");
        }

        let artifacts = repo.list_by_task(&task_id).await.expect("Failed to list");
        assert_eq!(artifacts.len(), 2);
    }

    #[tokio::test]
    async fn test_update_status() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool);

        let input = CreateArtifactInput {
            workspace_id: workspace_id.clone(),
            task_id: None,
            artifact_type: ArtifactType::ComparisonTable,
            input: serde_json::json!({}),
        };

        let artifact = repo.create(input).await.expect("Failed to create artifact");
        
        repo.update_status(&artifact.id, ArtifactStatus::Generating).await.expect("Failed to update status");
        
        let updated = repo.get(&artifact.id).await.expect("Failed to get").unwrap();
        assert_eq!(updated.status, ArtifactStatus::Generating);
    }

    #[tokio::test]
    async fn test_update_output() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool);

        let input = CreateArtifactInput {
            workspace_id: workspace_id.clone(),
            task_id: None,
            artifact_type: ArtifactType::ComparisonTable,
            input: serde_json::json!({}),
        };

        let artifact = repo.create(input).await.expect("Failed to create artifact");
        
        let output = serde_json::json!({"result": "success", "data": [1, 2, 3]});
        repo.update_output(&artifact.id, output.clone()).await.expect("Failed to update output");
        
        let updated = repo.get(&artifact.id).await.expect("Failed to get").unwrap();
        assert_eq!(updated.output, Some(output));
        assert_eq!(updated.status, ArtifactStatus::Completed);
    }

    #[tokio::test]
    async fn test_set_error() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool);

        let input = CreateArtifactInput {
            workspace_id: workspace_id.clone(),
            task_id: None,
            artifact_type: ArtifactType::ComparisonTable,
            input: serde_json::json!({}),
        };

        let artifact = repo.create(input).await.expect("Failed to create artifact");
        
        repo.set_error(&artifact.id, "Test error message").await.expect("Failed to set error");
        
        let updated = repo.get(&artifact.id).await.expect("Failed to get").unwrap();
        assert_eq!(updated.error, Some("Test error message".to_string()));
        assert_eq!(updated.status, ArtifactStatus::Failed);
    }

    #[tokio::test]
    async fn test_delete_artifact() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool);

        let input = CreateArtifactInput {
            workspace_id: workspace_id.clone(),
            task_id: None,
            artifact_type: ArtifactType::ComparisonTable,
            input: serde_json::json!({}),
        };

        let artifact = repo.create(input).await.expect("Failed to create artifact");
        
        repo.delete(&artifact.id).await.expect("Failed to delete");
        
        let deleted = repo.get(&artifact.id).await.expect("Failed to query");
        assert!(deleted.is_none());
    }

    #[tokio::test]
    async fn test_custom_artifact_type() {
        let pool = setup_test_db().await;
        let workspace_id = create_test_workspace(&pool).await;
        let repo = create_repo(pool);

        let custom_type = ArtifactType::Custom("custom_report".to_string());
        let input = CreateArtifactInput {
            workspace_id: workspace_id.clone(),
            task_id: None,
            artifact_type: custom_type.clone(),
            input: serde_json::json!({"custom": true}),
        };

        let artifact = repo.create(input).await.expect("Failed to create artifact");
        assert_eq!(artifact.artifact_type, custom_type);

        let fetched = repo.get(&artifact.id).await.expect("Failed to get").unwrap();
        assert_eq!(fetched.artifact_type, custom_type);
    }
}