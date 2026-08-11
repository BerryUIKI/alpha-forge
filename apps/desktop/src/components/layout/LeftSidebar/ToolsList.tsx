/**
 * Tools List Component
 *
 * Displays a list of tools for the current functional view.
 * Located in the middle section of the left sidebar.
 *
 * @module components/layout/LeftSidebar/ToolsList
 */

import { useNavigate } from "react-router-dom";
import { useFunctionalView } from "@/hooks/layout";
import { useLocale } from "@/lib/i18n/useLocale";
import type { ToolItem } from "@/components/layout/types";

export function ToolsList() {
  const { t } = useLocale();
  const navigate = useNavigate();
  const { tools } = useFunctionalView();

  const handleToolClick = (tool: ToolItem) => {
    if (tool.disabled) {
      return;
    }

    if (tool.route) {
      navigate(tool.route);
    } else if (tool.action) {
      tool.action();
    }
  };

  if (tools.length === 0) {
    return (
      <div className="flex flex-1 items-center justify-center p-4 text-center">
        <p className="text-sm text-muted-foreground">
          {t("noToolsAvailable" as any) || "暂无可用工具"}
        </p>
      </div>
    );
  }

  return (
    <div className="flex-1 overflow-y-auto p-2">
      <div className="space-y-1">
        {tools.map((tool) => {
          const Icon = tool.icon;
          return (
            <button
              key={tool.id}
              onClick={() => handleToolClick(tool)}
              disabled={tool.disabled}
              className={`flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left text-sm transition-colors ${
                tool.disabled
                  ? "cursor-not-allowed opacity-50"
                  : "hover:bg-accent"
              }`}
              title={tool.disabled ? (t("comingSoon" as any) || "即将推出") : undefined}
            >
              <Icon className="h-4 w-4 flex-shrink-0 text-muted-foreground" />
              <span className="truncate">{tool.label}</span>
            </button>
          );
        })}
      </div>
    </div>
  );
}
