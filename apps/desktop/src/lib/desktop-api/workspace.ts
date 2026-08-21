// Workspace desktop API — S2 Normalized with strict Zod validation.

import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";

/**
 * Zod schema for Workspace DTO received over IPC.
 * Strictly enforces camelCase property names from the Rust backend.
 */
export const WorkspaceSchema = z
  .object({
    id: z.string(),
    name: z.string(),
    createdAt: z.string(),
    updatedAt: z.string(),
  })
  .strict();

export type Workspace = z.infer<typeof WorkspaceSchema>;

const VoidResponseSchema = z.union([z.null(), z.undefined()]);

/**
 * Creates a new workspace.
 * @param name The name of the workspace
 * @returns The created workspace
 */
export async function createWorkspace(name: string): Promise<Workspace> {
  const response: unknown = await invoke("create_workspace", { name });
  return WorkspaceSchema.parse(response);
}

/**
 * Lists all workspaces.
 * @returns Array of workspaces
 */
export async function listWorkspaces(): Promise<Workspace[]> {
  const response: unknown = await invoke("list_workspaces");
  return z.array(WorkspaceSchema).parse(response);
}

/**
 * Gets a workspace by ID.
 * @param id The workspace ID
 * @returns The workspace or null if not found
 */
export async function getWorkspace(id: string): Promise<Workspace | null> {
  const response: unknown = await invoke("get_workspace", { id });
  return z.nullable(WorkspaceSchema).parse(response);
}

/**
 * Updates a workspace.
 * @param id The workspace ID
 * @param name The new name
 * @returns The updated workspace
 */
export async function updateWorkspace(id: string, name: string): Promise<Workspace> {
  const response: unknown = await invoke("update_workspace", { id, name });
  return WorkspaceSchema.parse(response);
}

/**
 * Deletes a workspace.
 * @param id The workspace ID
 */
export async function deleteWorkspace(id: string): Promise<void> {
  const response: unknown = await invoke("delete_workspace", { id });
  VoidResponseSchema.parse(response);
}