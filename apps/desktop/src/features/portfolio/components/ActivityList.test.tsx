import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { type ReactNode } from "react";
import { ActivityList } from "./ActivityList";
import { desktopApi } from "@/lib/desktop-api";

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    financial: {
      listActivitiesByAccount: vi.fn(),
    },
  },
}));

vi.mock("@/lib/i18n/useLocale", () => ({
  useLocale: () => ({
    t: (key: string) => {
      const map: Record<string, string> = {
        failedToLoadTransactions: "Failed to load transactions",
        noTransactionsImported: "No transactions yet",
        recentActivity: "Recent Activity",
      };
      return map[key] || key;
    },
  }),
}));

function renderWithQuery(ui: ReactNode) {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return render(
    <QueryClientProvider client={queryClient}>{ui}</QueryClientProvider>,
  );
}

beforeEach(() => {
  vi.clearAllMocks();
});

describe("ActivityList", () => {
  it("shows loading spinner while fetching", () => {
    vi.mocked(desktopApi.financial.listActivitiesByAccount).mockReturnValue(
      new Promise(() => {}),
    );
    const { container } = renderWithQuery(<ActivityList accountId="acct-1" />);
    expect(container.querySelector(".animate-spin")).toBeTruthy();
  });

  it("shows error state when API fails", async () => {
    vi.mocked(desktopApi.financial.listActivitiesByAccount).mockRejectedValue(
      new Error("API error"),
    );
    renderWithQuery(<ActivityList accountId="acct-1" />);
    expect(
      await screen.findByText("Failed to load transactions"),
    ).toBeInTheDocument();
  });

  it("shows empty state when no activities", async () => {
    vi.mocked(desktopApi.financial.listActivitiesByAccount).mockResolvedValue([]);
    renderWithQuery(<ActivityList accountId="acct-1" />);
    expect(await screen.findByText("No transactions yet")).toBeInTheDocument();
  });

  it("renders activity items", async () => {
    vi.mocked(desktopApi.financial.listActivitiesByAccount).mockResolvedValue([
      {
        id: "act-1",
        account_id: "acct-1",
        activity_type: "buy",
        quantity: "10",
        unit_price: "100",
        amount: "1000",
        activity_date: "2026-08-15",
        currency: "USD",
      },
      {
        id: "act-2",
        account_id: "acct-1",
        activity_type: "sell",
        quantity: "5",
        amount: "600",
        activity_date: "2026-08-17",
        currency: "USD",
      },
    ] as any);
    renderWithQuery(<ActivityList accountId="acct-1" />);
    expect(await screen.findByText("buy")).toBeInTheDocument();
    expect(screen.getByText("sell")).toBeInTheDocument();
    expect(screen.getByText("2026-08-15")).toBeInTheDocument();
    expect(screen.getByText("2026-08-17")).toBeInTheDocument();
  });

  it("respects the limit prop", async () => {
    const items = Array.from({ length: 10 }, (_, i) => ({
      id: `act-${i}`,
      account_id: "acct-1",
      activity_type: "buy",
      activity_date: "2026-08-15",
      currency: "USD",
      amount: "100",
    }));
    vi.mocked(desktopApi.financial.listActivitiesByAccount).mockResolvedValue(
      items as any,
    );
    renderWithQuery(<ActivityList accountId="acct-1" limit={3} />);
    // Should only show 3 buy items
    const buyElements = await screen.findAllByText("buy");
    expect(buyElements).toHaveLength(3);
  });
});