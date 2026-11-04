// AI provider configuration service.
//
// Owns provider settings and secret storage. Status responses never include
// the API key itself.

use std::sync::Arc;

use serde::Serialize;

use crate::error::AppError;
use crate::security::credentials::{CredentialStore, OPENAI_API_KEY};

const MAX_API_KEY_LENGTH: usize = 4096;

/// Provider configuration exposed to the frontend.
#[derive(Debug, Clone, Serialize)]
pub struct ProviderStatus {
    pub provider: String,
    pub configured: bool,
    pub model: String,
}

/// Manages provider credentials and reports configuration status.
pub struct ProviderService {
    credentials: Arc<dyn CredentialStore>,
    model: String,
}

impl ProviderService {
    pub fn new(credentials: Arc<dyn CredentialStore>, model: impl Into<String>) -> Self {
        Self {
            credentials,
            model: model.into(),
        }
    }

    /// Returns whether a provider key is configured. Never returns the key.
    pub fn status(&self) -> Result<ProviderStatus, AppError> {
        Ok(ProviderStatus {
            provider: "OpenAI".to_string(),
            configured: self.credentials.get(OPENAI_API_KEY)?.is_some(),
            model: self.model.clone(),
        })
    }

    /// Stores an API key in the OS keychain.
    pub fn set_api_key(&self, api_key: &str) -> Result<(), AppError> {
        let api_key = api_key.trim();

        if api_key.is_empty() {
            return Err(AppError::Validation("API key cannot be empty".to_string()));
        }
        if api_key.len() > MAX_API_KEY_LENGTH {
            return Err(AppError::Validation("API key is too long".to_string()));
        }

        self.credentials.set(OPENAI_API_KEY, api_key)
    }

    /// Removes the stored API key.
    pub fn clear_api_key(&self) -> Result<(), AppError> {
        self.credentials.delete(OPENAI_API_KEY)
    }
}
