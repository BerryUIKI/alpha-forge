// Today page - Dashboard with workspace selector.

import { useState } from "react";
import { FolderOpen } from "lucide-react";
import { WorkspaceList, CreateWorkspaceDialog } from "@/features/workspace";
import { AgentWorkspacePanel } from "@/features/agent";
import type { Workspace } from "@/lib/desktop-api/workspace";

export function TodayPage() {
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
      updated_at: new Date().toISOString(),
    });
  };

  return (
    <div className="p-6">
      <div className="mb-6">
        <h1 className="text-2xl font-bold">Today</h1>
        <p className="mt-2 text-muted-foreground">Your investment research dashboard</p>
      </div>

      {!selectedWorkspace ? (
        <div className="max-w-2xl">
          <div className="mb-4 flex items-center gap-2">
            <FolderOpen className="h-5 w-5 text-primary" />
            <h2 className="text-lg font-semibold">Select a Workspace</h2>
          </div>
          <p className="mb-6 text-sm text-muted-foreground">
            Choose a workspace to view and manage your research, or create a new one to get started.
          </p>
          <WorkspaceList onSelect={handleWorkspaceSelect} onCreateNew={handleCreateNew} />
        </div>
      ) : (
        <div className="max-w-5xl">
          <div className="mb-6 flex flex-wrap items-center justify-between gap-3 rounded-lg border border-border bg-card p-4">
            <div>
              <p className="text-xs uppercase tracking-wide text-muted-foreground">Workspace</p>
              <h2 className="text-lg font-semibold">{selectedWorkspace.name}</h2>
            </div>
            <button
              onClick={() => setSelectedWorkspace(null)}
              className="text-sm text-primary hover:underline"
            >
              Change workspace
            </button>
          </div>
          <AgentWorkspacePanel workspaceId={selectedWorkspace.id} />
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
