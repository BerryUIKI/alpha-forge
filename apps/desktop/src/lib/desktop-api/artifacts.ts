// Artifacts desktop API — M3 Artifact Intelligence System.

import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";

/**
 * Status of an artifact.
 */
export const ArtifactStatusSchema = z.enum([
  "pending",
  "generating",
  "completed",
  "viewing",
  "closed",
  "failed",
]);

export type ArtifactStatus = z.infer<typeof ArtifactStatusSchema>;

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
export const ArtifactSchema = z
  .object({
    id: z.string().min(1),
    workspaceId: z.string().min(1),
    taskId: z.string().nullable(),
    artifactType: z.string().min(1),
    status: ArtifactStatusSchema,
    input: z.unknown(),
    output: z.unknown().nullable(),
    error: z.string().nullable(),
    createdAt: z.string().min(1),
    updatedAt: z.string().min(1),
  })
  .strict();

export type Artifact = z.infer<typeof ArtifactSchema>;

/**
 * Input for creating a new artifact.
 */
export interface CreateArtifactInput {
  workspaceId: string;
  taskId?: string;
  artifactType: ArtifactType;
  input: unknown;
}

const VoidResponseSchema = z.union([z.null(), z.undefined()]);

/**
 * Creates a new artifact.
 */
export async function createArtifact(
  input: CreateArtifactInput
): Promise<Artifact> {
  const response: unknown = await invoke("create_artifact", {
    workspaceId: input.workspaceId,
    taskId: input.taskId || null,
    artifactType: input.artifactType,
    input: input.input,
  });
  return ArtifactSchema.parse(response);
}

/**
 * Gets an artifact by ID.
 */
export async function getArtifact(id: string): Promise<Artifact | null> {
  const response: unknown = await invoke("get_artifact", { id });
  return z.nullable(ArtifactSchema).parse(response);
}

/**
 * Lists all artifacts for a workspace.
 */
export async function listArtifacts(workspaceId: string): Promise<Artifact[]> {
  const response: unknown = await invoke("list_artifacts", { workspaceId });
  return z.array(ArtifactSchema).parse(response);
}

/**
 * Lists all artifacts for a task.
 */
export async function listTaskArtifacts(taskId: string): Promise<Artifact[]> {
  const response: unknown = await invoke("list_task_artifacts", { taskId });
  return z.array(ArtifactSchema).parse(response);
}

/**
 * Starts artifact generation.
 */
export async function startArtifactGeneration(id: string): Promise<Artifact> {
  const response: unknown = await invoke("start_artifact_generation", { id });
  return ArtifactSchema.parse(response);
}

/**
 * Completes artifact generation with output data.
 */
export async function completeArtifactGeneration(
  id: string,
  output: unknown
): Promise<Artifact> {
  const response: unknown = await invoke("complete_artifact_generation", { id, output });
  return ArtifactSchema.parse(response);
}

/**
 * Marks artifact generation as failed.
 */
export async function failArtifactGeneration(
  id: string,
  error: string
): Promise<Artifact> {
  const response: unknown = await invoke("fail_artifact_generation", { id, error });
  return ArtifactSchema.parse(response);
}

/**
 * Opens an artifact for viewing.
 */
export async function startViewingArtifact(id: string): Promise<Artifact> {
  const response: unknown = await invoke("start_viewing_artifact", { id });
  return ArtifactSchema.parse(response);
}

/**
 * Closes an artifact window.
 */
export async function closeArtifact(id: string): Promise<Artifact> {
  const response: unknown = await invoke("close_artifact", { id });
  return ArtifactSchema.parse(response);
}

/**
 * Deletes an artifact.
 */
export async function deleteArtifact(id: string): Promise<void> {
  const response: unknown = await invoke("delete_artifact", { id });
  VoidResponseSchema.parse(response);
}

/**
 * Lists all open artifact windows.
 */
export async function listOpenArtifacts(): Promise<string[]> {
  const response: unknown = await invoke("list_open_artifacts");
  return z.array(z.string()).parse(response);
}
