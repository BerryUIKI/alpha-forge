import "@testing-library/jest-dom";
import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
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
        linkedThesis: "Linked Thesis: {title}",
        evidencePositive: "Positive Confirmation (+{delta}% Conviction)",
        evidenceContra: "Contra-Evidence Alert (-{delta}% Conviction)",
        hoursAgo: "{hours} ago",
      };
      return map[key] ?? key;
    },
  }),
}));

describe("Today Cockpit Components", () => {
  it("renders MarketPulseBar with benchmark tickers", () => {
    render(<MarketPulseBar />);
    expect(screen.getByText("Market Pulse")).toBeInTheDocument();
    expect(screen.getByText("S&P 500")).toBeInTheDocument();
    expect(screen.getByText("NASDAQ 100")).toBeInTheDocument();
    expect(screen.getByText("US 10Y")).toBeInTheDocument();
    expect(screen.getByText("VIX")).toBeInTheDocument();
  });

  it("renders SecFilingFeed with filing cards and linked thesis tags", () => {
    render(<SecFilingFeed />);
    expect(screen.getByText("SEC Filing & Research Stream")).toBeInTheDocument();
    expect(screen.getByText("Automated SEC Parsing")).toBeInTheDocument();
    expect(screen.getByText("NVDA")).toBeInTheDocument();
    expect(screen.getByText("ASML")).toBeInTheDocument();
  });
});
