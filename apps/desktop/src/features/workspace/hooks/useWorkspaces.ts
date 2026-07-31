// Workspace hooks using TanStack Query.

import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";

const WORKSPACE_KEYS = {
  all: ["workspaces"] as const,
  lists: () => [...WORKSPACE_KEYS.all, "list"] as const,
  detail: (id: string) => [...WORKSPACE_KEYS.all, "detail", id] as const,
};

/**
 * Hook to list all workspaces.
 */
export function useWorkspaces() {
  return useQuery({
    queryKey: WORKSPACE_KEYS.lists(),
    queryFn: () => desktopApi.workspace.listWorkspaces(),
  });
}

/**
 * Hook to get a single workspace.
 */
export function useWorkspace(id: string) {
  return useQuery({
    queryKey: WORKSPACE_KEYS.detail(id),
    queryFn: () => desktopApi.workspace.getWorkspace(id),
    enabled: !!id,
  });
}

/**
 * Hook to create a workspace.
 */
export function useCreateWorkspace() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (name: string) => desktopApi.workspace.createWorkspace(name),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: WORKSPACE_KEYS.lists() });
    },
  });
}

/**
 * Hook to update a workspace.
 */
export function useUpdateWorkspace() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, name }: { id: string; name: string }) =>
      desktopApi.workspace.updateWorkspace(id, name),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: WORKSPACE_KEYS.lists() });
      queryClient.invalidateQueries({ queryKey: WORKSPACE_KEYS.detail(id) });
    },
  });
}

/**
 * Hook to delete a workspace.
 */
export function useDeleteWorkspace() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => desktopApi.workspace.deleteWorkspace(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: WORKSPACE_KEYS.lists() });
    },
  });
}