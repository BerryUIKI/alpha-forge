/**
 * MainContent Component
 *
 * Main content area container (Module D).
 * Combines OperationBar, workspace views canvas, and StatusBar.
 *
 * Structure (top to bottom):
 * 1. OperationBar - context title and action buttons
 * 2. Workspace Canvas - switchable workspace views
 * 3. StatusBar - workspace name, status, hints
 *
 * Features:
 * - Workspace switching with state persistence
 * - Welcome page when no view selected
 * - Responsive layout adaptation
 *
 * @version GUI-M1-4
 */

import type { MainContentProps } from "../types";
import { OperationBar } from "./OperationBar";
import { StatusBar } from "./StatusBar";
import { WelcomePage } from "./WelcomePage";
import {
  AnalyzeView,
  QuantificationView,
  ComprehensiveMarketView,
  OptionsView,
  FuturesView,
  OtherDerivativesView,
} from "./WorkspaceViews";
import { useWorkspaceState } from "@/hooks/layout";

const WORKSPACE_NAMES: Record<string, string> = {
  "analyze": "Analyze",
  "quantification": "Quantification",
  "comprehensive-market": "Comprehensive Market",
  "options": "Options",
  "futures": "Futures",
  "other-derivatives": "Other Derivatives",
};

export function MainContent({
  activeWorkspace: externalWorkspace,
  isRightSidebarVisible = false,
  onToggleRightSidebar,
}: MainContentProps) {
  // Use workspace state hook for persistence
  const { activeWorkspace, setActiveWorkspace, workspaceNames, isWorkspaceActive } = useWorkspaceState({
    defaultWorkspace: externalWorkspace || "analyze",
  });

  // Update external workspace if provided
  // Note: We keep internal state management for persistence

  return (
    <div className="flex h-full flex-col bg-background">
      {/* Top: Operation Bar */}
      <OperationBar
        title={workspaceNames[activeWorkspace]}
        isRightSidebarExpanded={isRightSidebarVisible}
        onToggleRightSidebar={onToggleRightSidebar}
      />

      {/* Middle: Workspace Canvas */}
      <div className="flex-1 overflow-auto">
        {/* Render all views, show only active one */}
        <AnalyzeView workspace="analyze" isActive={isWorkspaceActive("analyze")} />
        <QuantificationView workspace="quantification" isActive={isWorkspaceActive("quantification")} />
        <ComprehensiveMarketView workspace="comprehensive-market" isActive={isWorkspaceActive("comprehensive-market")} />
        <OptionsView workspace="options" isActive={isWorkspaceActive("options")} />
        <FuturesView workspace="futures" isActive={isWorkspaceActive("futures")} />
        <OtherDerivativesView workspace="other-derivatives" isActive={isWorkspaceActive("other-derivatives")} />

        {/* TODO: [GUI-M1-4] Consider lazy loading for performance */}
        {/* TODO: [GUI-M1-4] Add view transition animations */}
      </div>

      {/* Bottom: Status Bar */}
      <StatusBar
        workspaceName={workspaceNames[activeWorkspace]}
        status="idle"
        hint="Press Ctrl+K for quick actions"
      />
    </div>
  );
}