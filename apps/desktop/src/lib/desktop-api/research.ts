// Research desktop API.

import { invoke } from "@tauri-apps/api/core";

export type ResearchDocumentKind = "note" | "web" | "pdf" | "agent";
export type ResearchDocumentFormat = "markdown" | "text";

export interface ResearchDocument {
  id: string;
  workspace_id: string;
  title: string;
  content: string | null;
  kind: ResearchDocumentKind;
  format: ResearchDocumentFormat;
  summary: string | null;
  archived_at: string | null;
  created_at: string;
  updated_at: string;
}

export async function listResearchDocuments(workspaceId: string): Promise<ResearchDocument[]> {
  return invoke("list_research_documents", { workspaceId });
}

export async function createResearchDocument(
  workspaceId: string,
  title: string,
  content?: string | null,
): Promise<ResearchDocument> {
  return invoke("create_research_document", {
    workspaceId,
    title,
    content: content || null,
  });
}

export async function getResearchDocument(id: string): Promise<ResearchDocument | null> {
  return invoke("get_research_document", { id });
}

export async function updateResearchDocument(
  id: string,
  title: string,
  content?: string | null,
): Promise<ResearchDocument> {
  return invoke("update_research_document", {
    id,
    title,
    content: content || null,
  });
}

export async function deleteResearchDocument(id: string): Promise<void> {
  return invoke("delete_research_document", { id });
}

/**
 * Permanently deletes an archived document and its sources/notes.
 */
export async function purgeResearchDocument(id: string): Promise<void> {
  return invoke("purge_research_document", { id });
}
