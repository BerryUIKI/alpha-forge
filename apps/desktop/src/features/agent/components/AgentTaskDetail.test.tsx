// Tests for AgentTaskDetail component.

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import "@testing-library/jest-dom";
import { invoke } from "@tauri-apps/api/core";

import { AgentTaskDetail } from "./AgentTaskDetail";
import type { AgentTask, AgentTaskEvent } from "@/lib/desktop-api/agent";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

function makeTask(overrides: Partial<AgentTask> = {}): AgentTask {
  return {
    id: "task-1",
    workspace_id: "ws-1",
    title: "Analyze Tesla",
    description: "Look at Q4 earnings",
    status: "completed",
    output: null,
    error: null,
    created_at: "2026-01-01T00:00:00Z",
    updated_at: "2026-01-01T00:00:00Z",
    ...overrides,
  };
}

function makeEvent(overrides: Partial<AgentTaskEvent> = {}): AgentTaskEvent {
  return {
    id: "event-1",
    task_id: "task-1",
    event_type: "task_created",
    payload: null,
    created_at: "2026-01-01T00:00:00Z",
    ...overrides,
  };
}

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

function renderDetail(onClose?: () => void) {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });

  return render(
    <QueryClientProvider client={queryClient}>
      <AgentTaskDetail taskId="task-1" onClose={onClose} />
    </QueryClientProvider>,
  );
}

describe("AgentTaskDetail", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    cleanup();
  });

  it("renders the execution timeline and formatted output", async () => {
    mockCommands({
      get_agent_task: () => makeTask({ output: '{"summary":"ok","confidence":0.8}' }),
      get_task_events: () => [
        makeEvent({ id: "e1", event_type: "task_created" }),
        makeEvent({ id: "e2", event_type: "task_started" }),
        makeEvent({ id: "e3", event_type: "task_progress", payload: "Analyzing request…" }),
        makeEvent({ id: "e4", event_type: "task_completed", payload: "done" }),
      ],
    });

    renderDetail();

    expect(await screen.findByText("Execution timeline")).toBeInTheDocument();
    expect(screen.getByText("Task created")).toBeInTheDocument();
    expect(screen.getByText("Execution started")).toBeInTheDocument();
    expect(screen.getByText("Analyzing request…")).toBeInTheDocument();
    expect(screen.getByText("Output")).toBeInTheDocument();
    expect(screen.getByText(/"summary": "ok"/)).toBeInTheDocument();
  });

  it("shows the failure reason in the timeline and error section", async () => {
    mockCommands({
      get_agent_task: () => makeTask({ status: "failed", error: "provider unavailable" }),
      get_task_events: () => [
        makeEvent({ id: "e1", event_type: "task_started" }),
        makeEvent({ id: "e2", event_type: "task_failed", payload: "provider unavailable" }),
      ],
    });

    renderDetail();

    expect(await screen.findByText("Error")).toBeInTheDocument();
    expect(screen.getAllByText("provider unavailable").length).toBeGreaterThanOrEqual(1);
  });

  it("closes the detail when the close button is clicked", async () => {
    const onClose = vi.fn();
    mockCommands({
      get_agent_task: () => makeTask(),
      get_task_events: () => [makeEvent()],
    });

    renderDetail(onClose);

    fireEvent.click(await screen.findByRole("button", { name: "Close task detail" }));

    expect(onClose).toHaveBeenCalled();
  });

  it("reports a missing task", async () => {
    mockCommands({
      get_agent_task: () => null,
      get_task_events: () => [],
    });

    renderDetail();

    expect(await screen.findByText("Task not found.")).toBeInTheDocument();
  });
});
