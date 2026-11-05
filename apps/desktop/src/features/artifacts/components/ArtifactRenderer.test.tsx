// Tests for artifact renderers.

import { afterEach, describe, expect, it } from "vitest";
import { cleanup, render, screen } from "@testing-library/react";
import "@testing-library/jest-dom";

import { ArtifactRenderer } from "./ArtifactRenderer";
import type { Artifact } from "@/lib/desktop-api";

function makeArtifact(artifactType: string, data: unknown): Artifact {
  return {
    id: "artifact-1",
    task_id: "task-1",
    workspace_id: "ws-1",
    artifact_type: artifactType,
    title: "Artifact",
    data: JSON.stringify(data),
    status: "completed",
    created_at: "2026-01-01T00:00:00Z",
    updated_at: "2026-01-01T00:00:00Z",
  };
}

describe("ArtifactRenderer", () => {
  afterEach(() => {
    cleanup();
  });

  it("renders a comparison table", () => {
    render(
      <ArtifactRenderer
        artifact={makeArtifact("comparison-table", {
          columns: [
            { key: "pe", label: "P/E" },
            { key: "growth", label: "Growth" },
          ],
          rows: [
            { label: "Tesla", values: { pe: 28.5, growth: "5%" } },
            { label: "Nvidia", values: { pe: 40 } },
          ],
        })}
      />,
    );

    expect(screen.getByText("Tesla")).toBeInTheDocument();
    expect(screen.getByText("28.5")).toBeInTheDocument();
    expect(screen.getByText("—")).toBeInTheDocument();
    expect(screen.getByRole("columnheader", { name: "P/E" })).toBeInTheDocument();
  });

  it("renders a timeline", () => {
    render(
      <ArtifactRenderer
        artifact={makeArtifact("timeline", {
          events: [
            {
              date: "2026-01-15",
              title: "Q4 earnings",
              description: "Beat estimates",
            },
          ],
        })}
      />,
    );

    expect(screen.getByText("2026-01-15")).toBeInTheDocument();
    expect(screen.getByText("Q4 earnings")).toBeInTheDocument();
    expect(screen.getByText("Beat estimates")).toBeInTheDocument();
  });

  it("shows a fallback for unsupported artifacts", () => {
    render(<ArtifactRenderer artifact={makeArtifact("pie-chart", {})} />);

    expect(screen.getByText('Unsupported artifact type "pie-chart".')).toBeInTheDocument();
  });
});
