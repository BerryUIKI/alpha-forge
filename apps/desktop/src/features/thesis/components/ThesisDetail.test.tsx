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
  return render(<QueryClientProvider client={queryClient}>{ui}</QueryClientProvider>);
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
    renderWithClient(<ThesisDetail thesis={mockThesis} onDeleted={vi.fn()} />);

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

    renderWithClient(<ThesisDetail thesis={mockThesis} onDeleted={vi.fn()} />);

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

    expect(await screen.findByText("证据搜集任务已创建并启动")).toBeInTheDocument();
  });

  it("resets unsaved drafts when the selected thesis changes", async () => {
    const { rerender } = renderWithClient(<ThesisDetail thesis={mockThesis} onDeleted={vi.fn()} />);

    const slider = screen.getByRole("slider") as HTMLInputElement;
    fireEvent.change(slider, { target: { value: "42" } });
    expect(slider.value).toBe("42");

    const otherThesis: InvestmentThesis = {
      ...mockThesis,
      id: "th-2",
      title: "AMD Share Gains",
      confidence: 30,
    };

    rerender(
      <QueryClientProvider client={new QueryClient()}>
        <ThesisDetail thesis={otherThesis} onDeleted={vi.fn()} />
      </QueryClientProvider>,
    );

    await waitFor(() => {
      expect(screen.getByText("AMD Share Gains")).toBeInTheDocument();
      expect((screen.getByRole("slider") as HTMLInputElement).value).toBe("30");
    });
    // The evidence draft belongs to the previous thesis and must not follow.
    expect(
      (screen.getByPlaceholderText("添加事实、数据点或论点...") as HTMLTextAreaElement).value,
    ).toBe("");
  });

  it("requires confirmation before deleting a thesis", async () => {
    const { container } = renderWithClient(
      <ThesisDetail thesis={mockThesis} onDeleted={vi.fn()} />,
    );

    fireEvent.click(screen.getByLabelText("删除论点"));

    expect(desktopApi.thesis.deleteThesis).not.toHaveBeenCalled();
    expect(await screen.findByText(/永久删除论点「NVIDIA Blackwell Ramp」/)).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: "取消" }));
    await waitFor(() =>
      expect(screen.queryByText(/永久删除论点「NVIDIA Blackwell Ramp」/)).not.toBeInTheDocument(),
    );
    expect(desktopApi.thesis.deleteThesis).not.toHaveBeenCalled();
    expect(container).toBeTruthy();
  });

  it("deletes only after confirmation and closes the record", async () => {
    const onDeleted = vi.fn();
    vi.mocked(desktopApi.thesis.deleteThesis).mockResolvedValue(undefined as any);

    renderWithClient(<ThesisDetail thesis={mockThesis} onDeleted={onDeleted} />);

    fireEvent.click(screen.getByLabelText("删除论点"));
    fireEvent.click(await screen.findByRole("button", { name: "删除" }));

    await waitFor(() => {
      expect(desktopApi.thesis.deleteThesis).toHaveBeenCalledWith("th-1");
      expect(onDeleted).toHaveBeenCalled();
    });
  });

  it("keeps the thesis selected when deletion fails", async () => {
    const onDeleted = vi.fn();
    vi.mocked(desktopApi.thesis.deleteThesis).mockRejectedValue(new Error("delete failed"));

    renderWithClient(<ThesisDetail thesis={mockThesis} onDeleted={onDeleted} />);

    fireEvent.click(screen.getByLabelText("删除论点"));
    fireEvent.click(await screen.findByRole("button", { name: "删除" }));

    await waitFor(() => {
      expect(screen.getByText("delete failed")).toBeInTheDocument();
    });
    expect(onDeleted).not.toHaveBeenCalled();
  });
});
