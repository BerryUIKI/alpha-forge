import { render, screen, fireEvent } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ShadowAnalysis } from "./ShadowAnalysis";
import * as gooseHooks from "@/hooks/useGooseAnalysis";
import { LocaleProvider } from "@/lib/i18n/LocaleProvider";

vi.mock("@/hooks/useGooseAnalysis");

describe("ShadowAnalysis Component", () => {
  const mockStartAnalysis = vi.fn();
  const mockCancelAnalysis = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
  });

  const renderComponent = (props: {
    workspaceId: string;
    thesisId?: string;
    onComplete?: (result: any) => void;
  }) => {
    return render(
      <LocaleProvider>
        <ShadowAnalysis {...props} />
      </LocaleProvider>
    );
  };

  it("renders loading spinner when health status is loading", () => {
    vi.mocked(gooseHooks.useGooseShadowAnalysis).mockReturnValue({
      isReady: false,
      health: undefined,
      startAnalysis: mockStartAnalysis,
      isRunning: false,
      result: null,
      startError: null,
      validationError: null,
      cancelAnalysis: mockCancelAnalysis,
      status: "idle",
      isPending: false,
      isSuccess: false,
      isError: false,
    } as any);

    renderComponent({ workspaceId: "ws-1" });
    expect(screen.getByLabelText("Checking Goose service status")).toBeInTheDocument();
  });

  it("renders unavailable state when binary is not available", () => {
    vi.mocked(gooseHooks.useGooseShadowAnalysis).mockReturnValue({
      isReady: false,
      health: {
        binary_available: false,
        shadow_mode_enabled: true,
        max_concurrent: 1,
      },
      startAnalysis: mockStartAnalysis,
      isRunning: false,
      result: null,
      startError: null,
      validationError: null,
      cancelAnalysis: mockCancelAnalysis,
      status: "idle",
      isPending: false,
      isSuccess: false,
      isError: false,
    } as any);

    renderComponent({ workspaceId: "ws-1" });
    expect(screen.getByText("Shadow Analysis Unavailable")).toBeInTheDocument();
  });

  it("renders initial form and starts analysis on click", () => {
    vi.mocked(gooseHooks.useGooseShadowAnalysis).mockReturnValue({
      isReady: true,
      health: {
        binary_available: true,
        shadow_mode_enabled: true,
        max_concurrent: 1,
      },
      startAnalysis: mockStartAnalysis,
      isRunning: false,
      result: null,
      startError: null,
      validationError: null,
      cancelAnalysis: mockCancelAnalysis,
      status: "idle",
      isPending: false,
      isSuccess: false,
      isError: false,
    } as any);

    renderComponent({ workspaceId: "ws-1", thesisId: "thesis-100" });

    expect(screen.getByText("Shadow Analysis")).toBeInTheDocument();
    expect(screen.getByText("Focusing on thesis: thesis-100")).toBeInTheDocument();

    const textarea = screen.getByPlaceholderText(/Focus on financial metrics/i);
    fireEvent.change(textarea, { target: { value: "Analyze pricing power" } });

    const startBtn = screen.getByRole("button", { name: "Start Shadow Analysis" });
    fireEvent.click(startBtn);

    expect(mockStartAnalysis).toHaveBeenCalledWith({
      thesis_id: "thesis-100",
      instructions: "Analyze pricing power",
    });
  });

  it("renders running state with cancel button", () => {
    vi.mocked(gooseHooks.useGooseShadowAnalysis).mockReturnValue({
      isReady: true,
      health: {
        binary_available: true,
        shadow_mode_enabled: true,
        max_concurrent: 1,
      },
      startAnalysis: mockStartAnalysis,
      isRunning: true,
      result: { run_id: "run-999" } as any,
      startError: null,
      validationError: null,
      cancelAnalysis: mockCancelAnalysis,
      status: "pending",
      isPending: true,
      isSuccess: false,
      isError: false,
    } as any);

    renderComponent({ workspaceId: "ws-1" });

    expect(screen.getByText(/This may take a few minutes/i)).toBeInTheDocument();
    const cancelBtn = screen.getByRole("button", { name: "Cancel Analysis" });
    fireEvent.click(cancelBtn);

    expect(mockCancelAnalysis).toHaveBeenCalledWith("run-999");
  });

  it("renders structured analysis results on completion", () => {
    const mockResult = {
      run_id: "run-101",
      workspace_id: "ws-1",
      duration_ms: 3200,
      response: {
        summary: "Semiconductor supply chain remains robust",
        confidence: 88,
        claims: [
          {
            id: "claim-1",
            claim: "Lead times shortened by 4 weeks",
            confidence: 90,
            source_ids: ["src-1"],
            contradicting_source_ids: [],
          },
        ],
        evidence: [
          {
            claim_id: "claim-1",
            source_id: "src-1",
            excerpt: "Supplier survey indicates lead times normalized",
            relation: "supports",
            confidence: 92,
          },
        ],
        risks: [
          {
            id: "risk-1",
            risk: "Geopolitical trade restrictions",
            severity: "high",
            related_claim_ids: ["claim-1"],
            mitigation: "Regional fabrication redundancy",
          },
        ],
        unknowns: ["Inventory levels at tier-2 distributors"],
        source_ids: ["src-1"],
      },
    };

    const onComplete = vi.fn();

    vi.mocked(gooseHooks.useGooseShadowAnalysis).mockReturnValue({
      isReady: true,
      health: {
        binary_available: true,
        shadow_mode_enabled: true,
        max_concurrent: 1,
      },
      startAnalysis: mockStartAnalysis,
      isRunning: false,
      result: mockResult as any,
      startError: null,
      validationError: null,
      cancelAnalysis: mockCancelAnalysis,
      status: "success",
      isPending: false,
      isSuccess: true,
      isError: false,
    } as any);

    renderComponent({ workspaceId: "ws-1", onComplete });

    expect(screen.getByText("Analysis Complete")).toBeInTheDocument();
    expect(screen.getByText("Semiconductor supply chain remains robust")).toBeInTheDocument();
    expect(screen.getByText("Lead times shortened by 4 weeks")).toBeInTheDocument();
    expect(screen.getByText('"Supplier survey indicates lead times normalized"')).toBeInTheDocument();
    expect(screen.getByText("Geopolitical trade restrictions")).toBeInTheDocument();
    expect(screen.getByText("Inventory levels at tier-2 distributors")).toBeInTheDocument();

    const resetBtn = screen.getByRole("button", { name: "Run New Analysis" });
    fireEvent.click(resetBtn);
    expect(onComplete).toHaveBeenCalledWith(mockResult.response);
  });
});
