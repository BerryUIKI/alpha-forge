import "@testing-library/jest-dom";
import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { MarketPulseBar } from "./MarketPulseBar";
import { SecFilingFeed } from "./SecFilingFeed";

vi.mock("@/lib/i18n/useLocale", () => ({
  useLocale: () => ({
    t: (key: string) => {
      const map: Record<string, string> = {
        marketPulse: "Market Pulse",
        dataSynced: "Synced: {time}",
        secFilingFeed: "SEC Filing & Research Stream",
        secAutomatedParsing: "Automated SEC Parsing",
        secLiveEdgar: "Live SEC EDGAR",
        secLoading: "Ingesting live SEC filings...",
        secViewDocument: "Open SEC Filing",
        linkedThesis: "Linked Thesis: {title}",
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
          filingUrl: "https://www.sec.gov/Archives/edgar/data/1045810/000104581024000123/nvda-10q.htm",
          summary: "Revenue up 154% YoY",
        },
      ]),
    },
  },
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

describe("Today Cockpit Components", () => {
  it("renders MarketPulseBar with benchmark tickers", () => {
    renderWithQueryClient(<MarketPulseBar />);
    expect(screen.getByText("Market Pulse")).toBeInTheDocument();
    expect(screen.getByText("S&P 500")).toBeInTheDocument();
    expect(screen.getByText("NASDAQ 100")).toBeInTheDocument();
    expect(screen.getByText("US 10Y")).toBeInTheDocument();
    expect(screen.getByText("VIX")).toBeInTheDocument();
  });

  it("renders SecFilingFeed with filing cards and linked thesis tags", async () => {
    renderWithQueryClient(<SecFilingFeed />);
    expect(screen.getByText("SEC Filing & Research Stream")).toBeInTheDocument();
    expect(screen.getByText("Live SEC EDGAR")).toBeInTheDocument();
    expect(await screen.findByText("NVDA")).toBeInTheDocument();
  });
});
