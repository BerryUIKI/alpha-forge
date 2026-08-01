// Settings service — handles application settings business logic.

use crate::database::repositories::settings_repository::SettingsRepository;
use crate::error::AppError;

/// Service for managing application settings.
pub struct SettingsService {
    repo: SettingsRepository,
}

impl SettingsService {
    /// Creates a new settings service.
    pub fn new(repo: SettingsRepository) -> Self {
        Self { repo }
    }

    /// Gets a setting value by key.
    pub async fn get(&self, key: &str) -> Result<Option<String>, AppError> {
        // Validate key is not empty
        if key.trim().is_empty() {
            return Err(AppError::Validation(
                "Setting key cannot be empty".to_string(),
            ));
        }

        self.repo.get(key).await
    }

    /// Sets a setting value by key.
    pub async fn set(&self, key: &str, value: &str) -> Result<(), AppError> {
        // Validate key is not empty
        if key.trim().is_empty() {
            return Err(AppError::Validation(
                "Setting key cannot be empty".to_string(),
            ));
        }

        // Validate value length (reasonable limit)
        if value.len() > 10000 {
            return Err(AppError::Validation(
                "Setting value exceeds maximum length".to_string(),
            ));
        }

        self.repo.set(key, value).await
    }

    /// Deletes a setting by key.
    pub async fn delete(&self, key: &str) -> Result<(), AppError> {
        if key.trim().is_empty() {
            return Err(AppError::Validation(
                "Setting key cannot be empty".to_string(),
            ));
        }

        self.repo.delete(key).await
    }
}
