/**
 * MainLayout Component
 *
 * Main layout orchestrator for Codex-style desktop interface.
 * Combines all four modules:
 * - Module A: Left Collapsible Sidebar
 * - Module B: Top Native Menu Bar (Tauri integration)
 * - Module C: Right Collapsible Sidebar (Agent)
 * - Module D: Main Content Area
 *
 * State Management:
 * - Sidebar states (expanded/collapsed) are managed here
 * - Workspace selection state is managed via useWorkspaceState hook
 * - Right sidebar toggle is synchronized between MainContent and RightSidebar
 * - Keyboard shortcuts for sidebar toggles
 *
 * @version GUI-M1-4
 */

import { useState, useCallback } from "react";
import type { SidebarState, WorkspaceType } from "./types";
import { LeftSidebar } from "./LeftSidebar";
import { RightSidebar } from "./RightSidebar";
import { MainContent } from "./MainContent";
import { useSidebarShortcuts } from "@/hooks/layout";

export function MainLayout() {
  // Sidebar states
  const [leftState, setLeftState] = useState<SidebarState>("expanded");
  const [rightState, setRightState] = useState<SidebarState>("collapsed");

  // Workspace selection is now managed in MainContent via useWorkspaceState

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

  // Handle workspace selection changes from left sidebar
  const handleWorkspaceChange = useCallback((workspace: WorkspaceType) => {
    // Note: Workspace state is now managed in MainContent via useWorkspaceState hook
    // This callback is kept for future integration with left sidebar workspace selector
    console.log("Workspace changed to:", workspace);
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
        selectedWorkspace="analyze" // Will be connected to workspace state in future
        onWorkspaceChange={handleWorkspaceChange}
      />

      {/* Module D: Main Content Area */}
      <div className="flex flex-1 flex-col overflow-hidden">
        <MainContent
          isRightSidebarVisible={rightState === "expanded"}
          onToggleRightSidebar={handleToggleRightSidebar}
        />
      </div>

      {/* Module C: Right Sidebar (Agent) */}
      <RightSidebar
        state={rightState}
        onStateChange={handleRightStateChange}
      />

      {/* Module B: Native Menu Bar - configured via Tauri */}
    </div>
  );
}
