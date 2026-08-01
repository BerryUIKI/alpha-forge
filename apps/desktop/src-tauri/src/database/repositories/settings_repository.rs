// Settings repository — handles app_settings table operations.

use sqlx::SqlitePool;

use crate::error::AppError;

/// Repository for accessing application settings.
pub struct SettingsRepository {
    pool: SqlitePool,
}

impl SettingsRepository {
    /// Creates a new settings repository.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Gets a setting value by key.
    pub async fn get(&self, key: &str) -> Result<Option<String>, AppError> {
        let value = sqlx::query_scalar::<_, String>("SELECT value FROM app_settings WHERE key = ?")
            .bind(key)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to get setting '{}': {}", key, e)))?;

        Ok(value)
    }

    /// Sets a setting value by key.
    pub async fn set(&self, key: &str, value: &str) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO app_settings (key, value, created_at, updated_at)
             VALUES (?, ?, datetime('now'), datetime('now'))
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = datetime('now')",
        )
        .bind(key)
        .bind(value)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to set setting '{}': {}", key, e)))?;

        Ok(())
    }

    /// Deletes a setting by key.
    pub async fn delete(&self, key: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM app_settings WHERE key = ?")
            .bind(key)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                AppError::Internal(format!("Failed to delete setting '{}': {}", key, e))
            })?;

        Ok(())
    }

    /// Lists all settings.
    pub async fn list(&self) -> Result<Vec<(String, String)>, AppError> {
        let rows = sqlx::query_as::<_, (String, String)>(
            "SELECT key, value FROM app_settings ORDER BY key",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list settings: {}", e)))?;

        Ok(rows)
    }
}
