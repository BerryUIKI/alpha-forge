// Tests for database migrations.

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;
    use sqlx::sqlite::SqlitePoolOptions;

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
            "SELECT name FROM sqlite_master WHERE type='table' AND name='app_settings'"
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
            "SELECT name FROM sqlite_master WHERE type='table' AND name='workspaces'"
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

        let columns: Vec<(i32, String, String, i32, Option<String>)> = sqlx::query_as(
            "PRAGMA table_info(app_settings)"
        )
        .fetch_all(&pool)
        .await
        .expect("Failed to query table info");

        assert!(columns.iter().any(|(_, name, _, _, _)| name == "key"));
        assert!(columns.iter().any(|(_, name, _, _, _)| name == "value"));
        assert!(columns.iter().any(|(_, name, _, _, _)| name == "created_at"));
        assert!(columns.iter().any(|(_, name, _, _, _)| name == "updated_at"));
    }

    #[tokio::test]
    async fn test_workspaces_table_has_correct_columns() {
        let pool = setup_test_db().await;

        let columns: Vec<(i32, String, String, i32, Option<String>)> = sqlx::query_as(
            "PRAGMA table_info(workspaces)"
        )
        .fetch_all(&pool)
        .await
        .expect("Failed to query table info");

        assert!(columns.iter().any(|(_, name, _, _, _)| name == "id"));
        assert!(columns.iter().any(|(_, name, _, _, _)| name == "name"));
        assert!(columns.iter().any(|(_, name, _, _, _)| name == "created_at"));
        assert!(columns.iter().any(|(_, name, _, _, _)| name == "updated_at"));
    }
}