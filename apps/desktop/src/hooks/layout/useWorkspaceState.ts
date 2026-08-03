/**
 * useWorkspaceState Hook
 *
 * Manages workspace selection state with localStorage persistence.
 * Provides state management for workspace switching and view rendering.
 *
 * @module hooks/layout
 */

import { useState, useEffect, useCallback } from "react";
import type { WorkspaceType } from "@/components/layout/types";

interface UseWorkspaceStateConfig {
  /** Storage key for persistence */
  storageKey?: string;
  /** Default workspace */
  defaultWorkspace?: WorkspaceType;
}

interface UseWorkspaceStateReturn {
  /** Currently active workspace */
  activeWorkspace: WorkspaceType;
  /** Set active workspace */
  setActiveWorkspace: (workspace: WorkspaceType) => void;
  /** Workspace names mapping */
  workspaceNames: Record<WorkspaceType, string>;
  /** Whether a specific workspace is active */
  isWorkspaceActive: (workspace: WorkspaceType) => boolean;
}

const WORKSPACE_NAMES: Record<WorkspaceType, string> = {
  "analyze": "Analyze",
  "quantification": "Quantification",
  "comprehensive-market": "Comprehensive Market",
  "options": "Options",
  "futures": "Futures",
  "other-derivatives": "Other Derivatives",
};

/**
 * Hook for managing workspace state with persistence
 */
export function useWorkspaceState({
  storageKey = "active-workspace",
  defaultWorkspace = "analyze",
}: UseWorkspaceStateConfig = {}): UseWorkspaceStateReturn {
  // Initialize state from localStorage or default
  const [activeWorkspace, setActiveWorkspaceState] = useState<WorkspaceType>(() => {
    if (typeof window === "undefined") return defaultWorkspace;
    const stored = localStorage.getItem(storageKey);
    return (stored as WorkspaceType) || defaultWorkspace;
  });

  // Persist state changes
  useEffect(() => {
    localStorage.setItem(storageKey, activeWorkspace);
  }, [activeWorkspace, storageKey]);

  // Set active workspace
  const setActiveWorkspace = useCallback((workspace: WorkspaceType) => {
    setActiveWorkspaceState(workspace);
  }, []);

  // Check if workspace is active
  const isWorkspaceActive = useCallback(
    (workspace: WorkspaceType) => activeWorkspace === workspace,
    [activeWorkspace]
  );

  return {
    activeWorkspace,
    setActiveWorkspace,
    workspaceNames: WORKSPACE_NAMES,
    isWorkspaceActive,
  };
}