// Workspace desktop API.

import { invoke } from "@tauri-apps/api/core";

/**
 * A workspace represents an independent research environment.
 * Types must match Rust domain model exactly (snake_case).
 * Backend: crates/domain/src/workspace.rs
 */
export interface Workspace {
  id: string;
  name: string;
  created_at: string;
  updated_at: string;
}

/**
 * Creates a new workspace.
 * @param name The name of the workspace
 * @returns The created workspace
 */
export async function createWorkspace(name: string): Promise<Workspace> {
  return invoke("create_workspace", { name });
}

/**
 * Lists all workspaces.
 * @returns Array of workspaces
 */
export async function listWorkspaces(): Promise<Workspace[]> {
  return invoke("list_workspaces");
}

/**
 * Gets a workspace by ID.
 * @param id The workspace ID
 * @returns The workspace or null if not found
 */
export async function getWorkspace(id: string): Promise<Workspace | null> {
  return invoke("get_workspace", { id });
}

/**
 * Updates a workspace.
 * @param id The workspace ID
 * @param name The new name
 * @returns The updated workspace
 */
export async function updateWorkspace(id: string, name: string): Promise<Workspace> {
  return invoke("update_workspace", { id, name });
}

/**
 * Deletes a workspace.
 * @param id The workspace ID
 */
export async function deleteWorkspace(id: string): Promise<void> {
  return invoke("delete_workspace", { id });
}