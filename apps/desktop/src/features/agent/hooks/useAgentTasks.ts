// Hooks for agent tasks.

import { useEffect } from "react";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import type { AgentTaskEvent } from "@/lib/desktop-api/agent";

const AGENT_TASK_EVENT = "agent-task-event";

export const AGENT_KEYS = {
  all: ["agent"] as const,
  tasks: (workspaceId: string) => [...AGENT_KEYS.all, "tasks", workspaceId] as const,
  task: (id: string) => [...AGENT_KEYS.all, "task", id] as const,
  events: (taskId: string) => [...AGENT_KEYS.all, "events", taskId] as const,
};

/**
 * Subscribe to backend agent task events and invalidate cached queries.
 */
export function useAgentTaskEvents(taskId?: string) {
  const queryClient = useQueryClient();

  useEffect(() => {
    let disposed = false;
    let unlisten: UnlistenFn | undefined;

    listen<AgentTaskEvent>(AGENT_TASK_EVENT, (event) => {
      if (taskId && event.payload.task_id !== taskId) {
        return;
      }
      void queryClient.invalidateQueries({ queryKey: AGENT_KEYS.all });
    })
      .then((stop) => {
        if (disposed) {
          stop();
        } else {
          unlisten = stop;
        }
      })
      .catch((error) => {
        console.warn("Failed to subscribe to agent task events", error);
      });

    return () => {
      disposed = true;
      unlisten?.();
    };
  }, [taskId, queryClient]);
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
