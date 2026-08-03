// Today page - Dashboard with workspace selector.

import { useState } from "react";
import { FolderOpen } from "lucide-react";
import { WorkspaceList, CreateWorkspaceDialog } from "@/features/workspace";
import { useLocale } from "@/lib/i18n/useLocale";
import type { Workspace } from "@/lib/desktop-api/workspace";

export function TodayPage() {
  const { t } = useLocale();
  const [showCreateDialog, setShowCreateDialog] = useState(false);
  const [selectedWorkspace, setSelectedWorkspace] = useState<Workspace | null>(null);

  const handleWorkspaceSelect = (workspace: Workspace) => {
    setSelectedWorkspace(workspace);
  };

  const handleCreateNew = () => {
    setShowCreateDialog(true);
  };

  const handleCreateSuccess = (workspace: { id: string; name: string }) => {
    // Refresh will happen automatically via TanStack Query invalidation
    setSelectedWorkspace({
      id: workspace.id,
      name: workspace.name,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString()
    });
  };

  return (
    <div className="p-6">
      <div className="mb-6">
        <h1 className="text-2xl font-bold">{t("today")}</h1>
        <p className="mt-2 text-muted-foreground">
          {t("todayDescription")}
        </p>
      </div>

      {!selectedWorkspace ? (
        <div className="max-w-2xl">
          <div className="mb-4 flex items-center gap-2">
            <FolderOpen className="h-5 w-5 text-primary" />
            <h2 className="text-lg font-semibold">{t("selectWorkspace")}</h2>
          </div>
          <p className="mb-6 text-sm text-muted-foreground">
            {t("selectWorkspaceDescription")}
          </p>
          <WorkspaceList
            onSelect={handleWorkspaceSelect}
            onCreateNew={handleCreateNew}
          />
        </div>
      ) : (
        <div className="max-w-2xl">
          <div className="rounded-lg border border-border bg-card p-6">
            <h2 className="mb-2 text-lg font-semibold">{selectedWorkspace.name}</h2>
            <p className="text-sm text-muted-foreground">
              {t("workspaceSelected")}
            </p>
            <button
              onClick={() => setSelectedWorkspace(null)}
              className="mt-4 text-sm text-primary hover:underline"
            >
              {t("changeWorkspace")}
            </button>
          </div>
        </div>
      )}

      <CreateWorkspaceDialog
        isOpen={showCreateDialog}
        onClose={() => setShowCreateDialog(false)}
        onSuccess={handleCreateSuccess}
      />
    </div>
  );
}