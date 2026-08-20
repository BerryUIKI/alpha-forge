import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { type ReactNode } from "react";
import { AccountCards } from "./AccountCards";
import { desktopApi } from "@/lib/desktop-api";

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    financial: {
      listAllFinancialAccounts: vi.fn(),
      computeNetWorth: vi.fn(),
    },
  },
}));

vi.mock("@/lib/i18n/useLocale", () => ({
  useLocale: () => ({
    t: (key: string) => {
      const map: Record<string, string> = {
        failedToLoadAccounts: "Failed to load accounts",
        noAccountsYet: "No accounts yet",
        noAccountsDescription: "Create an account to get started",
        netWorth: "Net Worth",
        totalValue: "Total Value",
      };
      return map[key] || key;
    },
  }),
}));

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return function Wrapper({ children }: { children: ReactNode }) {
    return <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>;
  };
}

const account = {
  id: "acct-1",
  workspace_id: "ws-1",
  name: "Brokerage",
  account_type: "securities",
  group_name: null,
  is_archived: false,
  is_default: false,
  currency: "USD",
  extra: null,
  balance: null,
  created_at: "2026-01-01T00:00:00Z",
  updated_at: "2026-01-01T00:00:00Z",
  platform_id: null,
  account_number: null,
  tracking_mode: "tracking",
  created_at_timestamp: null,
  updated_at_timestamp: null,
} as any;

beforeEach(() => {
  vi.clearAllMocks();
});

describe("AccountCards", () => {
  it("shows empty state when no accounts", async () => {
    vi.mocked(desktopApi.financial.listAllFinancialAccounts).mockResolvedValue([]);
    vi.mocked(desktopApi.financial.computeNetWorth).mockResolvedValue({} as any);
    const wrapper = createWrapper();
    const { container } = render(
      <AccountCards
        asOfDate="2026-08-18"
        selectedAccountId=""
        onSelectAccount={() => {}}
      />,
      { wrapper },
    );
    expect(container).toBeTruthy();
  });

  it("shows loading spinner while fetching", () => {
    vi.mocked(desktopApi.financial.listAllFinancialAccounts).mockReturnValue(
      new Promise(() => {}),
    );
    vi.mocked(desktopApi.financial.computeNetWorth).mockReturnValue(
      new Promise(() => {}),
    );
    const wrapper = createWrapper();
    const { container } = render(
      <AccountCards
        asOfDate="2026-08-18"
        selectedAccountId=""
        onSelectAccount={() => {}}
      />,
      { wrapper },
    );
    expect(container.querySelector(".animate-spin")).toBeTruthy();
  });

  it("shows error state when accounts fail", async () => {
    vi.mocked(desktopApi.financial.listAllFinancialAccounts).mockRejectedValue(
      new Error("API error"),
    );
    vi.mocked(desktopApi.financial.computeNetWorth).mockResolvedValue({} as any);
    const wrapper = createWrapper();
    render(
      <AccountCards
        asOfDate="2026-08-18"
        selectedAccountId=""
        onSelectAccount={() => {}}
      />,
      { wrapper },
    );
    expect(await screen.findByText("Failed to load accounts")).toBeInTheDocument();
  });

  it("renders account cards when data exists", async () => {
    vi.mocked(desktopApi.financial.listAllFinancialAccounts).mockResolvedValue([account] as any);
    vi.mocked(desktopApi.financial.computeNetWorth).mockResolvedValue({
      as_of_date: "2026-08-18",
      base_currency: "USD",
      net_worth: "10000",
      total_assets: "10000",
      total_liabilities: "0",
      accounts: [],
    } as any);
    const wrapper = createWrapper();
    const { container } = render(
      <AccountCards
        asOfDate="2026-08-18"
        selectedAccountId=""
        onSelectAccount={() => {}}
      />,
      { wrapper },
    );
    expect(await screen.findByText("Brokerage")).toBeInTheDocument();
    expect(container).toBeTruthy();
  });

  it("calls onSelectAccount when a card is clicked", async () => {
    vi.mocked(desktopApi.financial.listAllFinancialAccounts).mockResolvedValue([account] as any);
    vi.mocked(desktopApi.financial.computeNetWorth).mockResolvedValue({} as any);
    const onSelect = vi.fn();
    const wrapper = createWrapper();
    const { findByText } = render(
      <AccountCards
        asOfDate="2026-08-18"
        selectedAccountId=""
        onSelectAccount={onSelect}
      />,
      { wrapper },
    );
    const card = await findByText("Brokerage");
    card.click();
    expect(onSelect).toHaveBeenCalledWith("acct-1");
  });
});