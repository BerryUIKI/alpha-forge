// Hooks for research documents.

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";

export const RESEARCH_KEYS = {
  all: ["research"] as const,
  documents: (workspaceId: string) => [...RESEARCH_KEYS.all, "documents", workspaceId] as const,
  document: (id: string) => [...RESEARCH_KEYS.all, "document", id] as const,
};

/**
 * Hook to list research documents in a workspace.
 */
export function useResearchDocuments(workspaceId: string) {
  return useQuery({
    queryKey: RESEARCH_KEYS.documents(workspaceId),
    queryFn: () => desktopApi.research.listResearchDocuments(workspaceId),
    enabled: !!workspaceId,
  });
}

/**
 * Hook to create a research document.
 */
export function useCreateResearchDocument() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      workspaceId,
      title,
      content,
    }: {
      workspaceId: string;
      title: string;
      content?: string;
    }) => desktopApi.research.createResearchDocument(workspaceId, title, content),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: RESEARCH_KEYS.all });
    },
  });
}

/**
 * Hook to update a research document.
 */
export function useUpdateResearchDocument() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, title, content }: { id: string; title: string; content?: string }) =>
      desktopApi.research.updateResearchDocument(id, title, content),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: RESEARCH_KEYS.all });
    },
  });
}

/**
 * Hook to delete a research document.
 */
export function useDeleteResearchDocument() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => desktopApi.research.deleteResearchDocument(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: RESEARCH_KEYS.all });
    },
  });
}
