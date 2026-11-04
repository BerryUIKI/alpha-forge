// AI provider desktop API.

import { invoke } from "@tauri-apps/api/core";

/**
 * Provider configuration status. Never contains the API key.
 */
export interface ProviderStatus {
  provider: string;
  configured: boolean;
  model: string;
}

/**
 * Gets the configured provider status.
 */
export async function getProviderStatus(): Promise<ProviderStatus> {
  return invoke("get_provider_status");
}

/**
 * Stores an API key in the OS keychain.
 */
export async function setProviderApiKey(apiKey: string): Promise<void> {
  return invoke("set_provider_api_key", { apiKey });
}

/**
 * Removes the stored API key.
 */
export async function clearProviderApiKey(): Promise<void> {
  return invoke("clear_provider_api_key");
}
