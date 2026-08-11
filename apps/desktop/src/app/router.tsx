import { createBrowserRouter } from "react-router-dom";
import { MainLayout } from "@/components/layout/MainLayout";
import { TodayPage } from "@/pages/today/TodayPage";
import { ResearchPage } from "@/pages/research/ResearchPage";
import { JournalPage } from "@/pages/journal/JournalPage";
import { PortfolioPage } from "@/pages/portfolio/PortfolioPage";
import { ArtifactsPage } from "@/pages/artifacts/ArtifactsPage";
import { SettingsPage } from "@/pages/settings/SettingsPage";
import { OptionsPage } from "@/pages/options/OptionsPage";

export const router = createBrowserRouter([
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
      { path: "options", element: <OptionsPage /> },
    ],
  },
]);
