import { useEffect, useState } from "react";
import { FileText, Trash2 } from "lucide-react";
import { EmptyState, ErrorState, LoadingSpinner } from "@/components/common";
import { useWorkspaces } from "@/features/workspace/hooks/useWorkspaces";
import { useActiveWorkspaceId } from "@/features/workspace/hooks/useActiveWorkspace";
import { useArtifacts, useDeleteArtifact } from "@/features/artifacts";
import { ArtifactViewer } from "@/features/artifacts/components/ArtifactViewer";
import { CompanyComparisonArtifactForm } from "@/features/plugins";
import { useLocale } from "@/lib/i18n/useLocale";
import type { MessageKey } from "@/lib/i18n/locale";
import type { Artifact } from "@/lib/desktop-api/artifacts";

export function ArtifactsPage() {
  const { t } = useLocale();
  // Loading/error states come from the workspace list query; the active
  // workspace itself comes from the global context (ADR-0008).
  const workspaces = useWorkspaces();
  const workspaceId = useActiveWorkspaceId();
  const [selectedArtifactId, setSelectedArtifactId] = useState<string | null>(null);
  const [deleteError, setDeleteError] = useState<string | null>(null);
  const artifacts = useArtifacts(workspaceId);
  const deleteArtifact = useDeleteArtifact();

  // Reset the selected artifact when the active workspace changes.
  useEffect(() => {
    setSelectedArtifactId(null);
  }, [workspaceId]);

  // Handle artifact deletion
  const handleDeleteArtifact = async (id: string) => {
    if (!window.confirm(t("confirmDeleteArtifact"))) {
      return;
    }
    setDeleteError(null);
    try {
      await deleteArtifact.mutateAsync(id);
      if (selectedArtifactId === id) {
        setSelectedArtifactId(null);
      }
    } catch {
      setDeleteError(t("failedToDeleteArtifact"));
    }
  };

  // Loading state
  if (workspaces.isLoading) {
    return <LoadingSpinner className="p-8" ariaLabel={t("loading")} />;
  }

  // Error state
  if (workspaces.error) {
    return (
      <ErrorState
        message={t("failedToLoadWorkspaces")}
        retryLabel={t("retry")}
        onRetry={() => workspaces.refetch()}
      />
    );
  }

  // Empty workspace state
  if (!workspaces.data?.length) {
    return (
      <div className="p-6">
        <h1 className="text-2xl font-bold">{t("artifactsTitle")}</h1>
        <div className="mt-6">
          <EmptyState
            title={t("createWorkspaceFirst")}
            description={t("createWorkspaceFirstDescription")}
          />
        </div>
      </div>
    );
  }

  return (
    <div className="flex h-full flex-col">
      {/* Header */}
      <div className="border-b border-border p-6">
        <h1 className="text-2xl font-bold">{t("artifactsTitle")}</h1>
        <p className="mt-1 text-sm text-muted-foreground">
          {t("artifactsDescription")}
        </p>
      </div>

      {deleteError && (
        <div
          className="border-b border-destructive/30 bg-destructive/10 px-6 py-2 text-sm text-destructive"
          role="alert"
        >
          {deleteError}
        </div>
      )}

      <CompanyComparisonArtifactForm
        workspaceId={workspaceId}
        onArtifactCreated={setSelectedArtifactId}
      />

      {/* Main Content */}
      <div className="flex flex-1 overflow-hidden">
        {/* Artifact List */}
        <div className="w-80 border-r border-border overflow-y-auto">
          {artifacts.isLoading ? (
            <LoadingSpinner className="p-6" />
          ) : artifacts.error ? (
            <div className="p-4">
              <ErrorState
                message={t("failedToLoadArtifacts")}
                onRetry={() => artifacts.refetch()}
              />
            </div>
          ) : !artifacts.data?.length ? (
            <div className="p-4">
              <EmptyState
                title={t("noArtifacts")}
                description={t("noArtifactsDescription")}
              />
            </div>
          ) : (
            <div className="p-2 space-y-1">
              {artifacts.data.map((artifact) => (
                <ArtifactListItem
                  key={artifact.id}
                  artifact={artifact}
                  isSelected={selectedArtifactId === artifact.id}
                  onSelect={() => setSelectedArtifactId(artifact.id)}
                  _onDelete={() => handleDeleteArtifact(artifact.id)}
                  t={t}
                />
              ))}
            </div>
          )}
        </div>

        {/* Artifact Viewer */}
        <div className="flex-1 overflow-hidden">
          {selectedArtifactId ? (
            <ArtifactViewer artifactId={selectedArtifactId} />
          ) : (
            <div className="flex h-full items-center justify-center">
              <EmptyState
                title={t("selectArtifact")}
                description={t("selectArtifactDescription")}
              />
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

interface ArtifactListItemProps {
  artifact: Artifact;
  isSelected: boolean;
  onSelect: () => void;
  _onDelete: () => void;
  t: (key: MessageKey) => string;
}

function ArtifactListItem({
  artifact,
  isSelected,
  onSelect,
  _onDelete,
  t,
}: ArtifactListItemProps) {
  const getStatusColor = (status: string) => {
    switch (status) {
      case "completed":
        return "bg-green-500";
      case "generating":
        return "bg-blue-500";
      case "failed":
        return "bg-red-500";
      default:
        return "bg-gray-400";
    }
  };

  const getStatusLabel = (status: string) => {
    const statusMap: Record<string, string> = {
      pending: t("artifactStatusPending"),
      generating: t("artifactStatusGenerating"),
      completed: t("artifactStatusCompleted"),
      viewing: t("artifactStatusViewing"),
      closed: t("artifactStatusClosed"),
      failed: t("artifactStatusFailed"),
    };
    return statusMap[status] || status;
  };

  return (
    <div
      onClick={onSelect}
      role="button"
      tabIndex={0}
      onKeyDown={(e) => {
        if (e.target !== e.currentTarget) return;
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          onSelect();
        }
      }}
      className={`group w-full text-left p-3 rounded-lg border transition-colors cursor-pointer ${
        isSelected
          ? "border-primary bg-primary/5"
          : "border-border hover:bg-accent"
      }`}
    >
      <div className="flex items-start gap-2">
        <FileText className="h-4 w-4 mt-0.5 text-muted-foreground flex-shrink-0" />
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2">
            <span className="font-medium text-sm truncate">
              {artifact.artifact_type}
            </span>
            <div
              className={`w-2 h-2 rounded-full ${getStatusColor(artifact.status)}`}
              title={getStatusLabel(artifact.status)}
            />
          </div>
          <div className="mt-1 text-xs text-muted-foreground">
            {new Date(artifact.created_at).toLocaleDateString()}
          </div>
          {artifact.error && (
            <div className="mt-1 text-xs text-destructive truncate">
              {artifact.error}
            </div>
          )}
        </div>
        <button
          type="button"
          onClick={(e) => {
            e.stopPropagation();
            _onDelete();
          }}
          aria-label={t("deleteArtifact")}
          title={t("deleteArtifact")}
          className="rounded-md p-1 text-muted-foreground opacity-0 transition-opacity group-hover:opacity-100 hover:bg-destructive/10 hover:text-destructive focus:opacity-100"
        >
          <Trash2 className="h-3.5 w-3.5" />
        </button>
      </div>
    </div>
  );
}
