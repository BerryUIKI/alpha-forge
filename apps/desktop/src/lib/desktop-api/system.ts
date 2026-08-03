import { invoke } from "@tauri-apps/api/core";

export interface ReleaseCheck {
  currentVersion: string;
  latestVersion: string;
  releaseUrl: string;
  updateAvailable: boolean;
}

export interface SystemInfo {
  os: string;
  arch: string;
  version: string;
}

export function getSystemInfo(): Promise<SystemInfo> {
  return invoke("get_system_info");
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
