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
 *
 * TODO: [GUI-M1-2] Integrate with Tauri native menu bar
 * TODO: [GUI-M1-5] Add keyboard shortcuts for sidebar toggles
 * TODO: [GUI-M1-5] Persist layout state to local storage
 */

import { useState, useCallback } from "react";
import type { WorkspaceType, SidebarState } from "./types";
import { LeftSidebar } from "./LeftSidebar";
import { RightSidebar } from "./RightSidebar";
import { MainContent } from "./MainContent";

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
    // TODO: [GUI-M1-5] Persist to local storage
  }, []);

  // Handle right sidebar state changes (synchronized toggle)
  const handleRightStateChange = useCallback((state: SidebarState) => {
    setRightState(state);
    // TODO: [GUI-M1-5] Persist to local storage
  }, []);

  // Handle right sidebar toggle from MainContent
  const handleToggleRightSidebar = useCallback(() => {
    setRightState((prev) => (prev === "expanded" ? "collapsed" : "expanded"));
    // TODO: [GUI-M1-5] Persist to local storage
  }, []);

  // Handle workspace selection changes
  const handleWorkspaceChange = useCallback((workspace: WorkspaceType) => {
    setActiveWorkspace(workspace);
    // TODO: [GUI-M1-4] Trigger view switching in MainContent
    // TODO: [GUI-M1-5] Persist to local storage
  }, []);

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

// TODO: [GUI-M1-2] Add Tauri menu bar configuration
// TODO: [GUI-M1-5] Add keyboard shortcuts:
//   - ⌘/Ctrl + 1: Toggle left sidebar
//   - ⌘/Ctrl + 2: Toggle right sidebar
//   - ⌘/Ctrl + B: Toggle both sidebars
// TODO: [GUI-M1-5] Add layout state persistence:
//   - Save sidebar states to localStorage
//   - Save active workspace to localStorage
//   - Restore on app startup
