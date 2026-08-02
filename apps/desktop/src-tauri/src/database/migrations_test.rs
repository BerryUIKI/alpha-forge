// Tests for database migrations.

#[cfg(test)]
mod tests {
    use crate::database::migrations;
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;

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

        let columns: Vec<(i32, String, String, i32, Option<String>)> =
            sqlx::query_as("PRAGMA table_info(app_settings)")
                .fetch_all(&pool)
                .await
                .expect("Failed to query table info");

        assert!(columns.iter().any(|(_, name, _, _, _)| name == "key"));
        assert!(columns.iter().any(|(_, name, _, _, _)| name == "value"));
        assert!(columns
            .iter()
            .any(|(_, name, _, _, _)| name == "created_at"));
        assert!(columns
            .iter()
            .any(|(_, name, _, _, _)| name == "updated_at"));
    }

    #[tokio::test]
    async fn test_workspaces_table_has_correct_columns() {
        let pool = setup_test_db().await;

        let columns: Vec<(i32, String, String, i32, Option<String>)> =
            sqlx::query_as("PRAGMA table_info(workspaces)")
                .fetch_all(&pool)
                .await
                .expect("Failed to query table info");

        assert!(columns.iter().any(|(_, name, _, _, _)| name == "id"));
        assert!(columns.iter().any(|(_, name, _, _, _)| name == "name"));
        assert!(columns
            .iter()
            .any(|(_, name, _, _, _)| name == "created_at"));
        assert!(columns
            .iter()
            .any(|(_, name, _, _, _)| name == "updated_at"));
    }

    #[tokio::test]
    async fn normalizes_legacy_thesis_timestamps() {
        let pool = setup_test_db().await;
        migrations::run(&pool)
            .await
            .expect("Failed to establish current schema");
        sqlx::query(
            "INSERT INTO workspaces (id, name, created_at, updated_at) VALUES ('workspace', 'Test workspace', '2026-08-01T12:34:56Z', '2026-08-01T12:34:56Z')",
        )
        .execute(&pool)
        .await
        .expect("Failed to insert workspace");
        sqlx::query(
            "INSERT INTO investment_theses (id, workspace_id, title, thesis, confidence, status, created_at, updated_at) VALUES ('legacy-thesis', 'workspace', 'Legacy', 'Legacy thesis', 50, 'draft', '2026-08-01 12:34:56', '2026-08-01 12:34:56')",
        )
        .execute(&pool)
        .await
        .expect("Failed to insert legacy thesis");
        sqlx::query("DROP TRIGGER update_thesis_updated_at")
            .execute(&pool)
            .await
            .expect("Failed to remove current timestamp trigger");
        sqlx::query(
            "UPDATE investment_theses SET created_at = '2026-08-01 12:34:56', updated_at = '2026-08-01 12:34:56' WHERE id = 'legacy-thesis'",
        )
        .execute(&pool)
        .await
        .expect("Failed to simulate legacy timestamps");
        sqlx::query("DELETE FROM _migrations WHERE name = '0013_thesis_timestamp_normalization'")
            .execute(&pool)
            .await
            .expect("Failed to reset timestamp migration");

        migrations::run(&pool)
            .await
            .expect("Failed to run migrations");

        let timestamp: String = sqlx::query_scalar(
            "SELECT updated_at FROM investment_theses WHERE id = 'legacy-thesis'",
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to read normalized timestamp");

        assert_eq!(timestamp, "2026-08-01T12:34:56.000Z");
    }
}
