// Shared test-database setup for repository tests.
//
// Runs the full migration chain (research + financial) so repository tests
// exercise the real schema instead of a hand-rolled subset. Foreign keys are
// enabled so FK violations (e.g. accounts.workspace_id) are caught exactly as
// they would be in the running application (connection.rs does the same).

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

use crate::database::migrations;

pub(crate) async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(":memory:")
        .await
        .expect("Failed to create test database");

    migrations::run(&pool)
        .await
        .expect("Failed to run migrations");

    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await
        .expect("Failed to enable foreign keys");

    pool
}
