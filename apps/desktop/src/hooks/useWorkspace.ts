/**
 * Workspace Data Fetching Hooks
 *
 * Provides TanStack Query hooks for workspace operations.
 * Follows integration standards from docs/FRONTEND_BACKEND_INTEGRATION.md
 *
 * Backend Commands: src-tauri/src/commands/workspace.rs
 * Domain Types: crates/domain/src/workspace.rs
 * API Layer: src/lib/desktop-api/workspace.ts
 *
 * @module hooks/workspace
 */

import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import type { Locale } from "@/lib/i18n/locale";
import { processErrorResponse } from "@/lib/i18n/errorMessages";
import type { Workspace } from "@/lib/desktop-api/workspace";

// Re-export Workspace type for convenience
export type { Workspace } from "@/lib/desktop-api/workspace";

// ============================================================================
// Query Hooks
// ============================================================================

/**
 * Hook to fetch all workspaces
 *
 * @example
 * const { data: workspaces, isLoading, error } = useWorkspaces();
 */
export function useWorkspaces() {
  return useQuery({
    queryKey: ["workspaces"],
    queryFn: () => desktopApi.workspace.listWorkspaces(),
    staleTime: 5000, // 5 seconds
  });
}

/**
 * Hook to fetch a single workspace by ID
 *
 * @param id - Workspace ID
 * @param enabled - Whether to enable the query (default: true if id is provided)
 *
 * @example
 * const { data: workspace } = useWorkspace(workspaceId);
 */
export function useWorkspace(id: string, enabled = !!id) {
  return useQuery({
    queryKey: ["workspace", id],
    queryFn: () => desktopApi.workspace.getWorkspace(id),
    enabled,
    staleTime: 5000,
  });
}

// ============================================================================
// Mutation Hooks
// ============================================================================

/**
 * Hook to create a new workspace
 *
 * @example
 * const createMutation = useCreateWorkspace();
 * createMutation.mutate("My Workspace");
 */
export function useCreateWorkspace(locale: Locale) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (name: string) =>
      desktopApi.workspace.createWorkspace(name),
    onSuccess: (data) => {
      // Invalidate workspaces list to trigger refetch
      queryClient.invalidateQueries({ queryKey: ["workspaces"] });
      // Optionally update the individual workspace cache
      queryClient.setQueryData(["workspace", data.id], data);
    },
    onError: (error) => {
      // Error is logged but UI handling is left to the component
      console.error("Failed to create workspace:", error);
    },
  });
}

/**
 * Hook to update an existing workspace
 *
 * @example
 * const updateMutation = useUpdateWorkspace();
 * updateMutation.mutate({ id: workspaceId, name: "New Name" });
 */
export function useUpdateWorkspace(locale: Locale) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, name }: { id: string; name: string }) =>
      desktopApi.workspace.updateWorkspace(id, name),
    onSuccess: (data) => {
      // Update the workspace in cache
      queryClient.setQueryData(["workspace", data.id], data);
      // Invalidate workspaces list
      queryClient.invalidateQueries({ queryKey: ["workspaces"] });
    },
    onError: (error) => {
      console.error("Failed to update workspace:", error);
    },
  });
}

/**
 * Hook to delete a workspace
 *
 * @example
 * const deleteMutation = useDeleteWorkspace();
 * deleteMutation.mutate(workspaceId);
 */
export function useDeleteWorkspace(locale: Locale) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) =>
      desktopApi.workspace.deleteWorkspace(id),
    onSuccess: () => {
      // Invalidate workspaces list
      queryClient.invalidateQueries({ queryKey: ["workspaces"] });
    },
    onError: (error) => {
      console.error("Failed to delete workspace:", error);
    },
  });
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Process workspace error for display
 *
 * @param locale - Current locale
 * @param error - Error from mutation
 * @returns Localized error messages
 */
export function processWorkspaceError(locale: Locale, error: unknown) {
  return processErrorResponse(locale, error as any);
}