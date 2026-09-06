/**
 * RightSidebar Component
 *
 * Main container for the right collapsible sidebar (Agent Copilot drawer).
 * Mirrors left sidebar interaction patterns with persistence and resize.
 *
 * Features:
 * - Collapsible drawer with smooth animation
 * - Drag-to-resize functionality with localStorage persistence
 * - Modern AI copilot presence with live status pulsing
 * - Shortcuts: ⌘J / Ctrl+2
 * - 100% standard pure i18n
 */

import { useCallback } from "react";
import { ChevronRight, GripVertical, Sparkles } from "lucide-react";
import { cn, isMacPlatform } from "@/lib/utils";
import { useLocale } from "@/lib/i18n/useLocale";
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
  const { t } = useLocale();
  const isMac = isMacPlatform();

  // Use sidebar state hook for persistence
  const {
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
    // Collapsed state - minimal UI with sleek AI Copilot presence
    return (
      <aside
        className="flex h-full flex-col border-l border-border bg-card transition-[width] duration-300 ease-in-out"
        style={{ width: "48px" }}
        aria-label="Right sidebar (collapsed)"
      >
        {/* Expand Button */}
        <button
          onClick={handleToggle}
          className="flex h-14 items-center justify-center border-b border-border text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
          aria-label={t("openAgentPanel")}
          title={`${t("openAgentPanel")} (${isMac ? "⌘J" : "Ctrl+2"})`}
        >
          <ChevronRight className="h-5 w-5" />
        </button>

        {/* Collapsed agent indicator */}
        <button
          type="button"
          onClick={handleToggle}
          className="flex flex-1 flex-col items-center justify-center gap-2 text-muted-foreground transition-colors hover:text-primary"
          aria-label={t("openAgentPanel")}
          title={`${t("openAgentPanel")} (${isMac ? "⌘J" : "Ctrl+2"})`}
        >
          <div className="relative flex h-8 w-8 items-center justify-center rounded-xl bg-primary/10 text-primary transition-transform hover:scale-110">
            <Sparkles className="h-4 w-4" />
            <span className="absolute -top-0.5 -right-0.5 flex h-2 w-2">
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
              <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
            </span>
          </div>
        </button>
      </aside>
    );
  }

  // Expanded state with resize handle
  return (
    <aside
      className={cn(
        "relative flex h-full flex-col border-l border-border bg-card transition-[width] duration-300 ease-in-out",
        isResizing && "select-none",
      )}
      style={{ width: `${width}px`, minWidth: `${minWidth}px`, maxWidth: `${maxWidth}px` }}
      aria-label="Right sidebar (Agent panel)"
    >
      {/* Header with collapse button */}
      <div className="flex items-center justify-between border-b border-border pl-4 pr-2 h-14">
        <h3 className="flex items-center gap-2 text-sm font-semibold">
          <span className="relative flex h-2 w-2">
            <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
            <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
          </span>
          <span>Agent</span>
        </h3>
        <button
          onClick={handleToggle}
          className="flex h-7 w-7 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
          aria-label={t("closeAgentPanel")}
          title={`${t("closeAgentPanel")} (${isMac ? "⌘J" : "Ctrl+2"})`}
        >
          <ChevronRight className="h-4 w-4 rotate-180" />
        </button>
      </div>

      {/* Agent Panel Content */}
      <AgentPanel />

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
