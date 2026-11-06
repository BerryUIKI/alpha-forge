// Tests for citation parsing.

import { describe, expect, it } from "vitest";

import { parseCitations } from "./citations";

describe("parseCitations", () => {
  it("returns an empty list for missing or invalid output", () => {
    expect(parseCitations(null)).toEqual([]);
    expect(parseCitations("not json")).toEqual([]);
    expect(parseCitations("{}")).toEqual([]);
    expect(parseCitations('{"citations":"nope"}')).toEqual([]);
  });

  it("keeps valid citations and drops invalid entries", () => {
    const output = JSON.stringify({
      citations: [
        { kind: "document", id: "doc-1", title: "Tesla memo" },
        { kind: "task", id: "task-1", title: "Earlier analysis" },
        { kind: "document", id: " ", title: "Missing id" },
        { kind: "other", id: "x", title: "Bad kind" },
        { kind: "task", id: "task-2" },
      ],
    });

    const citations = parseCitations(output);

    expect(citations).toHaveLength(2);
    expect(citations[0]).toEqual({
      kind: "document",
      id: "doc-1",
      title: "Tesla memo",
    });
    expect(citations[1]?.id).toBe("task-1");
  });
});
