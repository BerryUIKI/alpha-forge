// Settings desktop API.

import { invoke } from "@tauri-apps/api/core";

export interface AppInfo {
  name: string;
  version: string;
  identifier: string;
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
