import { describe, expect, it } from "vitest";
import { parseResearchCompletion } from "./ResearchCompletion";

describe("parseResearchCompletion", () => {
  it("parses valid JSON correctly", () => {
    const valid = JSON.stringify({
      summary: "Test summary",
      claims: ["claim 1"],
      evidence: ["evidence 1"],
      risks: ["risk 1"],
      confidence: 85,
    });
    const result = parseResearchCompletion(valid);
    expect(result).not.toBeNull();
    expect(result?.summary).toBe("Test summary");
    expect(result?.confidence).toBe(85);
  });

  it("returns null on missing summary", () => {
    const invalid = JSON.stringify({
      claims: [],
      evidence: [],
      risks: [],
      confidence: 85,
    });
    expect(parseResearchCompletion(invalid)).toBeNull();
  });

  it("returns null on empty summary", () => {
    const invalid = JSON.stringify({
      summary: "",
      claims: [],
      evidence: [],
      risks: [],
      confidence: 85,
    });
    expect(parseResearchCompletion(invalid)).toBeNull();
  });

  it("returns null on confidence > 100", () => {
    const invalid = JSON.stringify({
      summary: "Test",
      claims: [],
      evidence: [],
      risks: [],
      confidence: 101,
    });
    expect(parseResearchCompletion(invalid)).toBeNull();
  });

  it("returns null on confidence < 0", () => {
    const invalid = JSON.stringify({
      summary: "Test",
      claims: [],
      evidence: [],
      risks: [],
      confidence: -1,
    });
    expect(parseResearchCompletion(invalid)).toBeNull();
  });

  it("returns null on non-JSON string", () => {
    expect(parseResearchCompletion("not json")).toBeNull();
  });

  it("returns null on null input", () => {
    expect(parseResearchCompletion(null)).toBeNull();
  });

  it("strips extra fields", () => {
    const extra = JSON.stringify({
      summary: "Test",
      claims: [],
      evidence: [],
      risks: [],
      confidence: 85,
      extraField: "should be ignored",
    });
    const result = parseResearchCompletion(extra);
    expect(result).not.toBeNull();
    expect(Object.keys(result!)).not.toContain("extraField");
  });
});
