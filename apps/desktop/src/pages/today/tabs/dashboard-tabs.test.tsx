import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import { type ReactElement } from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { MemoryRouter } from "react-router-dom";
import { OverviewTab } from "./OverviewTab";
import { ActivityTab } from "./ActivityTab";

vi.mock("@/pages/today/hooks/useDashboardData", () => ({
  useActiveWorkspaceId: () => "ws-1",
  useDashboardSummary: vi.fn(),
  useDashboardActivity: vi.fn(),
}));

import { useDashboardSummary, useDashboardActivity } from "@/pages/today/hooks/useDashboardData";

function renderTab(Component: () => ReactElement) {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return render(
    <QueryClientProvider client={queryClient}>
      <MemoryRouter>
        <Component />
      </MemoryRouter>
    </QueryClientProvider>,
  );
}

beforeEach(() => {
  vi.clearAllMocks();
});

describe("OverviewTab", () => {
  it("shows loading state", () => {
    vi.mocked(useDashboardSummary).mockReturnValue({
      data: undefined,
      isLoading: true,
      error: null,
    } as unknown as ReturnType<typeof useDashboardSummary>);
    vi.mocked(useDashboardActivity).mockReturnValue({
      data: undefined,
      isLoading: true,
      error: null,
    } as unknown as ReturnType<typeof useDashboardActivity>);

    renderTab(OverviewTab);
    const skeletons = document.querySelectorAll(".animate-pulse");
    expect(skeletons.length).toBeGreaterThanOrEqual(3);
  });

  it("shows error state", () => {
    vi.mocked(useDashboardSummary).mockReturnValue({
      data: undefined,
      isLoading: false,
      error: new Error("API error"),
    } as unknown as ReturnType<typeof useDashboardSummary>);
    vi.mocked(useDashboardActivity).mockReturnValue({
      data: undefined,
      isLoading: false,
      error: null,
    } as unknown as ReturnType<typeof useDashboardActivity>);

    renderTab(OverviewTab);
    expect(screen.getByText("Failed to load dashboard data")).toBeInTheDocument();
  });

  it("shows empty state when no accounts and no theses", () => {
    vi.mocked(useDashboardSummary).mockReturnValue({
      data: { portfolioValue: 0, activeTheses: 0, holdings: [] },
      isLoading: false,
      error: null,
    } as unknown as ReturnType<typeof useDashboardSummary>);
    vi.mocked(useDashboardActivity).mockReturnValue({
      data: [],
      isLoading: false,
      error: null,
    } as unknown as ReturnType<typeof useDashboardActivity>);

    renderTab(OverviewTab);
    expect(screen.getByText("No data yet")).toBeInTheDocument();
  });

  it("renders stat cards with data", () => {
    vi.mocked(useDashboardSummary).mockReturnValue({
      data: {
        portfolioValue: 50000,
        activeTheses: 3,
        holdings: [
          {
            id: "h1",
            ticker: "AAPL",
            name: "Apple Inc.",
            sector: "Technology",
            allocation: "25.0%",
            value: "$12,500",
            change: "+2.1%",
            changePositive: true,
          },
        ],
      },
      isLoading: false,
      error: null,
    } as unknown as ReturnType<typeof useDashboardSummary>);
    vi.mocked(useDashboardActivity).mockReturnValue({
      data: [
        {
          id: "a1",
          type: "research",
          title: "Research",
          description: "New analysis",
          timestamp: "12m ago",
        },
      ],
      isLoading: false,
      error: null,
    } as unknown as ReturnType<typeof useDashboardActivity>);

    renderTab(OverviewTab);
    expect(screen.getByText("$50,000")).toBeInTheDocument();
    expect(screen.getByText("3")).toBeInTheDocument();
    expect(screen.getByText("Apple Inc.")).toBeInTheDocument();
    expect(screen.getByText("New analysis")).toBeInTheDocument();
  });
});

describe("ActivityTab", () => {
  it("shows loading state", () => {
    vi.mocked(useDashboardActivity).mockReturnValue({
      data: undefined,
      isLoading: true,
      error: null,
    } as unknown as ReturnType<typeof useDashboardActivity>);

    renderTab(ActivityTab);
    const skeletons = document.querySelectorAll(".animate-pulse");
    expect(skeletons.length).toBeGreaterThanOrEqual(3);
  });

  it("shows empty state when no activity", () => {
    vi.mocked(useDashboardActivity).mockReturnValue({
      data: [],
      isLoading: false,
      error: null,
    } as unknown as ReturnType<typeof useDashboardActivity>);

    renderTab(ActivityTab);
    expect(screen.getByText("No recent activity")).toBeInTheDocument();
  });

  it("renders activity items", () => {
    vi.mocked(useDashboardActivity).mockReturnValue({
      data: [
        {
          id: "a1",
          type: "research",
          title: "Research",
          description: "NVDA analysis",
          timestamp: "12m ago",
        },
        {
          id: "a2",
          type: "thesis",
          title: "Thesis",
          description: "Energy thesis",
          timestamp: "2h ago",
        },
      ],
      isLoading: false,
      error: null,
    } as unknown as ReturnType<typeof useDashboardActivity>);

    renderTab(ActivityTab);
    expect(screen.getByText("NVDA analysis")).toBeInTheDocument();
    expect(screen.getByText("Energy thesis")).toBeInTheDocument();
  });
});