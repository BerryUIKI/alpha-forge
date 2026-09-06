/**
 * UnifiedHeader Component
 *
 * Consolidates window title controls, workspace selector, breadcrumb,
 * global search trigger, and Agent Copilot toggles into a single,
 * highly ergonomic, 46px macOS-first toolbar (GUI-M4-MACOS).
 */

import { useLocation, useNavigate } from "react-router-dom";
import {
  Search,
  PanelLeftClose,
  PanelLeftOpen,
  PanelRightClose,
  PanelRightOpen,
  Plus,
  Minus,
  Square,
  X,
  Sparkles,
} from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { isMacPlatform } from "@/lib/utils";
import { desktopApi } from "@/lib/desktop-api";
import { WorkspaceSwitcher } from "../TopBar/WorkspaceSwitcher";
import type { UnifiedHeaderProps } from "../types";

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

function runWindowAction(label: string, action: () => Promise<void>) {
  void action().catch(() => {
    console.error(`Window action failed: ${label}`);
  });
}

export function UnifiedHeader({
  isLeftSidebarExpanded = true,
  onToggleLeftSidebar,
  isRightSidebarExpanded = false,
  onToggleRightSidebar,
  onOpenSearch,
}: UnifiedHeaderProps) {
  const { t } = useLocale();
  const location = useLocation();
  const navigate = useNavigate();
  const isMac = isMacPlatform();

  const pathname = location.pathname;
  const pageInfo = ROUTE_PAGE_NAMES[pathname] || { labelKey: "", groupKey: "" };

  return (
    <header
      className={`relative z-40 flex h-[46px] shrink-0 select-none items-center justify-between border-b border-border/80 bg-background/95 backdrop-blur-md px-3 text-foreground transition-colors ${
        isMac ? "pl-[76px]" : "pl-3"
      }`}
      aria-label="Application header"
      data-tauri-drag-region
      onDoubleClick={() => runWindowAction("toggle maximize", desktopApi.window.toggleMaximize)}
    >
      {/* Left: Navigation toggle + Workspace switcher + Breadcrumb */}
      <div className="flex items-center gap-2.5" data-window-interactive>
        {onToggleLeftSidebar && (
          <button
            type="button"
            onClick={onToggleLeftSidebar}
            className="flex h-7 w-7 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
            aria-label={isLeftSidebarExpanded ? t("collapseSidebar") : t("expandSidebar")}
            title={isLeftSidebarExpanded ? `${t("collapseSidebar")} (${isMac ? "⌘1" : "Ctrl+1"})` : `${t("expandSidebar")} (${isMac ? "⌘1" : "Ctrl+1"})`}
            data-window-interactive
          >
            {isLeftSidebarExpanded ? (
              <PanelLeftClose className="h-4 w-4" />
            ) : (
              <PanelLeftOpen className="h-4 w-4" />
            )}
          </button>
        )}

        <div className="h-4 w-px bg-border/60" aria-hidden="true" />

        <WorkspaceSwitcher />

        {pageInfo.labelKey && (
          <div className="hidden items-center gap-1.5 text-xs text-muted-foreground sm:flex">
            <span>/</span>
            <span className="font-medium text-foreground">{t(pageInfo.labelKey as never)}</span>
          </div>
        )}
      </div>

      {/* Center: Spotlight Search Pill (⌘K) */}
      <div className="mx-4 flex max-w-md flex-1 items-center justify-center" data-tauri-drag-region>
        <button
          type="button"
          onClick={onOpenSearch}
          className="group flex w-full max-w-sm items-center justify-between rounded-lg border border-border/70 bg-muted/40 px-3 py-1 text-xs text-muted-foreground shadow-inner transition-all hover:border-border hover:bg-muted/70 hover:text-foreground"
          aria-label={t("unifiedSearchPlaceholder")}
          title={`Search (${isMac ? "⌘K" : "Ctrl+K"})`}
          data-window-interactive
        >
          <div className="flex items-center gap-2 truncate">
            <Search className="h-3.5 w-3.5 text-muted-foreground transition-colors group-hover:text-primary" />
            <span className="truncate">{t("unifiedSearchPlaceholder")}</span>
          </div>
          <kbd className="ml-2 hidden shrink-0 rounded border border-border/80 bg-background/80 px-1.5 py-0.5 text-[10px] font-mono text-muted-foreground md:inline-block">
            {isMac ? "⌘K" : "Ctrl+K"}
          </kbd>
        </button>
      </div>

      {/* Right: Quick Actions + Agent Panel Toggle + Windows controls */}
      <div className="flex items-center gap-1.5" data-window-interactive>
        {/* Quick New Research Action */}
        <button
          type="button"
          onClick={() => navigate("/research")}
          className="flex h-7 items-center gap-1.5 rounded-lg bg-primary/10 px-2.5 text-xs font-medium text-primary transition-colors hover:bg-primary/20"
          aria-label={t("newResearch")}
          title={`${t("newResearch")} (${isMac ? "⌘N" : "Ctrl+N"})`}
          data-window-interactive
        >
          <Plus className="h-3.5 w-3.5" />
          <span className="hidden lg:inline">{t("newResearch")}</span>
        </button>

        <div className="h-4 w-px bg-border/60" aria-hidden="true" />

        {/* AI Agent Panel Toggle */}
        {onToggleRightSidebar && (
          <button
            type="button"
            onClick={onToggleRightSidebar}
            className={`flex h-7 items-center gap-1.5 rounded-lg border px-2 text-xs font-medium transition-all ${
              isRightSidebarExpanded
                ? "border-primary/40 bg-primary/15 text-primary"
                : "border-border/60 text-muted-foreground hover:bg-accent hover:text-foreground"
            }`}
            aria-label={isRightSidebarExpanded ? t("closeAgentPanel") : t("openAgentPanel")}
            title={isRightSidebarExpanded ? `${t("closeAgentPanel")} (${isMac ? "⌘J" : "Ctrl+2"})` : `${t("openAgentPanel")} (${isMac ? "⌘J" : "Ctrl+2"})`}
            data-window-interactive
          >
            <Sparkles className="h-3.5 w-3.5 text-primary" />
            <span className="hidden xl:inline">{isRightSidebarExpanded ? t("closeAgentPanel") : t("openAgentPanel")}</span>
            {isRightSidebarExpanded ? (
              <PanelRightClose className="h-3.5 w-3.5" />
            ) : (
              <PanelRightOpen className="h-3.5 w-3.5" />
            )}
          </button>
        )}

        {/* Windows / Linux controls (only when not on macOS) */}
        {!isMac && (
          <div className="ml-1 flex items-center gap-0.5 border-l border-border/60 pl-1" data-window-interactive>
            <button
              type="button"
              onClick={() => runWindowAction("minimize", desktopApi.window.minimize)}
              className="flex h-7 w-8 items-center justify-center rounded text-muted-foreground hover:bg-accent hover:text-foreground"
              aria-label={t("windowMinimize")}
              title={t("windowMinimize")}
              data-window-interactive
            >
              <Minus className="h-3.5 w-3.5" />
            </button>
            <button
              type="button"
              onClick={() => runWindowAction("toggle maximize", desktopApi.window.toggleMaximize)}
              className="flex h-7 w-8 items-center justify-center rounded text-muted-foreground hover:bg-accent hover:text-foreground"
              aria-label={t("windowMaximize")}
              title={t("windowMaximize")}
              data-window-interactive
            >
              <Square className="h-3 w-3" />
            </button>
            <button
              type="button"
              onClick={() => runWindowAction("close", desktopApi.window.close)}
              className="flex h-7 w-8 items-center justify-center rounded text-muted-foreground hover:bg-destructive hover:text-destructive-foreground"
              aria-label={t("windowClose")}
              title={t("windowClose")}
              data-window-interactive
            >
              <X className="h-3.5 w-3.5" />
            </button>
          </div>
        )}
      </div>
    </header>
  );
}
