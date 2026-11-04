import { act, renderHook, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

import { AGENT_KEYS, useAgentTaskEvents, useRunAgentTask } from "./useAgentTasks";

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(),
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

function makeWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });

  return function Wrapper({ children }: { children: React.ReactNode }) {
    return <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>;
  };
}

describe("useAgentTaskEvents", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("invalidates agent queries when a matching task event arrives", async () => {
    let handler: ((event: { payload: { task_id: string } }) => void) | undefined;

    vi.mocked(listen).mockImplementation((_event, callback) => {
      handler = callback as typeof handler;
      return Promise.resolve(vi.fn());
    });

    const queryClient = new QueryClient({
      defaultOptions: { queries: { retry: false } },
    });
    const invalidate = vi.spyOn(queryClient, "invalidateQueries");

    const wrapper = ({ children }: { children: React.ReactNode }) => (
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    );

    renderHook(() => useAgentTaskEvents("task-1"), { wrapper });

    await waitFor(() => expect(handler).toBeDefined());

    handler?.({ payload: { task_id: "task-2" } });
    expect(invalidate).not.toHaveBeenCalled();

    handler?.({ payload: { task_id: "task-1" } });
    expect(invalidate).toHaveBeenCalledWith({ queryKey: AGENT_KEYS.all });
  });
});

describe("useRunAgentTask", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("queues a created task before starting it", async () => {
    vi.mocked(invoke).mockResolvedValue({});

    const { result } = renderHook(() => useRunAgentTask(), { wrapper: makeWrapper() });

    await act(async () => {
      await result.current.mutateAsync({ id: "task-1", status: "created" });
    });

    expect(vi.mocked(invoke).mock.calls.map(([command]) => command)).toEqual([
      "queue_agent_task",
      "start_agent_task",
    ]);
  });

  it("starts a queued task directly", async () => {
    vi.mocked(invoke).mockResolvedValue({});

    const { result } = renderHook(() => useRunAgentTask(), { wrapper: makeWrapper() });

    await act(async () => {
      await result.current.mutateAsync({ id: "task-1", status: "queued" });
    });

    expect(vi.mocked(invoke).mock.calls.map(([command]) => command)).toEqual(["start_agent_task"]);
  });
});
