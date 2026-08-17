/**
 * Tests for the PerformanceTab component.
 * Covers: loading, empty, error, and data-renders states.
 */

import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { MemoryRouter } from "react-router-dom";
import { PerformanceTab } from "./PerformanceTab";
import { desktopApi } from "@/lib/desktop-api";

// Mock the workspaces hook to return a workspace
vi.mock("@/pages/today/hooks/useDashboardData", () => ({
  useActiveWorkspaceId: () => "ws-1",
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    portfolio: {
      listPortfolioAccounts: vi.fn(),
    },
    financial: {
      getPerformanceTimeSeries: vi.fn(),
      computePerformanceSummary: vi.fn(),
    },
  },
}));

function renderTab() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return render(
    <QueryClientProvider client={queryClient}>
      <MemoryRouter>
        <PerformanceTab />
      </MemoryRouter>
    </QueryClientProvider>,
  );
}

beforeEach(() => {
  vi.clearAllMocks();
});

describe("PerformanceTab", () => {
  it("shows loading state initially", () => {
    vi.mocked(desktopApi.portfolio.listPortfolioAccounts).mockReturnValue(
      new Promise(() => {}),
    );
    renderTab();
    expect(document.querySelector(".animate-spin")).toBeTruthy();
  });

  it("shows empty state when no accounts exist", async () => {
    vi.mocked(desktopApi.portfolio.listPortfolioAccounts).mockResolvedValue([]);
    renderTab();
    const empty = await screen.findByText("No valuation data for this period");
    expect(empty).toBeTruthy();
  });

  it("shows empty state when no valuations in period", async () => {
    vi.mocked(desktopApi.portfolio.listPortfolioAccounts).mockResolvedValue([
      {
        id: "a1",
        name: "Brokerage",
        account_type: "securities",
        currency: "USD",
        workspace_id: "ws-1",
        created_at: "2026-01-01T00:00:00Z",
        updated_at: "2026-01-01T00:00:00Z",
      },
    ]);
    vi.mocked(desktopApi.financial.getPerformanceTimeSeries).mockResolvedValue(
      [],
    );
    vi.mocked(
      desktopApi.financial.computePerformanceSummary,
    ).mockResolvedValue({
      account_id: "a1",
      start_date: "2026-07-17",
      end_date: "2026-08-17",
      total_return_pct: null,
      xirr_pct: null,
      twr_pct: null,
      start_value: "0",
      end_value: "0",
      net_contribution: "0",
      total_gain: "0",
      total_gain_base: "0",
      data_quality: "insufficient",
    });

    renderTab();
    const empty = await screen.findByText("No valuation data for this period");
    expect(empty).toBeTruthy();
  });

  it("shows error state with retry when API fails", async () => {
    vi.mocked(desktopApi.portfolio.listPortfolioAccounts).mockRejectedValue(
      new Error("API error"),
    );
    renderTab();
    const errorMsg = await screen.findByText(
      "Failed to load portfolio performance data",
    );
    expect(errorMsg).toBeTruthy();
  });

  it("renders chart and performance chips when data exists", async () => {
    vi.mocked(desktopApi.portfolio.listPortfolioAccounts).mockResolvedValue([
      {
        id: "a1",
        name: "Brokerage",
        account_type: "securities",
        currency: "USD",
        workspace_id: "ws-1",
        created_at: "2026-01-01T00:00:00Z",
        updated_at: "2026-01-01T00:00:00Z",
      },
    ]);
    vi.mocked(desktopApi.financial.getPerformanceTimeSeries).mockResolvedValue(
      [
        {
          date: "2026-08-01",
          total_value: "10000",
          total_value_base: "10000",
          net_contribution: "0",
          net_contribution_base: "0",
          cumulative_return_pct: null,
          daily_return_pct: null,
        },
        {
          date: "2026-08-15",
          total_value: "10500",
          total_value_base: "10500",
          net_contribution: "0",
          net_contribution_base: "0",
          cumulative_return_pct: null,
          daily_return_pct: null,
        },
      ],
    );
    vi.mocked(
      desktopApi.financial.computePerformanceSummary,
    ).mockResolvedValue({
      account_id: "a1",
      start_date: "2026-07-17",
      end_date: "2026-08-17",
      total_return_pct: "0.05",
      xirr_pct: "0.12",
      twr_pct: "0.05",
      start_value: "10000",
      end_value: "10500",
      net_contribution: "0",
      total_gain: "500",
      total_gain_base: "500",
      data_quality: "good",
    });

    renderTab();

    // Wait for performance chips to appear — check for Account label
    const accountsChip = await screen.findByText("Accounts");
    expect(accountsChip).toBeTruthy();

    // Verify the chart container rendered
    await screen.findByText("Total Return");
    await screen.findByText("XIRR");
    await screen.findByText("TWR");
  });
});