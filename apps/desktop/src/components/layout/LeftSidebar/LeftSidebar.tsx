/**
 * LeftSidebar Component
 *
 * Core navigation sidebar aligned with the AlphaForge investment loop:
 * - Cockpit: Dashboard
 * - Core Pipeline: Research, Theses, Portfolio, Journal
 * - Knowledge & Tools: Knowledge Base, Options, Artifacts
 *
 * Features:
 * - Smooth collapsible animation (220px <-> 64px)
 * - Drag-to-resize functionality with localStorage persistence
 * - Real-time Agent readiness pulse status indicator
 * - 100% standard pure i18n
 */

import { useEffect } from "react";
import {
  GripVertical,
  LayoutDashboard,
  Search,
  FileText,
  Briefcase,
  BookOpen,
  BookMarked,
  LineChart,
  Puzzle,
} from "lucide-react";
import { NavItem } from "./NavItem";
import { NavGroup } from "./NavGroup";
import { useSidebarState, useResize } from "@/hooks/layout";
import { useLocale } from "@/lib/i18n/useLocale";
import { cn } from "@/lib/utils";
import type { LeftSidebarProps, NavGroup as NavGroupType } from "../types";
import { DEFAULT_SIDEBAR_WIDTHS } from "../types";
import { AccountMenu } from "./AccountMenu";

export function LeftSidebar({
  state: externalState,
  defaultWidth = DEFAULT_SIDEBAR_WIDTHS.left.default,
  minWidth = DEFAULT_SIDEBAR_WIDTHS.left.min,
  maxWidth = DEFAULT_SIDEBAR_WIDTHS.left.max,
}: LeftSidebarProps) {
  const { t } = useLocale();

  // Navigation configuration aligned with AlphaForge product loop
  const NAV_GROUPS: NavGroupType[] = [
    {
      id: "cockpit",
      label: t("navCockpit"),
      items: [
        { id: "dashboard", label: t("navDashboard"), icon: LayoutDashboard, route: "/" },
      ],
    },
    {
      id: "pipeline",
      label: t("navPipeline"),
      items: [
        { id: "research", label: t("navResearch"), icon: Search, route: "/research" },
        { id: "theses", label: t("navTheses"), icon: FileText, route: "/theses" },
        { id: "portfolio", label: t("navPortfolio"), icon: Briefcase, route: "/portfolio" },
        { id: "journal", label: t("navJournal"), icon: BookMarked, route: "/journal" },
      ],
    },
    {
      id: "tools",
      label: t("navKnowledgeTools"),
      items: [
        { id: "knowledge", label: t("navKnowledge"), icon: BookOpen, route: "/knowledge" },
        { id: "options", label: t("navOptions"), icon: LineChart, route: "/options" },
        { id: "artifacts", label: t("navArtifacts"), icon: Puzzle, route: "/artifacts" },
      ],
    },
  ];

  // Use sidebar state hook for persistence
  const {
    width,
    setWidth,
    isExpanded,
    setState,
  } = useSidebarState({
    storageKey: "left-sidebar",
    defaultState: externalState || "expanded",
    defaultWidth,
    minWidth,
    maxWidth,
  });

  useEffect(() => {
    if (externalState) setState(externalState);
  }, [externalState, setState]);

  // Use resize hook for drag-to-resize
  const { isResizing, startResize } = useResize({
    initialWidth: width,
    minWidth,
    maxWidth,
    direction: "right",
    onWidthChange: setWidth,
  });

  if (!isExpanded) {
    // Collapsed state - minimal UI with icons
    return (
      <aside
        className="flex h-full flex-col border-r border-border bg-card transition-[width] duration-300 ease-in-out"
        style={{ width: "64px" }}
        aria-label="Left sidebar (collapsed)"
      >
        <div className="flex h-12 items-center justify-center border-b border-border">
          <div className="flex h-7 w-7 items-center justify-center rounded-lg bg-gradient-to-br from-indigo-500 to-purple-600 text-xs font-bold text-white shadow-sm">
            α
          </div>
        </div>

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

        {/* Footer: Agent status pulse dot + Account Menu */}
        <div className="space-y-1.5 border-t border-border p-2">
          <div className="flex items-center justify-center py-1" title={t("agentReadyStatus")}>
            <span className="relative flex h-2 w-2">
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
              <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
            </span>
          </div>
          <div className="flex items-center justify-center">
            <AccountMenu collapsed={true} />
          </div>
        </div>
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
      {/* Header with product identity */}
      <div className="flex h-12 items-center border-b border-border px-4">
        <div className="flex items-center gap-3">
          <div className="flex h-7 w-7 items-center justify-center rounded-lg bg-gradient-to-br from-indigo-500 to-purple-600 text-xs font-bold text-white shadow-sm">
            α
          </div>
          <span className="text-sm font-bold tracking-tight">AlphaForge</span>
        </div>
      </div>

      {/* Navigation Groups */}
      <nav className="flex-1 overflow-y-auto p-2 space-y-1">
        {NAV_GROUPS.map((group) => (
          <NavGroup key={group.id} label={group.label} collapsed={false}>
            {group.items.map((item) => (
              <NavItem key={item.id} item={item} collapsed={false} />
            ))}
          </NavGroup>
        ))}
      </nav>

      {/* Footer: Agent Readiness Card + Account Menu */}
      <div className="space-y-2 border-t border-border p-2.5">
        <div className="flex items-center justify-between rounded-lg border border-border/60 bg-muted/40 px-2.5 py-1.5 text-xs">
          <div className="flex items-center gap-2 truncate">
            <span className="relative flex h-2 w-2 shrink-0">
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
              <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
            </span>
            <span className="truncate text-[11px] font-medium text-foreground">{t("agentReadyStatus")}</span>
          </div>
        </div>

        <div className="flex items-center justify-between px-0.5">
          <span className="text-xs text-muted-foreground truncate font-medium">AlphaForge Desk</span>
          <AccountMenu collapsed={false} />
        </div>
      </div>

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
