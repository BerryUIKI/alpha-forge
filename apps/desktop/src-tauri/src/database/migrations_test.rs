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

    async fn migration_count(pool: &SqlitePool, name: &str) -> i64 {
        sqlx::query_scalar("SELECT COUNT(*) FROM _migrations WHERE name = ?")
            .bind(name)
            .fetch_one(pool)
            .await
            .expect("Failed to inspect migration record")
    }

    async fn option_table_names(pool: &SqlitePool) -> Vec<String> {
        sqlx::query_scalar::<_, String>(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name IN
             ('option_chains', 'option_contracts', 'greeks', 'option_strategies',
              'strategy_legs', 'option_positions', 'greeks_snapshots') ORDER BY name",
        )
        .fetch_all(pool)
        .await
        .expect("Failed to list Option tables")
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

    #[tokio::test]
    async fn option_migration_creates_canonical_schema_and_records_once() {
        let pool = setup_test_db().await;

        migrations::run(&pool)
            .await
            .expect("Failed to run Option migration");

        assert_eq!(option_table_names(&pool).await.len(), 7);
        let contract_columns: Vec<(i32, String, String, i32, Option<String>, i32)> =
            sqlx::query_as("PRAGMA table_info(option_contracts)")
                .fetch_all(&pool)
                .await
                .expect("Failed to inspect option_contracts columns");
        assert!(contract_columns
            .iter()
            .any(|(_, name, _, _, _, _)| name == "workspace_id"));
        assert!(contract_columns
            .iter()
            .any(|(_, name, _, _, _, _)| name == "last"));
        let strategy_columns: Vec<(i32, String, String, i32, Option<String>, i32)> =
            sqlx::query_as("PRAGMA table_info(option_strategies)")
                .fetch_all(&pool)
                .await
                .expect("Failed to inspect option_strategies columns");
        assert!(strategy_columns
            .iter()
            .any(|(_, name, _, _, _, _)| name == "break_even_points"));
        assert_eq!(migration_count(&pool, "0014_options_support").await, 1);

        migrations::run(&pool)
            .await
            .expect("Option migration should be idempotent");
        assert_eq!(migration_count(&pool, "0014_options_support").await, 1);
    }

    #[tokio::test]
    async fn option_migration_preserves_existing_canonical_data() {
        let pool = setup_test_db().await;
        sqlx::raw_sql(include_str!("../../migrations/0004_options_support.sql"))
            .execute(&pool)
            .await
            .expect("Failed to create historical Option schema");
        sqlx::query("INSERT INTO workspaces (id, name) VALUES ('option-workspace', 'Options')")
            .execute(&pool)
            .await
            .expect("Failed to create workspace");
        sqlx::query(
            "INSERT INTO option_chains (id, workspace_id, symbol, underlying_price, as_of, data_source, created_at)
             VALUES ('chain-1', 'option-workspace', 'AAPL', 200.0, '2026-08-13T00:00:00Z', 'demo', '2026-08-13T00:00:00Z')",
        )
        .execute(&pool)
        .await
        .expect("Failed to insert existing Option row");

        migrations::run(&pool)
            .await
            .expect("Failed to reconcile existing Option schema");

        let row: (String, f64) = sqlx::query_as(
            "SELECT symbol, underlying_price FROM option_chains WHERE id = 'chain-1'",
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to read preserved Option row");
        assert_eq!(row.0, "AAPL");
        assert_eq!(row.1, 200.0);
        assert_eq!(migration_count(&pool, "0014_options_support").await, 1);
    }

    #[tokio::test]
    async fn option_migration_completes_partial_canonical_schema_without_touching_rows() {
        let pool = setup_test_db().await;
        sqlx::query(
            "CREATE TABLE option_chains (
                id TEXT PRIMARY KEY NOT NULL,
                workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
                symbol TEXT NOT NULL,
                underlying_price REAL NOT NULL CHECK(underlying_price > 0),
                as_of TEXT NOT NULL,
                data_source TEXT NOT NULL CHECK(data_source IN ('live', 'demo', 'file')),
                created_at TEXT NOT NULL,
                UNIQUE(workspace_id, symbol, as_of)
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create partial Option schema");
        sqlx::query("INSERT INTO workspaces (id, name) VALUES ('partial-workspace', 'Partial')")
            .execute(&pool)
            .await
            .expect("Failed to create workspace");
        sqlx::query(
            "INSERT INTO option_chains (id, workspace_id, symbol, underlying_price, as_of, data_source, created_at)
             VALUES ('partial-chain', 'partial-workspace', 'MSFT', 400.0, '2026-08-13T00:00:00Z', 'demo', '2026-08-13T00:00:00Z')",
        )
        .execute(&pool)
        .await
        .expect("Failed to insert partial Option row");

        migrations::run(&pool)
            .await
            .expect("Failed to complete partial Option schema");

        assert_eq!(option_table_names(&pool).await.len(), 7);
        let row: (String,) =
            sqlx::query_as("SELECT symbol FROM option_chains WHERE id = 'partial-chain'")
                .fetch_one(&pool)
                .await
                .expect("Failed to read partial Option row");
        assert_eq!(row.0, "MSFT");
    }

    #[tokio::test]
    async fn option_migration_rejects_incompatible_nested_schema_without_changes() {
        let pool = setup_test_db().await;
        sqlx::query(
            "CREATE TABLE option_chains (
                id TEXT PRIMARY KEY,
                workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
                symbol TEXT NOT NULL,
                source TEXT NOT NULL,
                expiration_date TEXT NOT NULL,
                spot_price REAL NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create nested legacy schema");
        sqlx::query("INSERT INTO workspaces (id, name) VALUES ('legacy-workspace', 'Legacy')")
            .execute(&pool)
            .await
            .expect("Failed to create workspace");
        sqlx::query(
            "INSERT INTO option_chains (id, workspace_id, symbol, source, expiration_date, spot_price, created_at, updated_at)
             VALUES ('legacy-chain', 'legacy-workspace', 'TSLA', 'demo', '2026-09-18', 250.0, '2026-08-13T00:00:00Z', '2026-08-13T00:00:00Z')",
        )
        .execute(&pool)
        .await
        .expect("Failed to insert nested legacy row");

        let error = migrations::run(&pool)
            .await
            .expect_err("Nested legacy schema should be rejected");
        assert!(error
            .to_string()
            .contains("incompatible legacy Option schema"));
        assert_eq!(migration_count(&pool, "0014_options_support").await, 0);
        assert_eq!(option_table_names(&pool).await, vec!["option_chains"]);
        let row: (String, f64) = sqlx::query_as(
            "SELECT symbol, spot_price FROM option_chains WHERE id = 'legacy-chain'",
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to read untouched nested legacy row");
        assert_eq!(row.0, "TSLA");
        assert_eq!(row.1, 250.0);
    }

    #[tokio::test]
    async fn option_migration_rolls_back_tables_when_recording_fails() {
        let pool = setup_test_db().await;
        sqlx::query(
            "CREATE TABLE _migrations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                applied_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create migration tracking table");
        sqlx::query(
            "CREATE TRIGGER fail_option_migration_record
             BEFORE INSERT ON _migrations
             WHEN NEW.name = '0014_options_support'
             BEGIN SELECT RAISE(ABORT, 'forced Option migration failure'); END",
        )
        .execute(&pool)
        .await
        .expect("Failed to install migration failure trigger");

        let error = migrations::run(&pool)
            .await
            .expect_err("Forced migration record failure should surface");
        assert!(error
            .to_string()
            .contains("option migration transaction failed"));
        assert_eq!(migration_count(&pool, "0014_options_support").await, 0);
        assert!(option_table_names(&pool).await.is_empty());
    }
}
