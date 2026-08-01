// Artifacts desktop API — M3 Artifact Intelligence System.

import { invoke } from "@tauri-apps/api/core";

/**
 * Status of an artifact.
 */
export type ArtifactStatus =
  | "pending"
  | "generating"
  | "completed"
  | "viewing"
  | "closed"
  | "failed";

/**
 * Type of artifact.
 */
export type ArtifactType =
  | "comparison_table"
  | "timeline"
  | "industry_map"
  | "valuation_model"
  | "risk_dashboard"
  | "earnings_analysis"
  | "macro_dashboard"
  | string; // Custom types

/**
 * An artifact represents an interactive research visualization.
 */
export interface Artifact {
  id: string;
  workspace_id: string;
  task_id: string | null;
  artifact_type: ArtifactType;
  status: ArtifactStatus;
  input: unknown;
  output: unknown | null;
  error: string | null;
  created_at: string;
  updated_at: string;
}

/**
 * Input for creating a new artifact.
 */
export interface CreateArtifactInput {
  workspaceId: string;
  taskId?: string;
  artifactType: ArtifactType;
  input: unknown;
}

/**
 * Creates a new artifact.
 */
export async function createArtifact(
  input: CreateArtifactInput
): Promise<Artifact> {
  return invoke("create_artifact", {
    workspaceId: input.workspaceId,
    taskId: input.taskId || null,
    artifactType: input.artifactType,
    input: input.input,
  });
}

/**
 * Gets an artifact by ID.
 */
export async function getArtifact(id: string): Promise<Artifact | null> {
  return invoke("get_artifact", { id });
}

/**
 * Lists all artifacts for a workspace.
 */
export async function listArtifacts(workspaceId: string): Promise<Artifact[]> {
  return invoke("list_artifacts", { workspaceId });
}

/**
 * Lists all artifacts for a task.
 */
export async function listTaskArtifacts(taskId: string): Promise<Artifact[]> {
  return invoke("list_task_artifacts", { taskId });
}

/**
 * Starts artifact generation.
 */
export async function startArtifactGeneration(id: string): Promise<Artifact> {
  return invoke("start_artifact_generation", { id });
}

/**
 * Completes artifact generation with output data.
 */
export async function completeArtifactGeneration(
  id: string,
  output: unknown
): Promise<Artifact> {
  return invoke("complete_artifact_generation", { id, output });
}

/**
 * Marks artifact generation as failed.
 */
export async function failArtifactGeneration(
  id: string,
  error: string
): Promise<Artifact> {
  return invoke("fail_artifact_generation", { id, error });
}

/**
 * Opens an artifact for viewing.
 */
export async function startViewingArtifact(id: string): Promise<Artifact> {
  return invoke("start_viewing_artifact", { id });
}

/**
 * Closes an artifact window.
 */
export async function closeArtifact(id: string): Promise<Artifact> {
  return invoke("close_artifact", { id });
}

/**
 * Deletes an artifact.
 */
export async function deleteArtifact(id: string): Promise<void> {
  return invoke("delete_artifact", { id });
}

/**
 * Lists all open artifact windows.
 */
export async function listOpenArtifacts(): Promise<string[]> {
  return invoke("list_open_artifacts");
}
