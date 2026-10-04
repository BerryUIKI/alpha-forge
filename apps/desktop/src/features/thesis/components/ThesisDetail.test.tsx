import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { type ReactNode } from "react";
import { desktopApi } from "@/lib/desktop-api";
import { ThesisDetail } from "./ThesisDetail";
import type { InvestmentThesis } from "@/lib/desktop-api/thesis";

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    thesis: {
      listThesisEvidence: vi.fn(),
      listThesisConfidenceHistory: vi.fn(),
      addThesisEvidence: vi.fn(),
      deleteThesisEvidence: vi.fn(),
      activateThesis: vi.fn(),
      startThesisValidation: vi.fn(),
      completeThesisValidation: vi.fn(),
      updateThesisConfidence: vi.fn(),
      closeThesis: vi.fn(),
      deleteThesis: vi.fn(),
      linkThesisAsset: vi.fn(),
    },
    knowledgeGraph: {
      listKnowledgeEntities: vi.fn(),
      listThesisKnowledgeLinks: vi.fn(),
      linkThesisKnowledgeEntity: vi.fn(),
    },
    financial: {
      listActiveAssets: vi.fn(),
    },
    agent: {
      createAgentTask: vi.fn(),
      queueAgentTask: vi.fn(),
      startAgentTask: vi.fn(),
      listAgentTasks: vi.fn(),
    },
    goose: {
      checkGooseHealth: vi.fn(),
      startGooseAnalysis: vi.fn(),
      cancelGooseAnalysis: vi.fn(),
    },
  },
}));

function renderWithClient(ui: ReactNode) {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return render(
    <QueryClientProvider client={queryClient}>{ui}</QueryClientProvider>,
  );
}

const mockThesis: InvestmentThesis = {
  id: "th-1",
  workspaceId: "ws-1",
  title: "NVIDIA Blackwell Ramp",
  thesis: "Datacenter demand accelerates gross margins to 76%",
  confidence: 85,
  status: "active",
  validationDate: null,
  outcome: null,
  portfolioAssetId: null,
  createdAt: "2026-09-01T00:00:00Z",
  updatedAt: "2026-09-01T00:00:00Z",
};

describe("ThesisDetail & Agent Research Loop (Direction 3)", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(desktopApi.thesis.listThesisEvidence).mockResolvedValue([]);
    vi.mocked(desktopApi.thesis.listThesisConfidenceHistory).mockResolvedValue([]);
    vi.mocked(desktopApi.knowledgeGraph.listThesisKnowledgeLinks).mockResolvedValue([]);
    vi.mocked(desktopApi.knowledgeGraph.listKnowledgeEntities).mockResolvedValue([]);
    vi.mocked(desktopApi.financial.listActiveAssets).mockResolvedValue([]);
    vi.mocked(desktopApi.goose.checkGooseHealth).mockResolvedValue({
      healthy: true,
      shadow_mode_available: true,
      binary_path: "/usr/local/bin/goose",
      version: "1.0.0",
    } as any);
  });

  it("renders thesis details and the Agent research action banner", async () => {
    renderWithClient(
      <ThesisDetail thesis={mockThesis} onDeleted={vi.fn()} />,
    );

    expect(screen.getByText("NVIDIA Blackwell Ramp")).toBeInTheDocument();
    expect(screen.getByText("AI 证据搜集与论点证伪")).toBeInTheDocument();
    expect(screen.getByText("启动 Agent 搜集证据")).toBeInTheDocument();
  });

  it("dispatches agent task when clicking launch agent evidence button", async () => {
    vi.mocked(desktopApi.agent.createAgentTask).mockResolvedValue({
      id: "task-99",
      workspaceId: "ws-1",
      title: "Investigate thesis: NVIDIA Blackwell Ramp",
      status: "created",
      created_at: "2026-10-05T00:00:00Z",
      updated_at: "2026-10-05T00:00:00Z",
    } as any);
    vi.mocked(desktopApi.agent.queueAgentTask).mockResolvedValue({} as any);
    vi.mocked(desktopApi.agent.startAgentTask).mockResolvedValue({} as any);

    renderWithClient(
      <ThesisDetail thesis={mockThesis} onDeleted={vi.fn()} />,
    );

    const launchBtn = screen.getByText("启动 Agent 搜集证据");
    fireEvent.click(launchBtn);

    await waitFor(() => {
      expect(desktopApi.agent.createAgentTask).toHaveBeenCalledWith(
        "ws-1",
        "Investigate thesis: NVIDIA Blackwell Ramp",
        expect.stringContaining("Datacenter demand accelerates"),
      );
      expect(desktopApi.agent.queueAgentTask).toHaveBeenCalledWith("task-99");
      expect(desktopApi.agent.startAgentTask).toHaveBeenCalledWith("task-99");
    });

    expect(
      await screen.findByText("证据搜集任务已创建并启动"),
    ).toBeInTheDocument();
  });
});
