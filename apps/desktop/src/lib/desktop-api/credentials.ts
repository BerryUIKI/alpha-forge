// Credentials Desktop API.
//
// Secure credential management using OS keychain.
// API keys and other secrets are stored securely in the operating system's
// keychain and never cross the IPC boundary as plaintext.

import { invoke } from "@tauri-apps/api/core";

/**
 * Save a credential to the OS keychain.
 *
 * The credential value is stored securely in:
 * - Keychain Access on macOS
 * - Credential Manager on Windows
 * - Secret Service on Linux
 *
 * @param name - Unique identifier for the credential (e.g., "api_key")
 * @param value - The secret value to store
 * @throws Error if the credential cannot be saved
 */
export async function saveCredential(
  name: string,
  value: string,
): Promise<void> {
  await invoke("save_credential", { name, value });
}

/**
 * Check if a credential exists in the OS keychain.
 *
 * This does NOT retrieve the actual value for security reasons.
 * Use this to check if an API key has been configured.
 *
 * @param name - Unique identifier for the credential
 * @returns true if the credential exists, false otherwise
 */
export async function hasCredential(name: string): Promise<boolean> {
  return await invoke<boolean>("has_credential", { name });
}

/**
 * Delete a credential from the OS keychain.
 *
 * @param name - Unique identifier for the credential
 * @throws Error if the credential cannot be deleted
 */
export async function deleteCredential(name: string): Promise<void> {
  await invoke("delete_credential", { name });
}