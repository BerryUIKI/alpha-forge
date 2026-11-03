// Tests for WorkspaceList component.

import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { cleanup, render, screen } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import "@testing-library/jest-dom";
import { invoke } from "@tauri-apps/api/core";

import { WorkspaceList } from "./WorkspaceList";

// Mock Tauri invoke so the component can run outside the desktop shell.
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

function renderList() {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
      },
    },
  });

  return render(
    <QueryClientProvider client={queryClient}>
      <WorkspaceList />
    </QueryClientProvider>,
  );
}

describe("WorkspaceList", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    cleanup();
  });

  it("shows the loading state while workspaces are pending", () => {
    vi.mocked(invoke).mockReturnValue(new Promise<never>(() => {}));

    const { container } = renderList();

    expect(container.querySelector(".animate-spin")).not.toBeNull();
  });

  it("shows the empty state when there are no workspaces", async () => {
    vi.mocked(invoke).mockResolvedValue([]);

    renderList();

    expect(await screen.findByText("No workspaces yet")).toBeInTheDocument();
    expect(
      screen.getByText("Create your first workspace to start organizing your research."),
    ).toBeInTheDocument();
  });

  it("shows an error state when loading fails", async () => {
    vi.mocked(invoke).mockRejectedValue({
      code: "INTERNAL",
      message: "database unavailable",
      recoverable: false,
    });

    renderList();

    expect(await screen.findByText("Failed to load workspaces")).toBeInTheDocument();
    expect(screen.getByText("Try Again")).toBeInTheDocument();
  });

  it("renders workspaces returned by the desktop API", async () => {
    vi.mocked(invoke).mockResolvedValue([
      {
        id: "workspace-1",
        name: "Alpha Research",
        created_at: "2024-01-01T00:00:00Z",
        updated_at: "2024-01-01T00:00:00Z",
      },
    ]);

    renderList();

    expect(await screen.findByText("Alpha Research")).toBeInTheDocument();
    expect(screen.getByText(/Created/)).toBeInTheDocument();
  });
});
