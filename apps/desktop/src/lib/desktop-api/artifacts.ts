// Artifacts desktop API.

import { invoke } from "@tauri-apps/api/core";

export async function listArtifacts(): Promise<string[]> {
  return invoke("list_artifacts");
}
