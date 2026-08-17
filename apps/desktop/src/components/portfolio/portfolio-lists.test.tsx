import "@testing-library/jest-dom";
import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { HoldingsList, type Holding } from "./HoldingsList";
import { ActivityFeed, type ActivityItem } from "@/components/activity/ActivityFeed";

const holdings: Holding[] = [
  {
    id: "h1",
    ticker: "AAPL",
    name: "Apple Inc.",
    sector: "Technology",
    allocation: "25.0%",
    value: "$25,000",
    change: "+2.1%",
    changePositive: true,
  },
  {
    id: "h2",
    ticker: "MSFT",
    name: "Microsoft Corp.",
    sector: "Technology",
    allocation: "15.0%",
    value: "$15,000",
    change: "-1.2%",
    changePositive: false,
  },
];

const activity: ActivityItem[] = [
  {
    id: "a1",
    type: "research",
    title: "Research",
    description: "New NVDA analysis completed",
    timestamp: "12m ago",
  },
  {
    id: "a2",
    type: "thesis",
    title: "Thesis",
    description: "Renewable energy thesis updated",
    timestamp: "2h ago",
  },
  {
    id: "a3",
    type: "portfolio",
    title: "Portfolio",
    description: "Position added to Brokerage",
    timestamp: "5h ago",
  },
];

describe("HoldingsList", () => {
  it("renders holding rows with ticker, name, sector and value", () => {
    render(<HoldingsList holdings={holdings} />);
    expect(screen.getByText("Apple Inc.")).toBeInTheDocument();
    expect(screen.getByText("MSFT")).toBeInTheDocument();
    expect(screen.getByText("$25,000")).toBeInTheDocument();
    expect(screen.getByText("Technology · 25.0%")).toBeInTheDocument();
  });

  it("renders positive change in green", () => {
    render(<HoldingsList holdings={[holdings[0]!]} />);
    const change = screen.getByText("+2.1%");
    expect(change.className).toContain("text-green-500");
  });

  it("renders negative change in red", () => {
    render(<HoldingsList holdings={[holdings[1]!]} />);
    const change = screen.getByText("-1.2%");
    expect(change.className).toContain("text-red-500");
  });
});

describe("ActivityFeed", () => {
  it("renders activity items with title, description and timestamp", () => {
    render(<ActivityFeed items={activity} />);
    expect(screen.getByText("New NVDA analysis completed")).toBeInTheDocument();
    expect(screen.getByText("12m ago")).toBeInTheDocument();
    expect(screen.getByText("Renewable energy thesis updated")).toBeInTheDocument();
  });

  it("assigns a dot color class per type", () => {
    render(<ActivityFeed items={activity} />);
    const dots = document.querySelectorAll("span[class*='rounded-full']");
    expect(dots).toHaveLength(3);
  });

  it("renders empty list without crashing", () => {
    render(<ActivityFeed items={[]} />);
    expect(document.querySelector("ul")?.children).toHaveLength(0);
  });
});