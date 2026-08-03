/**
 * MainLayout Component
 *
 * Main layout orchestrator for Codex-style desktop interface.
 * Combines all four modules:
 * - Module A: Left Collapsible Sidebar
 * - Module B: Top Native Menu Bar (TODO: Tauri integration)
 * - Module C: Right Collapsible Sidebar (Agent)
 * - Module D: Main Content Area
 *
 * State Management:
 * - Sidebar states (expanded/collapsed) are managed here
 * - Workspace selection state is managed here
 * - Right sidebar toggle is synchronized between MainContent and RightSidebar
 * - Keyboard shortcuts for sidebar toggles
 *
 * @version GUI-M1-1
 */

import { useState, useCallback } from "react";
import type { WorkspaceType, SidebarState } from "./types";
import { LeftSidebar } from "./LeftSidebar";
import { RightSidebar } from "./RightSidebar";
import { MainContent } from "./MainContent";
import { useSidebarShortcuts } from "@/hooks/layout";

interface MainLayoutProps {
  /** Initial left sidebar state */
  leftSidebarState?: SidebarState;
  /** Initial right sidebar state */
  rightSidebarState?: SidebarState;
  /** Initial workspace type */
  initialWorkspace?: WorkspaceType;
}

export function MainLayout({
  leftSidebarState = "expanded",
  rightSidebarState = "collapsed",
  initialWorkspace = "analyze",
}: MainLayoutProps) {
  // Sidebar states
  const [leftState, setLeftState] = useState<SidebarState>(leftSidebarState);
  const [rightState, setRightState] = useState<SidebarState>(rightSidebarState);

  // Workspace selection
  const [activeWorkspace, setActiveWorkspace] = useState<WorkspaceType>(initialWorkspace);

  // Handle left sidebar state changes
  const handleLeftStateChange = useCallback((state: SidebarState) => {
    setLeftState(state);
  }, []);

  // Handle right sidebar state changes (synchronized toggle)
  const handleRightStateChange = useCallback((state: SidebarState) => {
    setRightState(state);
  }, []);

  // Handle right sidebar toggle from MainContent
  const handleToggleRightSidebar = useCallback(() => {
    setRightState((prev) => (prev === "expanded" ? "collapsed" : "expanded"));
  }, []);

  // Handle workspace selection changes
  const handleWorkspaceChange = useCallback((workspace: WorkspaceType) => {
    setActiveWorkspace(workspace);
  }, []);

  // Toggle left sidebar (for keyboard shortcut)
  const toggleLeftSidebar = useCallback(() => {
    setLeftState((prev) => (prev === "expanded" ? "collapsed" : "expanded"));
  }, []);

  // Toggle right sidebar (for keyboard shortcut)
  const toggleRightSidebar = useCallback(() => {
    setRightState((prev) => (prev === "expanded" ? "collapsed" : "expanded"));
  }, []);

  // Toggle both sidebars (for keyboard shortcut)
  const toggleBothSidebars = useCallback(() => {
    const bothExpanded = leftState === "expanded" && rightState === "expanded";
    const newState: SidebarState = bothExpanded ? "collapsed" : "expanded";
    setLeftState(newState);
    setRightState(newState);
  }, [leftState, rightState]);

  // Setup keyboard shortcuts
  useSidebarShortcuts({
    onToggleLeft: toggleLeftSidebar,
    onToggleRight: toggleRightSidebar,
    onToggleBoth: toggleBothSidebars,
    enabled: true,
  });

  return (
    <div className="flex h-screen overflow-hidden bg-background">
      {/* Module A: Left Sidebar */}
      <LeftSidebar
        state={leftState}
        onStateChange={handleLeftStateChange}
        selectedWorkspace={activeWorkspace}
        onWorkspaceChange={handleWorkspaceChange}
      />

      {/* Module D: Main Content Area */}
      <div className="flex flex-1 flex-col overflow-hidden">
        <MainContent
          activeWorkspace={activeWorkspace}
          isRightSidebarVisible={rightState === "expanded"}
          onToggleRightSidebar={handleToggleRightSidebar}
        />
      </div>

      {/* Module C: Right Sidebar (Agent) */}
      <RightSidebar
        state={rightState}
        onStateChange={handleRightStateChange}
      />

      {/* TODO: [GUI-M1-2] Module B: Native Menu Bar integration */}
      {/* This will be configured via Tauri menu API, not React component */}
    </div>
  );
}
