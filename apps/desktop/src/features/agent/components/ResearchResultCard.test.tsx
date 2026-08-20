import "@testing-library/jest-dom";
import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ResearchResultCard } from "./ResearchResultCard";

vi.mock("@/lib/i18n/useLocale", () => ({
  useLocale: () => ({
    t: (key: string) =>
      ({
        researchSummary: "Summary",
        researchClaims: "Key Claims",
        researchEvidence: "Evidence",
        researchRisks: "Risks",
        researchConfidence: "Confidence",
        noResultsAvailable: "No structured results available",
      })[key] ?? key,
  }),
}));

describe("ResearchResultCard", () => {
  it("shows fallback message for null payload", () => {
    render(<ResearchResultCard payload={null} />);
    expect(screen.getByText("No structured results available")).toBeInTheDocument();
  });

  it("shows fallback message for invalid JSON payload", () => {
    render(<ResearchResultCard payload="invalid-json" />);
    expect(screen.getByText("No structured results available")).toBeInTheDocument();
  });

  it("renders summary, claims, evidence, risks, confidence for valid payload", () => {
    const payload = JSON.stringify({
      summary: "Test summary text.",
      claims: ["Claim 1", "Claim 2"],
      evidence: ["Evidence 1"],
      risks: ["Risk 1", "Risk 2"],
      confidence: 80,
    });

    render(<ResearchResultCard payload={payload} />);

    expect(screen.getByText("Summary")).toBeInTheDocument();
    expect(screen.getByText("Test summary text.")).toBeInTheDocument();

    expect(screen.getByText("Key Claims")).toBeInTheDocument();
    expect(screen.getByText("Claim 1")).toBeInTheDocument();
    expect(screen.getByText("Claim 2")).toBeInTheDocument();

    expect(screen.getByText("Evidence")).toBeInTheDocument();
    expect(screen.getByText("Evidence 1")).toBeInTheDocument();

    expect(screen.getByText("Risks")).toBeInTheDocument();
    expect(screen.getByText("Risk 1")).toBeInTheDocument();
    expect(screen.getByText("Risk 2")).toBeInTheDocument();

    expect(screen.getByText("Confidence")).toBeInTheDocument();
    expect(screen.getByText("80%")).toBeInTheDocument();
  });

  it("hides empty sections", () => {
    const payload = JSON.stringify({
      summary: "Test summary text.",
      claims: [],
      evidence: [],
      risks: [],
      confidence: 50,
    });

    render(<ResearchResultCard payload={payload} />);

    expect(screen.getByText("Summary")).toBeInTheDocument();
    expect(screen.queryByText("Key Claims")).not.toBeInTheDocument();
    expect(screen.queryByText("Evidence")).not.toBeInTheDocument();
    expect(screen.queryByText("Risks")).not.toBeInTheDocument();
  });

  it("renders confidence badge with appropriate colors", () => {
    const makePayload = (conf: number) =>
      JSON.stringify({
        summary: "Test",
        claims: [],
        evidence: [],
        risks: [],
        confidence: conf,
      });

    const { unmount } = render(<ResearchResultCard payload={makePayload(20)} />);
    let badge = screen.getByText("20%");
    expect(badge).toHaveClass("text-destructive");
    unmount();

    render(<ResearchResultCard payload={makePayload(50)} />);
    badge = screen.getByText("50%");
    expect(badge).toHaveClass("text-amber-600");
  });
});
