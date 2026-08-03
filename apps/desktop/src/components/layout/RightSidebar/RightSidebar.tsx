/**
 * RightSidebar Component
 *
 * Main container for the right collapsible sidebar (Module C - Agent panel).
 * Mirrors left sidebar interaction patterns.
 *
 * Features:
 * - Collapsible sidebar with expand/collapse animation
 * - Drag-to-resize functionality (TODO: [GUI-M1-3])
 * - Synchronized toggle with MainContent operation bar
 *
 * TODO: [GUI-M1-3] Implement drag-to-resize functionality
 * TODO: [GUI-M1-3] Persist sidebar state across sessions
 * TODO: [GUI-M1-3] Add keyboard shortcuts for collapse/expand
 */

import { useState, useCallback } from "react";
import { ChevronRight } from "lucide-react";
import type { RightSidebarProps } from "../types";
import { DEFAULT_SIDEBAR_WIDTHS } from "../types";
import { AgentPanel } from "./AgentPanel";

export function RightSidebar({
  state = "collapsed",
  onStateChange,
  defaultWidth = DEFAULT_SIDEBAR_WIDTHS.right.default,
  minWidth = DEFAULT_SIDEBAR_WIDTHS.right.min,
  maxWidth = DEFAULT_SIDEBAR_WIDTHS.right.max,
}: RightSidebarProps) {
  const [internalState, setInternalState] = useState(state);

  const isExpanded = internalState === "expanded";

  const handleToggle = useCallback(() => {
    const newState = isExpanded ? "collapsed" : "expanded";
    setInternalState(newState);
    onStateChange?.(newState);

    // TODO: [GUI-M1-3] Persist sidebar state to local storage
  }, [isExpanded, onStateChange]);

  if (!isExpanded) {
    // Collapsed state - minimal UI (just toggle button)
    return (
      <aside
        className="flex h-full flex-col border-l border-border bg-card"
        style={{ width: "48px" }}
        aria-label="Right sidebar (collapsed)"
      >
        {/* Expand Button */}
        <button
          onClick={handleToggle}
          className="flex h-12 items-center justify-center border-b border-border transition-colors hover:bg-accent"
          aria-label="Expand agent sidebar"
          title="Expand agent sidebar"
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

  // Expanded state
  return (
    <aside
      className="flex h-full flex-col border-l border-border bg-card"
      style={{ width: `${defaultWidth}px`, minWidth: `${minWidth}px`, maxWidth: `${maxWidth}px` }}
      aria-label="Right sidebar (Agent panel)"
    >
      {/* Collapse Button */}
      <div className="flex items-center justify-start border-b border-border p-2">
        <button
          onClick={handleToggle}
          className="flex h-8 w-8 items-center justify-center rounded-lg transition-colors hover:bg-accent"
          aria-label="Collapse agent sidebar"
          title="Collapse agent sidebar"
        >
          <ChevronRight className="h-4 w-4 rotate-180" />
        </button>
      </div>

      {/* Agent Panel Content */}
      <AgentPanel
        status="Ready to assist"
        placeholder="Agent capabilities coming soon"
      />

      {/* TODO: [GUI-M1-3] Add drag-to-resize handle */}
      {/* TODO: [GUI-M1-3] Implement resize logic with mouse events */}
    </aside>
  );
}