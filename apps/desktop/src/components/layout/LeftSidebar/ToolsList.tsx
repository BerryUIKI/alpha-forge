/**
 * ToolsList Component
 *
 * Displays dynamic list of tools based on selected functional view.
 * Scrollable container with navigation to tool routes.
 *
 * @version GUI-M2
 */

import { useNavigate } from "react-router-dom";
import {
  Search,
  FileText,
  Lightbulb,
  LineChart,
  Shield,
  Zap,
  TrendingUp,
  PieChart,
  Network,
  Calculator,
  Layers,
  List,
  GitBranch,
  BarChart3,
  Package,
} from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import type { FunctionalView, Tool } from "@/components/layout/types";
import { getToolsForView } from "@/config/tools-config";

/**
 * Icon mapping for dynamic tool icons
 */
const ICON_MAP: Record<string, typeof Search> = {
  Search,
  FileText,
  Lightbulb,
  LineChart,
  Shield,
  Zap,
  TrendingUp,
  PieChart,
  Network,
  Calculator,
  Layers,
  List,
  GitBranch,
  BarChart3,
  Package,
};

interface ToolsListProps {
  /** Currently selected functional view */
  activeView: FunctionalView;
}

export function ToolsList({ activeView }: ToolsListProps) {
  const { t } = useLocale();
  const navigate = useNavigate();

  const tools = getToolsForView(activeView);

  const handleToolClick = (tool: Tool) => {
    if (tool.route) {
      navigate(tool.route);
    }
  };

  if (tools.length === 0) {
    return (
      <div className="flex flex-1 items-center justify-center p-4">
        <p className="text-sm text-muted-foreground">{t("noToolsAvailable")}</p>
      </div>
    );
  }

  return (
    <div className="flex-1 overflow-y-auto p-2">
      <div className="mb-2 px-2">
        <h3 className="text-xs font-medium text-muted-foreground">{t("tools")}</h3>
      </div>

      <ul className="space-y-1" role="list" aria-label={t("toolsList")}>
        {tools.map((tool) => {
          const IconComponent = ICON_MAP[tool.icon] || Search;

          return (
            <li key={tool.id}>
              <button
                onClick={() => handleToolClick(tool)}
                className="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50"
                disabled={!tool.route}
                aria-label={t(tool.nameKey as any)}
                title={tool.descriptionKey ? t(tool.descriptionKey as any) : undefined}
              >
                <IconComponent className="h-4 w-4 flex-shrink-0" />
                <span className="truncate">{t(tool.nameKey as any)}</span>
              </button>
            </li>
          );
        })}
      </ul>
    </div>
  );
}