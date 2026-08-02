use chrono::Utc;
use sqlx::SqlitePool;

use crate::error::AppError;
use crate::plugins::registry::{PluginManifest, PluginPermission};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstalledPlugin {
    pub manifest: PluginManifest,
    pub enabled: bool,
}

pub struct PluginRepository {
    pool: SqlitePool,
}

impl PluginRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn upsert_internal(&self, manifest: &PluginManifest) -> Result<(), AppError> {
        let manifest_json = serde_json::to_string(manifest)
            .map_err(|_| AppError::Internal("Failed to serialize plugin manifest".to_string()))?;
        let now = Utc::now().to_rfc3339();
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(database_error("start plugin sync"))?;

        sqlx::query(
            "INSERT INTO plugins (id, name, version, manifest, enabled, created_at, updated_at) \
             VALUES (?, ?, ?, ?, 1, ?, ?) \
             ON CONFLICT(id) DO UPDATE SET name = excluded.name, version = excluded.version, \
             manifest = excluded.manifest, updated_at = excluded.updated_at",
        )
        .bind(&manifest.id)
        .bind(&manifest.name)
        .bind(&manifest.version)
        .bind(manifest_json)
        .bind(&now)
        .bind(&now)
        .execute(&mut *transaction)
        .await
        .map_err(database_error("upsert plugin"))?;

        sqlx::query("DELETE FROM plugin_permissions WHERE plugin_id = ?")
            .bind(&manifest.id)
            .execute(&mut *transaction)
            .await
            .map_err(database_error("replace plugin permissions"))?;

        for permission in &manifest.permissions {
            sqlx::query(
                "INSERT INTO plugin_permissions (id, plugin_id, permission, created_at) VALUES (?, ?, ?, ?)",
            )
            .bind(permission_id(&manifest.id, permission))
            .bind(&manifest.id)
            .bind(permission_name(permission))
            .bind(&now)
            .execute(&mut *transaction)
            .await
            .map_err(database_error("store plugin permission"))?;
        }

        transaction
            .commit()
            .await
            .map_err(database_error("finish plugin sync"))
    }

    pub async fn list(&self) -> Result<Vec<InstalledPlugin>, AppError> {
        let rows = sqlx::query_as::<_, PluginRow>(
            "SELECT manifest, enabled FROM plugins ORDER BY name COLLATE NOCASE",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(database_error("list plugins"))?;

        rows.into_iter()
            .map(|row| {
                let manifest = PluginManifest::parse(&row.manifest).map_err(|_| {
                    AppError::Internal("Stored plugin manifest is invalid".to_string())
                })?;
                Ok(InstalledPlugin {
                    manifest,
                    enabled: row.enabled != 0,
                })
            })
            .collect()
    }

    pub async fn set_enabled(&self, plugin_id: &str, enabled: bool) -> Result<(), AppError> {
        let result = sqlx::query("UPDATE plugins SET enabled = ?, updated_at = ? WHERE id = ?")
            .bind(i64::from(enabled))
            .bind(Utc::now().to_rfc3339())
            .bind(plugin_id)
            .execute(&self.pool)
            .await
            .map_err(database_error("update plugin status"))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Plugin not found".to_string()));
        }
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct PluginRow {
    manifest: String,
    enabled: i64,
}

fn permission_name(permission: &PluginPermission) -> &'static str {
    match permission {
        PluginPermission::Network => "network",
    }
}

fn permission_id(plugin_id: &str, permission: &PluginPermission) -> String {
    format!("{plugin_id}:{}", permission_name(permission))
}

fn database_error(operation: &'static str) -> impl Fn(sqlx::Error) -> AppError {
    move |error| AppError::Internal(format!("Failed to {operation}: {error}"))
}

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use super::*;

    async fn setup_repository() -> PluginRepository {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("test database should connect");
        sqlx::raw_sql(include_str!("../../../migrations/0001_initial.sql"))
            .execute(&pool)
            .await
            .expect("initial schema should apply");
        sqlx::raw_sql(include_str!("../../../migrations/0012_plugin_registry.sql"))
            .execute(&pool)
            .await
            .expect("plugin schema should apply");
        PluginRepository::new(pool)
    }

    #[tokio::test]
    async fn sync_preserves_user_enabled_status_and_replaces_permissions() {
        let repository = setup_repository().await;
        let manifest = PluginManifest::parse(include_str!(
            "../../../../../../plugins/company-comparison/manifest.json"
        ))
        .expect("bundled manifest should be valid");

        repository
            .upsert_internal(&manifest)
            .await
            .expect("plugin should be stored");
        repository
            .set_enabled(&manifest.id, false)
            .await
            .expect("plugin should be disabled");
        repository
            .upsert_internal(&manifest)
            .await
            .expect("plugin should be refreshed");

        let plugins = repository.list().await.expect("plugins should list");
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].manifest, manifest);
        assert!(!plugins[0].enabled);
    }
}
