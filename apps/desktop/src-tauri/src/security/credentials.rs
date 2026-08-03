// Narrow OS-keychain adapter. Credentials never cross the Rust/React boundary.

use crate::error::AppError;

const SERVICE_NAME: &str = "com.berry.investmentos";
const MAX_CREDENTIAL_NAME_LENGTH: usize = 64;
const MAX_SECRET_LENGTH: usize = 16 * 1024;

pub struct OsKeychainCredentialStore;

impl OsKeychainCredentialStore {
    pub fn set(&self, credential_name: &str, secret: &str) -> Result<(), AppError> {
        validate_credential_name(credential_name)?;
        validate_secret(secret)?;
        self.entry(credential_name)?
            .set_password(secret)
            .map_err(|_| keychain_error("store"))
    }

    pub fn get(&self, credential_name: &str) -> Result<Option<String>, AppError> {
        validate_credential_name(credential_name)?;
        match self.entry(credential_name)?.get_password() {
            Ok(secret) => Ok(Some(secret)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(_) => Err(keychain_error("read")),
        }
    }

    pub fn delete(&self, credential_name: &str) -> Result<(), AppError> {
        validate_credential_name(credential_name)?;
        match self.entry(credential_name)?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(_) => Err(keychain_error("delete")),
        }
    }

    fn entry(&self, credential_name: &str) -> Result<keyring::Entry, AppError> {
        keyring::Entry::new(SERVICE_NAME, credential_name).map_err(|_| keychain_error("access"))
    }
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
}
