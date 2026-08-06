/**
 * FunctionalViewSelector Component
 *
 * Dropdown selector for 6 functional views.
 * Persists selection to localStorage and updates tools list.
 *
 * @version GUI-M2
 */

import { useState, useEffect } from "react";
import { ChevronDown, TrendingUp, LineChart, PieChart, Calculator, GitBranch, Package } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import type { FunctionalView } from "@/components/layout/types";
import { DEFAULT_FUNCTIONAL_VIEW, ACTIVE_VIEW_STORAGE_KEY } from "@/config/tools-config";

/**
 * Functional view configuration with icon and i18n key
 */
const VIEW_CONFIG: Record<FunctionalView, { icon: typeof TrendingUp; nameKey: string }> = {
  analyze: { icon: TrendingUp, nameKey: "functionalViewAnalyze" },
  quantification: { icon: LineChart, nameKey: "functionalViewQuantification" },
  "comprehensive-market": { icon: PieChart, nameKey: "functionalViewComprehensiveMarket" },
  options: { icon: Calculator, nameKey: "functionalViewOptions" },
  futures: { icon: GitBranch, nameKey: "functionalViewFutures" },
  "other-derivatives": { icon: Package, nameKey: "functionalViewOtherDerivatives" },
};

interface FunctionalViewSelectorProps {
  /** Currently selected view */
  value?: FunctionalView;
  /** Callback when view changes */
  onChange?: (view: FunctionalView) => void;
}

export function FunctionalViewSelector({ value, onChange }: FunctionalViewSelectorProps) {
  const { t } = useLocale();
  const [isOpen, setIsOpen] = useState(false);
  const [selectedView, setSelectedView] = useState<FunctionalView>(
    () => value || DEFAULT_FUNCTIONAL_VIEW
  );

  // Initialize from localStorage
  useEffect(() => {
    if (!value) {
      const stored = localStorage.getItem(ACTIVE_VIEW_STORAGE_KEY) as FunctionalView | null;
      if (stored && VIEW_CONFIG[stored]) {
        setSelectedView(stored);
        onChange?.(stored);
      }
    }
  }, [value, onChange]);

  const handleViewSelect = (view: FunctionalView) => {
    setSelectedView(view);
    setIsOpen(false);
    localStorage.setItem(ACTIVE_VIEW_STORAGE_KEY, view);
    onChange?.(view);
  };

  const currentConfig = VIEW_CONFIG[selectedView];
  const IconComponent = currentConfig.icon;

  return (
    <div className="border-b border-border p-3">
      <label className="mb-2 block text-xs font-medium text-muted-foreground">
        {t("functionalView")}
      </label>

      {/* Dropdown container */}
      <div className="relative">
        <button
          onClick={() => setIsOpen(!isOpen)}
          className="flex w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
          aria-expanded={isOpen}
          aria-haspopup="listbox"
          aria-label={t("selectFunctionalView")}
        >
          <div className="flex items-center gap-2">
            <IconComponent className="h-4 w-4" />
            <span>{t(currentConfig.nameKey as any)}</span>
          </div>
          <ChevronDown
            className={`h-4 w-4 transition-transform duration-200 ${isOpen ? "rotate-180" : ""}`}
          />
        </button>

        {/* Dropdown menu */}
        {isOpen && (
          <>
            {/* Backdrop */}
            <div
              className="fixed inset-0 z-10"
              onClick={() => setIsOpen(false)}
              aria-hidden="true"
            />

            {/* Menu */}
            <ul
              className="absolute left-0 z-20 mt-1 w-full rounded-md border border-border bg-popover p-1 shadow-md animate-in fade-in-0 zoom-in-95"
              role="listbox"
              aria-label={t("functionalViews")}
            >
              {(Object.entries(VIEW_CONFIG) as [FunctionalView, typeof VIEW_CONFIG.analyze][]).map(
                ([viewId, config]) => {
                  const ViewIcon = config.icon;
                  const isSelected = viewId === selectedView;

                  return (
                    <li key={viewId}>
                      <button
                        onClick={() => handleViewSelect(viewId)}
                        className={`flex w-full items-center gap-2 rounded-sm px-3 py-2 text-sm transition-colors ${
                          isSelected
                            ? "bg-primary/10 text-primary"
                            : "hover:bg-accent hover:text-accent-foreground"
                        }`}
                        role="option"
                        aria-selected={isSelected}
                      >
                        <ViewIcon className="h-4 w-4" />
                        <span>{t(config.nameKey as any)}</span>
                      </button>
                    </li>
                  );
                }
              )}
            </ul>
          </>
        )}
      </div>
    </div>
  );
}