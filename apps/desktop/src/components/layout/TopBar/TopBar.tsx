/**
 * TopBar Component
 *
 * Application header bar with breadcrumb, search, and action buttons.
 * Sits at the top of the main content area.
 *
 * @version GUI-E1
 */

import { useLocation, useNavigate } from "react-router-dom";
import {
  Search,
  PanelRightClose,
  PanelRightOpen,
  Plus,
} from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { WorkspaceSwitcher } from "./WorkspaceSwitcher";
import type { TopBarProps } from "../types";

/**
 * Page name mapping for breadcrumb display (i18n keys)
 */
const ROUTE_PAGE_NAMES: Record<string, { labelKey: string; groupKey: string }> = {
  "/": { labelKey: "navDashboard", groupKey: "navWorkspace" },
  "/today": { labelKey: "navDashboard", groupKey: "navWorkspace" },
  "/research": { labelKey: "navResearch", groupKey: "navWorkspace" },
  "/theses": { labelKey: "navTheses", groupKey: "navWorkspace" },
  "/journal": { labelKey: "navJournal", groupKey: "navWorkspace" },
  "/portfolio": { labelKey: "navPortfolio", groupKey: "navWorkspace" },
  "/knowledge": { labelKey: "navKnowledge", groupKey: "navWorkspace" },
  "/options": { labelKey: "navOptions", groupKey: "navTools" },
  "/artifacts": { labelKey: "navArtifacts", groupKey: "navTools" },
  "/settings": { labelKey: "navSettings", groupKey: "navAccount" },
};

export function TopBar({
  isRightSidebarExpanded = false,
  onToggleRightSidebar,
  onOpenSearch,
}: TopBarProps) {
  const { t } = useLocale();
  const location = useLocation();
  const navigate = useNavigate();

  // Determine current page from route
  const pathname = location.pathname;
  const pageInfo = ROUTE_PAGE_NAMES[pathname] || { labelKey: "", groupKey: "" };

  return (
    <header className="flex h-13 items-center justify-between border-b border-border bg-background px-6">
      {/* Left: Workspace switcher + Breadcrumb */}
      <div className="flex items-center gap-3">
        <WorkspaceSwitcher />
        <div className="flex items-center gap-2">
          {pageInfo.groupKey && (
            <span className="text-sm text-muted-foreground/70">
              {t(pageInfo.groupKey as never)}
            </span>
          )}
          {pageInfo.groupKey && (
            <svg
              className="h-3.5 w-3.5 text-muted-foreground/40"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
            >
              <path d="m9 18 6-6-6-6" />
            </svg>
          )}
          {pageInfo.labelKey && (
            <span className="text-sm font-semibold">{t(pageInfo.labelKey as never)}</span>
          )}
        </div>
      </div>

      {/* Right: Search + Actions */}
      <div className="flex items-center gap-1.5">
        {/* Search */}
        <button
          type="button"
          onClick={onOpenSearch}
          className="flex items-center gap-2 rounded-lg border border-border bg-muted/50 px-3 py-1.5 text-sm text-muted-foreground transition-colors focus-within:border-primary focus-within:ring-1 focus-within:ring-primary hover:border-primary/60"
          aria-label="Open search"
          title="Search (Ctrl+K)"
        >
          <Search className="h-4 w-4" />
          <span className="text-xs text-muted-foreground/50">Search...</span>
          <kbd className="hidden rounded border border-border bg-background px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground/60 md:inline-flex">
            ⌘K
          </kbd>
        </button>

        {/* Divider */}
        <div className="mx-1 h-5 w-px bg-border/60" />

        {/* New Button */}
        <button
          onClick={() => navigate("/research")}
          className="flex h-8 w-8 items-center justify-center rounded-lg transition-colors hover:bg-accent"
          aria-label="Create new"
          title="Create new"
        >
          <Plus className="h-4 w-4" />
        </button>

        {/* Divider */}
        <div className="mx-1 h-5 w-px bg-border/60" />

        {/* Agent Toggle */}
        <button
          onClick={onToggleRightSidebar}
          className="flex h-8 w-8 items-center justify-center rounded-lg transition-colors hover:bg-accent"
          aria-label={isRightSidebarExpanded ? "Close agent panel" : "Open agent panel"}
          title={isRightSidebarExpanded ? "Close agent panel (Ctrl+2)" : "Open agent panel (Ctrl+2)"}
        >
          {isRightSidebarExpanded ? (
            <PanelRightClose className="h-4 w-4" />
          ) : (
            <PanelRightOpen className="h-4 w-4" />
          )}
        </button>
      </div>
    </header>
  );
}
