/**
 * MainContent Component
 *
 * Main content area container combining OperationBar, page content, and StatusBar.
 * Renders routed pages via children (Outlet).
 *
 * @version GUI-M1-4
 */

import type { MainContentProps } from "../types";
import { OperationBar } from "./OperationBar";
import { StatusBar } from "./StatusBar";

export function MainContent({
  children,
  isRightSidebarExpanded = false,
  onToggleRightSidebar,
}: MainContentProps) {
  return (
    <div className="flex flex-1 flex-col overflow-hidden">
      {/* Top: Operation Bar */}
      <OperationBar
        isRightSidebarExpanded={isRightSidebarExpanded}
        onToggleRightSidebar={onToggleRightSidebar}
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