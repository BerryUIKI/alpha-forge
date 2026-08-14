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

    const FINANCIAL_MIGRATIONS: &[&str] = &[
        "0015_financial_platforms_accounts",
        "0016_financial_assets_quotes",
        "0017_financial_activities",
        "0018_financial_lots",
        "0019_financial_snapshots_valuation",
        "0020_financial_taxonomies_allocation",
        "0021_financial_valuation_unique",
    ];

    const FINANCIAL_TABLES: &[&str] = &[
        "platforms",
        "accounts",
        "assets",
        "quotes",
        "import_runs",
        "activities",
        "lots",
        "lot_disposals",
        "holdings_snapshots",
        "snapshot_positions",
        "daily_account_valuation",
        "taxonomies",
        "taxonomy_categories",
        "asset_taxonomy_assignments",
        "allocation_targets",
        "allocation_target_weights",
        "allocation_target_constraints",
    ];

    async fn existing_tables(pool: &SqlitePool, tables: &[&str]) -> Vec<String> {
        sqlx::query_scalar::<_, String>(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name IN (SELECT value FROM json_each(?)) ORDER BY name",
        )
        .bind(serde_json::to_string(tables).expect("Failed to encode table list"))
        .fetch_all(pool)
        .await
        .expect("Failed to list tables")
    }

    #[tokio::test]
    async fn financial_migrations_create_canonical_schema_and_record_once() {
        let pool = setup_test_db().await;

        migrations::run(&pool)
            .await
            .expect("Failed to run financial migrations");

        for name in FINANCIAL_MIGRATIONS {
            assert_eq!(
                migration_count(&pool, name).await,
                1,
                "financial migration {name} should be recorded exactly once"
            );
        }
        let found = existing_tables(&pool, FINANCIAL_TABLES).await;
        assert_eq!(found.len(), FINANCIAL_TABLES.len());

        migrations::run(&pool)
            .await
            .expect("Financial migrations should be idempotent");
        for name in FINANCIAL_MIGRATIONS {
            assert_eq!(
                migration_count(&pool, name).await,
                1,
                "financial migration {name} must not double-apply"
            );
        }
    }

    #[tokio::test]
    async fn financial_migrations_preserve_existing_placeholder_tables() {
        let pool = setup_test_db().await;
        sqlx::query(
            "INSERT INTO portfolio_accounts (id, name, account_type, currency) VALUES ('pa-1', 'Legacy', 'SECURITIES', 'CNY')",
        )
        .execute(&pool)
        .await
        .expect("Failed to insert placeholder portfolio account");

        migrations::run(&pool)
            .await
            .expect("Failed to run migrations");

        let row: (String,) =
            sqlx::query_as("SELECT name FROM portfolio_accounts WHERE id = 'pa-1'")
                .fetch_one(&pool)
                .await
                .expect("Placeholder portfolio account must survive");
        assert_eq!(row.0, "Legacy");
        let placeholder_tables = existing_tables(
            &pool,
            &[
                "portfolio_accounts",
                "positions",
                "transactions",
                "watchlists",
            ],
        )
        .await;
        assert_eq!(placeholder_tables.len(), 4);
    }

    #[tokio::test]
    async fn financial_assets_derive_instrument_key() {
        let pool = setup_test_db().await;
        migrations::run(&pool)
            .await
            .expect("Failed to run migrations");

        sqlx::query(
            "INSERT INTO assets (id, kind, quote_mode, quote_ccy, instrument_type, instrument_symbol, instrument_exchange_mic)
             VALUES ('asset-equity', 'INVESTMENT', 'MARKET', 'USD', 'EQUITY', 'AAPL', 'XNAS')",
        )
        .execute(&pool)
        .await
        .expect("Failed to insert equity asset");
        sqlx::query(
            "INSERT INTO assets (id, kind, quote_mode, quote_ccy, instrument_type, instrument_symbol)
             VALUES ('asset-crypto', 'INVESTMENT', 'MARKET', 'USD', 'CRYPTO', 'BTC')",
        )
        .execute(&pool)
        .await
        .expect("Failed to insert crypto asset");

        let equity_key: String =
            sqlx::query_scalar("SELECT instrument_key FROM assets WHERE id = 'asset-equity'")
                .fetch_one(&pool)
                .await
                .expect("Failed to read equity instrument key");
        assert_eq!(equity_key, "EQUITY:AAPL@XNAS");
        let crypto_key: String =
            sqlx::query_scalar("SELECT instrument_key FROM assets WHERE id = 'asset-crypto'")
                .fetch_one(&pool)
                .await
                .expect("Failed to read crypto instrument key");
        assert_eq!(crypto_key, "CRYPTO:BTC/USD");
    }

    #[tokio::test]
    async fn financial_activities_enforce_idempotency_key_uniqueness() {
        let pool = setup_test_db().await;
        migrations::run(&pool)
            .await
            .expect("Failed to run migrations");

        sqlx::query(
            "INSERT INTO accounts (id, name, account_type, currency) VALUES ('acct-1', 'Main', 'SECURITIES', 'USD')",
        )
        .execute(&pool)
        .await
        .expect("Failed to insert account");
        sqlx::query(
            "INSERT INTO activities (id, account_id, activity_type, status, activity_date, currency, idempotency_key)
             VALUES ('act-1', 'acct-1', 'BUY', 'POSTED', '2026-08-13', 'USD', 'src:1')",
        )
        .execute(&pool)
        .await
        .expect("Failed to insert activity");

        let duplicate = sqlx::query(
            "INSERT INTO activities (id, account_id, activity_type, status, activity_date, currency, idempotency_key)
             VALUES ('act-2', 'acct-1', 'BUY', 'POSTED', '2026-08-13', 'USD', 'src:1')",
        )
        .execute(&pool)
        .await;
        assert!(
            duplicate.is_err(),
            "duplicate idempotency key must be rejected"
        );

        let unknown_type = sqlx::query(
            "INSERT INTO activities (id, account_id, activity_type, status, activity_date, currency)
             VALUES ('act-3', 'acct-1', 'HODL', 'POSTED', '2026-08-13', 'USD')",
        )
        .execute(&pool)
        .await;
        assert!(
            unknown_type.is_err(),
            "invalid activity_type must be rejected"
        );
    }

    #[tokio::test]
    async fn financial_taxonomies_seed_system_reference_data() {
        let pool = setup_test_db().await;
        migrations::run(&pool)
            .await
            .expect("Failed to run migrations");

        let system_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM taxonomies WHERE is_system = 1")
                .fetch_one(&pool)
                .await
                .expect("Failed to count system taxonomies");
        assert_eq!(system_count, 6);
        let instrument_categories: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM taxonomy_categories WHERE taxonomy_id = 'instrument_type'",
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to count instrument categories");
        assert_eq!(instrument_categories, 49);
        let asset_categories: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM taxonomy_categories WHERE taxonomy_id = 'asset_classes'",
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to count asset class categories");
        assert_eq!(asset_categories, 79);
    }

    #[tokio::test]
    async fn financial_allocation_target_weights_must_match_target_taxonomy() {
        let pool = setup_test_db().await;
        migrations::run(&pool)
            .await
            .expect("Failed to run migrations");

        sqlx::query(
            "INSERT INTO allocation_targets (id, name, scope_type, taxonomy_id)
             VALUES ('target-1', 'Equity tilt', 'all', 'asset_classes')",
        )
        .execute(&pool)
        .await
        .expect("Failed to insert allocation target");

        let mismatched = sqlx::query(
            "INSERT INTO allocation_target_weights (id, target_id, taxonomy_id, category_id, target_bps)
             VALUES ('weight-1', 'target-1', 'instrument_type', 'EQUITY_SECURITY', 5000)",
        )
        .execute(&pool)
        .await;
        assert!(
            mismatched.is_err(),
            "mismatched weight taxonomy must be rejected"
        );
    }

    #[tokio::test]
    async fn financial_daily_valuation_unique_per_account_date() {
        let pool = setup_test_db().await;
        migrations::run(&pool)
            .await
            .expect("Failed to run migrations");

        sqlx::query(
            "INSERT INTO accounts (id, name, account_type, currency) VALUES ('acct-1', 'Main', 'SECURITIES', 'USD')",
        )
        .execute(&pool)
        .await
        .expect("Failed to insert account");
        sqlx::query(
            "INSERT INTO daily_account_valuation (id, account_id, valuation_date, account_currency, base_currency, fx_rate_to_base, cash_balance, investment_market_value, total_value, cost_basis, net_contribution)
             VALUES ('val-1', 'acct-1', '2026-08-13', 'USD', 'USD', '1', '100', '900', '1000', '800', '500')",
        )
        .execute(&pool)
        .await
        .expect("Failed to insert valuation");
        sqlx::query(
            "INSERT INTO daily_account_valuation (id, account_id, valuation_date, account_currency, base_currency, fx_rate_to_base, cash_balance, investment_market_value, total_value, cost_basis, net_contribution)
             VALUES ('val-2', 'acct-1', '2026-08-12', 'USD', 'USD', '1', '50', '950', '1000', '800', '500')",
        )
        .execute(&pool)
        .await
        .expect("Different dates must be allowed");

        let duplicate = sqlx::query(
            "INSERT INTO daily_account_valuation (id, account_id, valuation_date, account_currency, base_currency, fx_rate_to_base, cash_balance, investment_market_value, total_value, cost_basis, net_contribution)
             VALUES ('val-3', 'acct-1', '2026-08-13', 'USD', 'USD', '1', '0', '0', '0', '0', '0')",
        )
        .execute(&pool)
        .await;
        assert!(
            duplicate.is_err(),
            "duplicate (account, date) valuation must be rejected"
        );
    }

    #[tokio::test]
    async fn financial_migration_rolls_back_tables_when_recording_fails() {
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
            "CREATE TRIGGER fail_financial_migration_record
             BEFORE INSERT ON _migrations
             WHEN NEW.name = '0015_financial_platforms_accounts'
             BEGIN SELECT RAISE(ABORT, 'forced financial migration failure'); END",
        )
        .execute(&pool)
        .await
        .expect("Failed to install migration failure trigger");

        let error = migrations::run(&pool)
            .await
            .expect_err("Forced migration record failure should surface");
        assert!(error
            .to_string()
            .contains("migration transaction failed for 0015_financial_platforms_accounts"));
        assert_eq!(
            migration_count(&pool, "0015_financial_platforms_accounts").await,
            0
        );
        assert!(
            existing_tables(&pool, FINANCIAL_TABLES).await.is_empty(),
            "no financial tables may survive a rolled-back migration"
        );
    }
}
