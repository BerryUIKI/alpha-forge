/**
 * LeftSidebar Component
 *
 * Main container for the left collapsible sidebar (Module A).
 * Combines WorkspaceSelector, ScrollableList, and UserOperations.
 *
 * Features:
 * - Collapsible sidebar with expand/collapse animation
 * - Drag-to-resize functionality (TODO: [GUI-M1-1])
 * - Responsive width constraints
 *
 * TODO: [GUI-M1-1] Implement drag-to-resize functionality
 * TODO: [GUI-M1-1] Persist sidebar state across sessions
 * TODO: [GUI-M1-1] Add keyboard shortcuts for collapse/expand
 */

import { useState, useCallback } from "react";
import { ChevronLeft } from "lucide-react";
import type { LeftSidebarProps } from "../types";
import { DEFAULT_SIDEBAR_WIDTHS } from "../types";
import { WorkspaceSelector } from "./WorkspaceSelector";
import { ScrollableList } from "./ScrollableList";
import { UserOperations } from "./UserOperations";

export function LeftSidebar({
  state = "expanded",
  onStateChange,
  selectedWorkspace = "analyze",
  onWorkspaceChange,
  defaultWidth = DEFAULT_SIDEBAR_WIDTHS.left.default,
  minWidth = DEFAULT_SIDEBAR_WIDTHS.left.min,
  maxWidth = DEFAULT_SIDEBAR_WIDTHS.left.max,
}: LeftSidebarProps) {
  const [internalState, setInternalState] = useState(state);
  const [selectedId, setSelectedId] = useState<string | undefined>(undefined);

  const isExpanded = internalState === "expanded";

  const handleToggle = useCallback(() => {
    const newState = isExpanded ? "collapsed" : "expanded";
    setInternalState(newState);
    onStateChange?.(newState);

    // TODO: [GUI-M1-1] Persist sidebar state to local storage
  }, [isExpanded, onStateChange]);

  const handleWorkspaceChange = useCallback((workspace: typeof selectedWorkspace) => {
    onWorkspaceChange?.(workspace);
    // TODO: [GUI-M1-4] Trigger workspace view switching in MainContent
  }, [onWorkspaceChange]);

  const handleMenuItemClick = useCallback((item: "profile" | "theme-toggle" | "settings") => {
    // TODO: [GUI-M1-1] Handle user menu actions
    console.log(`Menu item clicked: ${item}`);
  }, []);

  if (!isExpanded) {
    // Collapsed state - minimal UI
    return (
      <aside
        className="flex h-full flex-col border-r border-border bg-card"
        style={{ width: "48px" }}
        aria-label="Left sidebar (collapsed)"
      >
        {/* Expand Button */}
        <button
          onClick={handleToggle}
          className="flex h-12 items-center justify-center border-b border-border transition-colors hover:bg-accent"
          aria-label="Expand sidebar"
          title="Expand sidebar"
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

  // Expanded state
  return (
    <aside
      className="flex h-full flex-col border-r border-border bg-card"
      style={{ width: `${defaultWidth}px`, minWidth: `${minWidth}px`, maxWidth: `${maxWidth}px` }}
      aria-label="Left sidebar"
    >
      {/* Collapse Button */}
      <div className="flex items-center justify-end border-b border-border p-2">
        <button
          onClick={handleToggle}
          className="flex h-8 w-8 items-center justify-center rounded-lg transition-colors hover:bg-accent"
          aria-label="Collapse sidebar"
          title="Collapse sidebar"
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

      {/* TODO: [GUI-M1-1] Add drag-to-resize handle */}
      {/* TODO: [GUI-M1-1] Implement resize logic with mouse events */}
    </aside>
  );
}