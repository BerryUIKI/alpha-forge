/**
 * MainLayout Component
 *
 * Primary layout orchestrator for the Investment OS desktop application.
 * Combines three zones:
 * - Left: Navigation sidebar with functional views and tools
 * - Center: Router outlet for page content
 * - Right: Collapsible Agent panel
 *
 * @version GUI-M2
 */

import { useState, useCallback } from "react";
import { Outlet } from "react-router-dom";
import { LeftSidebar } from "@/components/layout/LeftSidebar";
import { RightSidebar } from "@/components/layout/RightSidebar";
import { useSidebarShortcuts } from "@/hooks/layout";
import type { SidebarState } from "./types";

export function MainLayout() {
  // Left sidebar state
  const [leftState, setLeftState] = useState<SidebarState>("expanded");

  // Right sidebar state (Agent panel)
  const [rightState, setRightState] = useState<SidebarState>("collapsed");

  const handleLeftStateChange = useCallback((state: SidebarState) => {
    setLeftState(state);
  }, []);

  const handleRightStateChange = useCallback((state: SidebarState) => {
    setRightState(state);
  }, []);

  const toggleLeftSidebar = useCallback(() => {
    setLeftState((prev) => (prev === "expanded" ? "collapsed" : "expanded"));
  }, []);

  const toggleRightSidebar = useCallback(() => {
    setRightState((prev) => (prev === "expanded" ? "collapsed" : "expanded"));
  }, []);

  // Setup keyboard shortcuts
  useSidebarShortcuts({
    onToggleLeft: toggleLeftSidebar,
    onToggleRight: toggleRightSidebar,
    enabled: true,
  });

  return (
    <div className="flex h-screen overflow-hidden bg-background">
      {/* Left: Navigation Sidebar */}
      <LeftSidebar
        state={leftState}
        onStateChange={handleLeftStateChange}
      />

      {/* Center: Routed Pages */}
      <div className="flex flex-1 flex-col overflow-hidden">
        <Outlet />
      </div>

      {/* Right: Agent Panel */}
      <RightSidebar
        state={rightState}
        onStateChange={handleRightStateChange}
      />
    </div>
  );
}