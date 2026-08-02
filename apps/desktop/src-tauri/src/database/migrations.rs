// Database migration runner.

use sqlx::{Row, SqlitePool};
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

    // The first six historical SQL files overlap and some rebuild tables. Applying
    // them in lexical order is destructive, so 0007 reconciles the initial schema
    // to the shape required by the current repositories instead.
    apply_initial_migration(pool).await?;
    apply_schema_reconciliation(pool).await?;
    apply_thesis_confidence_history(pool).await?;
    apply_knowledge_graph(pool).await?;
    apply_portfolio_management(pool).await?;
    apply_portfolio_theme_links(pool).await?;
    apply_plugin_registry(pool).await?;

    info!("migrations complete");

    Ok(())
}

async fn apply_initial_migration(pool: &SqlitePool) -> Result<(), AppError> {
    // The first runtime recorded 0001 after creating only app_settings. Re-run
    // the idempotent canonical schema so those early databases receive the
    // missing core tables before reconciliation inspects them.
    sqlx::raw_sql(include_str!("../../migrations/0001_initial.sql"))
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("initial migration failed: {e}")))?;

    // Check whether the migration was already recorded.
    let already = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM _migrations WHERE name = '0001_initial'",
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(format!("migration check failed: {e}")))?;

    if already > 0 {
        return Ok(());
    }

    // Record migration
    sqlx::query("INSERT INTO _migrations (name) VALUES ('0001_initial')")
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("migration record failed: {e}")))?;

    Ok(())
}

async fn apply_schema_reconciliation(pool: &SqlitePool) -> Result<(), AppError> {
    let already = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM _migrations WHERE name = '0007_schema_reconciliation'",
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(format!("migration check failed: {e}")))?;

    if already > 0 {
        return Ok(());
    }

    // These columns are absent from databases created by the previous runtime.
    // They are nullable so no existing row is discarded or assigned to an
    // invented workspace/project during an automatic upgrade.
    add_column_if_missing(pool, "agent_tasks", "workspace_id", "TEXT REFERENCES workspaces(id)").await?;
    add_column_if_missing(pool, "agent_tasks", "title", "TEXT").await?;
    add_column_if_missing(pool, "agent_tasks", "description", "TEXT").await?;
    add_column_if_missing(pool, "artifacts", "workspace_id", "TEXT REFERENCES workspaces(id) ON DELETE CASCADE").await?;
    add_column_if_missing(pool, "artifacts", "error", "TEXT").await?;
    add_column_if_missing(pool, "research_documents", "project_id", "TEXT REFERENCES research_projects(id) ON DELETE CASCADE").await?;
    add_column_if_missing(pool, "research_documents", "document_type", "TEXT").await?;
    add_column_if_missing(pool, "research_documents", "source_url", "TEXT").await?;
    add_column_if_missing(pool, "research_documents", "file_path", "TEXT").await?;
    add_column_if_missing(pool, "investment_theses", "workspace_id", "TEXT REFERENCES workspaces(id) ON DELETE CASCADE").await?;

    sqlx::raw_sql(include_str!("../../migrations/0007_schema_reconciliation.sql"))
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("schema reconciliation failed: {e}")))?;

    sqlx::query("INSERT INTO _migrations (name) VALUES ('0007_schema_reconciliation')")
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("migration record failed: {e}")))?;

    Ok(())
}

async fn add_column_if_missing(
    pool: &SqlitePool,
    table: &str,
    column: &str,
    definition: &str,
) -> Result<(), AppError> {
    let rows = sqlx::query(&format!("PRAGMA table_info({table})"))
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::Internal(format!("schema inspection failed for {table}: {e}")))?;

    if rows.iter().any(|row| row.get::<String, _>("name") == column) {
        return Ok(());
    }

    sqlx::query(&format!("ALTER TABLE {table} ADD COLUMN {column} {definition}"))
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to add {table}.{column}: {e}")))?;

    Ok(())
}

async fn apply_thesis_confidence_history(pool: &SqlitePool) -> Result<(), AppError> {
    let already = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM _migrations WHERE name = '0008_thesis_confidence_history'",
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(format!("migration check failed: {e}")))?;

    if already > 0 {
        return Ok(());
    }

    sqlx::raw_sql(include_str!("../../migrations/0008_thesis_confidence_history.sql"))
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("confidence history migration failed: {e}")))?;

    sqlx::query("INSERT INTO _migrations (name) VALUES ('0008_thesis_confidence_history')")
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("migration record failed: {e}")))?;

    Ok(())
}

async fn apply_knowledge_graph(pool: &SqlitePool) -> Result<(), AppError> {
    let already = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM _migrations WHERE name = '0009_knowledge_graph'",
    ).fetch_one(pool).await.map_err(|e| AppError::Internal(format!("migration check failed: {e}")))?;
    if already > 0 { return Ok(()); }
    sqlx::raw_sql(include_str!("../../migrations/0009_knowledge_graph.sql")).execute(pool).await
        .map_err(|e| AppError::Internal(format!("knowledge graph migration failed: {e}")))?;
    sqlx::query("INSERT INTO _migrations (name) VALUES ('0009_knowledge_graph')").execute(pool).await
        .map_err(|e| AppError::Internal(format!("migration record failed: {e}")))?;
    Ok(())
}

async fn apply_portfolio_management(pool: &SqlitePool) -> Result<(), AppError> {
    let already = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM _migrations WHERE name = '0010_portfolio_management'",
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(format!("migration check failed: {e}")))?;
    if already > 0 { return Ok(()); }
    add_column_if_missing(pool, "portfolio_accounts", "workspace_id", "TEXT REFERENCES workspaces(id) ON DELETE CASCADE").await?;
    sqlx::raw_sql(include_str!("../../migrations/0010_portfolio_management.sql"))
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("portfolio migration failed: {e}")))?;
    sqlx::query("INSERT INTO _migrations (name) VALUES ('0010_portfolio_management')")
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("migration record failed: {e}")))?;
    Ok(())
}

async fn apply_portfolio_theme_links(pool: &SqlitePool) -> Result<(), AppError> {
    let already = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM _migrations WHERE name = '0011_portfolio_theme_links'").fetch_one(pool).await.map_err(|e| AppError::Internal(format!("migration check failed: {e}")))?;
    if already > 0 { return Ok(()); }
    sqlx::raw_sql(include_str!("../../migrations/0011_portfolio_theme_links.sql")).execute(pool).await.map_err(|e| AppError::Internal(format!("portfolio theme migration failed: {e}")))?;
    sqlx::query("INSERT INTO _migrations (name) VALUES ('0011_portfolio_theme_links')").execute(pool).await.map_err(|e| AppError::Internal(format!("migration record failed: {e}")))?;
    Ok(())
}

async fn apply_plugin_registry(pool: &SqlitePool) -> Result<(), AppError> {
    let already = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM _migrations WHERE name = '0012_plugin_registry'").fetch_one(pool).await.map_err(|e| AppError::Internal(format!("migration check failed: {e}")))?;
    if already > 0 { return Ok(()); }
    sqlx::raw_sql(include_str!("../../migrations/0012_plugin_registry.sql")).execute(pool).await.map_err(|e| AppError::Internal(format!("plugin registry migration failed: {e}")))?;
    sqlx::query("INSERT INTO _migrations (name) VALUES ('0012_plugin_registry')").execute(pool).await.map_err(|e| AppError::Internal(format!("migration record failed: {e}")))?;
    Ok(())
}
