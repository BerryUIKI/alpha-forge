/**
 * MainContent Component
 *
 * Main content area container combining TopBar, page content, and StatusBar.
 * Renders routed pages via children (Outlet).
 *
 * @version GUI-M0
 */

import type { MainContentProps } from "../types";
import { TopBar } from "../TopBar";
import { StatusBar } from "./StatusBar";

export function MainContent({
  children,
  isRightSidebarExpanded = false,
  onToggleRightSidebar,
  onOpenSearch,
}: MainContentProps) {
  return (
    <div className="flex flex-1 flex-col overflow-hidden">
      {/* Top: Top Bar */}
      <TopBar
        isRightSidebarExpanded={isRightSidebarExpanded}
        onToggleRightSidebar={onToggleRightSidebar}
        onOpenSearch={onOpenSearch}
      />

      {/* Middle: Page Content (Outlet) */}
      <div className="flex-1 overflow-auto">
        {children}
      </div>

      {/* Bottom: Status Bar */}
      <StatusBar />
    </div>
  );
}