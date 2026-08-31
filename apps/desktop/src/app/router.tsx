import { createBrowserRouter } from "react-router-dom";
import { MainLayout } from "@/components/layout/MainLayout";
import { SettingsLayout } from "@/components/layout/SettingsLayout";
import { FeatureErrorBoundary } from "@/components/common/FeatureErrorBoundary";
import { TodayPage } from "@/pages/today/TodayPage";
import { ResearchPage } from "@/pages/research/ResearchPage";
import { ThesesPage } from "@/pages/theses/ThesesPage";
import { JournalPage } from "@/pages/journal/JournalPage";
import { PortfolioPage } from "@/pages/portfolio/PortfolioPage";
import { KnowledgePage } from "@/pages/knowledge/KnowledgePage";
import { ArtifactsPage } from "@/pages/artifacts/ArtifactsPage";
import { SettingsPage } from "@/pages/settings/SettingsPage";
import { OptionsPage } from "@/pages/options/OptionsPage";
import { ArtifactWindowPage } from "@/pages/artifacts/ArtifactWindowPage";
import type { ReactNode } from "react";

function withFeatureBoundary(feature: string, element: ReactNode) {
  return <FeatureErrorBoundary feature={feature}>{element}</FeatureErrorBoundary>;
}

const artifactWindowRoute = {
  path: "/artifact/:artifactId/:artifactType",
  element: withFeatureBoundary("artifacts", <ArtifactWindowPage />),
};

export const router = createBrowserRouter([
  // Artifact windows are intentionally isolated from the main application layout.
  artifactWindowRoute,
  {
    path: "/settings",
    element: <SettingsLayout />,
    children: [
      { index: true, element: withFeatureBoundary("settings", <SettingsPage />) },
    ],
  },
  {
    path: "/",
    element: <MainLayout />,
    children: [
      { index: true, element: withFeatureBoundary("today", <TodayPage />) },
      { path: "today", element: withFeatureBoundary("today", <TodayPage />) },
      { path: "research", element: withFeatureBoundary("research", <ResearchPage />) },
      { path: "theses", element: withFeatureBoundary("thesis", <ThesesPage />) },
      { path: "journal", element: withFeatureBoundary("journal", <JournalPage />) },
      { path: "portfolio", element: withFeatureBoundary("portfolio", <PortfolioPage />) },
      { path: "knowledge", element: withFeatureBoundary("knowledge", <KnowledgePage />) },
      { path: "artifacts", element: withFeatureBoundary("artifacts", <ArtifactsPage />) },
      { path: "options", element: withFeatureBoundary("options", <OptionsPage />) },
    ],
  },
]);
