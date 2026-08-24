import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import * as api from "./goose";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
const mockInvoke = vi.mocked(invoke);

describe("desktop-api/goose", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  const mockResponse: api.StructuredResponse = {
    summary: "Comprehensive market analysis",
    claims: [
      {
        id: "claim-1",
        claim: "Revenue increased by 15%",
        confidence: 85,
        source_ids: ["src-1"],
        contradicting_source_ids: [],
      },
    ],
    evidence: [
      {
        claim_id: "claim-1",
        source_id: "src-1",
        excerpt: "Q3 earnings report showed 15% revenue growth",
        relation: "supports",
        confidence: 90,
      },
    ],
    contradictions: [],
    risks: [
      {
        id: "risk-1",
        risk: "Macro headwinds",
        severity: "medium",
        related_claim_ids: ["claim-1"],
        mitigation: "Diversification",
      },
    ],
    unknowns: ["Foreign currency impact"],
    source_ids: ["src-1"],
    confidence: 85,
    provider: "openai",
    model: "gpt-4o",
    recipe_version: "1.0",
  };

  const mockResult: api.ShadowAnalysisResult = {
    run_id: "goose-run-123",
    workspace_id: "ws-1",
    response: mockResponse,
    duration_ms: 2500,
    provider: "openai",
    model: "gpt-4o",
  };

  it("startShadowAnalysis validates and returns structured analysis result", async () => {
    mockInvoke.mockResolvedValueOnce(mockResult);

    const input: api.StartShadowAnalysisInput = {
      workspace_id: "ws-1",
      instructions: "Focus on profitability",
    };

    const res = await api.startShadowAnalysis(input);
    expect(mockInvoke).toHaveBeenCalledWith("start_goose_shadow_analysis", { input });
    expect(res.run_id).toBe("goose-run-123");
    expect(res.response.claims).toHaveLength(1);
    expect(res.response.claims[0]!.claim).toBe("Revenue increased by 15%");
  });

  it("cancelAnalysis invokes cancel_goose_analysis", async () => {
    mockInvoke.mockResolvedValueOnce(null);

    await api.cancelAnalysis("goose-run-123");
    expect(mockInvoke).toHaveBeenCalledWith("cancel_goose_analysis", { runId: "goose-run-123" });
  });

  it("checkGooseHealth validates and returns health status", async () => {
    const mockHealth: api.GooseHealthStatus = {
      binary_available: true,
      shadow_mode_enabled: true,
      max_concurrent: 2,
    };
    mockInvoke.mockResolvedValueOnce(mockHealth);

    const health = await api.checkGooseHealth();
    expect(mockInvoke).toHaveBeenCalledWith("check_goose_health", undefined);
    expect(health.binary_available).toBe(true);
    expect(health.shadow_mode_enabled).toBe(true);
    expect(health.max_concurrent).toBe(2);
  });

  it("rejects invalid analysis result", async () => {
    mockInvoke.mockResolvedValueOnce({
      run_id: "goose-run-123",
      // missing required fields
    });

    await expect(api.startShadowAnalysis({ workspace_id: "ws-1" })).rejects.toThrow();
  });
});
