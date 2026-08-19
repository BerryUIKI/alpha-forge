/**
 * MainLayout Component
 *
 * Primary layout orchestrator for the Investment OS desktop application.
 * Combines three zones:
 * - Left: Collapsible navigation sidebar
 * - Center: Main content area with TopBar, routed pages, StatusBar
 * - Right: Collapsible Agent panel
 *
 * @version GUI-M0
 */

import { useState, useCallback } from "react";
import { Outlet } from "react-router-dom";
import { LeftSidebar } from "./LeftSidebar";
import { MainContent } from "./MainContent";
import { RightSidebar } from "./RightSidebar";
import { GlobalSearchDialog } from "@/features/search";
import { useSidebarShortcuts, useKeyboardShortcut } from "@/hooks/layout";
import type { SidebarState } from "./types";

export function MainLayout() {
  // Right sidebar state (Agent panel)
  const [rightState, setRightState] = useState<SidebarState>("collapsed");
  // Global search palette state
  const [searchOpen, setSearchOpen] = useState(false);

  const handleRightStateChange = useCallback((state: SidebarState) => {
    setRightState(state);
  }, []);

  const toggleRightSidebar = useCallback(() => {
    setRightState((prev) => (prev === "expanded" ? "collapsed" : "expanded"));
  }, []);

  const openSearch = useCallback(() => setSearchOpen(true), []);
  const closeSearch = useCallback(() => setSearchOpen(false), []);

  // Setup keyboard shortcuts
  useSidebarShortcuts({
    onToggleRight: toggleRightSidebar,
    enabled: true,
  });

  // Cmd/Ctrl+K: open global search
  useKeyboardShortcut({
    key: "k",
    modifiers: ["ctrl"],
    callback: openSearch,
  });

  return (
    <div className="flex h-screen overflow-hidden bg-background">
      {/* Left: Navigation Sidebar */}
      <LeftSidebar />

      {/* Center: Main Content Area */}
      <MainContent
        isRightSidebarExpanded={rightState === "expanded"}
        onToggleRightSidebar={toggleRightSidebar}
        onOpenSearch={openSearch}
      >
        <Outlet />
      </MainContent>

      {/* Right: Agent Panel */}
      <RightSidebar
        state={rightState}
        onStateChange={handleRightStateChange}
      />

      {/* Global Search Palette */}
      <GlobalSearchDialog isOpen={searchOpen} onClose={closeSearch} />
    </div>
  );
}