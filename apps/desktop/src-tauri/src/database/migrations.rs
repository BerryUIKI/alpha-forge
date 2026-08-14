// Database migration runner.

use sqlx::{Connection, Executor, Row, SqlitePool};
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
    apply_thesis_timestamp_normalization(pool).await?;
    apply_option_support(pool).await?;
    apply_financial_migrations(pool).await?;

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
    add_column_if_missing(
        pool,
        "agent_tasks",
        "workspace_id",
        "TEXT REFERENCES workspaces(id)",
    )
    .await?;
    add_column_if_missing(pool, "agent_tasks", "title", "TEXT").await?;
    add_column_if_missing(pool, "agent_tasks", "description", "TEXT").await?;
    add_column_if_missing(
        pool,
        "artifacts",
        "workspace_id",
        "TEXT REFERENCES workspaces(id) ON DELETE CASCADE",
    )
    .await?;
    add_column_if_missing(pool, "artifacts", "error", "TEXT").await?;
    add_column_if_missing(
        pool,
        "research_documents",
        "project_id",
        "TEXT REFERENCES research_projects(id) ON DELETE CASCADE",
    )
    .await?;
    add_column_if_missing(pool, "research_documents", "document_type", "TEXT").await?;
    add_column_if_missing(pool, "research_documents", "source_url", "TEXT").await?;
    add_column_if_missing(pool, "research_documents", "file_path", "TEXT").await?;
    add_column_if_missing(
        pool,
        "investment_theses",
        "workspace_id",
        "TEXT REFERENCES workspaces(id) ON DELETE CASCADE",
    )
    .await?;

    sqlx::raw_sql(include_str!(
        "../../migrations/0007_schema_reconciliation.sql"
    ))
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

    if rows
        .iter()
        .any(|row| row.get::<String, _>("name") == column)
    {
        return Ok(());
    }

    sqlx::query(&format!(
        "ALTER TABLE {table} ADD COLUMN {column} {definition}"
    ))
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

    sqlx::raw_sql(include_str!(
        "../../migrations/0008_thesis_confidence_history.sql"
    ))
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
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(format!("migration check failed: {e}")))?;
    if already > 0 {
        return Ok(());
    }
    sqlx::raw_sql(include_str!("../../migrations/0009_knowledge_graph.sql"))
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("knowledge graph migration failed: {e}")))?;
    sqlx::query("INSERT INTO _migrations (name) VALUES ('0009_knowledge_graph')")
        .execute(pool)
        .await
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
    if already > 0 {
        return Ok(());
    }
    add_column_if_missing(
        pool,
        "portfolio_accounts",
        "workspace_id",
        "TEXT REFERENCES workspaces(id) ON DELETE CASCADE",
    )
    .await?;
    sqlx::raw_sql(include_str!(
        "../../migrations/0010_portfolio_management.sql"
    ))
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
    let already = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM _migrations WHERE name = '0011_portfolio_theme_links'",
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(format!("migration check failed: {e}")))?;
    if already > 0 {
        return Ok(());
    }
    sqlx::raw_sql(include_str!(
        "../../migrations/0011_portfolio_theme_links.sql"
    ))
    .execute(pool)
    .await
    .map_err(|e| AppError::Internal(format!("portfolio theme migration failed: {e}")))?;
    sqlx::query("INSERT INTO _migrations (name) VALUES ('0011_portfolio_theme_links')")
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("migration record failed: {e}")))?;
    Ok(())
}

async fn apply_plugin_registry(pool: &SqlitePool) -> Result<(), AppError> {
    let already = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM _migrations WHERE name = '0012_plugin_registry'",
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(format!("migration check failed: {e}")))?;
    if already > 0 {
        return Ok(());
    }
    sqlx::raw_sql(include_str!("../../migrations/0012_plugin_registry.sql"))
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("plugin registry migration failed: {e}")))?;
    sqlx::query("INSERT INTO _migrations (name) VALUES ('0012_plugin_registry')")
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("migration record failed: {e}")))?;
    Ok(())
}

async fn apply_thesis_timestamp_normalization(pool: &SqlitePool) -> Result<(), AppError> {
    let already = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM _migrations WHERE name = '0013_thesis_timestamp_normalization'",
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(format!("migration check failed: {e}")))?;

    if already > 0 {
        return Ok(());
    }

    sqlx::raw_sql(include_str!(
        "../../migrations/0013_thesis_timestamp_normalization.sql"
    ))
    .execute(pool)
    .await
    .map_err(|e| AppError::Internal(format!("thesis timestamp normalization failed: {e}")))?;

    sqlx::query("INSERT INTO _migrations (name) VALUES ('0013_thesis_timestamp_normalization')")
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("migration record failed: {e}")))?;

    Ok(())
}

const OPTION_MIGRATION_NAME: &str = "0014_options_support";

const OPTION_TABLE_COLUMNS: &[(&str, &[&str])] = &[
    (
        "option_chains",
        &[
            "id",
            "workspace_id",
            "symbol",
            "underlying_price",
            "as_of",
            "data_source",
            "created_at",
        ],
    ),
    (
        "option_contracts",
        &[
            "id",
            "workspace_id",
            "chain_id",
            "symbol",
            "option_type",
            "strike",
            "expiration",
            "contract_multiplier",
            "bid",
            "ask",
            "last",
            "volume",
            "open_interest",
            "implied_volatility",
            "created_at",
            "updated_at",
        ],
    ),
    (
        "greeks",
        &[
            "id",
            "option_contract_id",
            "delta",
            "gamma",
            "theta",
            "vega",
            "rho",
            "iv",
            "calculated_at",
            "calculation_model",
        ],
    ),
    (
        "option_strategies",
        &[
            "id",
            "workspace_id",
            "name",
            "strategy_type",
            "underlying",
            "total_cost",
            "max_profit",
            "max_loss",
            "break_even_points",
            "created_at",
            "updated_at",
        ],
    ),
    (
        "strategy_legs",
        &[
            "id",
            "strategy_id",
            "option_contract_id",
            "quantity",
            "position_type",
            "premium",
            "strike",
            "expiration",
            "option_type",
        ],
    ),
    (
        "option_positions",
        &[
            "id",
            "workspace_id",
            "account_id",
            "option_contract_id",
            "quantity",
            "cost_basis",
            "opened_at",
            "closed_at",
            "notes",
        ],
    ),
    (
        "greeks_snapshots",
        &[
            "id",
            "workspace_id",
            "position_id",
            "snapshot_date",
            "delta",
            "gamma",
            "theta",
            "vega",
            "rho",
            "created_at",
        ],
    ),
];

async fn apply_option_support(pool: &SqlitePool) -> Result<(), AppError> {
    let already = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM _migrations WHERE name = ?")
        .bind(OPTION_MIGRATION_NAME)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Internal(format!("option migration check failed: {e}")))?;

    if already > 0 {
        return Ok(());
    }

    // Preflight is read-only and happens before any DDL. This rejects an
    // incompatible legacy table without changing it or any other table.
    for (table, required_columns) in OPTION_TABLE_COLUMNS {
        let object_type = sqlx::query_scalar::<_, String>(
            "SELECT type FROM sqlite_master WHERE name = ? LIMIT 1",
        )
        .bind(*table)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            AppError::Internal(format!("option schema inspection failed for {table}: {e}"))
        })?;

        let Some(object_type) = object_type else {
            continue;
        };

        if object_type != "table" {
            return Err(AppError::Validation(format!(
                "incompatible legacy Option schema: {table} is a {object_type}, not a table"
            )));
        }

        let rows = sqlx::query(&format!("PRAGMA table_info({table})"))
            .fetch_all(pool)
            .await
            .map_err(|e| {
                AppError::Internal(format!("option schema inspection failed for {table}: {e}"))
            })?;

        let missing = required_columns
            .iter()
            .filter(|required| {
                !rows
                    .iter()
                    .any(|row| row.get::<String, _>("name") == **required)
            })
            .copied()
            .collect::<Vec<_>>();

        if !missing.is_empty() {
            return Err(AppError::Validation(format!(
                "incompatible legacy Option schema in {table}; missing columns: {}",
                missing.join(", ")
            )));
        }
    }

    let mut connection = pool
        .acquire()
        .await
        .map_err(|e| AppError::Internal(format!("option migration transaction failed: {e}")))?;
    let transaction_result: Result<(), sqlx::Error> =
        Connection::transaction(&mut *connection, |transaction| {
            Box::pin(async move {
                (&mut **transaction)
                    .execute(include_str!("../../migrations/0014_options_support.sql"))
                    .await?;
                sqlx::query("INSERT INTO _migrations (name) VALUES (?)")
                    .bind(OPTION_MIGRATION_NAME)
                    .execute(&mut **transaction)
                    .await?;
                Ok(())
            })
        })
        .await;
    transaction_result
        .map_err(|e| AppError::Internal(format!("option migration transaction failed: {e}")))?;

    Ok(())
}

/// Financial domain migrations (Wealthfolio port, Phase 1 storage).
///
/// Each entry runs its DDL and `_migrations` record inside one transaction so
/// a failure rolls back cleanly and the migration is retried from scratch.
/// The SQL files are pure additive DDL — fresh tables, indexes, triggers, and
/// reference seed data — and never touch the existing research tables.
const FINANCIAL_MIGRATIONS: &[(&str, &str)] = &[
    (
        "0015_financial_platforms_accounts",
        include_str!("../../migrations/0015_financial_platforms_accounts.sql"),
    ),
    (
        "0016_financial_assets_quotes",
        include_str!("../../migrations/0016_financial_assets_quotes.sql"),
    ),
    (
        "0017_financial_activities",
        include_str!("../../migrations/0017_financial_activities.sql"),
    ),
    (
        "0018_financial_lots",
        include_str!("../../migrations/0018_financial_lots.sql"),
    ),
    (
        "0019_financial_snapshots_valuation",
        include_str!("../../migrations/0019_financial_snapshots_valuation.sql"),
    ),
    (
        "0020_financial_taxonomies_allocation",
        include_str!("../../migrations/0020_financial_taxonomies_allocation.sql"),
    ),
    (
        "0021_financial_valuation_unique",
        include_str!("../../migrations/0021_financial_valuation_unique.sql"),
    ),
];

async fn apply_financial_migrations(pool: &SqlitePool) -> Result<(), AppError> {
    for (name, sql) in FINANCIAL_MIGRATIONS {
        apply_financial_migration(pool, name, sql).await?;
    }
    Ok(())
}

async fn apply_financial_migration(
    pool: &SqlitePool,
    name: &'static str,
    sql: &'static str,
) -> Result<(), AppError> {
    let already = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM _migrations WHERE name = ?")
        .bind(name)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Internal(format!("migration check failed for {name}: {e}")))?;

    if already > 0 {
        return Ok(());
    }

    let mut connection = pool
        .acquire()
        .await
        .map_err(|e| AppError::Internal(format!("migration transaction failed for {name}: {e}")))?;
    let transaction_result: Result<(), sqlx::Error> =
        Connection::transaction(&mut *connection, |transaction| {
            Box::pin(async move {
                (&mut **transaction).execute(sql).await?;
                sqlx::query("INSERT INTO _migrations (name) VALUES (?)")
                    .bind(name)
                    .execute(&mut **transaction)
                    .await?;
                Ok(())
            })
        })
        .await;
    transaction_result
        .map_err(|e| AppError::Internal(format!("migration transaction failed for {name}: {e}")))?;

    Ok(())
}
