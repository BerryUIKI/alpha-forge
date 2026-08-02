import { invoke } from "@tauri-apps/api/core";

export interface ReleaseCheck {
  currentVersion: string;
  latestVersion: string;
  releaseUrl: string;
  updateAvailable: boolean;
}

export function exportLocalBackup(): Promise<string | null> {
  return invoke("export_local_backup");
}

export function checkForUpdate(): Promise<ReleaseCheck> {
  return invoke("check_for_update");
}
