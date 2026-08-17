/**
 * Goose Analysis Hooks
 *
 * Provides TanStack Query hooks for Goose shadow-mode analysis (M10).
 * Follows integration standards from docs/FRONTEND_BACKEND_INTEGRATION.md
 *
 * Backend Commands: src-tauri/src/commands/goose.rs
 * Service: src-tauri/src/services/goose_service.rs
 * API Layer: src/lib/desktop-api/goose.ts
 *
 * @module hooks/goose
 */

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import { processAppError } from "@/lib/errors";
import type {
  StartShadowAnalysisInput,
  ShadowAnalysisResult,
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
} from "@/lib/desktop-api/goose";

// ============================================================================
// Query Hooks
// ============================================================================

/**
 * Hook to check Goose service health
 *
 * @example
 * const { data: health, isLoading } = useGooseHealth();
 * if (health?.binary_available && health?.shadow_mode_enabled) {
 *   // Goose is ready
 * }
 */
export function useGooseHealth() {
  return useQuery({
    queryKey: ["goose", "health"],
    queryFn: () => desktopApi.goose.checkGooseHealth(),
    staleTime: 30000, // 30 seconds
    retry: false, // Don't retry health checks
  });
}

// ============================================================================
// Mutation Hooks
// ============================================================================

/**
 * Hook to start a Goose shadow analysis
 *
 * @example
 * const mutation = useStartShadowAnalysis();
 *
 * const handleStart = () => {
 *   mutation.mutate({
 *     workspace_id: workspaceId,
 *     thesis_id: thesisId,
 *   }, {
 *     onSuccess: (result) => {
 *       console.log("Analysis completed:", result.run_id);
 *     },
 *     onError: (error) => {
 *       console.error("Analysis failed:", error);
 *     },
 *   });
 * };
 */
export function useStartShadowAnalysis() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (input: StartShadowAnalysisInput) =>
      desktopApi.goose.startShadowAnalysis(input),
    onSuccess: (result: ShadowAnalysisResult) => {
      // Invalidate relevant queries
      queryClient.invalidateQueries({ queryKey: ["goose", "analyses"] });

      // Cache the result
      queryClient.setQueryData(
        ["goose", "analysis", result.run_id],
        result
      );
    },
    onError: (error: Error) => {
      console.error("Shadow analysis failed:", error);
    },
  });
}

/**
 * Hook to cancel a running Goose analysis
 *
 * @example
 * const cancelMutation = useCancelAnalysis();
 *
 * const handleCancel = () => {
 *   cancelMutation.mutate(runId);
 * };
 */
export function useCancelAnalysis() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (runId: string) => desktopApi.goose.cancelAnalysis(runId),
    onSuccess: (_data, runId) => {
      // Update the analysis status
      queryClient.invalidateQueries({ queryKey: ["goose", "analysis", runId] });
    },
  });
}

// ============================================================================
// Combined Hook
// ============================================================================

/**
 * Combined hook for Goose shadow analysis operations
 *
 * Provides all Goose-related functionality in a single hook.
 *
 * @param workspaceId - Current workspace ID
 *
 * @example
 * const {
 *   health,
 *   startAnalysis,
 *   cancelAnalysis,
 *   isStarting,
 *   result,
 *   error,
 * } = useGooseShadowAnalysis(workspaceId);
 *
 * // Start analysis
 * const handleStart = () => {
 *   startAnalysis({ thesis_id: thesisId });
 * };
 *
 * // Cancel running analysis
 * const handleCancel = () => {
 *   if (currentRunId) cancelAnalysis(currentRunId);
 * };
 */
export function useGooseShadowAnalysis(workspaceId: string) {
  const healthQuery = useGooseHealth();
  const startMutation = useStartShadowAnalysis();
  const cancelMutation = useCancelAnalysis();

  const isReady =
    healthQuery.data?.binary_available &&
    healthQuery.data?.shadow_mode_enabled;

  const startAnalysis = (options?: {
    thesis_id?: string;
    research_project_id?: string;
    instructions?: string;
  }) => {
    if (!workspaceId) {
      console.error("No workspace ID provided");
      return;
    }

    startMutation.mutate({
      workspace_id: workspaceId,
      ...options,
    });
  };

  const cancelAnalysis = (runId: string) => {
    cancelMutation.mutate(runId);
  };

  return {
    // Health status
    health: healthQuery.data,
    isHealthLoading: healthQuery.isLoading,
    isReady,

    // Start analysis
    startAnalysis,
    isStarting: startMutation.isPending,
    result: startMutation.data,
    startError: startMutation.error
      ? processAppError("en", startMutation.error)
      : null,

    // Cancel analysis
    cancelAnalysis,
    isCancelling: cancelMutation.isPending,

    // Current state
    isRunning: startMutation.isPending,
    isSuccess: startMutation.isSuccess,
    isError: startMutation.isError,
  };
}