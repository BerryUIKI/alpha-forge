import "@testing-library/jest-dom";
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { TodayPage } from "./TodayPage";

vi.mock("./components/MarketPulseBar", () => ({
  MarketPulseBar: () => <div data-testid="market-pulse" />,
}));

vi.mock("./tabs/OverviewTab", () => ({
  OverviewTab: () => <div>Overview content</div>,
}));
vi.mock("./tabs/PerformanceTab", () => ({
  PerformanceTab: () => <div>Performance content</div>,
}));
vi.mock("./tabs/ActivityTab", () => ({
  ActivityTab: () => <div>Activity content</div>,
}));

vi.mock("@/lib/i18n/useLocale", () => ({
  useLocale: () => ({
    t: (key: string) =>
      ({
        todayTitle: "Dashboard",
        todayDescription: "Your cockpit",
        todayOverviewTab: "Overview",
        todayPerformanceTab: "Performance",
        todayActivityTab: "Activity",
      })[key] ?? key,
  }),
}));

function renderPage() {
  localStorage.clear();
  return render(
    <MemoryRouter initialEntries={["/"]}>
      <TodayPage />
    </MemoryRouter>,
  );
}

describe("TodayPage", () => {
  it("aligns the heading, tabs, and content in one page container", () => {
    renderPage();

    const heading = screen.getByRole("heading", { level: 1 });
    const tablist = screen.getByRole("tablist");
    const container = heading.closest("[data-page-container]");

    expect(container).not.toBeNull();
    expect(container).toContainElement(tablist);
  });

  it("renders linked tab panels and switches the active panel", () => {
    renderPage();

    expect(screen.getByText("Overview content")).toBeInTheDocument();
    expect(screen.queryByText("Performance content")).not.toBeInTheDocument();

    const performanceTab = screen.getByRole("tab", { name: "Performance" });
    fireEvent.click(performanceTab);

    expect(screen.getByText("Performance content")).toBeInTheDocument();
    expect(screen.queryByText("Overview content")).not.toBeInTheDocument();
    expect(performanceTab).toHaveAttribute("aria-selected", "true");
    expect(performanceTab).toHaveAttribute("aria-controls", "dashboard-panel-performance");
    expect(document.getElementById("dashboard-panel-performance")).not.toBeNull();
  });
});
