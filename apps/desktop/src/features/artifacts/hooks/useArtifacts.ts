// Hooks for artifacts.

import { useQuery } from "@tanstack/react-query";
import { desktopApi, type ArtifactFilters } from "@/lib/desktop-api";

export const ARTIFACT_KEYS = {
  all: ["artifacts"] as const,
  list: (filters: ArtifactFilters = {}) =>
    [...ARTIFACT_KEYS.all, "list", filters.workspaceId ?? null, filters.taskId ?? null] as const,
};

/**
 * Hook to list artifacts, optionally filtered by workspace and task.
 */
export function useArtifacts(filters: ArtifactFilters = {}) {
  return useQuery({
    queryKey: ARTIFACT_KEYS.list(filters),
    queryFn: () => desktopApi.artifacts.listArtifacts(filters),
  });
}
