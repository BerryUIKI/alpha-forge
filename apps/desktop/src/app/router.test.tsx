// Tests for route rendering.

import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { RouterProvider, createMemoryRouter } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import "@testing-library/jest-dom";

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
    }
  );

  return render(
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  );
}

describe("Router", () => {
  it("renders Today page at root path", () => {
    renderWithRouter("/");
    expect(screen.getByText("Today")).toBeInTheDocument();
  });

  it("renders Today page at /today path", () => {
    renderWithRouter("/today");
    expect(screen.getByText("Today")).toBeInTheDocument();
  });

  it("renders Research page", () => {
    renderWithRouter("/research");
    expect(screen.getByText("Research")).toBeInTheDocument();
  });

  it("renders Journal page", () => {
    renderWithRouter("/journal");
    expect(screen.getByText("Journal")).toBeInTheDocument();
  });

  it("renders Portfolio page", () => {
    renderWithRouter("/portfolio");
    expect(screen.getByText("Portfolio")).toBeInTheDocument();
  });

  it("renders Artifacts page", () => {
    renderWithRouter("/artifacts");
    expect(screen.getByText("Artifacts")).toBeInTheDocument();
  });

  it("renders Settings page", () => {
    renderWithRouter("/settings");
    expect(screen.getByText("Settings")).toBeInTheDocument();
  });

  it("renders sidebar navigation", () => {
    renderWithRouter("/");
    expect(screen.getByTitle("Today")).toBeInTheDocument();
    expect(screen.getByTitle("Research")).toBeInTheDocument();
    expect(screen.getByTitle("Journal")).toBeInTheDocument();
    expect(screen.getByTitle("Portfolio")).toBeInTheDocument();
    expect(screen.getByTitle("Artifacts")).toBeInTheDocument();
    expect(screen.getByTitle("Settings")).toBeInTheDocument();
  });
});