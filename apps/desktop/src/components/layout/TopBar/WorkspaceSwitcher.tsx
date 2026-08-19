/**
 * WorkspaceSwitcher Component
 *
 * Global active-workspace switcher for the TopBar (ADR-0008).
 * This is the single selector for the research dimension; the Portfolio page
 * ignores it (global dimension). Reads/writes the active workspace context,
 * which persists the selection in localStorage.
 *
 * Placed at the top-left of the TopBar, next to the breadcrumb.
 *
 * @version GUI-M6
 */

import { Layers } from "lucide-react";
import { useActiveWorkspace } from "@/features/workspace/hooks/useActiveWorkspace";
import { useLocale } from "@/lib/i18n/useLocale";

export function WorkspaceSwitcher() {
  const { t } = useLocale();
  const { workspaceId, workspaces, setActiveWorkspace } = useActiveWorkspace();

  // Hide while loading / before the context resolves the active workspace.
  if (workspaces.length === 0 || !workspaceId) return null;

  return (
    <label className="flex items-center gap-1.5" title={t("workspace")}>
      <Layers className="h-4 w-4 text-muted-foreground" />
      <select
        aria-label={t("workspace")}
        value={workspaceId}
        onChange={(event) => setActiveWorkspace(event.target.value)}
        className="max-w-[160px] cursor-pointer rounded-lg border border-border bg-muted/50 px-2 py-1.5 text-sm text-muted-foreground outline-none transition-colors hover:border-primary/60 focus:border-primary focus:ring-1 focus:ring-primary"
      >
        {workspaces.map((ws) => (
          <option key={ws.id} value={ws.id}>
            {ws.name}
          </option>
        ))}
      </select>
    </label>
  );
}
