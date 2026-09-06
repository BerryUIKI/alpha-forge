import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { UnifiedHeader } from "./UnifiedHeader";

const mockNavigate = vi.fn();
vi.mock("react-router-dom", async () => {
  const actual = await vi.importActual("react-router-dom");
  return {
    ...actual,
    useNavigate: () => mockNavigate,
  };
});

vi.mock("@/lib/i18n/useLocale", () => ({
  useLocale: () => ({
    t: (key: string) => {
      const map: Record<string, string> = {
        navDashboard: "Dashboard",
        navWorkspace: "Workspace",
        navResearch: "Research",
        navTheses: "Theses",
        navPortfolio: "Portfolio",
        unifiedSearchPlaceholder: "Search tickers, theses, filings, or ask Agent…",
        newResearch: "New Research",
        openAgentPanel: "Open Agent panel",
        closeAgentPanel: "Close Agent panel",
        collapseSidebar: "Collapse sidebar",
        expandSidebar: "Expand sidebar",
      };
      return map[key] || key;
    },
  }),
}));

vi.mock("@/features/workspace/hooks/useActiveWorkspace.context", () => ({
  useActiveWorkspace: () => ({
    workspaceId: "",
    workspace: null,
    workspaces: [],
    isLoading: false,
    setActiveWorkspace: vi.fn(),
  }),
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    window: {
      minimize: vi.fn(),
      toggleMaximize: vi.fn(),
      close: vi.fn(),
    },
  },
}));

describe("UnifiedHeader", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders search, action, and toggle elements", () => {
    render(
      <MemoryRouter initialEntries={["/today"]}>
        <UnifiedHeader
          isLeftSidebarExpanded={true}
          onToggleLeftSidebar={vi.fn()}
          isRightSidebarExpanded={false}
          onToggleRightSidebar={vi.fn()}
          onOpenSearch={vi.fn()}
        />
      </MemoryRouter>
    );

    expect(screen.getByText("Search tickers, theses, filings, or ask Agent…")).toBeInTheDocument();
    expect(screen.getByText("New Research")).toBeInTheDocument();
    expect(screen.getByLabelText("Collapse sidebar")).toBeInTheDocument();
    expect(screen.getByLabelText("Open Agent panel")).toBeInTheDocument();
  });

  it("calls onOpenSearch when search button is clicked", () => {
    const onOpenSearch = vi.fn();
    render(
      <MemoryRouter initialEntries={["/today"]}>
        <UnifiedHeader onOpenSearch={onOpenSearch} />
      </MemoryRouter>
    );

    const searchBtn = screen.getByLabelText("Search tickers, theses, filings, or ask Agent…");
    fireEvent.click(searchBtn);
    expect(onOpenSearch).toHaveBeenCalledTimes(1);
  });

  it("calls onToggleLeftSidebar when sidebar button is clicked", () => {
    const onToggleLeft = vi.fn();
    render(
      <MemoryRouter initialEntries={["/"]}>
        <UnifiedHeader isLeftSidebarExpanded={true} onToggleLeftSidebar={onToggleLeft} />
      </MemoryRouter>
    );

    const toggleBtn = screen.getByLabelText("Collapse sidebar");
    fireEvent.click(toggleBtn);
    expect(onToggleLeft).toHaveBeenCalledTimes(1);
  });

  it("calls onToggleRightSidebar when agent button is clicked", () => {
    const onToggleRight = vi.fn();
    render(
      <MemoryRouter initialEntries={["/"]}>
        <UnifiedHeader isRightSidebarExpanded={false} onToggleRightSidebar={onToggleRight} />
      </MemoryRouter>
    );

    const toggleBtn = screen.getByLabelText("Open Agent panel");
    fireEvent.click(toggleBtn);
    expect(onToggleRight).toHaveBeenCalledTimes(1);
  });

  it("navigates to /research when New Research button is clicked", () => {
    render(
      <MemoryRouter initialEntries={["/"]}>
        <UnifiedHeader />
      </MemoryRouter>
    );

    const newBtn = screen.getByLabelText("New Research");
    fireEvent.click(newBtn);
    expect(mockNavigate).toHaveBeenCalledWith("/research");
  });
});
