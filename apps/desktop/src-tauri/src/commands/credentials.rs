// Credentials Tauri commands.
//
// Secure credential management using OS keychain.
// API keys and other secrets never cross the Rust/React boundary as plaintext.

use crate::error::AppError;
use crate::security::credentials::OsKeychainCredentialStore;

/// Save a credential to the OS keychain.
///
/// The credential value is stored securely in the operating system's
/// keychain (Keychain Access on macOS, Credential Manager on Windows,
/// Secret Service on Linux).
#[tauri::command]
pub async fn save_credential(
    name: String,
    value: String,
) -> Result<(), AppError> {
    // Validate credential name
    if name.trim().is_empty() {
        return Err(AppError::Validation(
            "Credential name cannot be empty".to_string(),
        ));
    }

    // Validate credential value
    if value.trim().is_empty() {
        return Err(AppError::Validation(
            "Credential value cannot be empty".to_string(),
        ));
    }

    OsKeychainCredentialStore.set(&name, &value)?;

    Ok(())
}

/// Check if a credential exists in the OS keychain.
///
/// Returns true if the credential exists, false otherwise.
/// This does NOT return the actual credential value for security.
#[tauri::command]
pub async fn has_credential(name: String) -> Result<bool, AppError> {
    if name.trim().is_empty() {
        return Err(AppError::Validation(
            "Credential name cannot be empty".to_string(),
        ));
    }

    Ok(OsKeychainCredentialStore.get(&name)?.is_some())
}

/// Delete a credential from the OS keychain.
#[tauri::command]
pub async fn delete_credential(name: String) -> Result<(), AppError> {
    if name.trim().is_empty() {
        return Err(AppError::Validation(
            "Credential name cannot be empty".to_string(),
        ));
    }

    OsKeychainCredentialStore.delete(&name)?;

    Ok(())
}