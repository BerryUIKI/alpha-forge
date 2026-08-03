/**
 * RightSidebar Component
 *
 * Main container for the right collapsible sidebar (Module C - Agent panel).
 * Mirrors left sidebar interaction patterns with persistence and resize.
 *
 * Features:
 * - Collapsible sidebar with smooth animation
 * - Drag-to-resize functionality
 * - State persistence via localStorage
 * - Synchronized toggle with MainContent operation bar
 *
 * @version GUI-M1-1
 */

import { useCallback } from "react";
import { ChevronRight, GripVertical } from "lucide-react";
import type { RightSidebarProps } from "../types";
import { DEFAULT_SIDEBAR_WIDTHS } from "../types";
import { AgentPanel } from "./AgentPanel";
import { useSidebarState, useResize } from "@/hooks/layout";

export function RightSidebar({
  state: externalState,
  onStateChange,
  defaultWidth = DEFAULT_SIDEBAR_WIDTHS.right.default,
  minWidth = DEFAULT_SIDEBAR_WIDTHS.right.min,
  maxWidth = DEFAULT_SIDEBAR_WIDTHS.right.max,
}: RightSidebarProps) {
  // Use sidebar state hook for persistence
  const {
    state: internalState,
    width,
    toggleState,
    setWidth,
    isExpanded,
  } = useSidebarState({
    storageKey: "right-sidebar",
    defaultState: externalState || "collapsed",
    defaultWidth,
    minWidth,
    maxWidth,
  });

  // Use resize hook for drag-to-resize
  const { isResizing, startResize } = useResize({
    initialWidth: width,
    minWidth,
    maxWidth,
    direction: "left",
    onWidthChange: setWidth,
  });

  const handleToggle = useCallback(() => {
    toggleState();
    onStateChange?.(isExpanded ? "collapsed" : "expanded");
  }, [toggleState, isExpanded, onStateChange]);

  if (!isExpanded) {
    // Collapsed state - minimal UI (just toggle button)
    return (
      <aside
        className="flex h-full flex-col border-l border-border bg-card transition-all duration-300 ease-in-out"
        style={{ width: "48px" }}
        aria-label="Right sidebar (collapsed)"
      >
        {/* Expand Button */}
        <button
          onClick={handleToggle}
          className="flex h-12 items-center justify-center border-b border-border transition-colors hover:bg-accent"
          aria-label="Expand agent sidebar"
          title="Expand agent sidebar (Ctrl+2)"
        >
          <ChevronRight className="h-5 w-5" />
        </button>

        {/* Collapsed agent indicator */}
        <div className="flex flex-1 items-center justify-center">
          <span className="text-lg">🤖</span>
        </div>
      </aside>
    );
  }

  // Expanded state with resize handle
  return (
    <aside
      className={`relative flex h-full flex-col border-l border-border bg-card transition-all duration-300 ease-in-out ${
        isResizing ? "select-none" : ""
      }`}
      style={{ width: `${width}px`, minWidth: `${minWidth}px`, maxWidth: `${maxWidth}px` }}
      aria-label="Right sidebar (Agent panel)"
    >
      {/* Collapse Button */}
      <div className="flex items-center justify-start border-b border-border p-2">
        <button
          onClick={handleToggle}
          className="flex h-8 w-8 items-center justify-center rounded-lg transition-colors hover:bg-accent"
          aria-label="Collapse agent sidebar"
          title="Collapse agent sidebar (Ctrl+2)"
        >
          <ChevronRight className="h-4 w-4 rotate-180" />
        </button>
      </div>

      {/* Agent Panel Content */}
      <AgentPanel
        status="Ready to assist"
        placeholder="Agent capabilities coming soon"
      />

      {/* Drag-to-resize handle */}
      <div
        className="absolute left-0 top-0 h-full w-1 cursor-col-resize hover:bg-primary/20 active:bg-primary/40 transition-colors group"
        onMouseDown={startResize}
        role="separator"
        aria-orientation="vertical"
        aria-label="Resize sidebar"
        tabIndex={0}
      >
        <div className="absolute left-0 top-1/2 -translate-y-1/2 opacity-0 group-hover:opacity-100 transition-opacity">
          <GripVertical className="h-4 w-4 text-muted-foreground" />
        </div>
      </div>
    </aside>
  );
}