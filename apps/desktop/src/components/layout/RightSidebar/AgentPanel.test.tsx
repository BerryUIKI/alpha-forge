import "@testing-library/jest-dom";
import { fireEvent, render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import type { AgentTask } from "@/lib/desktop-api/agent";
import { AgentPanel } from "./AgentPanel";

const tasks = vi.hoisted(() => ({
  created: {
    id: "created-task",
    workspace_id: "workspace-1",
    title: "Created task",
    description: null,
    status: "created",
    created_at: "2026-08-13T00:00:00Z",
    updated_at: "2026-08-13T00:00:00Z",
  } as AgentTask,
  running: {
    id: "running-task",
    workspace_id: "workspace-1",
    title: "Running task",
    description: null,
    status: "running",
    created_at: "2026-08-13T00:00:00Z",
    updated_at: "2026-08-13T00:00:00Z",
  } as AgentTask,
  queued: {
    id: "queued-task",
    workspace_id: "workspace-1",
    title: "Queued task",
    description: null,
    status: "queued",
    created_at: "2026-08-13T00:00:00Z",
    updated_at: "2026-08-13T00:00:00Z",
  } as AgentTask,
}));

const hookMocks = vi.hoisted(() => ({
  run: {
    mutate: vi.fn(),
    reset: vi.fn(),
    isPending: false,
    isError: false,
    error: null as unknown,
  },
  cancel: { mutate: vi.fn(), reset: vi.fn(), isPending: false, isError: false },
}));

vi.mock("@/features/workspace/hooks/useWorkspaces", () => ({
  useWorkspaces: () => ({
    data: [{ id: "workspace-1", name: "Workspace" }],
    isLoading: false,
    error: null,
  }),
}));

vi.mock("@/hooks/useAgentStatus", () => ({
  useAgentStatus: () => ({ status: "idle", hasRunningTasks: false }),
}));

vi.mock("@/features/agent/hooks/useAgentTasks", () => ({
  useAgentTask: (taskId: string) => ({
    data:
      taskId === tasks.running.id
        ? tasks.running
        : taskId === tasks.queued.id
          ? tasks.queued
          : taskId
            ? tasks.created
            : undefined,
  }),
  useRunAgentTask: () => hookMocks.run,
  useCancelAgentTask: () => hookMocks.cancel,
}));

vi.mock("@/features/agent/components/AgentTaskList", () => ({
  AgentTaskList: ({ onSelectTask }: { onSelectTask?: (task: AgentTask) => void }) => (
    <div>
      <button type="button" onClick={() => onSelectTask?.(tasks.created)}>
        Select created task
      </button>
      <button type="button" onClick={() => onSelectTask?.(tasks.running)}>
        Select running task
      </button>
      <button type="button" onClick={() => onSelectTask?.(tasks.queued)}>
        Select queued task
      </button>
    </div>
  ),
}));

vi.mock("@/features/agent/components/CreateAgentTask", () => ({
  CreateAgentTask: () => null,
}));

vi.mock("@/features/agent/components/AgentConfigGuide", () => ({
  AgentConfigGuide: () => null,
}));

vi.mock("@/features/agent/components/TaskStatusBadge", () => ({
  TaskStatusBadge: ({ status }: { status: string }) => <span>{status}</span>,
}));

vi.mock("@/lib/i18n/useLocale", () => ({
  useLocale: () => ({
    t: (key: string) =>
      ({
        agent: "Agent",
        statusIdle: "Idle",
        statusRunning: "Running",
        statusUnconfigured: "Configuration required",
        statusError: "Error",
        startTask: "Start",
        retryStartTask: "Retry Start",
        cancelTask: "Cancel",
        startingTask: "Starting...",
        cancellingTask: "Cancelling...",
        taskStartFailed: "Unable to start this task. It remains queued; retry when ready.",
        taskQueueFailed: "Unable to queue this task. Please try again.",
      })[key] ?? key,
  }),
}));

function renderPanel() {
  return render(<AgentPanel />);
}

describe("AgentPanel task actions", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    hookMocks.run.isPending = false;
    hookMocks.run.isError = false;
    hookMocks.run.error = null;
    hookMocks.cancel.isPending = false;
  });

  it("shows a recoverable start error and a queued retry action", () => {
    hookMocks.run.isError = true;
    hookMocks.run.error = { queued: true };
    renderPanel();

    fireEvent.click(screen.getByRole("button", { name: "Select queued task" }));

    expect(screen.getByRole("alert")).toHaveTextContent("remains queued");
    fireEvent.click(screen.getByRole("button", { name: "Retry Start" }));
    expect(hookMocks.run.mutate).toHaveBeenCalledWith({
      taskId: tasks.queued.id,
      status: "queued",
    });
  });

  it("shows a queue error without claiming the task is queued", () => {
    hookMocks.run.isError = true;
    hookMocks.run.error = { queued: false };
    renderPanel();

    fireEvent.click(screen.getByRole("button", { name: "Select created task" }));

    expect(screen.getByRole("alert")).toHaveTextContent("Unable to queue this task");
  });

  it("retries a queued task without queueing it again", () => {
    renderPanel();

    fireEvent.click(screen.getByRole("button", { name: "Select queued task" }));

    expect(screen.getByRole("button", { name: "Retry Start" })).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Retry Start" }));
    expect(hookMocks.run.mutate).toHaveBeenCalledWith({
      taskId: tasks.queued.id,
      status: "queued",
    });
  });

  it("offers cancellation only for a running task", () => {
    renderPanel();

    fireEvent.click(screen.getByRole("button", { name: "Select running task" }));

    expect(screen.getByRole("button", { name: "Cancel" })).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Start" })).not.toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
    expect(hookMocks.cancel.mutate).toHaveBeenCalledWith(tasks.running.id);
  });

  it("resets stale mutation state when selecting another task", () => {
    hookMocks.run.isError = true;
    hookMocks.run.error = { queued: true };
    renderPanel();

    fireEvent.click(screen.getByRole("button", { name: "Select queued task" }));
    expect(screen.getByRole("alert")).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: "Select running task" }));

    expect(hookMocks.run.reset).toHaveBeenCalledTimes(2);
    expect(hookMocks.cancel.reset).toHaveBeenCalledTimes(2);
  });
});
