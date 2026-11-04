// OS keychain credential storage.
//
// Secrets are stored in the operating system keychain (Windows Credential
// Manager, macOS Keychain, libsecret on Linux). Values are never written to
// disk by the application and are never returned to the frontend.

use crate::error::AppError;

/// Key under which the OpenAI API key is stored.
pub const OPENAI_API_KEY: &str = "openai_api_key";

/// Storage abstraction for secrets.
pub trait CredentialStore: Send + Sync {
    fn set(&self, key: &str, value: &str) -> Result<(), AppError>;
    fn get(&self, key: &str) -> Result<Option<String>, AppError>;
    fn delete(&self, key: &str) -> Result<(), AppError>;
}

/// Keychain-backed credential store.
pub struct KeyringCredentialStore {
    service: String,
}

impl KeyringCredentialStore {
    pub fn new(service: impl Into<String>) -> Self {
        Self {
            service: service.into(),
        }
    }

    fn entry(&self, key: &str) -> Result<keyring::Entry, AppError> {
        keyring::Entry::new(&self.service, key)
            .map_err(|error| AppError::Internal(format!("failed to access OS keychain: {error}")))
    }
}

impl CredentialStore for KeyringCredentialStore {
    fn set(&self, key: &str, value: &str) -> Result<(), AppError> {
        self.entry(key)?
            .set_password(value)
            .map_err(|error| AppError::Internal(format!("failed to store credential: {error}")))
    }

    fn get(&self, key: &str) -> Result<Option<String>, AppError> {
        match self.entry(key)?.get_password() {
            Ok(value) => Ok(Some(value)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(error) => Err(AppError::Internal(format!(
                "failed to read credential: {error}"
            ))),
        }
    }

    fn delete(&self, key: &str) -> Result<(), AppError> {
        match self.entry(key)?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(error) => Err(AppError::Internal(format!(
                "failed to delete credential: {error}"
            ))),
        }
    }
}

/// In-memory credential store for tests.
#[cfg(test)]
pub(crate) struct MemoryCredentialStore {
    values: std::sync::Mutex<std::collections::HashMap<String, String>>,
}

#[cfg(test)]
impl MemoryCredentialStore {
    pub(crate) fn new() -> Self {
        Self {
            values: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }
}

#[cfg(test)]
impl Default for MemoryCredentialStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
impl CredentialStore for MemoryCredentialStore {
    fn set(&self, key: &str, value: &str) -> Result<(), AppError> {
        self.values
            .lock()
            .expect("credential store lock")
            .insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn get(&self, key: &str) -> Result<Option<String>, AppError> {
        Ok(self
            .values
            .lock()
            .expect("credential store lock")
            .get(key)
            .cloned())
    }

    fn delete(&self, key: &str) -> Result<(), AppError> {
        self.values
            .lock()
            .expect("credential store lock")
            .remove(key);
        Ok(())
    }
}
