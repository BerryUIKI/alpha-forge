// Database migration runner.

use sqlx::SqlitePool;
use tracing::info;

use crate::error::AppError;

pub async fn run(pool: &SqlitePool) -> Result<(), AppError> {
    // Create the migration tracking table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS _migrations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
    )
    .execute(pool)
    .await
    .map_err(|e| AppError::Internal(format!("migration setup failed: {e}")))?;

    // Apply the initial migration
    // In production we would track applied migrations, but for Phase 1 we embed directly.
    apply_initial_migration(pool).await?;

    info!("migrations complete");

    Ok(())
}

async fn apply_initial_migration(pool: &SqlitePool) -> Result<(), AppError> {
    // Check if already applied
    let already = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM _migrations WHERE name = '0001_initial'",
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(format!("migration check failed: {e}")))?;

    if already > 0 {
        return Ok(());
    }

    // Create the core table for Phase 1: app_settings
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS app_settings (
            key TEXT PRIMARY KEY NOT NULL,
            value TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
    )
    .execute(pool)
    .await
    .map_err(|e| AppError::Internal(format!("initial migration failed: {e}")))?;

    // Record migration
    sqlx::query("INSERT INTO _migrations (name) VALUES ('0001_initial')")
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("migration record failed: {e}")))?;

    Ok(())
}
