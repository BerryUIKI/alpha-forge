/**
 * ScrollableList Component
 *
 * Middle section of left sidebar for displaying workspaces/projects.
 * Connected to backend via useWorkspaces hook.
 *
 * @version GUI-M1-1
 */

import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { FolderOpen, Plus } from "lucide-react";
import { useWorkspaces } from "@/features/workspace";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { ErrorState } from "@/components/common/ErrorState";
import { EmptyState } from "@/components/common/EmptyState";
import { CreateWorkspaceDialog } from "@/features/workspace/components/CreateWorkspaceDialog";
import { useLocale } from "@/lib/i18n/useLocale";
import type { ScrollableListProps } from "../types";

export function ScrollableList({
  selectedId,
  onSelect,
  emptyMessage: _emptyMessage,
}: ScrollableListProps) {
  const { t } = useLocale();
  const navigate = useNavigate();
  const { data: workspaces, isLoading, error, refetch } = useWorkspaces();
  const [showCreateDialog, setShowCreateDialog] = useState(false);

  // Handle loading state
  if (isLoading) {
    return (
      <div className="flex flex-1 items-center justify-center p-4">
        <LoadingSpinner size="sm" />
      </div>
    );
  }

  // Handle error state
  if (error) {
    return (
      <div className="flex flex-1 flex-col items-center justify-center p-4">
        <ErrorState
          message={t("failedToLoadWorkspaces" as any) || "Failed to load workspaces"}
          onRetry={() => refetch()}
        />
      </div>
    );
  }

  // Handle empty state
  if (!workspaces || workspaces.length === 0) {
    return (
      <div className="flex flex-1 flex-col items-center justify-center p-4">
        <EmptyState
          icon={<FolderOpen className="h-8 w-8 text-muted-foreground" />}
          title={t("noWorkspaces" as any) || "No workspaces"}
          description={t("createWorkspaceHint" as any) || "Create a workspace to get started"}
        />
      </div>
    );
  }

  // Handle item click - navigate to research page with workspace context
  const handleItemClick = (workspaceId: string) => {
    onSelect?.(workspaceId);
    navigate(`/research?workspace=${workspaceId}`);
  };

  // Handle workspace creation success - navigate to new workspace
  const handleCreateSuccess = (workspace: { id: string; name: string }) => {
    setShowCreateDialog(false);
    onSelect?.(workspace.id);
    navigate(`/research?workspace=${workspace.id}`);
  };

  return (
    <div className="flex-1 overflow-y-auto">
      {/* Create Workspace Button */}
      <div className="p-2 border-b border-border">
        <button
          onClick={() => setShowCreateDialog(true)}
          className="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
          aria-label={t("createWorkspace" as any) || "Create workspace"}
        >
          <Plus className="h-4 w-4 flex-shrink-0" />
          <span>{t("createWorkspace" as any) || "Create Workspace"}</span>
        </button>
      </div>

      {/* Workspace List */}
      <div className="space-y-1 p-2">
        {workspaces.map((workspace) => (
          <button
            key={workspace.id}
            onClick={() => handleItemClick(workspace.id)}
            className={`flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left text-sm transition-colors hover:bg-accent ${
              selectedId === workspace.id
                ? "bg-primary/10 font-medium text-primary"
                : ""
            }`}
            aria-label={workspace.name}
          >
            <FolderOpen className="h-4 w-4 flex-shrink-0 text-muted-foreground" />
            <span className="truncate">{workspace.name}</span>
          </button>
        ))}
      </div>

      {/* Create Workspace Dialog */}
      <CreateWorkspaceDialog
        isOpen={showCreateDialog}
        onClose={() => setShowCreateDialog(false)}
        onSuccess={handleCreateSuccess}
      />
    </div>
  );
}