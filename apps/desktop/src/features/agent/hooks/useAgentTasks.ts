// Hooks for agent tasks.

import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import type { TaskStatus } from "@/lib/desktop-api/agent";

const AGENT_KEYS = {
  all: ["agent"] as const,
  tasks: (workspaceId: string) => [...AGENT_KEYS.all, "tasks", workspaceId] as const,
  task: (id: string) => [...AGENT_KEYS.all, "task", id] as const,
  events: (taskId: string) => [...AGENT_KEYS.all, "events", taskId] as const,
};

type RunAgentTaskError = Error & { queued: boolean };

function asRunAgentTaskError(error: unknown, queued: boolean): RunAgentTaskError {
  const structuredFields = typeof error === "object" && error !== null ? error : undefined;
  const message =
    error instanceof Error
      ? error.message
      : structuredFields &&
          "message" in structuredFields &&
          typeof structuredFields.message === "string"
        ? structuredFields.message
        : String(error);
  const normalized =
    error instanceof Error ? error : Object.assign(new Error(message), structuredFields);
  return Object.assign(normalized, { queued }) as RunAgentTaskError;
}

/**
 * Hook to list agent tasks for a workspace.
 */
export function useAgentTasks(workspaceId: string) {
  return useQuery({
    queryKey: AGENT_KEYS.tasks(workspaceId),
    queryFn: () => desktopApi.agent.listAgentTasks(workspaceId),
    enabled: !!workspaceId,
  });
}

/**
 * Hook to get a single agent task.
 */
export function useAgentTask(taskId: string) {
  return useQuery({
    queryKey: AGENT_KEYS.task(taskId),
    queryFn: () => desktopApi.agent.getAgentTask(taskId),
    enabled: !!taskId,
    // Poll so a selected task's status advances without a manual refresh.
    // Matches the 5s interval already used by useAgentStatus.
    refetchInterval: 5000,
  });
}

/**
 * Hook to get task events.
 */
export function useTaskEvents(taskId: string) {
  return useQuery({
    queryKey: AGENT_KEYS.events(taskId),
    queryFn: () => desktopApi.agent.getTaskEvents(taskId),
    enabled: !!taskId,
  });
}

/**
 * Hook to create an agent task.
 */
export function useCreateAgentTask() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      workspaceId,
      title,
      description,
    }: {
      workspaceId: string;
      title: string;
      description?: string;
    }) => desktopApi.agent.createAgentTask(workspaceId, title, description),
    onSuccess: (_, { workspaceId }) => {
      queryClient.invalidateQueries({ queryKey: AGENT_KEYS.tasks(workspaceId) });
    },
  });
}

/**
 * Hook to queue a task.
 */
export function useQueueAgentTask() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (taskId: string) => desktopApi.agent.queueAgentTask(taskId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: AGENT_KEYS.all });
    },
  });
}

/**
 * Hook to start a task.
 */
export function useStartAgentTask() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (taskId: string) => desktopApi.agent.startAgentTask(taskId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: AGENT_KEYS.all });
    },
  });
}

/**
 * Queue and start a task, or retry starting a task that is already queued.
 *
 * Queueing is deliberately kept in the frontend workflow so a task created by
 * the user always follows the explicit created -> queued -> running path.
 * When executor admission fails after queueing, invalidating the task queries
 * lets the UI recover with a queued task and a retry action.
 */
export function useRunAgentTask() {
  const queryClient = useQueryClient();

  const invalidateAgentQueries = () => queryClient.invalidateQueries({ queryKey: AGENT_KEYS.all });

  return useMutation({
    mutationFn: async ({
      taskId,
      status,
    }: {
      taskId: string;
      status: Extract<TaskStatus, "created" | "queued">;
    }) => {
      if (status === "created") {
        try {
          await desktopApi.agent.queueAgentTask(taskId);
        } catch (error) {
          throw asRunAgentTaskError(error, false);
        }
      }

      try {
        return await desktopApi.agent.startAgentTask(taskId);
      } catch (error) {
        throw asRunAgentTaskError(error, true);
      }
    },
    onSuccess: invalidateAgentQueries,
    onError: invalidateAgentQueries,
  });
}

/**
 * Hook to cancel a task.
 */
export function useCancelAgentTask() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (taskId: string) => desktopApi.agent.cancelAgentTask(taskId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: AGENT_KEYS.all });
    },
  });
}
