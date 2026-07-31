// Workspace list component.

import { Plus, Folder } from "lucide-react";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { EmptyState } from "@/components/common/EmptyState";
import { ErrorState } from "@/components/common/ErrorState";
import { useWorkspaces } from "@/features/workspace/hooks/useWorkspaces";
import type { Workspace } from "@/lib/desktop-api/workspace";

interface WorkspaceListProps {
  onSelect?: (workspace: Workspace) => void;
  onCreateNew?: () => void;
}

export function WorkspaceList({ onSelect, onCreateNew }: WorkspaceListProps) {
  const { data: workspaces, isLoading, error, refetch } = useWorkspaces();

  if (isLoading) {
    return <LoadingSpinner className="p-8" />;
  }

  if (error) {
    return (
      <ErrorState
        message="Failed to load workspaces"
        onRetry={() => refetch()}
      />
    );
  }

  if (!workspaces || workspaces.length === 0) {
    return (
      <EmptyState
        icon={<Folder className="h-8 w-8" />}
        title="No workspaces yet"
        description="Create your first workspace to start organizing your research."
        action={
          onCreateNew && (
            <button
              onClick={onCreateNew}
              className="inline-flex items-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
            >
              <Plus className="mr-2 h-4 w-4" />
              Create Workspace
            </button>
          )
        }
      />
    );
  }

  return (
    <div className="space-y-2">
      {workspaces.map((workspace) => (
        <button
          key={workspace.id}
          onClick={() => onSelect?.(workspace)}
          className="w-full rounded-lg border border-border bg-card p-4 text-left transition-colors hover:border-primary hover:bg-accent"
        >
          <h3 className="font-semibold">{workspace.name}</h3>
          <p className="text-sm text-muted-foreground">
            Created {new Date(workspace.createdAt).toLocaleDateString()}
          </p>
        </button>
      ))}
    </div>
  );
}