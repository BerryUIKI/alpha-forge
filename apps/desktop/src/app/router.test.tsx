// Tests for route rendering with the redesigned GUI layout

import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { RouterProvider, createMemoryRouter } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import "@testing-library/jest-dom";
import { LocaleProvider } from "@/lib/i18n";

// Import components
import { MainLayout } from "@/components/layout/MainLayout";
import { SettingsLayout } from "@/components/layout/SettingsLayout";
import { TodayPage } from "@/pages/today/TodayPage";
import { ResearchPage } from "@/pages/research/ResearchPage";
import { ThesesPage } from "@/pages/theses/ThesesPage";
import { JournalPage } from "@/pages/journal/JournalPage";
import { PortfolioPage } from "@/pages/portfolio/PortfolioPage";
import { KnowledgePage } from "@/pages/knowledge/KnowledgePage";
import { ArtifactsPage } from "@/pages/artifacts/ArtifactsPage";
import { SettingsPage } from "@/pages/settings/SettingsPage";
import { OptionsPage } from "@/pages/options/OptionsPage";

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
        path: "/settings",
        element: <SettingsLayout />,
        children: [{ index: true, element: <SettingsPage /> }],
      },
      {
        path: "/",
        element: <MainLayout />,
        children: [
          { index: true, element: <TodayPage /> },
          { path: "today", element: <TodayPage /> },
          { path: "research", element: <ResearchPage /> },
          { path: "theses", element: <ThesesPage /> },
          { path: "journal", element: <JournalPage /> },
          { path: "portfolio", element: <PortfolioPage /> },
          { path: "knowledge", element: <KnowledgePage /> },
          { path: "artifacts", element: <ArtifactsPage /> },
          { path: "options", element: <OptionsPage /> },
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

  it("renders Theses page without crashing", () => {
    const { container } = renderWithRouter("/theses");
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

  it("renders Knowledge page without crashing", () => {
    const { container } = renderWithRouter("/knowledge");
    expect(container.firstChild).toBeTruthy();
  });

  it("renders Artifacts page without crashing", () => {
    const { container } = renderWithRouter("/artifacts");
    expect(container.firstChild).toBeTruthy();
  });

  it("renders Settings page without crashing", () => {
    const { container } = renderWithRouter("/settings");
    expect(container.firstChild).toBeTruthy();
    expect(screen.getByLabelText(/settings navigation/i)).toBeInTheDocument();
    expect(screen.queryByLabelText(/^left sidebar/i)).not.toBeInTheDocument();
  });

  it("renders Options page without crashing", () => {
    const { container } = renderWithRouter("/options");
    expect(container.firstChild).toBeTruthy();
  });
});

describe("Router - GUI Layout Structure", () => {
  it("renders left sidebar", () => {
    renderWithRouter("/");
    const sidebar = screen.getByLabelText(/left sidebar/i);
    expect(sidebar).toBeInTheDocument();
  });

  it("renders dashboard page with tab bar", () => {
    renderWithRouter("/");
    // The dashboard shows tabs
    expect(screen.getByRole("button", { name: /overview/i })).toBeInTheDocument();
  });

  it("renders main content area", () => {
    renderWithRouter("/");
    // Main content area exists (flex-1 container)
    const main = document.querySelector("[class*='flex-1']");
    expect(main).toBeTruthy();
  });
});
