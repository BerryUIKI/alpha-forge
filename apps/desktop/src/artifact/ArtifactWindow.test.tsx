// Tests for the standalone artifact window entry.

import { afterEach, describe, expect, it } from "vitest";
import { cleanup, render, screen } from "@testing-library/react";
import "@testing-library/jest-dom";

import { ArtifactWindow } from "./ArtifactWindow";

describe("ArtifactWindow", () => {
  afterEach(() => {
    cleanup();
    delete window.__ALPHAFORGE_ARTIFACT__;
  });

  it("renders the injected artifact payload", () => {
    window.__ALPHAFORGE_ARTIFACT__ = {
      id: "artifact-1",
      title: "Tesla vs Nvidia",
      artifact_type: "comparison-table",
      data: {
        columns: [{ key: "pe", label: "P/E" }],
        rows: [{ label: "Tesla", values: { pe: 28.5 } }],
      },
      created_at: "2026-01-01T00:00:00Z",
    };

    render(<ArtifactWindow />);

    expect(screen.getByText("Tesla vs Nvidia")).toBeInTheDocument();
    expect(screen.getByText("Tesla")).toBeInTheDocument();
    expect(screen.getByText("28.5")).toBeInTheDocument();
  });

  it("shows a fallback when no payload was injected", () => {
    render(<ArtifactWindow />);

    expect(screen.getByText("No artifact data was provided.")).toBeInTheDocument();
  });
});
