// Tests for the Artifacts page.

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import "@testing-library/jest-dom";
import { invoke } from "@tauri-apps/api/core";

import { ToastProvider } from "@/components/common/ToastProvider";
import { ArtifactsPage } from "./ArtifactsPage";
import type { Artifact } from "@/lib/desktop-api";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

const comparisonArtifact: Artifact = {
  id: "artifact-1",
  task_id: "task-1",
  workspace_id: "ws-1",
  artifact_type: "comparison-table",
  title: "Tesla vs Nvidia",
  data: JSON.stringify({
    columns: [{ key: "pe", label: "P/E" }],
    rows: [
      { label: "Tesla", values: { pe: 28.5 } },
      { label: "Nvidia", values: { pe: 40 } },
    ],
  }),
  status: "completed",
  created_at: "2026-01-01T00:00:00Z",
  updated_at: "2026-01-01T00:00:00Z",
};

function renderPage() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });

  return render(
    <QueryClientProvider client={queryClient}>
      <ToastProvider>
        <ArtifactsPage />
      </ToastProvider>
    </QueryClientProvider>,
  );
}

describe("ArtifactsPage", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    cleanup();
  });

  it("renders artifacts and opens a renderer", async () => {
    vi.mocked(invoke).mockResolvedValue([comparisonArtifact]);

    renderPage();

    fireEvent.click(await screen.findByText("Tesla vs Nvidia"));

    expect(await screen.findByText("P/E")).toBeInTheDocument();
    expect(screen.getByText("28.5")).toBeInTheDocument();
    expect(screen.getByText("40")).toBeInTheDocument();
  });

  it("closes the artifact detail", async () => {
    vi.mocked(invoke).mockResolvedValue([comparisonArtifact]);

    renderPage();

    fireEvent.click(await screen.findByText("Tesla vs Nvidia"));
    fireEvent.click(await screen.findByRole("button", { name: "Close artifact detail" }));

    expect(screen.queryByText("28.5")).not.toBeInTheDocument();
  });

  it("shows the empty state", async () => {
    vi.mocked(invoke).mockResolvedValue([]);

    renderPage();

    expect(await screen.findByText("No artifacts yet")).toBeInTheDocument();
  });

  it("shows an error state", async () => {
    vi.mocked(invoke).mockRejectedValue({
      code: "INTERNAL",
      message: "database unavailable",
      recoverable: false,
    });

    renderPage();

    expect(await screen.findByText("Failed to load artifacts")).toBeInTheDocument();
  });

  it("opens an artifact window", async () => {
    vi.mocked(invoke).mockImplementation(((command: string) => {
      if (command === "list_artifacts") {
        return Promise.resolve([comparisonArtifact]);
      }
      if (command === "open_artifact_window") {
        return Promise.resolve(null);
      }
      return Promise.reject(new Error(`Unexpected command: ${command}`));
    }) as unknown as typeof invoke);

    renderPage();

    fireEvent.click(await screen.findByText("Tesla vs Nvidia"));
    fireEvent.click(await screen.findByRole("button", { name: /open in window/i }));

    await waitFor(() => {
      expect(vi.mocked(invoke)).toHaveBeenCalledWith("open_artifact_window", {
        artifactId: "artifact-1",
      });
    });
  });
});
