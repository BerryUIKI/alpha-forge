import "@testing-library/jest-dom";
import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { DashboardCard } from "./DashboardCard";
import { StatCard } from "./StatCard";
import { TabBar } from "./TabBar";
import { SearchBar } from "./SearchBar";
import { Settings } from "lucide-react";

describe("DashboardCard", () => {
  it("renders title and children", () => {
    render(
      <DashboardCard title="My Card">
        <p>Card body content</p>
      </DashboardCard>,
    );
    expect(screen.getByText("My Card")).toBeInTheDocument();
    expect(screen.getByText("Card body content")).toBeInTheDocument();
  });

  it("renders subtitle when provided", () => {
    render(
      <DashboardCard title="Card" subtitle="Sub title">
        <p>body</p>
      </DashboardCard>,
    );
    expect(screen.getByText("Sub title")).toBeInTheDocument();
  });

  it("renders uppercase meta text when no action given", () => {
    render(
      <DashboardCard title="Card" meta="view all">
        <p>body</p>
      </DashboardCard>,
    );
    expect(screen.getByText("view all")).toBeInTheDocument();
  });

  it("prefers action slot over meta", () => {
    render(
      <DashboardCard title="Card" meta="view all" action={<button>Custom</button>}>
        <p>body</p>
      </DashboardCard>,
    );
    expect(screen.getByRole("button", { name: "Custom" })).toBeInTheDocument();
    expect(screen.queryByText("view all")).not.toBeInTheDocument();
  });
});

describe("StatCard", () => {
  it("renders label and value", () => {
    render(<StatCard label="Revenue" value="$1,000" />);
    expect(screen.getByText("Revenue")).toBeInTheDocument();
    expect(screen.getByText("$1,000")).toBeInTheDocument();
  });

  it("renders positive change in green", () => {
    render(<StatCard label="Revenue" value="$1,000" change="+12%" isPositive />);
    const change = screen.getByText("+12%");
    expect(change).toBeInTheDocument();
    expect(change.className).toContain("text-green-500");
  });

  it("renders negative change in red", () => {
    render(<StatCard label="Revenue" value="$1,000" change="-3%" isPositive={false} />);
    const change = screen.getByText("-3%");
    expect(change).toBeInTheDocument();
    expect(change.className).toContain("text-red-500");
  });

  it("renders icon when provided", () => {
    render(<StatCard label="Settings" value="On" icon={<Settings />} />);
    expect(document.querySelector("svg")).toBeTruthy();
  });
});

describe("TabBar", () => {
  const tabs = [
    { id: "overview", label: "Overview" },
    { id: "performance", label: "Performance" },
    { id: "activity", label: "Activity" },
  ];

  it("renders all tabs", () => {
    render(<TabBar tabs={tabs} activeTab="overview" onTabChange={() => {}} />);
    expect(screen.getByRole("button", { name: "Overview" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Performance" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Activity" })).toBeInTheDocument();
  });

  it("marks active tab with aria-current", () => {
    render(<TabBar tabs={tabs} activeTab="performance" onTabChange={() => {}} />);
    const active = screen.getByRole("button", { name: "Performance" });
    expect(active).toHaveAttribute("aria-current", "page");
  });

  it("fires onTabChange on click", () => {
    const onTabChange = vi.fn();
    render(<TabBar tabs={tabs} activeTab="overview" onTabChange={onTabChange} />);
    fireEvent.click(screen.getByRole("button", { name: "Activity" }));
    expect(onTabChange).toHaveBeenCalledWith("activity");
  });
});

describe("SearchBar", () => {
  it("renders with default placeholder", () => {
    render(<SearchBar />);
    expect(screen.getByPlaceholderText("Search...")).toBeInTheDocument();
  });

  it("renders with custom placeholder", () => {
    render(<SearchBar placeholder="Find tickers" />);
    expect(screen.getByPlaceholderText("Find tickers")).toBeInTheDocument();
  });

  it("fires onChange when typing", () => {
    const onChange = vi.fn();
    render(<SearchBar onChange={onChange} />);
    fireEvent.change(screen.getByPlaceholderText("Search..."), {
      target: { value: "AAPL" },
    });
    expect(onChange).toHaveBeenCalledWith("AAPL");
  });
});