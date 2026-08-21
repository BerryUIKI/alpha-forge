// Tests for agent desktop API.

import { describe, it, expect, vi, beforeEach } from "vitest";
import { invoke } from "@tauri-apps/api/core";

// Mock Tauri invoke
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("Agent API", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("createAgentTask calls invoke with correct parameters and parses response", async () => {
    const mockTask = {
      id: "task-uuid",
      workspaceId: "workspace-uuid",
      title: "Analyze Stocks",
      description: "Research top tech stocks",
      status: "created",
      createdAt: "2024-01-01T00:00:00Z",
      updatedAt: "2024-01-01T00:00:00Z",
    };

    vi.mocked(invoke).mockResolvedValueOnce(mockTask);

    const { createAgentTask } = await import("@/lib/desktop-api/agent");
    const result = await createAgentTask(
      "workspace-uuid",
      "Analyze Stocks",
      "Research top tech stocks"
    );

    expect(invoke).toHaveBeenCalledWith("create_agent_task", {
      workspaceId: "workspace-uuid",
      title: "Analyze Stocks",
      description: "Research top tech stocks",
    });
    expect(result).toEqual(mockTask);
  });

  it("createAgentTask handles null description", async () => {
    const mockTask = {
      id: "task-uuid",
      workspaceId: "workspace-uuid",
      title: "Simple Task",
      description: null,
      status: "created",
      createdAt: "2024-01-01T00:00:00Z",
      updatedAt: "2024-01-01T00:00:00Z",
    };

    vi.mocked(invoke).mockResolvedValueOnce(mockTask);

    const { createAgentTask } = await import("@/lib/desktop-api/agent");
    const result = await createAgentTask("workspace-uuid", "Simple Task");

    expect(invoke).toHaveBeenCalledWith("create_agent_task", {
      workspaceId: "workspace-uuid",
      title: "Simple Task",
      description: null,
    });
    expect(result).toEqual(mockTask);
  });

  it("getAgentTask calls invoke with correct parameters", async () => {
    const mockTask = {
      id: "task-uuid",
      workspaceId: "workspace-uuid",
      title: "Test Task",
      description: null,
      status: "queued",
      createdAt: "2024-01-01T00:00:00Z",
      updatedAt: "2024-01-01T00:00:00Z",
    };

    vi.mocked(invoke).mockResolvedValueOnce(mockTask);

    const { getAgentTask } = await import("@/lib/desktop-api/agent");
    const result = await getAgentTask("task-uuid");

    expect(invoke).toHaveBeenCalledWith("get_agent_task", { id: "task-uuid" });
    expect(result).toEqual(mockTask);
  });

  it("getAgentTask returns null for non-existent task", async () => {
    vi.mocked(invoke).mockResolvedValueOnce(null);

    const { getAgentTask } = await import("@/lib/desktop-api/agent");
    const result = await getAgentTask("non-existent");

    expect(invoke).toHaveBeenCalledWith("get_agent_task", { id: "non-existent" });
    expect(result).toBeNull();
  });

  it("listAgentTasks calls invoke with correct parameters", async () => {
    const mockTasks = [
      {
        id: "task-1",
        workspaceId: "workspace-uuid",
        title: "Task 1",
        description: null,
        status: "completed",
        createdAt: "2024-01-01T00:00:00Z",
        updatedAt: "2024-01-01T00:00:00Z",
      },
      {
        id: "task-2",
        workspaceId: "workspace-uuid",
        title: "Task 2",
        description: null,
        status: "running",
        createdAt: "2024-01-01T00:00:00Z",
        updatedAt: "2024-01-01T00:00:00Z",
      },
    ];

    vi.mocked(invoke).mockResolvedValueOnce(mockTasks);

    const { listAgentTasks } = await import("@/lib/desktop-api/agent");
    const result = await listAgentTasks("workspace-uuid");

    expect(invoke).toHaveBeenCalledWith("list_agent_tasks", {
      workspaceId: "workspace-uuid",
    });
    expect(result).toEqual(mockTasks);
    expect(result.length).toBe(2);
  });

  it("listAgentTasks returns empty array for workspace with no tasks", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([]);

    const { listAgentTasks } = await import("@/lib/desktop-api/agent");
    const result = await listAgentTasks("empty-workspace");

    expect(invoke).toHaveBeenCalledWith("list_agent_tasks", {
      workspaceId: "empty-workspace",
    });
    expect(result).toEqual([]);
  });

  it("getTaskEvents calls invoke with correct parameters", async () => {
    const mockEvents = [
      {
        id: "event-1",
        taskId: "task-uuid",
        eventType: "task_created",
        payload: null,
        createdAt: "2024-01-01T00:00:00Z",
      },
      {
        id: "event-2",
        taskId: "task-uuid",
        eventType: "task_queued",
        payload: null,
        createdAt: "2024-01-01T00:00:01Z",
      },
    ];

    vi.mocked(invoke).mockResolvedValueOnce(mockEvents);

    const { getTaskEvents } = await import("@/lib/desktop-api/agent");
    const result = await getTaskEvents("task-uuid");

    expect(invoke).toHaveBeenCalledWith("get_task_events", { taskId: "task-uuid" });
    expect(result).toEqual(mockEvents);
    expect(result.length).toBe(2);
  });

  it("queueAgentTask calls invoke with correct parameters", async () => {
    const mockTask = {
      id: "task-uuid",
      workspaceId: "workspace-uuid",
      title: "Test Task",
      description: null,
      status: "queued",
      createdAt: "2024-01-01T00:00:00Z",
      updatedAt: "2024-01-01T00:00:00Z",
    };

    vi.mocked(invoke).mockResolvedValueOnce(mockTask);

    const { queueAgentTask } = await import("@/lib/desktop-api/agent");
    const result = await queueAgentTask("task-uuid");

    expect(invoke).toHaveBeenCalledWith("queue_agent_task", { taskId: "task-uuid" });
    expect(result.status).toBe("queued");
  });

  it("startAgentTask calls invoke with correct parameters", async () => {
    const mockTask = {
      id: "task-uuid",
      workspaceId: "workspace-uuid",
      title: "Test Task",
      description: null,
      status: "running",
      createdAt: "2024-01-01T00:00:00Z",
      updatedAt: "2024-01-01T00:00:00Z",
    };

    vi.mocked(invoke).mockResolvedValueOnce(mockTask);

    const { startAgentTask } = await import("@/lib/desktop-api/agent");
    const result = await startAgentTask("task-uuid");

    expect(invoke).toHaveBeenCalledWith("start_agent_task", { taskId: "task-uuid" });
    expect(result.status).toBe("running");
  });

  it("cancelAgentTask calls invoke with correct parameters", async () => {
    const mockTask = {
      id: "task-uuid",
      workspaceId: "workspace-uuid",
      title: "Test Task",
      description: null,
      status: "cancelled",
      createdAt: "2024-01-01T00:00:00Z",
      updatedAt: "2024-01-01T00:00:00Z",
    };

    vi.mocked(invoke).mockResolvedValueOnce(mockTask);

    const { cancelAgentTask } = await import("@/lib/desktop-api/agent");
    const result = await cancelAgentTask("task-uuid");

    expect(invoke).toHaveBeenCalledWith("cancel_agent_task", { taskId: "task-uuid" });
    expect(result.status).toBe("cancelled");
  });

  it("rejects malformed agent task responses at runtime", async () => {
    vi.mocked(invoke).mockResolvedValueOnce({
      id: "task-uuid",
      workspace_id: "legacy_snake_case", // Missing required camelCase workspaceId
      status: "invalid_status",
    });

    const { getAgentTask } = await import("@/lib/desktop-api/agent");
    await expect(getAgentTask("task-uuid")).rejects.toThrow();
  });

  it("rejects malformed task event responses at runtime", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "event-1",
        task_id: "legacy_snake_case",
        eventType: "unknown_event_type",
      },
    ]);

    const { getTaskEvents } = await import("@/lib/desktop-api/agent");
    await expect(getTaskEvents("task-uuid")).rejects.toThrow();
  });

  it("handles API errors correctly", async () => {
    const mockError = {
      code: "NOT_FOUND",
      message: "Task 'non-existent' not found",
      recoverable: false,
    };

    vi.mocked(invoke).mockRejectedValueOnce(mockError);

    const { getAgentTask } = await import("@/lib/desktop-api/agent");

    await expect(getAgentTask("non-existent")).rejects.toEqual(mockError);
  });

  it("handles validation errors", async () => {
    const mockError = {
      code: "VALIDATION",
      message: "Task title cannot be empty",
      recoverable: true,
    };

    vi.mocked(invoke).mockRejectedValueOnce(mockError);

    const { createAgentTask } = await import("@/lib/desktop-api/agent");

    await expect(createAgentTask("workspace-uuid", "")).rejects.toEqual(mockError);
  });

  it("handles state transition errors", async () => {
    const mockError = {
      code: "VALIDATION",
      message: "Cannot start task in 'created' state",
      recoverable: true,
    };

    vi.mocked(invoke).mockRejectedValueOnce(mockError);

    const { startAgentTask } = await import("@/lib/desktop-api/agent");

    await expect(startAgentTask("task-uuid")).rejects.toEqual(mockError);
  });
});