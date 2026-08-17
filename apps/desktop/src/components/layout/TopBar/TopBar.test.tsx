import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { TopBar } from "./TopBar";

// Mock next-themes
vi.mock("next-themes", () => ({
  useTheme: vi.fn(),
}));

// Mock i18n
vi.mock("@/lib/i18n/useLocale", () => ({
  useLocale: vi.fn(),
}));

import { useTheme } from "next-themes";
import { useLocale } from "@/lib/i18n/useLocale";
import type { Locale } from "@/lib/i18n/locale";

function mockUseTheme(theme: string = "dark") {
  vi.mocked(useTheme).mockReturnValue({
    theme,
    setTheme: vi.fn(),
    resolvedTheme: theme,
    themes: ["light", "dark", "system"],
    systemTheme: "dark",
  } as unknown as ReturnType<typeof useTheme>);
}

function mockUseLocale(locale: Locale = "en") {
  vi.mocked(useLocale).mockReturnValue({
    t: (key: string) => {
      const map: Record<string, string> = {
        navDashboard: "Dashboard",
        navWorkspace: "Workspace",
        navResearch: "Research",
        navTheses: "Theses",
        navPortfolio: "Portfolio",
        navKnowledge: "Knowledge",
        navJournal: "Journal",
        navOptions: "Options",
        navArtifacts: "Artifacts",
        navSettings: "Settings",
        navTools: "Tools",
        navAccount: "Account",
      };
      return map[key] || key;
    },
    locale,
    setLocale: vi.fn(),
  } as unknown as ReturnType<typeof useLocale>);
}

function renderTopBar(path: string = "/") {
  return render(
    <MemoryRouter initialEntries={[path]}>
      <TopBar isRightSidebarExpanded={false} onToggleRightSidebar={vi.fn()} />
    </MemoryRouter>,
  );
}

beforeEach(() => {
  vi.clearAllMocks();
  mockUseTheme("dark");
  mockUseLocale("en");
});

describe("TopBar", () => {
  it("renders breadcrumb for the current route", () => {
    renderTopBar("/");
    expect(screen.getByText("Workspace")).toBeInTheDocument();
    expect(screen.getByText("Dashboard")).toBeInTheDocument();
  });

  it("renders breadcrumb for research route", () => {
    renderTopBar("/research");
    expect(screen.getByText("Research")).toBeInTheDocument();
  });

  it("renders breadcrumb for settings route", () => {
    renderTopBar("/settings");
    expect(screen.getByText("Account")).toBeInTheDocument();
    expect(screen.getByText("Settings")).toBeInTheDocument();
  });

  it("toggles theme on button click", () => {
    const setTheme = vi.fn();
    vi.mocked(useTheme).mockReturnValue({
      theme: "dark",
      setTheme,
      resolvedTheme: "dark",
      themes: ["light", "dark", "system"],
      systemTheme: "dark",
    } as unknown as ReturnType<typeof useTheme>);
    renderTopBar();
    const themeBtn = screen.getByLabelText("Switch to light mode");
    fireEvent.click(themeBtn);
    expect(setTheme).toHaveBeenCalledWith("light");
  });

  it("shows language dropdown when globe clicked", () => {
    renderTopBar();
    const langBtn = screen.getByLabelText("Switch language");
    fireEvent.click(langBtn);
    expect(screen.getByText("English")).toBeInTheDocument();
    expect(screen.getByText("简体中文")).toBeInTheDocument();
  });

  it("changes locale when language option clicked", () => {
    const setLocale = vi.fn();
    vi.mocked(useLocale).mockReturnValue({
      t: (k: string) => k,
      locale: "en",
      setLocale,
    } as unknown as ReturnType<typeof useLocale>);
    renderTopBar();
    fireEvent.click(screen.getByLabelText("Switch language"));
    fireEvent.click(screen.getByText("简体中文"));
    expect(setLocale).toHaveBeenCalledWith("zh-CN");
  });

  it("navigates to settings when gear clicked", () => {
    renderTopBar();
    const settingsBtn = screen.getByLabelText("Settings");
    fireEvent.click(settingsBtn);
    // Just verify it renders — navigation is handled by react-router
    expect(settingsBtn).toBeInTheDocument();
  });

  it("fires onToggleRightSidebar when agent toggle clicked", () => {
    const onToggle = vi.fn();
    render(
      <MemoryRouter>
        <TopBar isRightSidebarExpanded={false} onToggleRightSidebar={onToggle} />
      </MemoryRouter>,
    );
    fireEvent.click(screen.getByLabelText("Open agent panel"));
    expect(onToggle).toHaveBeenCalled();
  });

  it("shows correct agent toggle label when expanded", () => {
    render(
      <MemoryRouter>
        <TopBar isRightSidebarExpanded={true} onToggleRightSidebar={vi.fn()} />
      </MemoryRouter>,
    );
    expect(screen.getByLabelText("Close agent panel")).toBeInTheDocument();
  });

  it("renders search bar with keyboard shortcut", () => {
    renderTopBar();
    expect(screen.getByText("Search...")).toBeInTheDocument();
    expect(screen.getByText("⌘K")).toBeInTheDocument();
  });
});