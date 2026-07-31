// Agent desktop API.

import { invoke } from "@tauri-apps/api/core";

export interface AgentTask {
  id: string;
  status: string;
  input: string;
}

export async function createTask(input: string): Promise<AgentTask> {
  return invoke("create_task", { input });
}

export async function listTasks(): Promise<AgentTask[]> {
  return invoke("list_tasks");
}
