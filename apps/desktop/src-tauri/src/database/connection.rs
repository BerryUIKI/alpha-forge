// SQLite connection pool.

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;

use crate::error::AppError;

pub async fn create_pool(db_url: &str) -> Result<SqlitePool, AppError> {
    let options: SqliteConnectOptions = db_url
        .parse()
        .map_err(|e| AppError::Internal(format!("invalid database URL: {e}")))?;

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .map_err(|e| AppError::Internal(format!("database connection failed: {e}")))?;

    // Enable WAL mode and foreign keys
    sqlx::query("PRAGMA journal_mode = WAL;")
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to set WAL mode: {e}")))?;

    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to enable foreign keys: {e}")))?;

    Ok(pool)
}
