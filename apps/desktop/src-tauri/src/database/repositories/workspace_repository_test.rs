// Tests for workspace repository.

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;

    use crate::database::repositories::workspace_repository::WorkspaceRepository;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("Failed to create test database");

        // Run migrations
        sqlx::query(include_str!("../../../../migrations/0001_initial.sql"))
            .execute(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    #[tokio::test]
    async fn test_create_workspace() {
        let pool = setup_test_db().await;
        let repo = WorkspaceRepository::new(pool);

        let workspace = repo
            .create("test-uuid", "My Test Workspace")
            .await
            .expect("Failed to create workspace");

        assert_eq!(workspace.id, "test-uuid");
        assert_eq!(workspace.name, "My Test Workspace");
    }

    #[tokio::test]
    async fn test_list_workspaces_empty() {
        let pool = setup_test_db().await;
        let repo = WorkspaceRepository::new(pool);

        let workspaces = repo.list().await.expect("Failed to list workspaces");

        assert_eq!(workspaces.len(), 0);
    }

    #[tokio::test]
    async fn test_list_workspaces_with_data() {
        let pool = setup_test_db().await;
        let repo = WorkspaceRepository::new(pool);

        repo.create("uuid-1", "Workspace 1")
            .await
            .expect("Failed to create workspace 1");
        repo.create("uuid-2", "Workspace 2")
            .await
            .expect("Failed to create workspace 2");

        let workspaces = repo.list().await.expect("Failed to list workspaces");

        assert_eq!(workspaces.len(), 2);
    }

    #[tokio::test]
    async fn test_get_workspace_by_id() {
        let pool = setup_test_db().await;
        let repo = WorkspaceRepository::new(pool);

        repo.create("test-uuid", "Test Workspace")
            .await
            .expect("Failed to create workspace");

        let workspace = repo
            .get("test-uuid")
            .await
            .expect("Failed to get workspace");

        assert!(workspace.is_some());
        assert_eq!(workspace.unwrap().name, "Test Workspace");
    }

    #[tokio::test]
    async fn test_get_workspace_not_found() {
        let pool = setup_test_db().await;
        let repo = WorkspaceRepository::new(pool);

        let workspace = repo
            .get("non-existent")
            .await
            .expect("Failed to get workspace");

        assert!(workspace.is_none());
    }

    #[tokio::test]
    async fn test_update_workspace() {
        let pool = setup_test_db().await;
        let repo = WorkspaceRepository::new(pool);

        repo.create("test-uuid", "Original Name")
            .await
            .expect("Failed to create workspace");

        let updated = repo
            .update("test-uuid", "Updated Name")
            .await
            .expect("Failed to update workspace");

        assert_eq!(updated.name, "Updated Name");
    }

    #[tokio::test]
    async fn test_update_workspace_not_found() {
        let pool = setup_test_db().await;
        let repo = WorkspaceRepository::new(pool);

        let result = repo.update("non-existent", "New Name").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_workspace() {
        let pool = setup_test_db().await;
        let repo = WorkspaceRepository::new(pool);

        repo.create("test-uuid", "Test Workspace")
            .await
            .expect("Failed to create workspace");

        repo.delete("test-uuid")
            .await
            .expect("Failed to delete workspace");

        let workspace = repo
            .get("test-uuid")
            .await
            .expect("Failed to get workspace");
        assert!(workspace.is_none());
    }

    #[tokio::test]
    async fn test_delete_workspace_not_found() {
        let pool = setup_test_db().await;
        let repo = WorkspaceRepository::new(pool);

        let result = repo.delete("non-existent").await;

        assert!(result.is_err());
    }
}
