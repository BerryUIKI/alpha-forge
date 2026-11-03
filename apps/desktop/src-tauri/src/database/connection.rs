// SQLite connection pool.

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::SqlitePool;

use crate::error::AppError;

pub async fn create_pool(db_url: &str) -> Result<SqlitePool, AppError> {
    let options = db_url
        .parse::<SqliteConnectOptions>()
        .map_err(|e| AppError::Internal(format!("invalid database URL: {e}")))?
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true);

    // Connection-level options are applied to every pooled connection. Running
    // PRAGMA statements against the pool would only configure one connection.
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .map_err(|e| AppError::Internal(format!("database connection failed: {e}")))?;

    Ok(pool)
}
