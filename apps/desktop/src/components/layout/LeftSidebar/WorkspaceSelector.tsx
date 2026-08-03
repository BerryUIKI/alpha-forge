/**
 * WorkspaceSelector Component
 *
 * Dropdown selector for workspace types (Module A - top section).
 * UI-only implementation with selection state management.
 *
 * TODO: [GUI-M1-1] Implement workspace switching business logic
 * TODO: [GUI-M1-4] Add i18n support for workspace labels
 */

import { useState } from "react";
import { ChevronDown } from "lucide-react";
import type { WorkspaceSelectorProps, WorkspaceType } from "../types";
import { WORKSPACE_OPTIONS } from "../types";

export function WorkspaceSelector({
  selected = "analyze",
  onSelect,
  isOpen: externalIsOpen,
  onOpenChange,
}: WorkspaceSelectorProps) {
  const [internalIsOpen, setInternalIsOpen] = useState(false);
  const isOpen = externalIsOpen ?? internalIsOpen;

  const handleToggle = () => {
    const newState = !isOpen;
    setInternalIsOpen(newState);
    onOpenChange?.(newState);
  };

  const handleSelect = (workspace: WorkspaceType) => {
    onSelect?.(workspace);
    setInternalIsOpen(false);
    onOpenChange?.(false);

    // TODO: [GUI-M1-1] Trigger workspace switching logic
    // Placeholder for business logic integration
  };

  const selectedLabel = WORKSPACE_OPTIONS.find((opt) => opt.value === selected)?.label ?? "Analyze";

  return (
    <div className="relative border-b border-border p-3">
      {/* Dropdown Button */}
      <button
        onClick={handleToggle}
        className="flex w-full items-center justify-between rounded-lg px-3 py-2 text-sm font-medium transition-colors hover:bg-accent"
        aria-expanded={isOpen}
        aria-haspopup="listbox"
        aria-label={`Workspace selector, currently: ${selectedLabel}`}
      >
        <span className="truncate">{selectedLabel}</span>
        <ChevronDown
          className={`h-4 w-4 transition-transform duration-200 ${
            isOpen ? "rotate-180" : ""
          }`}
        />
      </button>

      {/* Dropdown Menu */}
      {isOpen && (
        <div
          className="absolute left-0 right-0 top-full z-50 mt-1 rounded-lg border border-border bg-popover shadow-lg"
          role="listbox"
          aria-label="Workspace options"
        >
          {WORKSPACE_OPTIONS.map((option) => (
            <button
              key={option.value}
              onClick={() => handleSelect(option.value)}
              className={`flex w-full items-center px-3 py-2 text-left text-sm transition-colors first:rounded-t-lg last:rounded-b-lg hover:bg-accent ${
                selected === option.value
                  ? "bg-primary/10 font-medium text-primary"
                  : ""
              }`}
              role="option"
              aria-selected={selected === option.value}
            >
              {option.label}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}