/**
 * LeftSidebar Component
 *
 * Main container for the left sidebar with three-section layout:
 * - Section 1: Functional View Selector
 * - Section 2: Dynamic Tools List (scrollable)
 * - Section 3: User Operations (theme, language, settings)
 *
 * Features:
 * - Default width: 240px, resizable 180-400px
 * - Collapse/expand with smooth animation
 * - Drag-to-resize functionality
 * - State persistence via localStorage
 *
 * @version GUI-M2
 */

import { useState, useCallback } from "react";
import { useTheme } from "next-themes";
import { ChevronLeft, GripVertical } from "lucide-react";
import type { FunctionalView, LeftSidebarProps } from "../types";
import { DEFAULT_SIDEBAR_WIDTHS } from "../types";
import { FunctionalViewSelector } from "./FunctionalViewSelector";
import { ToolsList } from "./ToolsList";
import { UserOperations } from "./UserOperations";
import { useSidebarState, useResize } from "@/hooks/layout";
import { DEFAULT_FUNCTIONAL_VIEW } from "@/config/tools-config";

export function LeftSidebar({
  state: externalState,
  onStateChange,
  defaultWidth = DEFAULT_SIDEBAR_WIDTHS.left.default,
  minWidth = DEFAULT_SIDEBAR_WIDTHS.left.min,
  maxWidth = DEFAULT_SIDEBAR_WIDTHS.left.max,
}: LeftSidebarProps) {
  const { theme, setTheme } = useTheme();
  const [activeView, setActiveView] = useState<FunctionalView>(DEFAULT_FUNCTIONAL_VIEW);

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

  const handleToggle = useCallback(() => {
    toggleState();
    onStateChange?.(isExpanded ? "collapsed" : "expanded");
  }, [toggleState, isExpanded, onStateChange]);

  const handleThemeChange = useCallback(
    (newTheme: "light" | "dark") => {
      setTheme(newTheme);
    },
    [setTheme]
  );

  const handleViewChange = useCallback((view: FunctionalView) => {
    setActiveView(view);
  }, []);

  if (!isExpanded) {
    // Collapsed state - minimal UI
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

        {/* Collapsed icon indicator */}
        <div className="flex flex-1 items-center justify-center">
          <span className="text-lg">📊</span>
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
      {/* Header with collapse button */}
      <div className="flex items-center justify-between border-b border-border p-3">
        <h2 className="text-sm font-semibold">Investment OS</h2>
        <button
          onClick={handleToggle}
          className="flex h-8 w-8 items-center justify-center rounded-lg transition-colors hover:bg-accent"
          aria-label="Collapse sidebar"
          title="Collapse sidebar (Ctrl+1)"
        >
          <ChevronLeft className="h-4 w-4" />
        </button>
      </div>

      {/* Section 1: Functional View Selector */}
      <FunctionalViewSelector value={activeView} onChange={handleViewChange} />

      {/* Section 2: Tools List (scrollable) */}
      <ToolsList activeView={activeView} />

      {/* Section 3: User Operations */}
      <UserOperations
        theme={theme as "light" | "dark"}
        onThemeChange={handleThemeChange}
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