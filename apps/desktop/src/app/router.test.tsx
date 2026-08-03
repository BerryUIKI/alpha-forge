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
    },
  );

  return render(
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>,
  );
}

describe("Router", () => {
  it("renders Today page at root path", () => {
    renderWithRouter("/");
    expect(screen.getByText("今日")).toBeInTheDocument();
  });

  it("renders Today page at /today path", () => {
    renderWithRouter("/today");
    expect(screen.getByText("今日")).toBeInTheDocument();
  });

  it("renders Research page", () => {
    renderWithRouter("/research");
    expect(screen.getByText("研究")).toBeInTheDocument();
  });

  it("renders Journal page", () => {
    renderWithRouter("/journal");
    expect(screen.getByText("投资日志")).toBeInTheDocument();
  });

  it("renders Portfolio page", async () => {
    renderWithRouter("/portfolio");
    const heading = await screen.findByRole("heading", { level: 1 });
    expect(heading).toBeInTheDocument();
  });

  it("renders Artifacts page", async () => {
    renderWithRouter("/artifacts");
    const heading = await screen.findByRole("heading", { level: 1 });
    expect(heading).toBeInTheDocument();
  });

  it("renders Settings page", () => {
    renderWithRouter("/settings");
    expect(screen.getByText("设置")).toBeInTheDocument();
  });

  it("renders sidebar navigation", () => {
    renderWithRouter("/");
    expect(screen.getByTitle("今日")).toBeInTheDocument();
    expect(screen.getByTitle("研究")).toBeInTheDocument();
    expect(screen.getByTitle("投资日志")).toBeInTheDocument();
    expect(screen.getByTitle("投资组合")).toBeInTheDocument();
    expect(screen.getByTitle("研究产物")).toBeInTheDocument();
    expect(screen.getByTitle("设置")).toBeInTheDocument();
  });
});