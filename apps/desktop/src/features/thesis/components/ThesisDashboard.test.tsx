import "@testing-library/jest-dom";
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { ThesisDashboard } from "./ThesisDashboard";
import type { InvestmentThesis } from "@/lib/desktop-api/thesis";

const theses = vi.hoisted(
  () =>
    [
      {
        id: "th-1",
        workspaceId: "ws-1",
        title: "GPU demand",
        thesis: "Hyperscaler capex grows",
        confidence: 70,
        status: "active",
        validationDate: null,
        outcome: null,
        portfolioAssetId: null,
        createdAt: "2026-08-01T00:00:00Z",
        updatedAt: "2026-08-01T00:00:00Z",
      },
      {
        id: "th-2",
        workspaceId: "ws-1",
        title: "Memory cycle",
        thesis: "HBM supply tightens",
        confidence: 55,
        status: "draft",
        validationDate: null,
        outcome: null,
        portfolioAssetId: null,
        createdAt: "2026-08-02T00:00:00Z",
        updatedAt: "2026-08-02T00:00:00Z",
      },
    ] as InvestmentThesis[],
);

vi.mock("@/features/workspace/hooks/useWorkspaces", () => ({
  useWorkspaces: () => ({
    data: [{ id: "ws-1", name: "Workspace" }],
    isLoading: false,
    error: null,
    refetch: vi.fn(),
  }),
}));

vi.mock("@/features/workspace/hooks/useActiveWorkspace.context", () => ({
  useActiveWorkspaceId: () => "ws-1",
}));

vi.mock("../hooks/useTheses", () => ({
  useTheses: () => ({
    data: theses,
    isSuccess: true,
    isLoading: false,
    error: null,
    refetch: vi.fn(),
  }),
}));

vi.mock("./ThesisPipelineBoard", () => ({
  ThesisPipelineBoard: ({ onSelect }: { onSelect: (thesis: InvestmentThesis) => void }) => (
    <button type="button" onClick={() => onSelect(theses[1]!)}>
      Select memory thesis
    </button>
  ),
}));

vi.mock("./ThesisList", () => ({
  ThesisList: () => <div>List view</div>,
}));

vi.mock("./ThesisDetail", () => ({
  ThesisDetail: ({ thesis }: { thesis: InvestmentThesis }) => <div>detail:{thesis.id}</div>,
}));

vi.mock("./CreateThesisForm", () => ({
  CreateThesisForm: () => <div>Create form</div>,
}));

vi.mock("./KnowledgeGraphPanel", () => ({
  KnowledgeGraphPanel: () => null,
}));

vi.mock("@/lib/i18n/useLocale", () => ({
  useLocale: () => ({ t: (key: string) => key }),
}));

function renderDashboard(entry: string) {
  return render(
    <MemoryRouter initialEntries={[entry]}>
      <ThesisDashboard />
    </MemoryRouter>,
  );
}

describe("ThesisDashboard deep links", () => {
  it("selects the thesis named by the URL and opens its detail", () => {
    renderDashboard("/theses?thesis=th-2");

    expect(screen.getByText("detail:th-2")).toBeInTheDocument();
  });

  it("writes the selected thesis into the URL", () => {
    renderDashboard("/theses");

    fireEvent.click(screen.getByRole("button", { name: "Select memory thesis" }));

    expect(screen.getByText("detail:th-2")).toBeInTheDocument();
  });

  it("ignores a stale thesis id instead of breaking the page", () => {
    renderDashboard("/theses?thesis=missing");

    expect(screen.queryByText(/detail:/)).not.toBeInTheDocument();
  });
});
