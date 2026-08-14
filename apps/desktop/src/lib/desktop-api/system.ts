import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";

const SystemInfoSchema = z
  .object({
    appName: z.string(),
    appVersion: z.string(),
    platform: z.string(),
    architecture: z.string(),
  })
  .strict();

export interface ReleaseCheck {
  currentVersion: string;
  latestVersion: string;
  releaseUrl: string;
  updateAvailable: boolean;
}

export interface SystemInfo {
  appName: string;
  appVersion: string;
  platform: string;
  architecture: string;
}

export async function getSystemInfo(): Promise<SystemInfo> {
  const response: unknown = await invoke("get_system_info");
  return SystemInfoSchema.parse(response);
}

export function getConfigDir(): Promise<string> {
  return invoke("get_config_dir");
}

export function getDataDir(): Promise<string> {
  return invoke("get_data_dir");
}

export function checkDatabaseHealth(): Promise<string> {
  return invoke("check_database_health");
}

export function exportLocalBackup(): Promise<string | null> {
  return invoke("export_local_backup");
}

export function checkForUpdate(): Promise<ReleaseCheck> {
  return invoke("check_for_update");
}
