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
 * @param value - The secret value to store
 * @throws Error if the credential cannot be saved
 */
export async function saveOpenAiApiKey(value: string): Promise<void> {
  await invoke("save_openai_api_key", { value });
}

/**
 * Check if a credential exists in the OS keychain.
 *
 * This does NOT retrieve the actual value for security reasons.
 * Use this to check if an API key has been configured.
 *
 * @returns true if the credential exists, false otherwise
 */
export async function hasOpenAiApiKey(): Promise<boolean> {
  return await invoke<boolean>("has_openai_api_key");
}

/**
 * Delete a credential from the OS keychain.
 *
 * @throws Error if the credential cannot be deleted
 */
export async function deleteOpenAiApiKey(): Promise<void> {
  await invoke("delete_openai_api_key");
}
