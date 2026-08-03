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
 * TODO: [GUI-M1-4] Implement workspace switching logic
 * TODO: [GUI-M1-4] Add transition animations for view switching
 */

import { useState, useCallback } from "react";
import type { MainContentProps, WorkspaceType } from "../types";
import { OperationBar } from "./OperationBar";
import { StatusBar } from "./StatusBar";
import {
  AnalyzeView,
  QuantificationView,
  ComprehensiveMarketView,
  OptionsView,
  FuturesView,
  OtherDerivativesView,
} from "./WorkspaceViews";

const WORKSPACE_NAMES: Record<WorkspaceType, string> = {
  "analyze": "Analyze",
  "quantification": "Quantification",
  "comprehensive-market": "Comprehensive Market",
  "options": "Options",
  "futures": "Futures",
  "other-derivatives": "Other Derivatives",
};

export function MainContent({
  activeWorkspace = "analyze",
  isRightSidebarVisible = false,
  onToggleRightSidebar,
}: MainContentProps) {
  const [currentWorkspace, setCurrentWorkspace] = useState<WorkspaceType>(activeWorkspace);

  // TODO: [GUI-M1-4] This should be controlled by parent component state
  const handleWorkspaceChange = useCallback((workspace: WorkspaceType) => {
    setCurrentWorkspace(workspace);
    // TODO: [GUI-M1-4] Emit workspace change event to parent
  }, []);

  return (
    <div className="flex h-full flex-col bg-background">
      {/* Top: Operation Bar */}
      <OperationBar
        title={WORKSPACE_NAMES[currentWorkspace]}
        isRightSidebarExpanded={isRightSidebarVisible}
        onToggleRightSidebar={onToggleRightSidebar}
      />

      {/* Middle: Workspace Canvas */}
      <div className="flex-1 overflow-auto">
        {/* Render all views, show only active one */}
        <AnalyzeView workspace="analyze" isActive={currentWorkspace === "analyze"} />
        <QuantificationView workspace="quantification" isActive={currentWorkspace === "quantification"} />
        <ComprehensiveMarketView workspace="comprehensive-market" isActive={currentWorkspace === "comprehensive-market"} />
        <OptionsView workspace="options" isActive={currentWorkspace === "options"} />
        <FuturesView workspace="futures" isActive={currentWorkspace === "futures"} />
        <OtherDerivativesView workspace="other-derivatives" isActive={currentWorkspace === "other-derivatives"} />

        {/* TODO: [GUI-M1-4] Consider lazy loading for performance */}
        {/* TODO: [GUI-M1-4] Add view transition animations */}
      </div>

      {/* Bottom: Status Bar */}
      <StatusBar
        workspaceName={WORKSPACE_NAMES[currentWorkspace]}
        status="idle"
        hint="Press ⌘K for quick actions"
      />
    </div>
  );
}