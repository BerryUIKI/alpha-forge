/**
 * WorkspaceSelector Component
 *
 * Dropdown selector for workspace types (LeftSidebar - top section).
 * Selecting workspace updates the global workspace state.
 * Special handling for "options" type to navigate to /options route.
 *
 * @version GUI-M1-1
 */

import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { ChevronDown } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import type { WorkspaceSelectorProps, WorkspaceType } from "../types";
import { WORKSPACE_OPTIONS } from "../types";

export function WorkspaceSelector({
  selected = "analyze",
  onSelect,
  isOpen: externalIsOpen,
  onOpenChange,
}: WorkspaceSelectorProps) {
  const { t } = useLocale();
  const navigate = useNavigate();
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

    // Navigate to options route when "options" workspace type is selected
    if (workspace === "options") {
      navigate("/options");
    }
  };

  // Get the label for the selected workspace using i18n
  const selectedOption = WORKSPACE_OPTIONS.find((opt) => opt.value === selected);
  const selectedLabel = selectedOption ? t(selectedOption.labelKey as any) : t("workspaceTypeAnalyze" as any);

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
          {WORKSPACE_OPTIONS.map((option) => {
            const label = t(option.labelKey as any);
            return (
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
                {label}
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
}