/**
 * Tests for the ActiveWorkspaceProvider / useActiveWorkspace context.
 *
 * Covers the ADR-0008 resolution order:
 * URL `?workspace=` deep link (valid) > stored localStorage preference >
 * first workspace. The deep-link parameter is stripped from the URL once
 * consumed so it cannot override a later switch.
 */

import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor, act } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { MemoryRouter, useSearchParams } from "react-router-dom";
import { desktopApi } from "@/lib/desktop-api";
import {
  ActiveWorkspaceProvider,
  useActiveWorkspace,
} from "./useActiveWorkspace";

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    workspace: {
      listWorkspaces: vi.fn(),
    },
  },
}));

const STORAGE_KEY = "active-workspace-id";

const workspaces = [
  { id: "ws-1", name: "AI Infrastructure", created_at: "2026-01-01", updated_at: "2026-01-01" },
  { id: "ws-2", name: "Semiconductors", created_at: "2026-01-02", updated_at: "2026-01-02" },
];

beforeEach(() => {
  vi.clearAllMocks();
  localStorage.clear();
});

/** Renders the provider with a probe that exposes context + URL params. */
function renderProvider(initialEntries: string[] = ["/"]) {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });

  function Probe() {
    const { workspaceId, workspaces: list, setActiveWorkspace } = useActiveWorkspace();
    const [searchParams] = useSearchParams();
    return (
      <div>
        <span data-testid="workspace-id">{workspaceId}</span>
        <span data-testid="workspace-count">{list.length}</span>
        <span data-testid="url-params">{searchParams.toString()}</span>
        <button data-testid="switch-ws-2" onClick={() => setActiveWorkspace("ws-2")}>
          switch
        </button>
      </div>
    );
  }

  render(
    <QueryClientProvider client={queryClient}>
      <MemoryRouter initialEntries={initialEntries}>
        <ActiveWorkspaceProvider>
          <Probe />
        </ActiveWorkspaceProvider>
      </MemoryRouter>
    </QueryClientProvider>,
  );
}

describe("ActiveWorkspaceProvider", () => {
  it("resolves the first workspace when nothing is stored", async () => {
    vi.mocked(desktopApi.workspace.listWorkspaces).mockResolvedValue(workspaces as any);
    renderProvider(["/research"]);

    await waitFor(() =>
      expect(screen.getByTestId("workspace-id")).toHaveTextContent("ws-1"),
    );
    expect(localStorage.getItem(STORAGE_KEY)).toBe("ws-1");
  });

  it("uses the stored preference when it is valid", async () => {
    localStorage.setItem(STORAGE_KEY, "ws-2");
    vi.mocked(desktopApi.workspace.listWorkspaces).mockResolvedValue(workspaces as any);
    renderProvider(["/research"]);

    await waitFor(() =>
      expect(screen.getByTestId("workspace-id")).toHaveTextContent("ws-2"),
    );
  });

  it("falls back to the first workspace when the stored id is stale", async () => {
    localStorage.setItem(STORAGE_KEY, "ws-99");
    vi.mocked(desktopApi.workspace.listWorkspaces).mockResolvedValue(workspaces as any);
    renderProvider(["/research"]);

    await waitFor(() =>
      expect(screen.getByTestId("workspace-id")).toHaveTextContent("ws-1"),
    );
    expect(localStorage.getItem(STORAGE_KEY)).toBe("ws-1");
  });

  it("lets a valid URL deep link win over the stored preference and strips it", async () => {
    localStorage.setItem(STORAGE_KEY, "ws-1");
    vi.mocked(desktopApi.workspace.listWorkspaces).mockResolvedValue(workspaces as any);
    renderProvider(["/research?workspace=ws-2&project=p1"]);

    await waitFor(() =>
      expect(screen.getByTestId("workspace-id")).toHaveTextContent("ws-2"),
    );
    // The deep-link parameter is consumed and persisted.
    expect(localStorage.getItem(STORAGE_KEY)).toBe("ws-2");
    await waitFor(() =>
      expect(screen.getByTestId("url-params")).toHaveTextContent("project=p1"),
    );
  });

  it("ignores and strips an invalid URL deep link", async () => {
    localStorage.setItem(STORAGE_KEY, "ws-1");
    vi.mocked(desktopApi.workspace.listWorkspaces).mockResolvedValue(workspaces as any);
    renderProvider(["/research?workspace=ws-99&project=p1"]);

    await waitFor(() =>
      expect(screen.getByTestId("workspace-id")).toHaveTextContent("ws-1"),
    );
    await waitFor(() =>
      expect(screen.getByTestId("url-params")).toHaveTextContent("project=p1"),
    );
  });

  it("setActiveWorkspace updates the context and persists the selection", async () => {
    vi.mocked(desktopApi.workspace.listWorkspaces).mockResolvedValue(workspaces as any);
    renderProvider(["/research"]);

    await waitFor(() =>
      expect(screen.getByTestId("workspace-id")).toHaveTextContent("ws-1"),
    );
    act(() => {
      screen.getByTestId("switch-ws-2").click();
    });

    await waitFor(() =>
      expect(screen.getByTestId("workspace-id")).toHaveTextContent("ws-2"),
    );
    expect(localStorage.getItem(STORAGE_KEY)).toBe("ws-2");
  });

  it("yields an empty id when there are no workspaces", async () => {
    vi.mocked(desktopApi.workspace.listWorkspaces).mockResolvedValue([]);
    renderProvider(["/research"]);

    await waitFor(() =>
      expect(screen.getByTestId("workspace-id")).toHaveTextContent(""),
    );
    expect(screen.getByTestId("workspace-count")).toHaveTextContent("0");
  });
});
