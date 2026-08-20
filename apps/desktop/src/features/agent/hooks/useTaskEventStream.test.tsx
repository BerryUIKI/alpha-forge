import { renderHook, act } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { useTaskEventStream } from "./useTaskEventStream";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import type { AgentTaskEvent } from "@/lib/desktop-api/agent";
import { AGENT_KEYS } from "./useAgentTasks";
import React from "react";

const mocks = vi.hoisted(() => ({
  listen: vi.fn(),
  unlisten: vi.fn(),
  listeners: new Map<string, (event: { payload: AgentTaskEvent }) => void>(),
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: mocks.listen,
}));

describe("useTaskEventStream", () => {
  let queryClient: QueryClient;

  beforeEach(() => {
    queryClient = new QueryClient();
    vi.spyOn(queryClient, "invalidateQueries");

    mocks.listeners.clear();
    mocks.listen.mockReset();
    mocks.unlisten.mockReset();

    mocks.listen.mockImplementation(
      async (event: string, callback: (event: { payload: AgentTaskEvent }) => void) => {
        mocks.listeners.set(event, callback);
        return mocks.unlisten;
      }
    );
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  const wrapper = ({ children }: { children: React.ReactNode }) => (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  );

  const createEvent = (
    eventType: AgentTaskEvent["event_type"],
    payload: string | null = null,
    id = "event-1"
  ): AgentTaskEvent => ({
    id,
    task_id: "task-1",
    event_type: eventType,
    payload,
    created_at: "2026-08-20T00:00:00Z",
  });

  it("collects progress messages from task:progress events", async () => {
    const { result } = renderHook(() => useTaskEventStream(), { wrapper });

    await act(async () => {
      await Promise.resolve();
    });

    const event = createEvent("task_progress", "Doing something...");
    act(() => {
      mocks.listeners.get("task:progress")?.({ payload: event });
    });

    expect(result.current.progressMessages).toHaveLength(1);
    expect(result.current.progressMessages[0]).toEqual({
      id: "event-1",
      taskId: "task-1",
      message: "Doing something...",
      timestamp: "2026-08-20T00:00:00Z",
    });
    expect(result.current.latestEvent).toEqual(event);
    expect(queryClient.invalidateQueries).toHaveBeenCalledWith({ queryKey: AGENT_KEYS.all });
  });

  it("limits to 20 messages maximum", async () => {
    const { result } = renderHook(() => useTaskEventStream(), { wrapper });

    await act(async () => {
      await Promise.resolve();
    });

    act(() => {
      for (let i = 0; i < 25; i++) {
        const event = createEvent("task_progress", `Message ${i}`, `event-${i}`);
        mocks.listeners.get("task:progress")?.({ payload: event });
      }
    });

    expect(result.current.progressMessages).toHaveLength(20);
    expect(result.current.progressMessages[19]?.message).toBe("Message 24");
    expect(result.current.progressMessages[0]?.message).toBe("Message 5");
  });

  it("invalidates queries on task:completed", async () => {
    const { result } = renderHook(() => useTaskEventStream(), { wrapper });

    await act(async () => {
      await Promise.resolve();
    });

    const event = createEvent("task_completed", "{}");
    act(() => {
      mocks.listeners.get("task:completed")?.({ payload: event });
    });

    expect(result.current.progressMessages).toHaveLength(1);
    expect(result.current.progressMessages[0]?.message).toBe("Task completed.");
    expect(queryClient.invalidateQueries).toHaveBeenCalledWith({ queryKey: AGENT_KEYS.all });
  });

  it("invalidates queries on task:failed", async () => {
    const { result } = renderHook(() => useTaskEventStream(), { wrapper });

    await act(async () => {
      await Promise.resolve();
    });

    const event = createEvent("task_failed", "Error occurred");
    act(() => {
      mocks.listeners.get("task:failed")?.({ payload: event });
    });

    expect(result.current.progressMessages).toHaveLength(1);
    expect(result.current.progressMessages[0]?.message).toBe("Task failed: Error occurred");
    expect(queryClient.invalidateQueries).toHaveBeenCalledWith({ queryKey: AGENT_KEYS.all });
  });

  it("clears messages for a specific task", async () => {
    const { result } = renderHook(() => useTaskEventStream(), { wrapper });

    await act(async () => {
      await Promise.resolve();
    });

    act(() => {
      mocks.listeners.get("task:progress")?.({
        payload: { ...createEvent("task_progress", "msg 1"), task_id: "task-1" },
      });
      mocks.listeners.get("task:progress")?.({
        payload: { ...createEvent("task_progress", "msg 2"), task_id: "task-2" },
      });
    });

    expect(result.current.progressMessages).toHaveLength(2);

    act(() => {
      result.current.clearProgress("task-1");
    });

    expect(result.current.progressMessages).toHaveLength(1);
    expect(result.current.progressMessages[0]?.taskId).toBe("task-2");
  });

  it("cleans up listeners on unmount", async () => {
    const resolveUnlisteners: Array<(unlisten: typeof mocks.unlisten) => void> = [];
    mocks.listen.mockImplementation(
      async (event: string, callback: (event: { payload: AgentTaskEvent }) => void) => {
        mocks.listeners.set(event, callback);
        return new Promise<typeof mocks.unlisten>((resolve) => {
          resolveUnlisteners.push(resolve);
        });
      }
    );

    const { unmount } = renderHook(() => useTaskEventStream(), { wrapper });

    unmount();

    await act(async () => {
      for (const resolve of resolveUnlisteners) {
        resolve(mocks.unlisten);
      }
    });

    expect(mocks.unlisten).toHaveBeenCalledTimes(4);
  });
});
