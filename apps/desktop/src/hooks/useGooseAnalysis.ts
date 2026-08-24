/**
 * Goose Analysis Hooks
 *
 * Provides TanStack Query hooks for Goose shadow-mode analysis and human-approved proposals (M10).
 * Follows integration standards from docs/FRONTEND_BACKEND_INTEGRATION.md
 *
 * Backend Commands: src-tauri/src/commands/goose.rs
 * Service: src-tauri/src/services/goose_service.rs, proposal_service.rs
 * API Layer: src/lib/desktop-api/goose.ts
 *
 * @module hooks/goose
 */

import { useEffect, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import { processAppError } from "@/lib/errors";
import type {
  StartShadowAnalysisInput,
  ShadowAnalysisResult,
  Proposal,
  ProposalStatus,
} from "@/lib/desktop-api/goose";

// Re-export types for convenience
export type {
  StartShadowAnalysisInput,
  ShadowAnalysisResult,
  GooseHealthStatus,
  StructuredResponse,
  Claim,
  Evidence,
  Risk,
  Proposal,
  ProposalType,
  ProposalStatus,
  CreateProposalInput,
} from "@/lib/desktop-api/goose";

// ============================================================================
// Query Hooks
// ============================================================================

/**
 * Hook to check Goose service health
 */
export function useGooseHealth() {
  return useQuery({
    queryKey: ["goose", "health"],
    queryFn: () => desktopApi.goose.checkGooseHealth(),
    staleTime: 30000, // 30 seconds
    retry: false, // Don't retry health checks
  });
}

/**
 * Hook to list proposals for a workspace (M10-G4)
 */
export function useGooseProposals(workspaceId: string, status?: ProposalStatus) {
  return useQuery({
    queryKey: ["goose", "proposals", workspaceId, status ?? "all"],
    queryFn: () => desktopApi.goose.listProposals(workspaceId, status),
    enabled: Boolean(workspaceId),
    staleTime: 10000,
  });
}

// ============================================================================
// Mutation Hooks
// ============================================================================

/**
 * Hook to start a Goose shadow analysis
 */
export function useStartShadowAnalysis() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (input: StartShadowAnalysisInput) =>
      desktopApi.goose.startShadowAnalysis(input),
    onSuccess: (result: ShadowAnalysisResult) => {
      queryClient.invalidateQueries({ queryKey: ["goose", "analyses"] });
      queryClient.setQueryData(
        ["goose", "analysis", result.run_id],
        result
      );
    },
  });
}

/**
 * Hook to cancel a running Goose analysis
 */
export function useCancelAnalysis() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (runId: string) => desktopApi.goose.cancelAnalysis(runId),
    onSuccess: (_data, runId) => {
      queryClient.invalidateQueries({ queryKey: ["goose", "analysis", runId] });
    },
  });
}

/**
 * Hook to accept an agent proposal and commit domain changes (M10-G4)
 */
export function useAcceptProposal() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (proposalId: string) => desktopApi.goose.acceptProposal(proposalId),
    onSuccess: (proposal: Proposal) => {
      queryClient.invalidateQueries({ queryKey: ["goose", "proposals"] });
      queryClient.invalidateQueries({ queryKey: ["theses"] });
      queryClient.invalidateQueries({ queryKey: ["notes"] });
    },
  });
}

/**
 * Hook to reject an agent proposal (M10-G4)
 */
export function useRejectProposal() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (proposalId: string) => desktopApi.goose.rejectProposal(proposalId),
    onSuccess: (_proposal: Proposal) => {
      queryClient.invalidateQueries({ queryKey: ["goose", "proposals"] });
    },
  });
}

// ============================================================================
// Combined Hook
// ============================================================================

/**
 * Combined hook for Goose shadow analysis operations
 */
export function useGooseShadowAnalysis(workspaceId: string) {
  const healthQuery = useGooseHealth();
  const startMutation = useStartShadowAnalysis();
  const cancelMutation = useCancelAnalysis();
  const [validationError, setValidationError] = useState(false);

  useEffect(() => {
    if (workspaceId) {
      setValidationError(false);
    }
  }, [workspaceId]);

  const isReady =
    healthQuery.data?.binary_available &&
    healthQuery.data?.shadow_mode_enabled;

  const startAnalysis = (options?: {
    thesis_id?: string;
    research_project_id?: string;
    instructions?: string;
  }) => {
    if (!workspaceId) {
      setValidationError(true);
      return;
    }

    setValidationError(false);
    startMutation.mutate({
      workspace_id: workspaceId,
      ...options,
    });
  };

  const cancelAnalysis = (runId: string) => {
    cancelMutation.mutate(runId);
  };

  return {
    health: healthQuery.data,
    isHealthLoading: healthQuery.isLoading,
    isReady,

    startAnalysis,
    isStarting: startMutation.isPending,
    result: startMutation.data,
    validationError,
    startError: startMutation.error
      ? processAppError("en", startMutation.error)
      : null,

    cancelAnalysis,
    isCancelling: cancelMutation.isPending,

    isRunning: startMutation.isPending,
    isSuccess: startMutation.isSuccess,
    isError: startMutation.isError,
  };
}