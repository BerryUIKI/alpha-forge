/**
 * OperationBar Component
 *
 * Top section of Main Content area (Module D - top).
 * Displays current context title and action buttons.
 *
 * TODO: [GUI-M1-4] Implement shortcut action button functionality
 * TODO: [GUI-M1-4] Add i18n for titles and labels
 */

import { Bot, Search, Plus } from "lucide-react";
import type { OperationBarProps } from "../types";

export function OperationBar({
  title = "Welcome",
  isRightSidebarExpanded = false,
  onToggleRightSidebar,
}: OperationBarProps) {
  return (
    <div className="flex items-center justify-between border-b border-border bg-card px-4 py-3">
      {/* Left: Context Title */}
      <div className="flex items-center gap-2">
        <h1 className="text-lg font-semibold">{title}</h1>
      </div>

      {/* Right: Action Buttons */}
      <div className="flex items-center gap-2">
        {/* Quick Actions */}
        <div className="flex items-center gap-1">
          {/* TODO: [GUI-M1-4] Implement search functionality */}
          <button
            className="flex h-8 w-8 items-center justify-center rounded-lg transition-colors hover:bg-accent"
            aria-label="Search"
            title="Search (coming soon)"
            disabled
          >
            <Search className="h-4 w-4 text-muted-foreground" />
          </button>

          {/* TODO: [GUI-M1-4] Implement create new functionality */}
          <button
            className="flex h-8 w-8 items-center justify-center rounded-lg transition-colors hover:bg-accent"
            aria-label="Create new"
            title="Create new (coming soon)"
            disabled
          >
            <Plus className="h-4 w-4 text-muted-foreground" />
          </button>
        </div>

        {/* Divider */}
        <div className="h-6 w-px bg-border" />

        {/* Agent Sidebar Toggle */}
        <button
          onClick={onToggleRightSidebar}
          className={`flex h-8 items-center gap-2 rounded-lg px-3 transition-colors ${
            isRightSidebarExpanded
              ? "bg-primary/10 text-primary"
              : "hover:bg-accent"
          }`}
          aria-label={isRightSidebarExpanded ? "Hide Agent panel" : "Show Agent panel"}
          aria-pressed={isRightSidebarExpanded}
        >
          <Bot className="h-4 w-4" />
          <span className="text-sm font-medium">Agent</span>
        </button>
      </div>

      {/* TODO markers for future implementation */}
      {/* TODO: [GUI-M1-4] Add keyboard shortcuts for quick actions */}
      {/* TODO: [GUI-M1-4] Implement context-aware actions based on workspace type */}
    </div>
  );
}