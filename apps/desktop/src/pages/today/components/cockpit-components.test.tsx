import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { MarketPulseBar } from "./MarketPulseBar";
import { SecFilingFeed } from "./SecFilingFeed";
import { desktopApi } from "@/lib/desktop-api";

vi.mock("@/lib/i18n/useLocale", () => ({
  useLocale: () => ({
    t: (key: string) => {
      const map: Record<string, string> = {
        marketPulse: "Market Pulse",
        dataSynced: "Synced: {time}",
        marketDemoBadge: "Demo data",
        marketDemoNotice: "Demo data · live market feed unavailable",
        marketRefreshUnavailable: "Live market data is not available yet",
        refresh: "Refresh",
        retry: "Retry",
        secFilingFeed: "SEC Filing & Research Stream",
        secAutomatedParsing: "Automated SEC Parsing",
        secLiveEdgar: "Live SEC EDGAR",
        secUnavailable: "SEC EDGAR unavailable",
        secLoading: "Ingesting live SEC filings...",
        secViewDocument: "Open SEC Filing",
        linkedThesis: "Linked Thesis: {title}",
        secNoAssets: "Add an asset with a ticker to follow its SEC filings.",
        secErrorTitle: "SEC EDGAR is unavailable",
        secErrorDescription: "Filings could not be retrieved. Try again.",
        secNoFilings: "No recent filings found for {ticker}.",
        secAnalysisUnavailable: "No linked thesis analysis",
        secSource: "Source: SEC EDGAR",
        evidencePositive: "Positive Confirmation (+{delta}% Conviction)",
        evidenceContra: "Contra-Evidence Alert (-{delta}% Conviction)",
        hoursAgo: "{hours} ago",
      };
      return map[key] ?? key;
    },
  }),
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    research: {
      fetchSecCompanyFilings: vi.fn().mockResolvedValue([
        {
          id: "0001045810-24-000123",
          accessionNumber: "0001045810-24-000123",
          cik: "0001045810",
          ticker: "NVDA",
          companyName: "NVIDIA CORP",
          formType: "10-Q",
          filingDate: "2024-08-28",
          reportDate: "2024-07-28",
          primaryDocument: "nvda-10q.htm",
          primaryDocDescription: "Form 10-Q Quarterly Report",
          filingUrl:
            "https://www.sec.gov/Archives/edgar/data/1045810/000104581024000123/nvda-10q.htm",
          summary: "Revenue up 154% YoY",
        },
      ]),
    },
    financial: {
      listActiveAssets: vi.fn().mockResolvedValue([
        {
          id: "asset-nvda",
          display_code: "NVDA",
          name: "NVIDIA Corp",
          kind: "investment",
        },
        {
          id: "asset-aapl",
          display_code: "AAPL",
          name: "Apple Inc",
          kind: "investment",
        },
      ]),
    },
    thesis: {
      listTheses: vi.fn().mockResolvedValue([
        {
          id: "thesis-nvda",
          title: "Blackwell Rack Ramp Ahead of Consensus",
          portfolioAssetId: "asset-nvda",
        },
      ]),
    },
  },
}));

vi.mock("@/features/workspace/hooks/useActiveWorkspace.context", () => ({
  useActiveWorkspaceId: () => "ws-test-1",
}));

function renderWithQueryClient(ui: React.ReactElement) {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
      },
    },
  });
  return render(<QueryClientProvider client={queryClient}>{ui}</QueryClientProvider>);
}

beforeEach(() => {
  vi.clearAllMocks();
  vi.mocked(desktopApi.research.fetchSecCompanyFilings).mockResolvedValue([
    {
      id: "0001045810-24-000123",
      accessionNumber: "0001045810-24-000123",
      cik: "0001045810",
      ticker: "NVDA",
      companyName: "NVIDIA CORP",
      formType: "10-Q",
      filingDate: "2024-08-28",
      reportDate: "2024-07-28",
      primaryDocument: "nvda-10q.htm",
      primaryDocDescription: "Form 10-Q Quarterly Report",
      filingUrl: "https://www.sec.gov/Archives/edgar/data/1045810/000104581024000123/nvda-10q.htm",
      summary: "Revenue up 154% YoY",
    } as any,
  ]);
});

describe("MarketPulseBar", () => {
  it("labels fixed benchmark values as demo data", () => {
    renderWithQueryClient(<MarketPulseBar />);
    expect(screen.getByText("Market Pulse")).toBeInTheDocument();
    expect(screen.getByText("Demo data")).toBeInTheDocument();
    expect(screen.getByText("S&P 500")).toBeInTheDocument();
    expect(screen.getByText("NASDAQ 100")).toBeInTheDocument();
    expect(screen.getByText("US 10Y")).toBeInTheDocument();
    expect(screen.getByText("VIX")).toBeInTheDocument();
  });

  it("does not claim a freshness timestamp or a working refresh", () => {
    renderWithQueryClient(<MarketPulseBar />);
    expect(screen.queryByText(/Synced:/)).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Refresh" })).toBeDisabled();
    expect(screen.getByText("Demo data · live market feed unavailable")).toBeInTheDocument();
  });
});

describe("SecFilingFeed", () => {
  it("renders retrieved filings with provenance and a real thesis link", async () => {
    renderWithQueryClient(<SecFilingFeed />);

    expect(screen.getByText("SEC Filing & Research Stream")).toBeInTheDocument();
    expect(await screen.findByText("Live SEC EDGAR")).toBeInTheDocument();
    expect((await screen.findAllByText("NVDA")).length).toBeGreaterThan(0);
    expect(await screen.findByText(/Blackwell Rack Ramp Ahead of Consensus/)).toBeInTheDocument();
    expect(screen.getByText("Source: SEC EDGAR")).toBeInTheDocument();
    expect(screen.getByText("Open SEC Filing")).toBeInTheDocument();
    // No synthetic impact badges.
    expect(screen.queryByText(/Positive Confirmation/)).not.toBeInTheDocument();
  });

  it("shows an empty state instead of demo filings", async () => {
    vi.mocked(desktopApi.research.fetchSecCompanyFilings).mockResolvedValue([]);
    renderWithQueryClient(<SecFilingFeed />);
    expect(await screen.findByText("No recent filings found for NVDA.")).toBeInTheDocument();
  });

  it("shows an error state and never falls back to demo filings", async () => {
    vi.mocked(desktopApi.research.fetchSecCompanyFilings).mockRejectedValue(
      new Error("EDGAR unavailable"),
    );
    renderWithQueryClient(<SecFilingFeed />);
    expect(await screen.findByText("SEC EDGAR is unavailable")).toBeInTheDocument();
    expect(screen.queryByText(/Data Center revenue grew/)).not.toBeInTheDocument();
  });

  it("shows guidance and performs no fetch when no assets exist", async () => {
    vi.mocked(desktopApi.financial.listActiveAssets).mockResolvedValue([]);
    renderWithQueryClient(<SecFilingFeed />);
    expect(
      await screen.findByText("Add an asset with a ticker to follow its SEC filings."),
    ).toBeInTheDocument();
    expect(desktopApi.research.fetchSecCompanyFilings).not.toHaveBeenCalled();
  });
});
