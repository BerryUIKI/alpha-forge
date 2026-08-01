// Agent desktop API.

import { invoke } from "@tauri-apps/api/core";

/**
 * Status of an agent task.
 */
export type TaskStatus =
  | "created"
  | "queued"
  | "running"
  | "waiting_for_input"
  | "completed"
  | "failed"
  | "cancelled";

/**
 * An agent task represents a research request.
 */
export interface AgentTask {
  id: string;
  workspace_id: string;
  title: string;
  description: string | null;
  status: TaskStatus;
  created_at: string;
  updated_at: string;
}

/**
 * Type of task event.
 */
export type TaskEventType =
  | "task_created"
  | "task_queued"
  | "task_started"
  | "task_progress"
  | "task_waiting_for_input"
  | "task_completed"
  | "task_failed"
  | "task_cancelled";

/**
 * An event that occurred during task execution.
 */
export interface AgentTaskEvent {
  id: string;
  task_id: string;
  event_type: TaskEventType;
  payload: string | null;
  created_at: string;
}

/**
 * Creates a new agent task.
 */
export async function createAgentTask(
  workspaceId: string,
  title: string,
  description?: string
): Promise<AgentTask> {
  return invoke("create_agent_task", {
    workspaceId,
    title,
    description: description || null,
  });
}

/**
 * Gets a task by ID.
 */
export async function getAgentTask(id: string): Promise<AgentTask | null> {
  return invoke("get_agent_task", { id });
}

/**
 * Lists all tasks for a workspace.
 */
export async function listAgentTasks(workspaceId: string): Promise<AgentTask[]> {
  return invoke("list_agent_tasks", { workspaceId });
}

/**
 * Gets events for a task.
 */
export async function getTaskEvents(taskId: string): Promise<AgentTaskEvent[]> {
  return invoke("get_task_events", { taskId });
}

/**
 * Queues a task for execution.
 */
export async function queueAgentTask(taskId: string): Promise<AgentTask> {
  return invoke("queue_agent_task", { taskId });
}

/**
 * Starts task execution.
 */
export async function startAgentTask(taskId: string): Promise<AgentTask> {
  return invoke("start_agent_task", { taskId });
}

/**
 * Cancels a task.
 */
export async function cancelAgentTask(taskId: string): Promise<AgentTask> {
  return invoke("cancel_agent_task", { taskId });
}