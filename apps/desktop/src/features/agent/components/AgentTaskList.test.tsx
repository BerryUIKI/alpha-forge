// Tests for AgentTaskList component.

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import "@testing-library/jest-dom";
import { invoke } from "@tauri-apps/api/core";

import { ToastProvider } from "@/components/common/ToastProvider";
import { AgentTaskList } from "./AgentTaskList";
import type { AgentTask } from "@/lib/desktop-api/agent";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(vi.fn()),
}));

function makeTask(overrides: Partial<AgentTask> = {}): AgentTask {
  return {
    id: "task-1",
    workspace_id: "ws-1",
    title: "Analyze Tesla",
    description: "Look at Q4 earnings",
    status: "created",
    output: null,
    error: null,
    created_at: "2026-01-01T00:00:00Z",
    updated_at: "2026-01-01T00:00:00Z",
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

function renderList(props: Partial<React.ComponentProps<typeof AgentTaskList>> = {}) {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });

  return render(
    <QueryClientProvider client={queryClient}>
      <ToastProvider>
        <AgentTaskList workspaceId="ws-1" {...props} />
      </ToastProvider>
    </QueryClientProvider>,
  );
}

function invokedCommands(): string[] {
  return vi.mocked(invoke).mock.calls.map(([command]) => command as string);
}

describe("AgentTaskList", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    cleanup();
  });

  it("renders tasks with their status and details", async () => {
    mockCommands({ list_agent_tasks: () => [makeTask()] });

    renderList();

    expect(await screen.findByText("Analyze Tesla")).toBeInTheDocument();
    expect(screen.getByText("Look at Q4 earnings")).toBeInTheDocument();
    expect(screen.getByText("Created")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /start/i })).toBeInTheDocument();
  });

  it("selects a task when its card is clicked", async () => {
    const onSelectTask = vi.fn();
    mockCommands({ list_agent_tasks: () => [makeTask()] });

    renderList({ onSelectTask });

    fireEvent.click(await screen.findByText("Analyze Tesla"));

    expect(onSelectTask).toHaveBeenCalledWith(expect.objectContaining({ id: "task-1" }));
  });

  it("queues then starts a created task", async () => {
    mockCommands({
      list_agent_tasks: () => [makeTask()],
      queue_agent_task: () => makeTask({ status: "queued" }),
      start_agent_task: () => makeTask({ status: "running" }),
    });

    renderList();

    fireEvent.click(await screen.findByRole("button", { name: /start/i }));

    await waitFor(() => {
      expect(invokedCommands()).toContain("queue_agent_task");
      expect(invokedCommands()).toContain("start_agent_task");
    });
    expect(invokedCommands().indexOf("queue_agent_task")).toBeLessThan(
      invokedCommands().indexOf("start_agent_task"),
    );
    expect(await screen.findByText("Task started")).toBeInTheDocument();
  });

  it("starts a queued task without queueing it again", async () => {
    mockCommands({
      list_agent_tasks: () => [makeTask({ status: "queued" })],
      start_agent_task: () => makeTask({ status: "running" }),
    });

    renderList();

    fireEvent.click(await screen.findByRole("button", { name: /start/i }));

    await waitFor(() => {
      expect(invokedCommands()).toContain("start_agent_task");
    });
    expect(invokedCommands()).not.toContain("queue_agent_task");
  });

  it("cancels a running task", async () => {
    mockCommands({
      list_agent_tasks: () => [makeTask({ status: "running" })],
      cancel_agent_task: () => makeTask({ status: "running" }),
    });

    renderList();

    fireEvent.click(await screen.findByRole("button", { name: /cancel/i }));

    await waitFor(() => {
      expect(invokedCommands()).toContain("cancel_agent_task");
    });
    expect(await screen.findByText("Cancelling task")).toBeInTheDocument();
  });

  it("shows a failed task's error message", async () => {
    mockCommands({
      list_agent_tasks: () => [makeTask({ status: "failed", error: "provider unavailable" })],
    });

    renderList();

    expect(await screen.findByText("provider unavailable")).toBeInTheDocument();
    expect(screen.getByText("Failed")).toBeInTheDocument();
  });

  it("retries a failed task", async () => {
    mockCommands({
      list_agent_tasks: () => [makeTask({ status: "failed", error: "boom" })],
      retry_agent_task: () => makeTask({ status: "queued" }),
      start_agent_task: () => makeTask({ status: "running" }),
    });

    renderList();

    fireEvent.click(await screen.findByRole("button", { name: /retry/i }));

    await waitFor(() => {
      expect(invokedCommands()).toContain("retry_agent_task");
      expect(invokedCommands()).toContain("start_agent_task");
    });
    expect(invokedCommands().indexOf("retry_agent_task")).toBeLessThan(
      invokedCommands().indexOf("start_agent_task"),
    );
    expect(await screen.findByText("Task restarted")).toBeInTheDocument();
  });
});
