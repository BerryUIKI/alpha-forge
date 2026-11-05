// Tests for artifact payload parsing.

import { describe, expect, it } from "vitest";

import { parseArtifactData } from "./parseArtifactData";

describe("parseArtifactData", () => {
  it("parses a comparison table", () => {
    const parsed = parseArtifactData(
      "comparison-table",
      JSON.stringify({
        columns: [
          { key: "pe", label: "P/E" },
          { key: "growth", label: "Growth" },
        ],
        rows: [
          { label: "Tesla", values: { pe: 28.5, growth: "5%" } },
          { label: "Nvidia", values: { pe: 40 } },
        ],
      }),
    );

    expect(parsed.kind).toBe("comparison-table");
    if (parsed.kind !== "comparison-table") {
      return;
    }
    expect(parsed.data.columns).toHaveLength(2);
    expect(parsed.data.rows[0]?.values.pe).toBe(28.5);
  });

  it("rejects a malformed comparison table", () => {
    const parsed = parseArtifactData("comparison-table", JSON.stringify({ columns: [], rows: [] }));

    expect(parsed.kind).toBe("unsupported");
  });

  it("parses a timeline and normalizes empty descriptions", () => {
    const parsed = parseArtifactData(
      "timeline",
      JSON.stringify({
        events: [
          { date: "2026-01-15", title: "Q4 earnings", description: "Beat estimates" },
          { date: "2026-02-01", title: "Review", description: "  " },
        ],
      }),
    );

    expect(parsed.kind).toBe("timeline");
    if (parsed.kind !== "timeline") {
      return;
    }
    expect(parsed.data.events[1]?.description).toBeNull();
  });

  it("reports unsupported artifact types", () => {
    const parsed = parseArtifactData("pie-chart", "{}");

    expect(parsed.kind).toBe("unsupported");
  });

  it("reports invalid JSON", () => {
    const parsed = parseArtifactData("timeline", "not json");

    expect(parsed.kind).toBe("unsupported");
  });
});
