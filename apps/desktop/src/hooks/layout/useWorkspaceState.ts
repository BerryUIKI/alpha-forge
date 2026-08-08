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
  /** Workspace names mapping (i18n keys) */
  workspaceLabelKeys: Record<WorkspaceType, string>;
  /** Whether a specific workspace is active */
  isWorkspaceActive: (workspace: WorkspaceType) => boolean;
}

/**
 * i18n label keys for each workspace type
 * These keys should be defined in locale.ts
 */
const WORKSPACE_LABEL_KEYS: Record<WorkspaceType, string> = {
  "analyze": "workspaceTypeAnalyze",
  "quantification": "workspaceTypeQuantification",
  "comprehensive-market": "workspaceTypeComprehensiveMarket",
  "options": "workspaceTypeOptions",
  "futures": "workspaceTypeFutures",
  "other-derivatives": "workspaceTypeOtherDerivatives",
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
    // Validate stored value is a valid WorkspaceType
    if (stored && Object.keys(WORKSPACE_LABEL_KEYS).includes(stored)) {
      return stored as WorkspaceType;
    }
    return defaultWorkspace;
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
    workspaceLabelKeys: WORKSPACE_LABEL_KEYS,
    isWorkspaceActive,
  };
}