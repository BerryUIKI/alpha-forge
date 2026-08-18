import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { type ReactNode } from "react";
import { desktopApi } from "@/lib/desktop-api";
import { useTheses, useCreateThesis, useThesisEvidence } from "./useTheses";

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    thesis: {
      listTheses: vi.fn(),
      createThesis: vi.fn(),
      listThesisEvidence: vi.fn(),
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

describe("useTheses", () => {
  it("fetches theses for a workspace", async () => {
    vi.mocked(desktopApi.thesis.listTheses).mockResolvedValue([
      { id: "t1", title: "AI thesis" },
    ] as any);
    const { result } = renderHook(() => useTheses("ws-1"), { wrapper });
    expect(desktopApi.thesis.listTheses).toHaveBeenCalledWith("ws-1");
    await waitFor(() => expect(result.current.data).toEqual([{ id: "t1", title: "AI thesis" }]));
  });

  it("is disabled without workspaceId", () => {
    const { result } = renderHook(() => useTheses(""), { wrapper });
    expect(result.current.fetchStatus).toBe("idle");
  });
});

describe("useThesisEvidence", () => {
  it("is disabled without thesisId", () => {
    const { result } = renderHook(() => useThesisEvidence(""), { wrapper });
    expect(result.current.fetchStatus).toBe("idle");
  });
});

describe("useCreateThesis", () => {
  it("calls createThesis and invalidates", async () => {
    vi.mocked(desktopApi.thesis.createThesis).mockResolvedValue({ id: "t-new" } as any);
    const { result } = renderHook(() => useCreateThesis(), { wrapper });
    result.current.mutate({ workspaceId: "ws-1", title: "New thesis", summary: "Test" } as any);
    await waitFor(() =>
      expect(desktopApi.thesis.createThesis).toHaveBeenCalledWith({
        workspaceId: "ws-1",
        title: "New thesis",
        summary: "Test",
      }),
    );
  });
});