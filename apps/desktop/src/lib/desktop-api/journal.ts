// Journal desktop API.

import { invoke } from "@tauri-apps/api/core";

/**
 * List journal entries for a workspace.
 * Note: Backend currently returns an empty array (placeholder).
 * @param workspaceId - The workspace ID to list entries for
 */
export async function listJournalEntries(workspaceId: string): Promise<string[]> {
  return invoke("list_journal_entries", { workspaceId });
}
