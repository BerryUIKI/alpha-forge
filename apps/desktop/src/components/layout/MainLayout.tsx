/**
 * MainLayout Component
 *
 * Primary layout orchestrator for the Investment OS desktop application.
 * Combines three zones:
 * - Left: Collapsible sidebar with workspace selector, project list, user operations
 * - Center: Main content area with OperationBar, routed pages, StatusBar
 * - Right: Collapsible Agent panel
 *
 * @version GUI-M2
 */

import { useState, useCallback } from "react";
import { Outlet } from "react-router-dom";
import { LeftSidebar } from "./LeftSidebar";
import { MainContent } from "./MainContent";
import { RightSidebar } from "./RightSidebar";
import { useSidebarShortcuts } from "@/hooks/layout";
import type { SidebarState } from "./types";

export function MainLayout() {
  // Right sidebar state (Agent panel)
  const [rightState, setRightState] = useState<SidebarState>("collapsed");

  const handleRightStateChange = useCallback((state: SidebarState) => {
    setRightState(state);
  }, []);

  const toggleRightSidebar = useCallback(() => {
    setRightState((prev) => (prev === "expanded" ? "collapsed" : "expanded"));
  }, []);

  // Setup keyboard shortcuts
  useSidebarShortcuts({
    onToggleRight: toggleRightSidebar,
    enabled: true,
  });

  return (
    <div className="flex h-screen overflow-hidden bg-background">
      {/* Left: Workspace Sidebar */}
      <LeftSidebar />

      {/* Center: Main Content Area */}
      <MainContent
        isRightSidebarExpanded={rightState === "expanded"}
        onToggleRightSidebar={toggleRightSidebar}
      >
        <Outlet />
      </MainContent>

      {/* Right: Agent Panel */}
      <RightSidebar
        state={rightState}
        onStateChange={handleRightStateChange}
      />
    </div>
  );
}