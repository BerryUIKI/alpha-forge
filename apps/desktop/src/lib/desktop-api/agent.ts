// Agent desktop API.

import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";

/**
 * Status of an agent task.
 */
export const TaskStatusSchema = z.enum([
  "created",
  "queued",
  "running",
  "waiting_for_input",
  "completed",
  "failed",
  "cancelled",
]);

export type TaskStatus = z.infer<typeof TaskStatusSchema>;

/**
 * An agent task represents a research request.
 */
export const AgentTaskSchema = z
  .object({
    id: z.string().min(1),
    workspaceId: z.string().min(1),
    title: z.string().min(1),
    description: z.string().nullable(),
    status: TaskStatusSchema,
    createdAt: z.string().min(1),
    updatedAt: z.string().min(1),
  })
  .strict();

export type AgentTask = z.infer<typeof AgentTaskSchema>;

/**
 * Type of task event.
 */
export const TaskEventTypeSchema = z.enum([
  "task_created",
  "task_queued",
  "task_started",
  "task_progress",
  "task_waiting_for_input",
  "task_completed",
  "task_failed",
  "task_cancelled",
]);

export type TaskEventType = z.infer<typeof TaskEventTypeSchema>;

/**
 * An event that occurred during task execution.
 */
export const AgentTaskEventSchema = z
  .object({
    id: z.string().min(1),
    taskId: z.string().min(1),
    eventType: TaskEventTypeSchema,
    payload: z.string().nullable(),
    createdAt: z.string().min(1),
  })
  .strict();

export type AgentTaskEvent = z.infer<typeof AgentTaskEventSchema>;

/**
 * Creates a new agent task.
 */
export async function createAgentTask(
  workspaceId: string,
  title: string,
  description?: string
): Promise<AgentTask> {
  const response: unknown = await invoke("create_agent_task", {
    workspaceId,
    title,
    description: description || null,
  });
  return AgentTaskSchema.parse(response);
}

/**
 * Gets a task by ID.
 */
export async function getAgentTask(id: string): Promise<AgentTask | null> {
  const response: unknown = await invoke("get_agent_task", { id });
  return z.nullable(AgentTaskSchema).parse(response);
}

/**
 * Lists all tasks for a workspace.
 */
export async function listAgentTasks(workspaceId: string): Promise<AgentTask[]> {
  const response: unknown = await invoke("list_agent_tasks", { workspaceId });
  return z.array(AgentTaskSchema).parse(response);
}

/**
 * Gets events for a task.
 */
export async function getTaskEvents(taskId: string): Promise<AgentTaskEvent[]> {
  const response: unknown = await invoke("get_task_events", { taskId });
  return z.array(AgentTaskEventSchema).parse(response);
}

/**
 * Queues a task for execution.
 */
export async function queueAgentTask(taskId: string): Promise<AgentTask> {
  const response: unknown = await invoke("queue_agent_task", { taskId });
  return AgentTaskSchema.parse(response);
}

/**
 * Starts task execution.
 */
export async function startAgentTask(taskId: string): Promise<AgentTask> {
  const response: unknown = await invoke("start_agent_task", { taskId });
  return AgentTaskSchema.parse(response);
}

/**
 * Cancels a task.
 */
export async function cancelAgentTask(taskId: string): Promise<AgentTask> {
  const response: unknown = await invoke("cancel_agent_task", { taskId });
  return AgentTaskSchema.parse(response);
}