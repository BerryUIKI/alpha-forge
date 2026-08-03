/**
 * ScrollableList Component
 *
 * Middle section of left sidebar for displaying projects/sessions/knowledge base.
 * UI-only implementation with placeholder content.
 *
 * TODO: [GUI-M1-1] Connect to actual data sources
 * TODO: [GUI-M1-1] Implement real list item rendering
 * TODO: [GUI-M1-4] Add i18n for empty state message
 */

import { FolderOpen, FileText, Database, type LucideIcon } from "lucide-react";
import type { ScrollableListProps } from "../types";

// Placeholder items for UI demonstration
const PLACEHOLDER_ITEMS: Array<{ id: string; label: string; icon: LucideIcon }> = [
  { id: "1", label: "Project Alpha", icon: FolderOpen },
  { id: "2", label: "Research Session", icon: FileText },
  { id: "3", label: "Knowledge Base", icon: Database },
  { id: "4", label: "Project Beta", icon: FolderOpen },
  { id: "5", label: "Thesis Draft", icon: FileText },
];

export function ScrollableList({
  items = PLACEHOLDER_ITEMS,
  selectedId,
  onSelect,
  emptyMessage = "No items yet",
}: ScrollableListProps) {
  if (items.length === 0) {
    return (
      <div className="flex flex-1 flex-col items-center justify-center p-4 text-center">
        <div className="rounded-lg border border-dashed border-border p-8">
          <p className="text-sm text-muted-foreground">{emptyMessage}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex-1 overflow-y-auto">
      <div className="space-y-1 p-2">
        {items.map((item) => {
          const Icon = ("icon" in item && item.icon ? item.icon : FolderOpen) as LucideIcon;
          return (
            <button
              key={item.id}
              onClick={() => onSelect?.(item.id)}
              className={`flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left text-sm transition-colors hover:bg-accent ${
                selectedId === item.id
                  ? "bg-primary/10 font-medium text-primary"
                  : ""
              }`}
              aria-label={item.label}
            >
              <Icon className="h-4 w-4 flex-shrink-0" />
              <span className="truncate">{item.label}</span>
            </button>
          );
        })}
      </div>

      {/* TODO marker for future implementation */}
      {/* TODO: [GUI-M1-1] Replace placeholder items with real data from backend */}
      {/* TODO: [GUI-M1-1] Implement list virtualization for large datasets */}
    </div>
  );
}