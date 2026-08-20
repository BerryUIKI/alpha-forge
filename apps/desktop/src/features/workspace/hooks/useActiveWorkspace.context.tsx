/**
 * ActiveWorkspaceContext — Type definitions, context, and hooks
 *
 * Extracted to a separate file per react-refresh best practices.
 *
 * @version GUI-M6
 */

import { createContext, useContext } from "react";
import type { Workspace } from "@/lib/desktop-api/workspace";

export interface ActiveWorkspaceContextValue {
  /** Id of the active workspace ("" while loading or when none exist). */
  workspaceId: string;
  /** The active workspace object, or null while loading / when none exist. */
  workspace: Workspace | null;
  /** All workspaces, for the global switcher. */
  workspaces: Workspace[];
  /** True while the workspace list is still loading. */
  isLoading: boolean;
  /** Switch the active workspace (persists to localStorage). */
  setActiveWorkspace: (id: string) => void;
}

export const ActiveWorkspaceContext = createContext<ActiveWorkspaceContextValue | null>(null);

/**
 * Reads the active workspace context. Must be used inside ActiveWorkspaceProvider.
 */
export function useActiveWorkspace() {
  const ctx = useContext(ActiveWorkspaceContext);
  if (!ctx) {
    throw new Error("useActiveWorkspace must be used within an ActiveWorkspaceProvider");
  }
  return ctx;
}

/**
 * Convenience selector for the active workspace id.
 */
export function useActiveWorkspaceId(): string {
  return useActiveWorkspace().workspaceId;
}

// Re-export the provider component from the main file
export { ActiveWorkspaceProvider } from "./useActiveWorkspace";
