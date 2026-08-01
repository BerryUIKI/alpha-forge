import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import type { CreateThesisParams, EvidenceDirection } from "@/lib/desktop-api/thesis";

const THESIS_KEYS = {
  all: ["theses"] as const,
  list: (workspaceId: string) => [...THESIS_KEYS.all, "list", workspaceId] as const,
  evidence: (thesisId: string) => [...THESIS_KEYS.all, "evidence", thesisId] as const,
  confidenceHistory: (thesisId: string) =>
    [...THESIS_KEYS.all, "confidence-history", thesisId] as const,
};

export function useTheses(workspaceId: string) {
  return useQuery({
    queryKey: THESIS_KEYS.list(workspaceId),
    queryFn: () => desktopApi.thesis.listTheses(workspaceId),
    enabled: Boolean(workspaceId),
  });
}

export function useThesisEvidence(thesisId: string) {
  return useQuery({
    queryKey: THESIS_KEYS.evidence(thesisId),
    queryFn: () => desktopApi.thesis.listThesisEvidence(thesisId),
    enabled: Boolean(thesisId),
  });
}

export function useThesisConfidenceHistory(thesisId: string) {
  return useQuery({
    queryKey: THESIS_KEYS.confidenceHistory(thesisId),
    queryFn: () => desktopApi.thesis.listThesisConfidenceHistory(thesisId),
    enabled: Boolean(thesisId),
  });
}

export function useCreateThesis() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: CreateThesisParams) => desktopApi.thesis.createThesis(input),
    onSuccess: (_, input) => queryClient.invalidateQueries({ queryKey: THESIS_KEYS.list(input.workspaceId) }),
  });
}

function useThesisMutation<T>(mutationFn: (input: T) => Promise<unknown>) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: THESIS_KEYS.all }),
  });
}

export function useActivateThesis() {
  return useThesisMutation((id: string) => desktopApi.thesis.activateThesis(id));
}

export function useStartThesisValidation() {
  return useThesisMutation((id: string) => desktopApi.thesis.startThesisValidation(id));
}

export function useCompleteThesisValidation() {
  return useThesisMutation(({ id, outcome, validated }: { id: string; outcome: string; validated: boolean }) =>
    desktopApi.thesis.completeThesisValidation(id, outcome, validated)
  );
}

export function useUpdateThesisConfidence() {
  return useThesisMutation(({ thesisId, confidence }: { thesisId: string; confidence: number }) =>
    desktopApi.thesis.updateThesisConfidence(thesisId, confidence)
  );
}

export function useCloseThesis() {
  return useThesisMutation((id: string) => desktopApi.thesis.closeThesis(id));
}

export function useDeleteThesis() {
  return useThesisMutation((id: string) => desktopApi.thesis.deleteThesis(id));
}

export function useAddThesisEvidence() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ thesisId, direction, evidence, sourceId }: { thesisId: string; direction: EvidenceDirection; evidence: string; sourceId?: string }) =>
      desktopApi.thesis.addThesisEvidence(thesisId, direction, evidence, sourceId),
    onSuccess: (_, input) => {
      queryClient.invalidateQueries({ queryKey: THESIS_KEYS.evidence(input.thesisId) });
      queryClient.invalidateQueries({ queryKey: THESIS_KEYS.all });
    },
  });
}

export function useDeleteThesisEvidence() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id }: { id: string; thesisId: string }) => desktopApi.thesis.deleteThesisEvidence(id),
    onSuccess: (_, input) => {
      queryClient.invalidateQueries({ queryKey: THESIS_KEYS.evidence(input.thesisId) });
      queryClient.invalidateQueries({ queryKey: THESIS_KEYS.all });
    },
  });
}
