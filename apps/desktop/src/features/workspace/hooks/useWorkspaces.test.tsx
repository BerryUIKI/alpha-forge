import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { type ReactNode } from "react";
import { desktopApi } from "@/lib/desktop-api";
import {
  useWorkspaces,
  useWorkspace,
  useCreateWorkspace,
  useUpdateWorkspace,
  useDeleteWorkspace,
} from "./useWorkspaces";

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    workspace: {
      listWorkspaces: vi.fn(),
      getWorkspace: vi.fn(),
      createWorkspace: vi.fn(),
      updateWorkspace: vi.fn(),
      deleteWorkspace: vi.fn(),
    },
  },
}));

function wrapper({ children }: { children: ReactNode }) {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>;
}

beforeEach(() => {
  vi.clearAllMocks();
});

describe("useWorkspaces", () => {
  it("fetches all workspaces", async () => {
    vi.mocked(desktopApi.workspace.listWorkspaces).mockResolvedValue([
      { id: "ws-1", name: "Test" },
    ] as any);
    const { result } = renderHook(() => useWorkspaces(), { wrapper });
    expect(desktopApi.workspace.listWorkspaces).toHaveBeenCalled();
    await waitFor(() => expect(result.current.data).toEqual([{ id: "ws-1", name: "Test" }]));
  });

  it("returns error when API fails", async () => {
    vi.mocked(desktopApi.workspace.listWorkspaces).mockRejectedValue(new Error("API error"));
    const { result } = renderHook(() => useWorkspaces(), { wrapper });
    await waitFor(() => expect(result.current.error).toBeTruthy());
  });
});

describe("useWorkspace", () => {
  it("is disabled without id", () => {
    const { result } = renderHook(() => useWorkspace(""), { wrapper });
    expect(result.current.fetchStatus).toBe("idle");
  });

  it("fetches a single workspace", async () => {
    vi.mocked(desktopApi.workspace.getWorkspace).mockResolvedValue({ id: "ws-1", name: "Test" } as any);
    const { result } = renderHook(() => useWorkspace("ws-1"), { wrapper });
    expect(desktopApi.workspace.getWorkspace).toHaveBeenCalledWith("ws-1");
    await waitFor(() => expect(result.current.data).toEqual({ id: "ws-1", name: "Test" }));
  });
});

describe("useCreateWorkspace", () => {
  it("calls createWorkspace", async () => {
    vi.mocked(desktopApi.workspace.createWorkspace).mockResolvedValue({ id: "ws-new", name: "New" } as any);
    const { result } = renderHook(() => useCreateWorkspace(), { wrapper });
    result.current.mutate("New Workspace");
    await waitFor(() =>
      expect(desktopApi.workspace.createWorkspace).toHaveBeenCalledWith("New Workspace"),
    );
  });
});

describe("useUpdateWorkspace", () => {
  it("calls updateWorkspace", async () => {
    vi.mocked(desktopApi.workspace.updateWorkspace).mockResolvedValue({ id: "ws-1", name: "Updated" } as any);
    const { result } = renderHook(() => useUpdateWorkspace(), { wrapper });
    result.current.mutate({ id: "ws-1", name: "Updated" });
    await waitFor(() =>
      expect(desktopApi.workspace.updateWorkspace).toHaveBeenCalledWith("ws-1", "Updated"),
    );
  });
});

describe("useDeleteWorkspace", () => {
  it("calls deleteWorkspace", async () => {
    vi.mocked(desktopApi.workspace.deleteWorkspace).mockResolvedValue(undefined);
    const { result } = renderHook(() => useDeleteWorkspace(), { wrapper });
    result.current.mutate("ws-1");
    await waitFor(() => expect(desktopApi.workspace.deleteWorkspace).toHaveBeenCalledWith("ws-1"));
  });
});