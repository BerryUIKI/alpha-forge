// Research desktop API.

import { invoke } from "@tauri-apps/api/core";

export async function listResearchDocuments(): Promise<string[]> {
  return invoke("list_research_documents");
}
