// Tests for workspace service.

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;

    use crate::database::repositories::workspace_repository::WorkspaceRepository;
    use crate::error::AppError;
    use crate::services::workspace_service::WorkspaceService;
    use domain::workspace::CreateWorkspaceInput;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("Failed to create test database");

        // Run migrations
        sqlx::query(include_str!("../../migrations/0001_initial.sql"))
            .execute(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    fn create_service(pool: SqlitePool) -> WorkspaceService {
        let repo = WorkspaceRepository::new(pool);
        WorkspaceService::new(repo)
    }

    #[tokio::test]
    async fn test_create_workspace_success() {
        let pool = setup_test_db().await;
        let service = create_service(pool);

        let input = CreateWorkspaceInput {
            name: "My Research Workspace".to_string(),
        };

        let workspace = service
            .create(input)
            .await
            .expect("Failed to create workspace");

        assert!(!workspace.id.is_empty());
        assert_eq!(workspace.name, "My Research Workspace");
    }

    #[tokio::test]
    async fn test_create_workspace_empty_name() {
        let pool = setup_test_db().await;
        let service = create_service(pool);

        let input = CreateWorkspaceInput {
            name: "".to_string(),
        };

        let result = service.create(input).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Validation(msg) => assert!(msg.contains("cannot be empty")),
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_create_workspace_whitespace_only() {
        let pool = setup_test_db().await;
        let service = create_service(pool);

        let input = CreateWorkspaceInput {
            name: "   ".to_string(),
        };

        let result = service.create(input).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_workspace_too_long() {
        let pool = setup_test_db().await;
        let service = create_service(pool);

        let long_name = "a".repeat(201);
        let input = CreateWorkspaceInput { name: long_name };

        let result = service.create(input).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Validation(msg) => assert!(msg.contains("200 characters")),
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_list_workspaces_empty() {
        let pool = setup_test_db().await;
        let service = create_service(pool);

        let workspaces = service.list().await.expect("Failed to list workspaces");

        assert_eq!(workspaces.len(), 0);
    }

    #[tokio::test]
    async fn test_list_workspaces_with_data() {
        let pool = setup_test_db().await;
        let service = create_service(pool);

        service
            .create(CreateWorkspaceInput {
                name: "Workspace 1".to_string(),
            })
            .await
            .unwrap();
        service
            .create(CreateWorkspaceInput {
                name: "Workspace 2".to_string(),
            })
            .await
            .unwrap();

        let workspaces = service.list().await.expect("Failed to list workspaces");

        assert_eq!(workspaces.len(), 2);
    }

    #[tokio::test]
    async fn test_get_workspace_success() {
        let pool = setup_test_db().await;
        let service = create_service(pool);

        let created = service
            .create(CreateWorkspaceInput {
                name: "Test".to_string(),
            })
            .await
            .unwrap();

        let retrieved = service
            .get(&created.id)
            .await
            .expect("Failed to get workspace");

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test");
    }

    #[tokio::test]
    async fn test_get_workspace_not_found() {
        let pool = setup_test_db().await;
        let service = create_service(pool);

        let result = service
            .get("non-existent-id")
            .await
            .expect("Failed to get workspace");

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_get_workspace_empty_id() {
        let pool = setup_test_db().await;
        let service = create_service(pool);

        let result = service.get("").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_workspace_success() {
        let pool = setup_test_db().await;
        let service = create_service(pool);

        let created = service
            .create(CreateWorkspaceInput {
                name: "Test".to_string(),
            })
            .await
            .unwrap();

        service
            .delete(&created.id)
            .await
            .expect("Failed to delete workspace");

        let retrieved = service.get(&created.id).await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_delete_workspace_not_found() {
        let pool = setup_test_db().await;
        let service = create_service(pool);

        let result = service.delete("non-existent-id").await;

        assert!(result.is_err());
    }
}
