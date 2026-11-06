// Settings service — handles application settings business logic.

use crate::database::repositories::settings_repository::SettingsRepository;
use crate::error::AppError;
use crate::text::non_empty_text;

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
        let key = non_empty_text(key, "Setting key")?;

        self.repo.get(&key).await
    }

    /// Sets a setting value by key.
    pub async fn set(&self, key: &str, value: &str) -> Result<(), AppError> {
        let key = non_empty_text(key, "Setting key")?;

        // Validate value length (reasonable limit)
        if value.len() > 10000 {
            return Err(AppError::Validation(
                "Setting value exceeds maximum length".to_string(),
            ));
        }

        self.repo.set(&key, value).await
    }

    /// Deletes a setting by key.
    pub async fn delete(&self, key: &str) -> Result<(), AppError> {
        let key = non_empty_text(key, "Setting key")?;

        self.repo.delete(&key).await
    }
}
