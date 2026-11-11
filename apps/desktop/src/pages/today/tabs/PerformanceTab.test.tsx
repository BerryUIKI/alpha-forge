/**
 * Tests for the PerformanceTab component.
 * Covers: loading, empty, error, consolidated data, and percentage units.
 */

import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { MemoryRouter } from "react-router-dom";
import { LocaleContext } from "@/lib/i18n/locale-context";
import { translate, type MessageKey } from "@/lib/i18n/locale";
import { PerformanceTab } from "./PerformanceTab";
import { desktopApi } from "@/lib/desktop-api";

vi.mock("@/pages/today/hooks/useDashboardData", () => ({
  useActiveWorkspaceId: () => "ws-1",
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    financial: {
      listAllFinancialAccounts: vi.fn(),
      computeConsolidatedPerformance: vi.fn(),
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
        <LocaleContext.Provider
          value={{
            locale: "en",
            setLocale: async () => undefined,
            t: (key: MessageKey) => translate("en", key),
          }}
        >
          <PerformanceTab />
        </LocaleContext.Provider>
      </MemoryRouter>
    </QueryClientProvider>,
  );
}

beforeEach(() => {
  vi.clearAllMocks();
});

function dateDaysAgo(days: number): string {
  const date = new Date();
  date.setDate(date.getDate() - days);
  return date.toISOString().slice(0, 10);
}

const account = {
  id: "a1",
  name: "Brokerage",
  account_type: "securities",
  currency: "USD",
  workspace_id: "ws-1",
  created_at: "2026-01-01T00:00:00Z",
  updated_at: "2026-01-01T00:00:00Z",
};

function consolidated(overrides: Record<string, unknown> = {}) {
  return {
    base_currency: "USD",
    points: [
      {
        date: dateDaysAgo(20),
        total_value_base: "10000",
        net_contribution_base: "0",
        covered_accounts: 1,
        total_accounts: 1,
      },
      {
        date: dateDaysAgo(5),
        total_value_base: "10500",
        net_contribution_base: "0",
        covered_accounts: 1,
        total_accounts: 1,
      },
    ],
    total_return_pct: "5",
    xirr_pct: "12",
    twr_pct: "5.5",
    start_value: "10000",
    end_value: "10500",
    net_contribution: "0",
    total_gain: "500",
    account_count: 1,
    covered_account_count: 1,
    skipped_accounts: 0,
    data_quality: "complete",
    ...overrides,
  };
}

describe("PerformanceTab", () => {
  it("shows loading state initially", () => {
    vi.mocked(desktopApi.financial.listAllFinancialAccounts).mockReturnValue(new Promise(() => {}));
    renderTab();
    expect(document.querySelector(".animate-spin")).toBeTruthy();
  });

  it("shows empty state when no accounts exist", async () => {
    vi.mocked(desktopApi.financial.listAllFinancialAccounts).mockResolvedValue([]);
    renderTab();
    const empty = await screen.findByText("No valuation data for this period");
    expect(empty).toBeTruthy();
    expect(desktopApi.financial.computeConsolidatedPerformance).not.toHaveBeenCalled();
  });

  it("shows empty state when no valuations in period", async () => {
    vi.mocked(desktopApi.financial.listAllFinancialAccounts).mockResolvedValue([account] as any);
    vi.mocked(desktopApi.financial.computeConsolidatedPerformance).mockResolvedValue(
      consolidated({
        points: [],
        total_return_pct: null,
        xirr_pct: null,
        twr_pct: null,
        data_quality: "insufficient_data",
      }) as any,
    );

    renderTab();
    const empty = await screen.findByText("No valuation data for this period");
    expect(empty).toBeTruthy();
  });

  it("shows error state with retry when API fails", async () => {
    vi.mocked(desktopApi.financial.listAllFinancialAccounts).mockRejectedValue(
      new Error("API error"),
    );
    renderTab();
    const errorMsg = await screen.findByText("Failed to load portfolio performance data");
    expect(errorMsg).toBeTruthy();
  });

  it("renders consolidated performance chips using percentage points once", async () => {
    vi.mocked(desktopApi.financial.listAllFinancialAccounts).mockResolvedValue([account] as any);
    vi.mocked(desktopApi.financial.computeConsolidatedPerformance).mockResolvedValue(
      consolidated() as any,
    );

    renderTab();

    await screen.findByText("Accounts");
    await screen.findByText("Total Return");
    await screen.findByText("XIRR");
    await screen.findByText("TWR");

    // The backend reports percentage points (5 = 5%), so the UI must not
    // multiply by 100 again.
    expect(screen.getByText("5.00%")).toBeInTheDocument();
    expect(screen.getByText("12.00%")).toBeInTheDocument();
    expect(screen.getByText("5.50%")).toBeInTheDocument();
  });

  it("surfaces partial coverage instead of treating missing accounts as zero", async () => {
    vi.mocked(desktopApi.financial.listAllFinancialAccounts).mockResolvedValue([account] as any);
    vi.mocked(desktopApi.financial.computeConsolidatedPerformance).mockResolvedValue(
      consolidated({
        total_return_pct: null,
        account_count: 2,
        covered_account_count: 1,
        skipped_accounts: 1,
        data_quality: "partial",
      }) as any,
    );

    renderTab();

    expect(
      await screen.findByText(/Partial coverage: 1 of 2 accounts have valuation data/),
    ).toBeInTheDocument();
    expect(
      screen.getByText(/1 account\(s\) excluded due to a different base currency\./),
    ).toBeInTheDocument();
  });
});
