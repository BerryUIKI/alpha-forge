// Settings desktop API — S2 Normalized with strict Zod validation.

import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";

export const AppInfoSchema = z
  .object({
    name: z.string(),
    version: z.string(),
    identifier: z.string(),
  })
  .strict();

export type AppInfo = z.infer<typeof AppInfoSchema>;

export const SettingItemSchema = z
  .object({
    key: z.string(),
    value: z.string(),
  })
  .strict();

export type SettingItem = z.infer<typeof SettingItemSchema>;

const VoidResponseSchema = z.union([z.null(), z.undefined()]);

export async function healthCheck(): Promise<string> {
  const response: unknown = await invoke("health_check");
  return z.string().parse(response);
}

export async function getAppInfo(): Promise<AppInfo> {
  const response: unknown = await invoke("get_app_info");
  return AppInfoSchema.parse(response);
}

export async function getSetting(key: string): Promise<string | null> {
  const response: unknown = await invoke("get_setting", { key });
  return z.nullable(z.string()).parse(response);
}

export async function setSetting(key: string, value: string): Promise<void> {
  const response: unknown = await invoke("set_setting", { key, value });
  VoidResponseSchema.parse(response);
}

export async function deleteSetting(key: string): Promise<void> {
  const response: unknown = await invoke("delete_setting", { key });
  VoidResponseSchema.parse(response);
}

export async function listSettings(): Promise<SettingItem[]> {
  const response: unknown = await invoke("list_settings");
  return z.array(SettingItemSchema).parse(response);
}
