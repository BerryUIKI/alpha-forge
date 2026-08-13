// Narrow OS-keychain adapter. Credentials never cross the Rust/React boundary.

use crate::error::AppError;

const SERVICE_NAME: &str = "com.berry.investmentos";
const OPENAI_CREDENTIAL_NAME: &str = "openai.api_key";
const LEGACY_OPENAI_CREDENTIAL_NAME: &str = "api_key";
const MAX_CREDENTIAL_NAME_LENGTH: usize = 64;
const MAX_SECRET_LENGTH: usize = 16 * 1024;

pub struct OsKeychainCredentialStore;

pub trait CredentialStore {
    fn set(&self, credential_name: &str, secret: &str) -> Result<(), AppError>;
    fn get(&self, credential_name: &str) -> Result<Option<String>, AppError>;
    fn delete(&self, credential_name: &str) -> Result<(), AppError>;
}

impl CredentialStore for OsKeychainCredentialStore {
    fn set(&self, credential_name: &str, secret: &str) -> Result<(), AppError> {
        validate_credential_name(credential_name)?;
        validate_secret(secret)?;
        self.entry(credential_name)?
            .set_password(secret)
            .map_err(|_| keychain_error("store"))
    }

    fn get(&self, credential_name: &str) -> Result<Option<String>, AppError> {
        validate_credential_name(credential_name)?;
        match self.entry(credential_name)?.get_password() {
            Ok(secret) => Ok(Some(secret)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(_) => Err(keychain_error("read")),
        }
    }

    fn delete(&self, credential_name: &str) -> Result<(), AppError> {
        validate_credential_name(credential_name)?;
        match self.entry(credential_name)?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(_) => Err(keychain_error("delete")),
        }
    }
}

impl OsKeychainCredentialStore {
    fn entry(&self, credential_name: &str) -> Result<keyring::Entry, AppError> {
        keyring::Entry::new(SERVICE_NAME, credential_name).map_err(|_| keychain_error("access"))
    }
}

pub fn save_openai_api_key(store: &impl CredentialStore, api_key: &str) -> Result<(), AppError> {
    store.set(OPENAI_CREDENTIAL_NAME, api_key)?;
    store.delete(LEGACY_OPENAI_CREDENTIAL_NAME)
}

pub fn load_openai_api_key(store: &impl CredentialStore) -> Result<Option<String>, AppError> {
    if let Some(api_key) = store.get(OPENAI_CREDENTIAL_NAME)? {
        return Ok(Some(api_key));
    }

    let Some(api_key) = store.get(LEGACY_OPENAI_CREDENTIAL_NAME)? else {
        return Ok(None);
    };

    store.set(OPENAI_CREDENTIAL_NAME, &api_key)?;
    store.delete(LEGACY_OPENAI_CREDENTIAL_NAME)?;
    Ok(Some(api_key))
}

pub fn has_openai_api_key(store: &impl CredentialStore) -> Result<bool, AppError> {
    Ok(load_openai_api_key(store)?.is_some())
}

pub fn delete_openai_api_key(store: &impl CredentialStore) -> Result<(), AppError> {
    store.delete(LEGACY_OPENAI_CREDENTIAL_NAME)?;
    store.delete(OPENAI_CREDENTIAL_NAME)
}

fn validate_credential_name(value: &str) -> Result<(), AppError> {
    if value.is_empty()
        || value.len() > MAX_CREDENTIAL_NAME_LENGTH
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(AppError::Validation(
            "Credential name must use letters, digits, dots, hyphens, or underscores".to_string(),
        ));
    }
    Ok(())
}

fn validate_secret(value: &str) -> Result<(), AppError> {
    if value.is_empty() || value.len() > MAX_SECRET_LENGTH {
        return Err(AppError::Validation(
            "Credential value must be between 1 and 16384 bytes".to_string(),
        ));
    }
    Ok(())
}

fn keychain_error(operation: &str) -> AppError {
    AppError::Internal(format!(
        "Could not {operation} credential in the operating system keychain"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    #[derive(Default)]
    struct FakeCredentialStore {
        credentials: Mutex<HashMap<String, String>>,
        fail_get: Mutex<Option<String>>,
        fail_set: Mutex<Option<String>>,
        fail_delete: Mutex<Option<String>>,
    }

    impl FakeCredentialStore {
        fn with_credential(name: &str, value: &str) -> Self {
            Self {
                credentials: Mutex::new(HashMap::from([(name.into(), value.into())])),
                ..Self::default()
            }
        }

        fn value(&self, name: &str) -> Option<String> {
            self.credentials.lock().unwrap().get(name).cloned()
        }
    }

    impl CredentialStore for FakeCredentialStore {
        fn set(&self, credential_name: &str, secret: &str) -> Result<(), AppError> {
            if self.fail_set.lock().unwrap().as_deref() == Some(credential_name) {
                return Err(keychain_error("store"));
            }
            self.credentials
                .lock()
                .unwrap()
                .insert(credential_name.into(), secret.into());
            Ok(())
        }

        fn get(&self, credential_name: &str) -> Result<Option<String>, AppError> {
            if self.fail_get.lock().unwrap().as_deref() == Some(credential_name) {
                return Err(keychain_error("read"));
            }
            Ok(self
                .credentials
                .lock()
                .unwrap()
                .get(credential_name)
                .cloned())
        }

        fn delete(&self, credential_name: &str) -> Result<(), AppError> {
            if self.fail_delete.lock().unwrap().as_deref() == Some(credential_name) {
                return Err(keychain_error("delete"));
            }
            self.credentials.lock().unwrap().remove(credential_name);
            Ok(())
        }
    }

    #[test]
    fn accepts_a_safe_credential_name() {
        assert!(validate_credential_name("openai.production-key_1").is_ok());
    }

    #[test]
    fn rejects_unsafe_credential_names() {
        assert!(validate_credential_name("../token").is_err());
        assert!(validate_credential_name("credential name").is_err());
        assert!(validate_credential_name("").is_err());
    }

    #[test]
    fn bounds_credential_values() {
        assert!(validate_secret("token").is_ok());
        assert!(validate_secret("").is_err());
        assert!(validate_secret(&"a".repeat(MAX_SECRET_LENGTH + 1)).is_err());
    }

    #[test]
    fn migrates_the_legacy_openai_key_without_returning_it_to_callers() {
        let store = FakeCredentialStore::with_credential(LEGACY_OPENAI_CREDENTIAL_NAME, "legacy");

        assert!(has_openai_api_key(&store).unwrap());
        assert_eq!(
            store.value(OPENAI_CREDENTIAL_NAME).as_deref(),
            Some("legacy")
        );
        assert_eq!(store.value(LEGACY_OPENAI_CREDENTIAL_NAME), None);
    }

    #[test]
    fn canonical_openai_key_takes_precedence() {
        let store = FakeCredentialStore::with_credential(OPENAI_CREDENTIAL_NAME, "canonical");
        store
            .credentials
            .lock()
            .unwrap()
            .insert(LEGACY_OPENAI_CREDENTIAL_NAME.into(), "legacy".into());

        assert_eq!(
            load_openai_api_key(&store).unwrap().as_deref(),
            Some("canonical")
        );
        assert_eq!(
            store.value(LEGACY_OPENAI_CREDENTIAL_NAME).as_deref(),
            Some("legacy")
        );
    }

    #[test]
    fn failed_migration_preserves_the_legacy_key() {
        let store = FakeCredentialStore::with_credential(LEGACY_OPENAI_CREDENTIAL_NAME, "legacy");
        *store.fail_set.lock().unwrap() = Some(OPENAI_CREDENTIAL_NAME.into());

        assert!(load_openai_api_key(&store).is_err());
        assert_eq!(
            store.value(LEGACY_OPENAI_CREDENTIAL_NAME).as_deref(),
            Some("legacy")
        );
        assert_eq!(store.value(OPENAI_CREDENTIAL_NAME), None);
    }

    #[test]
    fn failed_legacy_cleanup_after_migration_preserves_both_copies() {
        let store = FakeCredentialStore::with_credential(LEGACY_OPENAI_CREDENTIAL_NAME, "legacy");
        *store.fail_delete.lock().unwrap() = Some(LEGACY_OPENAI_CREDENTIAL_NAME.into());

        assert!(load_openai_api_key(&store).is_err());
        assert_eq!(
            store.value(OPENAI_CREDENTIAL_NAME).as_deref(),
            Some("legacy")
        );
        assert_eq!(
            store.value(LEGACY_OPENAI_CREDENTIAL_NAME).as_deref(),
            Some("legacy")
        );
    }

    #[test]
    fn saves_canonical_key_and_removes_legacy_key() {
        let store = FakeCredentialStore::with_credential(LEGACY_OPENAI_CREDENTIAL_NAME, "legacy");

        save_openai_api_key(&store, "new-key").unwrap();

        assert_eq!(
            store.value(OPENAI_CREDENTIAL_NAME).as_deref(),
            Some("new-key")
        );
        assert_eq!(store.value(LEGACY_OPENAI_CREDENTIAL_NAME), None);
    }

    #[test]
    fn failed_save_preserves_the_legacy_key() {
        let store = FakeCredentialStore::with_credential(LEGACY_OPENAI_CREDENTIAL_NAME, "legacy");
        *store.fail_set.lock().unwrap() = Some(OPENAI_CREDENTIAL_NAME.into());

        assert!(save_openai_api_key(&store, "new-key").is_err());
        assert_eq!(
            store.value(LEGACY_OPENAI_CREDENTIAL_NAME).as_deref(),
            Some("legacy")
        );
        assert_eq!(store.value(OPENAI_CREDENTIAL_NAME), None);
    }

    #[test]
    fn reports_missing_keys_and_keychain_errors() {
        let store = FakeCredentialStore::default();
        assert!(!has_openai_api_key(&store).unwrap());

        *store.fail_get.lock().unwrap() = Some(OPENAI_CREDENTIAL_NAME.into());
        assert!(has_openai_api_key(&store).is_err());
    }

    #[test]
    fn deletes_canonical_and_legacy_keys() {
        let store = FakeCredentialStore::with_credential(OPENAI_CREDENTIAL_NAME, "canonical");
        store
            .credentials
            .lock()
            .unwrap()
            .insert(LEGACY_OPENAI_CREDENTIAL_NAME.into(), "legacy".into());

        delete_openai_api_key(&store).unwrap();

        assert_eq!(store.value(OPENAI_CREDENTIAL_NAME), None);
        assert_eq!(store.value(LEGACY_OPENAI_CREDENTIAL_NAME), None);
    }

    #[test]
    fn failed_legacy_delete_keeps_the_canonical_key_available() {
        let store = FakeCredentialStore::with_credential(OPENAI_CREDENTIAL_NAME, "canonical");
        store
            .credentials
            .lock()
            .unwrap()
            .insert(LEGACY_OPENAI_CREDENTIAL_NAME.into(), "legacy".into());
        *store.fail_delete.lock().unwrap() = Some(LEGACY_OPENAI_CREDENTIAL_NAME.into());

        assert!(delete_openai_api_key(&store).is_err());
        assert_eq!(
            store.value(OPENAI_CREDENTIAL_NAME).as_deref(),
            Some("canonical")
        );
        assert_eq!(
            store.value(LEGACY_OPENAI_CREDENTIAL_NAME).as_deref(),
            Some("legacy")
        );
    }
}
