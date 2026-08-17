/**
 * LeftSidebar Component
 *
 * Simplified navigation sidebar with collapsible groups:
 * - Workspace: Dashboard, Research, Theses, Portfolio, Knowledge, Journal
 * - Tools: Options, Artifacts
 * - Account: Settings
 *
 * Features:
 * - Collapsible with smooth width animation (220px ↔ 64px)
 * - Drag-to-resize functionality
 * - State persistence via localStorage
 * - Active route highlighting
 * - Keyboard shortcuts (Ctrl+1)
 *
 * @version GUI-M0
 */

import { useCallback } from "react";
import { ChevronLeft, GripVertical, LayoutDashboard, Search, FileText, Briefcase, BookOpen, BookMarked, LineChart, Puzzle, Settings } from "lucide-react";
import { NavItem } from "./NavItem";
import { NavGroup } from "./NavGroup";
import { useSidebarState, useResize } from "@/hooks/layout";
import { useLocale } from "@/lib/i18n/useLocale";
import { cn } from "@/lib/utils";
import type { LeftSidebarProps, NavGroup as NavGroupType } from "../types";
import { DEFAULT_SIDEBAR_WIDTHS } from "../types";

export function LeftSidebar({
  state: externalState,
  onStateChange,
  defaultWidth = DEFAULT_SIDEBAR_WIDTHS.left.default,
  minWidth = DEFAULT_SIDEBAR_WIDTHS.left.min,
  maxWidth = DEFAULT_SIDEBAR_WIDTHS.left.max,
}: LeftSidebarProps) {
  const { t } = useLocale();

  // Navigation configuration with i18n labels
  const NAV_GROUPS: NavGroupType[] = [
    {
      id: "workspace",
      label: t("navWorkspace"),
      items: [
        { id: "dashboard", label: t("navDashboard"), icon: LayoutDashboard, route: "/" },
        { id: "research", label: t("navResearch"), icon: Search, route: "/research" },
        { id: "theses", label: t("navTheses"), icon: FileText, route: "/theses" },
        { id: "portfolio", label: t("navPortfolio"), icon: Briefcase, route: "/portfolio" },
        { id: "knowledge", label: t("navKnowledge"), icon: BookOpen, route: "/knowledge" },
        { id: "journal", label: t("navJournal"), icon: BookMarked, route: "/journal" },
      ],
    },
    {
      id: "tools",
      label: t("navTools"),
      items: [
        { id: "options", label: t("navOptions"), icon: LineChart, route: "/options" },
        { id: "artifacts", label: t("navArtifacts"), icon: Puzzle, route: "/artifacts" },
      ],
    },
    {
      id: "account",
      label: t("navAccount"),
      items: [
        { id: "settings", label: t("navSettings"), icon: Settings, route: "/settings" },
      ],
    },
  ];

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

  if (!isExpanded) {
    // Collapsed state - minimal UI with smooth animation
    return (
      <aside
        className="flex h-full flex-col border-r border-border bg-card transition-[width] duration-300 ease-in-out"
        style={{ width: "64px" }}
        aria-label="Left sidebar (collapsed)"
      >
        {/* Expand Button */}
        <button
          onClick={handleToggle}
          className="flex h-14 items-center justify-center border-b border-border transition-colors hover:bg-accent"
          aria-label="Expand sidebar"
          title="Expand sidebar (Ctrl+1)"
        >
          <ChevronLeft className="h-5 w-5 rotate-180" />
        </button>

        {/* Navigation items (icons only) */}
        <nav className="flex flex-1 flex-col gap-2 overflow-y-auto p-2">
          {NAV_GROUPS.map((group) => (
            <div key={group.id} className="flex flex-col items-center gap-0.5">
              {group.items.map((item) => (
                <NavItem key={item.id} item={item} collapsed={true} />
              ))}
            </div>
          ))}
        </nav>
      </aside>
    );
  }

  // Expanded state with resize handle
  return (
    <aside
      className={cn(
        "relative flex h-full flex-col border-r border-border bg-card transition-[width] duration-300 ease-in-out",
        isResizing && "select-none",
      )}
      style={{ width: `${width}px`, minWidth: `${minWidth}px`, maxWidth: `${maxWidth}px` }}
      aria-label="Left sidebar"
    >
      {/* Header with logo and collapse button */}
      <div className="flex items-center justify-between border-b border-border px-4 h-14">
        <div className="flex items-center gap-3">
          <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-gradient-to-br from-indigo-400 to-purple-500 text-sm font-bold text-white">
            α
          </div>
          <span className="text-base font-bold tracking-tight">AlphaForge</span>
        </div>
        <button
          onClick={handleToggle}
          className="flex h-7 w-7 items-center justify-center rounded-lg transition-colors hover:bg-accent"
          aria-label="Collapse sidebar"
          title="Collapse sidebar (Ctrl+1)"
        >
          <ChevronLeft className="h-4 w-4" />
        </button>
      </div>

      {/* Navigation */}
      <nav className="flex-1 overflow-y-auto p-2">
        {NAV_GROUPS.map((group) => (
          <NavGroup key={group.id} label={group.label} collapsed={false}>
            {group.items.map((item) => (
              <NavItem key={item.id} item={item} collapsed={false} />
            ))}
          </NavGroup>
        ))}
      </nav>

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