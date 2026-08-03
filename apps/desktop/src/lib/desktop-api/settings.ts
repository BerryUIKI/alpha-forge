// Settings desktop API.

import { invoke } from "@tauri-apps/api/core";

export interface AppInfo {
  name: string;
  version: string;
  identifier: string;
}

export interface SettingItem {
  key: string;
  value: string;
}

export async function healthCheck(): Promise<string> {
  return invoke("health_check");
}

export async function getAppInfo(): Promise<AppInfo> {
  return invoke("get_app_info");
}

export async function getSetting(key: string): Promise<string | null> {
  return invoke("get_setting", { key });
}

export async function setSetting(key: string, value: string): Promise<void> {
  return invoke("set_setting", { key, value });
}

export async function deleteSetting(key: string): Promise<void> {
  return invoke("delete_setting", { key });
}

export async function listSettings(): Promise<SettingItem[]> {
  return invoke("list_settings");
}
