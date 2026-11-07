// Shared helpers for tests that use an in-memory SQLite database.

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

pub(crate) async fn test_pool() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(":memory:")
        .await
        .expect("Failed to create test database");

    crate::database::migrations::run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

pub(crate) async fn insert_workspace(pool: &SqlitePool, id: &str, name: &str) {
    sqlx::query("INSERT INTO workspaces (id, name) VALUES (?, ?)")
        .bind(id)
        .bind(name)
        .execute(pool)
        .await
        .expect("Failed to insert workspace");
}

pub(crate) async fn test_pool_with_workspace() -> SqlitePool {
    let pool = test_pool().await;
    insert_workspace(&pool, "w1", "Workspace").await;
    pool
}
