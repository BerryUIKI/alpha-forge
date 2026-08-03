/**
 * MainLayout Component
 *
 * Primary layout orchestrator for the Investment OS desktop application.
 * Combines three zones:
 * - Left: Navigation sidebar with route-based icons
 * - Center: Router outlet for page content
 * - Right: Collapsible Agent panel
 *
 * @version GUI-M2
 */

import { useState, useCallback } from "react";
import { Outlet } from "react-router-dom";
import { Sidebar } from "@/components/navigation/Sidebar";
import { RightSidebar } from "@/components/layout/RightSidebar";
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
      {/* Left: Navigation Sidebar */}
      <Sidebar />

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