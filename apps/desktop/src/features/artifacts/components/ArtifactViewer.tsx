// Generic artifact viewer component.

import { useArtifact } from "../hooks/useArtifacts";
import { artifactRegistry } from "../renderers/registry";
import { ErrorState, LoadingSpinner, EmptyState } from "@/components/common";
import { useLocale } from "@/lib/i18n/useLocale";
import { formatMessage } from "@/lib/i18n/locale";

interface ArtifactViewerProps {
  artifactId: string;
}

/**
 * Generic artifact viewer that renders the appropriate component based on artifact type.
 */
export function ArtifactViewer({ artifactId }: ArtifactViewerProps) {
  const { t } = useLocale();
  const { data: artifact, isLoading, error, refetch } = useArtifact(artifactId);

  if (isLoading) {
    return <LoadingSpinner className="h-64" ariaLabel={t("loadingArtifact")} />;
  }

  if (error) {
    return (
      <ErrorState
        title={t("errorLoadingArtifact")}
        message={error.message}
        retryLabel={t("retry")}
        onRetry={() => refetch()}
      />
    );
  }

  if (!artifact) {
    return (
      <EmptyState
        title={t("artifactNotFound")}
        description={t("artifactNotFoundDescription")}
      />
    );
  }

  // Get renderer for this artifact type
  const Renderer = artifactRegistry.getRenderer(artifact.artifact_type);

  if (!Renderer) {
    return (
      <EmptyState
        title={t("noRendererAvailable")}
        description={formatMessage(t("noRendererAvailableDescription"), { type: artifact.artifact_type })}
      />
    );
  }

  // Render with output data if available, otherwise input
  const data = artifact.output || artifact.input;

  return (
    <div className="h-full flex flex-col">
      <div className="border-b p-4">
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-lg font-semibold">{artifact.artifact_type}</h2>
            <p className="text-sm text-muted-foreground">
              {t("artifactStatus")}: {artifact.status}
            </p>
          </div>
          <div className="text-sm text-muted-foreground">
            {t("artifactCreated")}: {new Date(artifact.created_at).toLocaleString()}
          </div>
        </div>
      </div>
      <div className="flex-1 overflow-auto">
        <Renderer artifactId={artifact.id} data={data} />
      </div>
    </div>
  );
}
