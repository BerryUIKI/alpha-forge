import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { type ReactNode } from "react";
import { desktopApi } from "@/lib/desktop-api";
import { TaxonomyAllocationPanel } from "./TaxonomyAllocationPanel";

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    financial: {
      listTaxonomies: vi.fn(),
      createTaxonomy: vi.fn(),
      listTaxonomyCategories: vi.fn(),
      createTaxonomyCategory: vi.fn(),
      listAllocationTargets: vi.fn(),
      createAllocationTarget: vi.fn(),
      listAllocationWeights: vi.fn(),
      addAllocationWeight: vi.fn(),
      getAllocation: vi.fn(),
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

describe("TaxonomyAllocationPanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders empty states when no taxonomies or targets exist", async () => {
    vi.mocked(desktopApi.financial.listTaxonomies).mockResolvedValue([]);
    vi.mocked(desktopApi.financial.listAllocationTargets).mockResolvedValue([]);
    vi.mocked(desktopApi.financial.getAllocation).mockResolvedValue({
      scope_type: "account",
      scope_id: null,
      total_market_value: "0.0",
      total_market_value_base: "0.0",
      categories: [],
      unassigned_market_value: "0.0",
      unassigned_market_value_base: "0.0",
    } as any);

    renderWithClient(<TaxonomyAllocationPanel asOfDate="2026-10-05" />);

    expect(
      await screen.findByTestId("taxonomy-allocation-panel"),
    ).toBeInTheDocument();
    expect(screen.getByText("新建分类体系")).toBeInTheDocument();
    expect(screen.getByText("新建配置目标")).toBeInTheDocument();
  });

  it("renders existing taxonomies, categories, targets and weights", async () => {
    vi.mocked(desktopApi.financial.listTaxonomies).mockResolvedValue([
      {
        id: "tx-1",
        name: "Asset Class",
        color: "#2563eb",
        description: null,
        is_system: true,
        is_single_select: true,
        sort_order: 10,
        created_at: "2026-10-01",
        updated_at: "2026-10-01",
      },
    ]);
    vi.mocked(desktopApi.financial.listTaxonomyCategories).mockResolvedValue([
      {
        id: "cat-1",
        taxonomy_id: "tx-1",
        parent_id: null,
        name: "Equities",
        key: "equities",
        color: "#3b82f6",
        description: null,
        sort_order: 10,
        created_at: "2026-10-01",
        updated_at: "2026-10-01",
      },
    ]);
    vi.mocked(desktopApi.financial.listAllocationTargets).mockResolvedValue([
      {
        id: "tg-1",
        name: "60/40 Balanced",
        scope_type: "account",
        scope_id: null,
        taxonomy_id: "tx-1",
        trigger_type: "drift_band",
        drift_band_bps: 500,
        rebalance_goal: "full",
        min_trade_amount: "100.00",
        whole_shares_only: false,
        allow_sells: true,
        max_turnover_bps: null,
        created_at: "2026-10-01",
        updated_at: "2026-10-01",
        archived_at: null,
      },
    ]);
    vi.mocked(desktopApi.financial.listAllocationWeights).mockResolvedValue([
      {
        id: "w-1",
        target_id: "tg-1",
        taxonomy_id: "tx-1",
        category_id: "cat-1",
        target_bps: 6000,
        is_locked: false,
        is_required: true,
        created_at: "2026-10-01",
        updated_at: "2026-10-01",
      },
    ]);
    vi.mocked(desktopApi.financial.getAllocation).mockResolvedValue({
      scope_type: "account",
      scope_id: null,
      total_market_value: "100000.0",
      total_market_value_base: "100000.0",
      categories: [
        {
          category_id: "cat-1",
          category_name: "Equities",
          taxonomy_id: "tx-1",
          taxonomy_name: "Asset Class",
          actual_bps: 6200,
          target_bps: 6000,
          difference_bps: 200,
          market_value: "62000.0",
          market_value_base: "62000.0",
          within_drift: true,
        },
      ],
      unassigned_market_value: "0.0",
      unassigned_market_value_base: "0.0",
    } as any);

    renderWithClient(<TaxonomyAllocationPanel asOfDate="2026-10-05" />);

    expect(await screen.findByText("Asset Class")).toBeInTheDocument();
    const equitiesElements = await screen.findAllByText("Equities");
    expect(equitiesElements.length).toBeGreaterThan(0);
    expect(await screen.findByText("60.0%")).toBeInTheDocument();
    expect(await screen.findByText("62.0%")).toBeInTheDocument();
    expect(await screen.findByText("偏离可控")).toBeInTheDocument();
  });
});
