/**
 * Functional View Selector Component
 *
 * Dropdown selector for functional views (分析/量化/期权等).
 * Located at the top of the left sidebar.
 * Selecting a view navigates to the corresponding route.
 *
 * @module components/layout/LeftSidebar/FunctionalViewSelector
 */

import { useState } from "react";
import { ChevronDown, Check } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { useFunctionalView } from "@/hooks/layout";
import type { FunctionalView } from "@/components/layout/types";
import { FUNCTIONAL_VIEW_OPTIONS } from "@/components/layout/types";

export function FunctionalViewSelector() {
  const { t } = useLocale();
  const { view, setView } = useFunctionalView();
  const [isOpen, setIsOpen] = useState(false);

  const currentOption = FUNCTIONAL_VIEW_OPTIONS.find(opt => opt.value === view);

  const handleSelect = (selectedView: FunctionalView) => {
    setView(selectedView);
    setIsOpen(false);
  };

  return (
    <div className="relative">
      {/* Trigger Button */}
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="flex w-full items-center justify-between gap-2 rounded-lg px-3 py-2 text-left text-sm font-medium transition-colors hover:bg-accent"
        aria-expanded={isOpen}
        aria-haspopup="listbox"
        aria-label={t("selectFunctionalView" as any) || "Select functional view"}
      >
        <span className="truncate">
          {currentOption ? t(currentOption.labelKey as any) : t("workspaceTypeAnalyze" as any) || "分析"}
        </span>
        <ChevronDown
          className={`h-4 w-4 flex-shrink-0 text-muted-foreground transition-transform ${
            isOpen ? "rotate-180" : ""
          }`}
        />
      </button>

      {/* Dropdown Menu */}
      {isOpen && (
        <>
          {/* Backdrop */}
          <div
            className="fixed inset-0 z-40"
            onClick={() => setIsOpen(false)}
          />

          {/* Menu */}
          <div
            className="absolute left-0 right-0 top-full z-50 mt-1 rounded-lg border border-border bg-card p-1 shadow-lg"
            role="listbox"
          >
            {FUNCTIONAL_VIEW_OPTIONS.map((option) => {
              const isSelected = view === option.value;
              return (
                <button
                  key={option.value}
                  onClick={() => handleSelect(option.value)}
                  className={`flex w-full items-center gap-2 rounded-md px-3 py-2 text-left text-sm transition-colors ${
                    isSelected
                      ? "bg-primary/10 font-medium text-primary"
                      : "hover:bg-accent"
                  }`}
                  role="option"
                  aria-selected={isSelected}
                >
                  <span className="flex-1">
                    {t(option.labelKey as any) || option.value}
                  </span>
                  {isSelected && (
                    <Check className="h-4 w-4 flex-shrink-0" />
                  )}
                </button>
              );
            })}
          </div>
        </>
      )}
    </div>
  );
}