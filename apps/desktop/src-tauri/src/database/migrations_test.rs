// Tests for database migrations.

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    async fn setup_test_db() -> SqlitePool {
        crate::test_support::test_pool().await
    }

    #[tokio::test]
    async fn test_migration_creates_app_settings_table() {
        let pool = setup_test_db().await;

        // Check that app_settings table exists
        let result: Option<(String,)> = sqlx::query_as(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='app_settings'",
        )
        .fetch_optional(&pool)
        .await
        .expect("Failed to query table");

        assert!(result.is_some());
        assert_eq!(result.unwrap().0, "app_settings");
    }

    #[tokio::test]
    async fn test_migration_creates_workspaces_table() {
        let pool = setup_test_db().await;

        // Check that workspaces table exists
        let result: Option<(String,)> = sqlx::query_as(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='workspaces'",
        )
        .fetch_optional(&pool)
        .await
        .expect("Failed to query table");

        assert!(result.is_some());
        assert_eq!(result.unwrap().0, "workspaces");
    }

    #[tokio::test]
    async fn test_app_settings_table_has_correct_columns() {
        let pool = setup_test_db().await;

        let columns: Vec<(String,)> =
            sqlx::query_as("SELECT name FROM pragma_table_info('app_settings')")
                .fetch_all(&pool)
                .await
                .expect("Failed to query table info");

        assert!(columns.iter().any(|(name,)| name == "key"));
        assert!(columns.iter().any(|(name,)| name == "value"));
        assert!(columns.iter().any(|(name,)| name == "created_at"));
        assert!(columns.iter().any(|(name,)| name == "updated_at"));
    }

    #[tokio::test]
    async fn test_workspaces_table_has_correct_columns() {
        let pool = setup_test_db().await;

        let columns: Vec<(String,)> =
            sqlx::query_as("SELECT name FROM pragma_table_info('workspaces')")
                .fetch_all(&pool)
                .await
                .expect("Failed to query table info");

        assert!(columns.iter().any(|(name,)| name == "id"));
        assert!(columns.iter().any(|(name,)| name == "name"));
        assert!(columns.iter().any(|(name,)| name == "created_at"));
        assert!(columns.iter().any(|(name,)| name == "updated_at"));
    }

    #[tokio::test]
    async fn test_all_migrations_are_recorded() {
        let pool = setup_test_db().await;

        let descriptions: Vec<(String,)> =
            sqlx::query_as("SELECT description FROM _sqlx_migrations ORDER BY version")
                .fetch_all(&pool)
                .await
                .expect("Failed to query migrations");

        let descriptions: Vec<String> = descriptions
            .into_iter()
            .map(|(description,)| description.replace(' ', "_"))
            .collect();
        assert_eq!(
            descriptions,
            vec![
                "initial".to_string(),
                "agent_tasks_enhancement".to_string(),
                "agent_tasks_align".to_string(),
                "artifacts_align".to_string(),
                "research_documents_metadata".to_string(),
            ]
        );
    }

    #[tokio::test]
    async fn test_migrations_are_idempotent() {
        let pool = setup_test_db().await;

        crate::database::migrations::run(&pool)
            .await
            .expect("Second migration run should succeed");

        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM _sqlx_migrations WHERE success = 1")
                .fetch_one(&pool)
                .await
                .expect("Failed to count migrations");

        assert_eq!(count, 5);
    }

    #[tokio::test]
    async fn test_agent_tasks_accept_created_status_without_input() {
        let pool = setup_test_db().await;

        sqlx::query("INSERT INTO workspaces (id, name) VALUES ('w1', 'Workspace')")
            .execute(&pool)
            .await
            .expect("Failed to insert workspace");

        sqlx::query(
            "INSERT INTO agent_tasks (id, workspace_id, title, status, created_at, updated_at)
             VALUES ('t1', 'w1', 'Research Tesla', 'created', datetime('now'), datetime('now'))",
        )
        .execute(&pool)
        .await
        .expect("Agent tasks should accept the created status without input");

        let status: String = sqlx::query_scalar("SELECT status FROM agent_tasks WHERE id = 't1'")
            .fetch_one(&pool)
            .await
            .expect("Failed to read task");

        assert_eq!(status, "created");
    }

    #[tokio::test]
    async fn test_agent_tasks_input_column_is_nullable() {
        let pool = setup_test_db().await;

        let not_null: i64 = sqlx::query_scalar(
            "SELECT \"notnull\" FROM pragma_table_info('agent_tasks') WHERE name = 'input'",
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to inspect agent_tasks schema");

        assert_eq!(not_null, 0);
    }

    #[tokio::test]
    async fn test_artifacts_table_has_presentation_columns() {
        let pool = setup_test_db().await;

        let columns: Vec<(String,)> =
            sqlx::query_as("SELECT name FROM pragma_table_info('artifacts')")
                .fetch_all(&pool)
                .await
                .expect("Failed to query table info");

        assert!(columns.iter().any(|(name,)| name == "title"));
        assert!(columns.iter().any(|(name,)| name == "workspace_id"));
        assert!(columns.iter().any(|(name,)| name == "artifact_type"));
    }

    #[tokio::test]
    async fn test_research_documents_have_metadata_columns() {
        let pool = setup_test_db().await;

        let columns: Vec<(String,)> =
            sqlx::query_as("SELECT name FROM pragma_table_info('research_documents')")
                .fetch_all(&pool)
                .await
                .expect("Failed to query table info");

        assert!(columns.iter().any(|(name,)| name == "kind"));
        assert!(columns.iter().any(|(name,)| name == "format"));
        assert!(columns.iter().any(|(name,)| name == "summary"));
        assert!(columns.iter().any(|(name,)| name == "archived_at"));
    }
}
