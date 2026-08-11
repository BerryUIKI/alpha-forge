/**
 * LeftSidebar Component
 *
 * Main container for the left collapsible sidebar with new structure:
 * - Top: Functional View Selector (功能视图下拉)
 * - Middle: Tools List (工具列表)
 * - Bottom: User Operations (用户菜单)
 *
 * Features:
 * - Collapsible sidebar with smooth animation
 * - Drag-to-resize functionality
 * - State persistence via localStorage
 * - Keyboard shortcuts support (Ctrl+1)
 *
 * @version GUI-M3
 */

import { useCallback } from "react";
import { ChevronLeft, GripVertical } from "lucide-react";
import { FunctionalViewSelector } from "./FunctionalViewSelector";
import { ToolsList } from "./ToolsList";
import { UserOperations } from "./UserOperations";
import { useSidebarState, useResize } from "@/hooks/layout";
import type { LeftSidebarProps, UserMenuItem } from "../types";
import { DEFAULT_SIDEBAR_WIDTHS } from "../types";

export function LeftSidebar({
  state: externalState,
  onStateChange,
  defaultWidth = DEFAULT_SIDEBAR_WIDTHS.left.default,
  minWidth = DEFAULT_SIDEBAR_WIDTHS.left.min,
  maxWidth = DEFAULT_SIDEBAR_WIDTHS.left.max,
}: LeftSidebarProps) {
  // Use sidebar state hook for persistence
  const {
    width,
    toggleState,
    setWidth,
    isExpanded,
  } = useSidebarState({
    storageKey: "left-sidebar",
    defaultState: externalState || "expanded",
    defaultWidth,
    minWidth,
    maxWidth,
  });

  // Use resize hook for drag-to-resize
  const { isResizing, startResize } = useResize({
    initialWidth: width,
    minWidth,
    maxWidth,
    direction: "right",
    onWidthChange: setWidth,
  });

  const handleToggle = useCallback(() => {
    toggleState();
    onStateChange?.(isExpanded ? "collapsed" : "expanded");
  }, [toggleState, isExpanded, onStateChange]);

  const handleMenuItemClick = useCallback((item: UserMenuItem) => {
    // Menu item click handling is done in UserOperations component
    console.log(`Menu item clicked: ${item}`);
  }, []);

  if (!isExpanded) {
    // Collapsed state - minimal UI with smooth animation
    return (
      <aside
        className="flex h-full flex-col border-r border-border bg-card transition-all duration-300 ease-in-out"
        style={{ width: "48px" }}
        aria-label="Left sidebar (collapsed)"
      >
        {/* Expand Button */}
        <button
          onClick={handleToggle}
          className="flex h-12 items-center justify-center border-b border-border transition-colors hover:bg-accent"
          aria-label="Expand sidebar"
          title="Expand sidebar (Ctrl+1)"
        >
          <ChevronLeft className="h-5 w-5 rotate-180" />
        </button>

        {/* Collapsed indicator */}
        <div className="flex flex-1 items-center justify-center">
          <span className="text-xs text-muted-foreground">☰</span>
        </div>
      </aside>
    );
  }

  // Expanded state with resize handle
  return (
    <aside
      className={`relative flex h-full flex-col border-r border-border bg-card transition-all duration-300 ease-in-out ${
        isResizing ? "select-none" : ""
      }`}
      style={{ width: `${width}px`, minWidth: `${minWidth}px`, maxWidth: `${maxWidth}px` }}
      aria-label="Left sidebar"
    >
      {/* Collapse Button */}
      <div className="flex items-center justify-end border-b border-border p-2">
        <button
          onClick={handleToggle}
          className="flex h-8 w-8 items-center justify-center rounded-lg transition-colors hover:bg-accent"
          aria-label="Collapse sidebar"
          title="Collapse sidebar (Ctrl+1)"
        >
          <ChevronLeft className="h-4 w-4" />
        </button>
      </div>

      {/* Top: Functional View Selector */}
      <div className="border-b border-border p-2">
        <FunctionalViewSelector />
      </div>

      {/* Middle: Tools List (scrollable) */}
      <ToolsList />

      {/* Bottom: User Operations (fixed position) */}
      <UserOperations
        username="Investor"
        onMenuItemClick={handleMenuItemClick}
      />

      {/* Drag-to-resize handle */}
      <div
        className="absolute right-0 top-0 h-full w-1 cursor-col-resize hover:bg-primary/20 active:bg-primary/40 transition-colors group"
        onMouseDown={startResize}
        role="separator"
        aria-orientation="vertical"
        aria-label="Resize sidebar"
        tabIndex={0}
      >
        <div className="absolute right-0 top-1/2 -translate-y-1/2 opacity-0 group-hover:opacity-100 transition-opacity">
          <GripVertical className="h-4 w-4 text-muted-foreground" />
        </div>
      </div>
    </aside>
  );
}
