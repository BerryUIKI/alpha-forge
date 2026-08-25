// Application bootstrap logic.
// Runs once at startup: database init, migration, logging setup.

use tauri::Manager;
use tracing::info;

use crate::database;

use crate::error::AppError;

use sqlx::SqlitePool;

pub async fn init_database(app_handle: &tauri::AppHandle) -> Result<SqlitePool, AppError> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Internal(format!("cannot resolve app data dir: {e}")))?;

    std::fs::create_dir_all(&app_dir)
        .map_err(|e| AppError::Internal(format!("cannot create app data dir: {e}")))?;

    let db_path = app_dir.join("alpha_forge.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    info!("opening application database");

    let pool = database::connection::create_pool(&db_url).await?;

    database::migrations::run(&pool).await?;

    info!("database initialized");

    Ok(pool)
}
