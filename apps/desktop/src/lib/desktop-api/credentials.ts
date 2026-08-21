// Credentials Desktop API — S2 Normalized with strict Zod validation.

import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";

const VoidResponseSchema = z.union([z.null(), z.undefined()]);

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
  const response: unknown = await invoke("save_openai_api_key", { value });
  VoidResponseSchema.parse(response);
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
  const response: unknown = await invoke("has_openai_api_key");
  return z.boolean().parse(response);
}

/**
 * Delete a credential from the OS keychain.
 *
 * @throws Error if the credential cannot be deleted
 */
export async function deleteOpenAiApiKey(): Promise<void> {
  const response: unknown = await invoke("delete_openai_api_key");
  VoidResponseSchema.parse(response);
}
