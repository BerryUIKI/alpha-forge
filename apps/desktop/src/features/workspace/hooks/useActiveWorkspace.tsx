/**
 * useActiveWorkspace — Active Workspace Context
 *
 * Single source of truth for the active research workspace across the app.
 * Implements the switching UX in ADR-0008 (workspace dimensions):
 *
 * - The selection is persisted in localStorage (`active-workspace-id`).
 * - The URL `?workspace=` parameter is a deep-link entry point only: when
 *   present and valid it wins over the stored preference, is persisted, and
 *   is then stripped from the URL so a stale parameter cannot override a
 *   later switch. Otherwise the stored preference is used, falling back to
 *   the first workspace.
 *
 * All research-dimension pages read the active workspace through
 * `useActiveWorkspaceId()`; the Portfolio page ignores it (global dimension).
 *
 * @version GUI-M6
 */

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";
import { useSearchParams } from "react-router-dom";
import { useWorkspaces } from "./useWorkspaces";
import type { Workspace } from "@/lib/desktop-api/workspace";

/** localStorage key that persists the active workspace across sessions. */
const STORAGE_KEY = "active-workspace-id";

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

function readStoredWorkspaceId(): string | null {
  try {
    return localStorage.getItem(STORAGE_KEY);
  } catch {
    // Storage unavailable (private mode / restricted webview); fall back below.
    return null;
  }
}

function persistWorkspaceId(id: string) {
  try {
    localStorage.setItem(STORAGE_KEY, id);
  } catch {
    // Persistence is best-effort; the session still switches.
  }
}

export function ActiveWorkspaceProvider({ children }: { children: ReactNode }) {
  const { data: workspaces = [], isLoading } = useWorkspaces();
  const [searchParams, setSearchParams] = useSearchParams();
  const [workspaceId, setWorkspaceId] = useState("");

  // Resolve the active workspace once the list loads, and re-resolve whenever
  // the URL carries a (new) deep-link workspace parameter.
  useEffect(() => {
    if (workspaces.length === 0) return;

    const urlParam = searchParams.get("workspace");
    const stored = readStoredWorkspaceId();

    let nextId: string;
    if (urlParam && workspaces.some((w) => w.id === urlParam)) {
      // A valid deep-link parameter wins over the stored preference.
      nextId = urlParam;
    } else if (stored && workspaces.some((w) => w.id === stored)) {
      nextId = stored;
    } else {
      // Guarded by the length check above.
      nextId = workspaces[0]!.id;
    }

    setWorkspaceId(nextId);
    persistWorkspaceId(nextId);

    // Consume the deep-link parameter so it cannot override later switches.
    if (searchParams.has("workspace")) {
      const next = new URLSearchParams(searchParams);
      next.delete("workspace");
      setSearchParams(next, { replace: true });
    }
  }, [workspaces, searchParams, setSearchParams]);

  const setActiveWorkspace = useCallback((id: string) => {
    setWorkspaceId(id);
    persistWorkspaceId(id);
  }, []);

  const value = useMemo<ActiveWorkspaceContextValue>(
    () => ({
      workspaceId,
      workspace: workspaces.find((w) => w.id === workspaceId) ?? null,
      workspaces,
      isLoading,
      setActiveWorkspace,
    }),
    [workspaceId, workspaces, isLoading, setActiveWorkspace],
  );

  return (
    <ActiveWorkspaceContext.Provider value={value}>{children}</ActiveWorkspaceContext.Provider>
  );
}

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
