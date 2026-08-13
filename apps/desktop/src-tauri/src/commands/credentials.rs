// Credentials Tauri commands.
//
// Secure credential management using OS keychain.
// API keys and other secrets never cross the Rust/React boundary as plaintext.

use crate::error::AppError;
use crate::security::credentials::{
    delete_openai_api_key as delete_key, has_openai_api_key as has_key,
    save_openai_api_key as save_key, OsKeychainCredentialStore,
};

/// Save a credential to the OS keychain.
///
/// The credential value is stored securely in the operating system's
/// keychain (Keychain Access on macOS, Credential Manager on Windows,
/// Secret Service on Linux).
#[tauri::command]
pub async fn save_openai_api_key(value: String) -> Result<(), AppError> {
    if value.trim().is_empty() {
        return Err(AppError::Validation(
            "OpenAI API key cannot be empty".to_string(),
        ));
    }

    save_key(&OsKeychainCredentialStore, &value)
}

/// Check if a credential exists in the OS keychain.
///
/// Returns true if the credential exists, false otherwise.
/// This does NOT return the actual credential value for security.
#[tauri::command]
pub async fn has_openai_api_key() -> Result<bool, AppError> {
    has_key(&OsKeychainCredentialStore)
}

/// Delete a credential from the OS keychain.
#[tauri::command]
pub async fn delete_openai_api_key() -> Result<(), AppError> {
    delete_key(&OsKeychainCredentialStore)
}
