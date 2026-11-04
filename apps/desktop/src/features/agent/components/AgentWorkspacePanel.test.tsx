// Tests for AgentWorkspacePanel component.

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import "@testing-library/jest-dom";
import { invoke } from "@tauri-apps/api/core";

import { ToastProvider } from "@/components/common/ToastProvider";
import { AgentWorkspacePanel } from "./AgentWorkspacePanel";
import type { AgentTask, AgentTaskEvent } from "@/lib/desktop-api/agent";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(vi.fn()),
}));

const task: AgentTask = {
  id: "task-1",
  workspace_id: "ws-1",
  title: "Analyze Tesla",
  description: "Look at Q4 earnings",
  status: "completed",
  output: '{"summary":"strong demand"}',
  error: null,
  created_at: "2026-01-01T00:00:00Z",
  updated_at: "2026-01-01T00:00:00Z",
};

const events: AgentTaskEvent[] = [
  {
    id: "e1",
    task_id: "task-1",
    event_type: "task_started",
    payload: null,
    created_at: "2026-01-01T00:00:01Z",
  },
  {
    id: "e2",
    task_id: "task-1",
    event_type: "task_completed",
    payload: null,
    created_at: "2026-01-01T00:00:02Z",
  },
];

type CommandHandler = (args?: Record<string, unknown>) => unknown;

function mockCommands(handlers: Record<string, CommandHandler>) {
  vi.mocked(invoke).mockImplementation(((command: string, args?: Record<string, unknown>) => {
    const handler = handlers[command];
    if (!handler) {
      return Promise.reject(new Error(`Unexpected command: ${command}`));
    }
    return Promise.resolve(handler(args));
  }) as unknown as typeof invoke);
}

function renderPanel() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });

  return render(
    <QueryClientProvider client={queryClient}>
      <ToastProvider>
        <AgentWorkspacePanel workspaceId="ws-1" />
      </ToastProvider>
    </QueryClientProvider>,
  );
}

describe("AgentWorkspacePanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    cleanup();
  });

  it("opens the task detail when a task is selected", async () => {
    mockCommands({
      list_agent_tasks: () => [task],
      get_agent_task: () => task,
      get_task_events: () => events,
    });

    renderPanel();

    fireEvent.click(await screen.findByText("Analyze Tesla"));

    expect(await screen.findByRole("button", { name: "Close task detail" })).toBeInTheDocument();
    expect(await screen.findByText("Execution timeline")).toBeInTheDocument();
    expect(screen.getByText(/"summary": "strong demand"/)).toBeInTheDocument();
  });

  it("closes the task detail", async () => {
    mockCommands({
      list_agent_tasks: () => [task],
      get_agent_task: () => task,
      get_task_events: () => events,
    });

    renderPanel();

    fireEvent.click(await screen.findByText("Analyze Tesla"));
    fireEvent.click(await screen.findByRole("button", { name: "Close task detail" }));

    expect(screen.queryByText("Execution timeline")).not.toBeInTheDocument();
    expect(screen.getByText("Analyze Tesla")).toBeInTheDocument();
  });
});
