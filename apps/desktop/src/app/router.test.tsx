// Tests for route rendering with GUI-M1 layout

import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { RouterProvider, createMemoryRouter } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import "@testing-library/jest-dom";
import { LocaleProvider } from "@/lib/i18n";

// Import components
import { MainLayout } from "@/components/layout/MainLayout";
import { TodayPage } from "@/pages/today/TodayPage";
import { ResearchPage } from "@/pages/research/ResearchPage";
import { JournalPage } from "@/pages/journal/JournalPage";
import { PortfolioPage } from "@/pages/portfolio/PortfolioPage";
import { ArtifactsPage } from "@/pages/artifacts/ArtifactsPage";
import { SettingsPage } from "@/pages/settings/SettingsPage";

function renderWithRouter(initialEntry: string = "/") {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
      },
    },
  });

  const router = createMemoryRouter(
    [
      {
        path: "/",
        element: <MainLayout />,
        children: [
          { index: true, element: <TodayPage /> },
          { path: "today", element: <TodayPage /> },
          { path: "research", element: <ResearchPage /> },
          { path: "journal", element: <JournalPage /> },
          { path: "portfolio", element: <PortfolioPage /> },
          { path: "artifacts", element: <ArtifactsPage /> },
          { path: "settings", element: <SettingsPage /> },
        ],
      },
    ],
    {
      initialEntries: [initialEntry],
    },
  );

  return render(
    <QueryClientProvider client={queryClient}>
      <LocaleProvider>
        <RouterProvider router={router} />
      </LocaleProvider>
    </QueryClientProvider>
  );
}

describe("Router - Basic Rendering", () => {
  it("renders Today page at root path without crashing", () => {
    const { container } = renderWithRouter("/");
    expect(container.firstChild).toBeTruthy();
  });

  it("renders Today page at /today path without crashing", () => {
    const { container } = renderWithRouter("/today");
    expect(container.firstChild).toBeTruthy();
  });

  it("renders Research page without crashing", () => {
    const { container } = renderWithRouter("/research");
    expect(container.firstChild).toBeTruthy();
  });

  it("renders Journal page without crashing", () => {
    const { container } = renderWithRouter("/journal");
    expect(container.firstChild).toBeTruthy();
  });

  it("renders Portfolio page without crashing", () => {
    const { container } = renderWithRouter("/portfolio");
    expect(container.firstChild).toBeTruthy();
  });

  it("renders Artifacts page without crashing", () => {
    const { container } = renderWithRouter("/artifacts");
    expect(container.firstChild).toBeTruthy();
  });

  it("renders Settings page without crashing", () => {
    const { container } = renderWithRouter("/settings");
    expect(container.firstChild).toBeTruthy();
  });
});

describe("Router - GUI-M1 Layout Structure", () => {
  it("renders left sidebar", () => {
    renderWithRouter("/");
    const sidebar = screen.getByLabelText(/left sidebar/i);
    expect(sidebar).toBeInTheDocument();
  });

  it("renders workspace selector", () => {
    renderWithRouter("/");
    const button = screen.getByRole("button", { name: /workspace selector|工作区/i });
    expect(button).toBeInTheDocument();
  });

  it("renders main content area", () => {
    renderWithRouter("/");
    // Main content area exists (might not have role="main")
    const main = document.querySelector("main, [class*='main-content'], [class*='flex-1']");
    expect(main).toBeTruthy();
  });
});