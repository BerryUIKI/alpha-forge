import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { BrowserRouter } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { QuotesPage } from "./QuotesPage";
import { desktopApi } from "@/lib/desktop-api";

const financialMock = vi.hoisted(() => ({
  listActiveAssets: vi.fn(),
  listQuotesForAsset: vi.fn(),
  refreshAllActiveQuotes: vi.fn(),
  refreshAssetQuote: vi.fn(),
}));

const thesisMock = vi.hoisted(() => ({
  listTheses: vi.fn(),
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    financial: financialMock,
    thesis: thesisMock,
  },
}));

vi.mock("@/features/workspace/hooks/useActiveWorkspace.context", () => ({
  useActiveWorkspaceId: () => "ws-test-1",
}));

describe("QuotesPage", () => {
  let queryClient: QueryClient;

  beforeEach(() => {
    vi.clearAllMocks();
    queryClient = new QueryClient({
      defaultOptions: {
        queries: { retry: false },
      },
    });

    vi.mocked(desktopApi.thesis.listTheses).mockResolvedValue([]);
  });

  it("renders empty state when no active assets exist", async () => {
    vi.mocked(desktopApi.financial.listActiveAssets).mockResolvedValue([]);

    render(
      <QueryClientProvider client={queryClient}>
        <BrowserRouter>
          <QuotesPage />
        </BrowserRouter>
      </QueryClientProvider>,
    );

    expect(await screen.findByText(/Market Quotes|行情看板/i)).toBeInTheDocument();
    expect(
      await screen.findByText(/No Active Assets|暂无活跃资产标的/i),
    ).toBeInTheDocument();
  });

  it("renders active asset cards and quote details", async () => {
    const mockAssets = [
      {
        id: "asset-1",
        kind: "investment" as const,
        name: "Apple Inc.",
        display_code: "AAPL",
        notes: null,
        metadata: null,
        is_active: true,
        quote_mode: "manual" as const,
        quote_ccy: "USD",
        instrument_type: "equity" as const,
        instrument_symbol: "AAPL",
        instrument_exchange_mic: "XNAS",
        instrument_key: "AAPL.XNAS",
        provider_config: null,
        created_at: "2026-01-01T00:00:00Z",
        updated_at: "2026-01-01T00:00:00Z",
      },
    ];

    const mockQuotes = [
      {
        id: "quote-1",
        asset_id: "asset-1",
        day: "2026-10-03",
        source: "mock",
        open: "220.00",
        high: "225.00",
        low: "219.50",
        close: "224.50",
        adjclose: "224.50",
        volume: "50000000",
        currency: "USD",
        notes: null,
        created_at: "2026-10-03T20:00:00Z",
        timestamp: "2026-10-03T20:00:00Z",
      },
    ];

    financialMock.listActiveAssets.mockResolvedValue(mockAssets);
    financialMock.listQuotesForAsset.mockResolvedValue(mockQuotes);

    render(
      <QueryClientProvider client={queryClient}>
        <BrowserRouter>
          <QuotesPage />
        </BrowserRouter>
      </QueryClientProvider>,
    );

    expect(await screen.findByText("AAPL")).toBeInTheDocument();
    expect(await screen.findByText("Apple Inc.")).toBeInTheDocument();
    expect(await screen.findByText("224.50")).toBeInTheDocument();
  });

  it("triggers refreshAllActiveQuotes when clicking refresh all button", async () => {
    const mockAssets = [
      {
        id: "asset-1",
        kind: "investment" as const,
        name: "Apple Inc.",
        display_code: "AAPL",
        notes: null,
        metadata: null,
        is_active: true,
        quote_mode: "manual" as const,
        quote_ccy: "USD",
        instrument_type: "equity" as const,
        instrument_symbol: "AAPL",
        instrument_exchange_mic: "XNAS",
        instrument_key: "AAPL.XNAS",
        provider_config: null,
        created_at: "2026-01-01T00:00:00Z",
        updated_at: "2026-01-01T00:00:00Z",
      },
    ];

    financialMock.listActiveAssets.mockResolvedValue(mockAssets);
    financialMock.listQuotesForAsset.mockResolvedValue([]);
    financialMock.refreshAllActiveQuotes.mockResolvedValue([]);

    render(
      <QueryClientProvider client={queryClient}>
        <BrowserRouter>
          <QuotesPage />
        </BrowserRouter>
      </QueryClientProvider>,
    );

    // Wait for asset query to settle so button is enabled
    expect(await screen.findByText("AAPL")).toBeInTheDocument();

    const refreshBtn = screen.getByRole("button", {
      name: /Refresh All Quotes|刷新全量行情/i,
    });
    expect(refreshBtn).not.toBeDisabled();
    fireEvent.click(refreshBtn);

    await waitFor(() => {
      expect(financialMock.refreshAllActiveQuotes).toHaveBeenCalledTimes(1);
    });
  });
});
