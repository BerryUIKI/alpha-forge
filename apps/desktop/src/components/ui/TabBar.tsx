/**
 * TabBar Component
 *
 * Segmented-control style tab bar for page-level navigation.
 * Active tab slides between options with a smooth transition.
 *
 * Accessibility contract:
 * - Renders a horizontal `tablist` with roving tabindex. Only the selected
 *   tab is in the Tab order; ArrowLeft/ArrowRight/Home/End move selection
 *   (automatic activation) and focus.
 * - Each tab links to a panel via `aria-controls`. The panel id convention is
 *   `${idPrefix}-panel-${tabId}` (see `getTabPanelId`).
 * - Selection is driven by the URL/local-storage state owned by the page.
 *
 * @version GUI-M2
 */

import { useId, useRef } from "react";
import { cn } from "@/lib/utils";
import { getTabId, getTabPanelId } from "./tab-ids";
import type { LucideIcon } from "lucide-react";

interface Tab {
  id: string;
  label: string;
  icon?: LucideIcon;
}

interface TabBarProps {
  /** Tab definitions */
  tabs: Tab[];
  /** Currently active tab ID */
  activeTab: string;
  /** Callback when tab changes */
  onTabChange: (tabId: string) => void;
  /** Extra classes */
  className?: string;
  /** Prefix for tab/panel element ids. Defaults to a generated unique id. */
  idPrefix?: string;
  /** Accessible name for the tab list */
  label?: string;
}

export function TabBar({ tabs, activeTab, onTabChange, className, idPrefix, label }: TabBarProps) {
  const generatedId = useId();
  const prefix = idPrefix ?? `tabs-${generatedId.replaceAll(":", "")}`;
  const listRef = useRef<HTMLDivElement>(null);
  const activeIndex = tabs.findIndex((tab) => tab.id === activeTab);

  const activate = (index: number) => {
    const target = tabs[index];
    if (!target) return;
    onTabChange(target.id);
    // Move focus with selection so the roving tabindex stays coherent.
    document.getElementById(getTabId(prefix, target.id))?.focus();
  };

  const handleKeyDown = (event: React.KeyboardEvent<HTMLDivElement>) => {
    if (tabs.length === 0) return;
    const lastIndex = tabs.length - 1;
    let nextIndex: number;

    switch (event.key) {
      case "ArrowRight":
        nextIndex = activeIndex >= lastIndex ? 0 : activeIndex + 1;
        break;
      case "ArrowLeft":
        nextIndex = activeIndex <= 0 ? lastIndex : activeIndex - 1;
        break;
      case "Home":
        nextIndex = 0;
        break;
      case "End":
        nextIndex = lastIndex;
        break;
      default:
        return;
    }

    event.preventDefault();
    activate(nextIndex);
  };

  return (
    <div
      ref={listRef}
      role="tablist"
      aria-orientation="horizontal"
      aria-label={label}
      onKeyDown={handleKeyDown}
      className={cn("flex gap-1 rounded-lg bg-muted p-1", className)}
    >
      {tabs.map((tab) => {
        const Icon = tab.icon;
        const isActive = tab.id === activeTab;

        return (
          <button
            key={tab.id}
            id={getTabId(prefix, tab.id)}
            type="button"
            role="tab"
            aria-selected={isActive}
            aria-controls={getTabPanelId(prefix, tab.id)}
            tabIndex={isActive ? 0 : -1}
            onClick={() => onTabChange(tab.id)}
            className={cn(
              "flex items-center gap-1.5 rounded-md px-3 py-1.5 text-sm font-medium transition-all",
              isActive
                ? "bg-background text-foreground shadow-sm"
                : "text-muted-foreground hover:text-foreground",
            )}
          >
            {Icon && <Icon className="h-4 w-4" aria-hidden="true" />}
            <span>{tab.label}</span>
          </button>
        );
      })}
    </div>
  );
}
