// Database migration runner.
//
// Migrations live in `apps/desktop/src-tauri/migrations/` and are embedded in
// the binary by sqlx::migrate!. SQLx tracks applied migrations in its own
// `_sqlx_migrations` table, so a migration that may already have been applied
// must never be edited. Append a new numbered file instead.
//
// Migrations that rebuild tables can opt out of the automatic transaction with
// `-- no-transaction` and manage foreign-key enforcement themselves.

use sqlx::SqlitePool;
use tracing::info;

use crate::error::AppError;

pub async fn run(pool: &SqlitePool) -> Result<(), AppError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| AppError::Internal(format!("database migration failed: {e}")))?;

    info!("migrations complete");
    Ok(())
}
