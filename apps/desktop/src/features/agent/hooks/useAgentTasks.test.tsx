import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { renderHook, act } from "@testing-library/react";
import type { ReactNode } from "react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { useRunAgentTask } from "./useAgentTasks";

const agentMock = vi.hoisted(() => ({
  queueAgentTask: vi.fn(),
  startAgentTask: vi.fn(),
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: { agent: agentMock },
}));

function createHarness() {
  const queryClient = new QueryClient({
    defaultOptions: { mutations: { retry: false }, queries: { retry: false } },
  });
  const invalidateQueries = vi.spyOn(queryClient, "invalidateQueries");
  const wrapper = ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  );
  return { queryClient, invalidateQueries, wrapper };
}

describe("useRunAgentTask", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    agentMock.queueAgentTask.mockResolvedValue({ id: "task-1", status: "queued" });
    agentMock.startAgentTask.mockResolvedValue({ id: "task-1", status: "running" });
  });

  it("queues a created task before starting it", async () => {
    const { wrapper } = createHarness();
    const { result } = renderHook(() => useRunAgentTask(), { wrapper });

    await act(async () => {
      await result.current.mutateAsync({ taskId: "task-1", status: "created" });
    });

    expect(agentMock.queueAgentTask).toHaveBeenCalledOnce();
    expect(agentMock.startAgentTask).toHaveBeenCalledOnce();
    expect(agentMock.queueAgentTask.mock.invocationCallOrder[0]!).toBeLessThan(
      agentMock.startAgentTask.mock.invocationCallOrder[0]!,
    );
  });

  it("retries start without queueing an already queued task", async () => {
    const { wrapper } = createHarness();
    const { result } = renderHook(() => useRunAgentTask(), { wrapper });

    await act(async () => {
      await result.current.mutateAsync({ taskId: "task-1", status: "queued" });
    });

    expect(agentMock.queueAgentTask).not.toHaveBeenCalled();
    expect(agentMock.startAgentTask).toHaveBeenCalledWith("task-1");
  });

  it("invalidates task state when starting fails so queued recovery is visible", async () => {
    agentMock.startAgentTask.mockRejectedValueOnce(new Error("executor unavailable"));
    const { wrapper, invalidateQueries } = createHarness();
    const { result } = renderHook(() => useRunAgentTask(), { wrapper });

    await act(async () => {
      await expect(
        result.current.mutateAsync({ taskId: "task-1", status: "created" }),
      ).rejects.toMatchObject({ message: "executor unavailable", queued: true });
    });

    expect(invalidateQueries).toHaveBeenCalledWith({ queryKey: ["agent"] });
  });

  it("does not start when queueing fails and marks the error as not queued", async () => {
    agentMock.queueAgentTask.mockRejectedValueOnce(new Error("queue unavailable"));
    const { wrapper } = createHarness();
    const { result } = renderHook(() => useRunAgentTask(), { wrapper });

    await act(async () => {
      await expect(
        result.current.mutateAsync({ taskId: "task-1", status: "created" }),
      ).rejects.toMatchObject({ message: "queue unavailable", queued: false });
    });

    expect(agentMock.startAgentTask).not.toHaveBeenCalled();
  });
});
