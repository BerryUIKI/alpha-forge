/**
 * Agent Connection Status Hook
 *
 * Determines agent status based on:
 * 1. Running/queued tasks → running (blue, blinking)
 * 2. API key not configured → unconfigured (yellow)
 * 3. Connection error → error (red)
 * 4. Default → idle (gray)
 *
 * @module hooks/useAgentStatus
 */

import { useQuery } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import type { AgentConnectionStatus } from "@/components/layout/types";

const AGENT_STATUS_KEY = ["agent", "status"];

interface AgentStatusResult {
  status: AgentConnectionStatus;
  hasRunningTasks: boolean;
  isConfigured: boolean;
  error?: string;
}

/**
 * Hook to determine agent connection status.
 *
 * Status priority:
 * 1. error (red) - Connection/API errors
 * 2. running (blue blinking) - Has running/queued tasks
 * 3. unconfigured (yellow) - API key not configured
 * 4. idle (gray) - Default, no active tasks
 */
export function useAgentStatus(workspaceId: string) {
  // Check if there are running tasks
  const tasksQuery = useQuery({
    queryKey: ["agent", "tasks", workspaceId],
    queryFn: () => desktopApi.agent.listAgentTasks(workspaceId),
    enabled: !!workspaceId,
    refetchInterval: 5000, // Poll every 5 seconds for task updates
    retry: false,
  });

  // Check if API key is configured (secure keychain check)
  const credentialQuery = useQuery({
    queryKey: ["credentials", "openai", "status"],
    queryFn: () => desktopApi.credentials.hasOpenAiApiKey(),
    retry: false,
  });

  // Determine status based on priority
  const determineStatus = (): AgentStatusResult => {
    // Check for errors first (highest priority)
    if (tasksQuery.isError || credentialQuery.isError) {
      const error = tasksQuery.error || credentialQuery.error;
      return {
        status: "error",
        hasRunningTasks: false,
        isConfigured: false,
        error: error?.message || "Connection failed",
      };
    }

    // Check for running/queued tasks
    const tasks = tasksQuery.data || [];
    const hasRunningTasks = tasks.some(
      (task) => task.status === "running" || task.status === "queued"
    );

    if (hasRunningTasks) {
      return {
        status: "running",
        hasRunningTasks: true,
        isConfigured: true,
      };
    }

    // Check if API key is configured (secure check)
    const isConfigured = credentialQuery.data === true;

    if (!isConfigured) {
      return {
        status: "unconfigured",
        hasRunningTasks: false,
        isConfigured: false,
      };
    }

    // Default: idle
    return {
      status: "idle",
      hasRunningTasks: false,
      isConfigured: true,
    };
  };

  const result = determineStatus();

  return {
    ...result,
    isLoading: tasksQuery.isLoading && credentialQuery.isLoading,
    isFetching: tasksQuery.isFetching || credentialQuery.isFetching,
  };
}

/**
 * Hook to get agent status without workspace context.
 * Uses a simplified check for global status display.
 */
export function useAgentGlobalStatus() {
  return useQuery({
    queryKey: AGENT_STATUS_KEY,
    queryFn: async (): Promise<AgentConnectionStatus> => {
      // Check if API key is configured globally (secure keychain check)
      const hasApiKey = await desktopApi.credentials.hasOpenAiApiKey();
      if (!hasApiKey) {
        return "unconfigured";
      }
      // Default to idle if configured but no workspace selected
      return "idle";
    },
    retry: false,
    staleTime: 10000, // Cache for 10 seconds
  });
}
