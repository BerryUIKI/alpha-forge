/**
 * LeftSidebar Component
 *
 * Main container for the left collapsible sidebar (Module A).
 * Combines WorkspaceSelector, ScrollableList, and UserOperations.
 *
 * Features:
 * - Collapsible sidebar with smooth animation
 * - Drag-to-resize functionality
 * - State persistence via localStorage
 * - Keyboard shortcuts support
 *
 * @version GUI-M1-1
 */

import { useCallback, useState } from "react";
import { ChevronLeft, GripVertical } from "lucide-react";
import type { LeftSidebarProps, UserMenuItem } from "../types";
import { DEFAULT_SIDEBAR_WIDTHS } from "../types";
import { WorkspaceSelector } from "./WorkspaceSelector";
import { ScrollableList } from "./ScrollableList";
import { UserOperations } from "./UserOperations";
import { useSidebarState, useResize } from "@/hooks/layout";

export function LeftSidebar({
  state: externalState,
  onStateChange,
  selectedWorkspace = "analyze",
  onWorkspaceChange,
  defaultWidth = DEFAULT_SIDEBAR_WIDTHS.left.default,
  minWidth = DEFAULT_SIDEBAR_WIDTHS.left.min,
  maxWidth = DEFAULT_SIDEBAR_WIDTHS.left.max,
}: LeftSidebarProps) {
  // Use sidebar state hook for persistence
  const {
    state: internalState,
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

  const [selectedId, setSelectedId] = useState<string | undefined>(undefined);

  const handleToggle = useCallback(() => {
    toggleState();
    onStateChange?.(isExpanded ? "collapsed" : "expanded");
  }, [toggleState, isExpanded, onStateChange]);

  const handleWorkspaceChange = useCallback((workspace: typeof selectedWorkspace) => {
    onWorkspaceChange?.(workspace);
  }, [onWorkspaceChange]);

  const handleMenuItemClick = useCallback((item: UserMenuItem) => {
    // TODO: [GUI-M1-1] Handle user menu actions
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

        {/* Collapsed workspace indicator */}
        <div className="flex flex-1 items-center justify-center">
          <span className="text-xs text-muted-foreground">
            {selectedWorkspace.charAt(0).toUpperCase()}
          </span>
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

      {/* Top: Workspace Selector */}
      <WorkspaceSelector
        selected={selectedWorkspace}
        onSelect={handleWorkspaceChange}
      />

      {/* Middle: Scrollable List */}
      <ScrollableList
        selectedId={selectedId}
        onSelect={setSelectedId}
        emptyMessage="No projects or sessions"
      />

      {/* Bottom: User Operations (fixed position) */}
      <UserOperations
        username="Investor"
        onMenuItemClick={handleMenuItemClick}
        theme="light"
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