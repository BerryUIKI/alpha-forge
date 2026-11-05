// Artifacts desktop API.

import { invoke } from "@tauri-apps/api/core";

export type ArtifactStatus = "pending" | "generating" | "completed" | "failed";

/**
 * A validated, renderable artifact produced by an agent task.
 * `data` is a JSON string matching the artifact type's schema.
 */
export interface Artifact {
  id: string;
  task_id: string | null;
  workspace_id: string | null;
  artifact_type: string;
  title: string;
  data: string;
  status: ArtifactStatus;
  created_at: string;
  updated_at: string;
}

export interface ArtifactFilters {
  workspaceId?: string;
  taskId?: string;
}

/**
 * Lists artifacts, optionally filtered by workspace and task.
 */
export async function listArtifacts(filters: ArtifactFilters = {}): Promise<Artifact[]> {
  return invoke("list_artifacts", {
    workspaceId: filters.workspaceId ?? null,
    taskId: filters.taskId ?? null,
  });
}
