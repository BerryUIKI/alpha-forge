// Hooks for artifacts.

import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import type { ArtifactType } from "@/lib/desktop-api/artifacts";
import type { CompanyComparisonPayload } from "@/lib/desktop-api/plugins";

const ARTIFACT_KEYS = {
  all: ["artifacts"] as const,
  workspace: (workspaceId: string) =>
    [...ARTIFACT_KEYS.all, "workspace", workspaceId] as const,
  task: (taskId: string) => [...ARTIFACT_KEYS.all, "task", taskId] as const,
  artifact: (id: string) => [...ARTIFACT_KEYS.all, "artifact", id] as const,
  open: () => [...ARTIFACT_KEYS.all, "open"] as const,
};

/**
 * Hook to list artifacts for a workspace.
 */
export function useArtifacts(workspaceId: string) {
  return useQuery({
    queryKey: ARTIFACT_KEYS.workspace(workspaceId),
    queryFn: () => desktopApi.artifacts.listArtifacts(workspaceId),
    enabled: !!workspaceId,
  });
}

/**
 * Hook to list artifacts for a task.
 */
export function useTaskArtifacts(taskId: string) {
  return useQuery({
    queryKey: ARTIFACT_KEYS.task(taskId),
    queryFn: () => desktopApi.artifacts.listTaskArtifacts(taskId),
    enabled: !!taskId,
  });
}

/**
 * Hook to get a single artifact.
 */
export function useArtifact(id: string) {
  return useQuery({
    queryKey: ARTIFACT_KEYS.artifact(id),
    queryFn: () => desktopApi.artifacts.getArtifact(id),
    enabled: !!id,
  });
}

/**
 * Hook to list open artifact windows.
 */
export function useOpenArtifacts() {
  return useQuery({
    queryKey: ARTIFACT_KEYS.open(),
    queryFn: () => desktopApi.artifacts.listOpenArtifacts(),
  });
}

/**
 * Hook to create an artifact.
 */
export function useCreateArtifact() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      workspaceId,
      taskId,
      artifactType,
      input,
    }: {
      workspaceId: string;
      taskId?: string;
      artifactType: ArtifactType;
      input: unknown;
    }) =>
      desktopApi.artifacts.createArtifact({
        workspaceId,
        taskId,
        artifactType,
        input,
      }),
    onSuccess: (_, { workspaceId }) => {
      queryClient.invalidateQueries({
        queryKey: ARTIFACT_KEYS.workspace(workspaceId),
      });
    },
  });
}

/**
 * Creates a completed Artifact through the controlled company-comparison plugin.
 */
export function useCreateCompanyComparisonArtifact() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      workspaceId,
      input,
    }: {
      workspaceId: string;
      input: CompanyComparisonPayload;
    }) => desktopApi.plugins.createPluginArtifact(workspaceId, "company-comparison", input),
    onSuccess: (artifact, { workspaceId }) => {
      queryClient.setQueryData(ARTIFACT_KEYS.artifact(artifact.id), artifact);
      void queryClient.invalidateQueries({
        queryKey: ARTIFACT_KEYS.workspace(workspaceId),
      });
    },
  });
}

/**
 * Hook to start artifact generation.
 */
export function useStartArtifactGeneration() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => desktopApi.artifacts.startArtifactGeneration(id),
    onSuccess: (_, id) => {
      queryClient.invalidateQueries({ queryKey: ARTIFACT_KEYS.artifact(id) });
    },
  });
}

/**
 * Hook to complete artifact generation.
 */
export function useCompleteArtifactGeneration() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      id,
      output,
    }: {
      id: string;
      output: unknown;
    }) => desktopApi.artifacts.completeArtifactGeneration(id, output),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: ARTIFACT_KEYS.artifact(id) });
    },
  });
}

/**
 * Hook to fail artifact generation.
 */
export function useFailArtifactGeneration() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, error }: { id: string; error: string }) =>
      desktopApi.artifacts.failArtifactGeneration(id, error),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: ARTIFACT_KEYS.artifact(id) });
    },
  });
}

/**
 * Hook to open an artifact for viewing.
 */
export function useStartViewingArtifact() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => desktopApi.artifacts.startViewingArtifact(id),
    onSuccess: (_, id) => {
      queryClient.invalidateQueries({ queryKey: ARTIFACT_KEYS.artifact(id) });
      queryClient.invalidateQueries({ queryKey: ARTIFACT_KEYS.open() });
    },
  });
}

/**
 * Hook to close an artifact.
 */
export function useCloseArtifact() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => desktopApi.artifacts.closeArtifact(id),
    onSuccess: (_, id) => {
      queryClient.invalidateQueries({ queryKey: ARTIFACT_KEYS.artifact(id) });
      queryClient.invalidateQueries({ queryKey: ARTIFACT_KEYS.open() });
    },
  });
}

/**
 * Hook to delete an artifact.
 */
export function useDeleteArtifact() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => desktopApi.artifacts.deleteArtifact(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ARTIFACT_KEYS.all });
    },
  });
}
