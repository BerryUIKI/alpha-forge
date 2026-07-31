// Journal desktop API.

import { invoke } from "@tauri-apps/api/core";

export async function listTheses(): Promise<string[]> {
  return invoke("list_theses");
}
