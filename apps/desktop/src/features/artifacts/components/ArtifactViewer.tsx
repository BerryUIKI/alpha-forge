// Generic artifact viewer component.

import { useArtifact } from "../hooks/useArtifacts";
import { artifactRegistry } from "./registry";

interface ArtifactViewerProps {
  artifactId: string;
}

/**
 * Generic artifact viewer that renders the appropriate component based on artifact type.
 */
export function ArtifactViewer({ artifactId }: ArtifactViewerProps) {
  const { data: artifact, isLoading, error } = useArtifact(artifactId);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-muted-foreground">Loading artifact...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-red-600">Error loading artifact: {error.message}</div>
      </div>
    );
  }

  if (!artifact) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-muted-foreground">Artifact not found</div>
      </div>
    );
  }

  // Get renderer for this artifact type
  const Renderer = artifactRegistry.getRenderer(artifact.artifact_type);

  if (!Renderer) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-muted-foreground">
          No renderer available for artifact type: {artifact.artifact_type}
        </div>
      </div>
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
              Status: {artifact.status}
            </p>
          </div>
          <div className="text-sm text-muted-foreground">
            Created: {new Date(artifact.created_at).toLocaleString()}
          </div>
        </div>
      </div>
      <div className="flex-1 overflow-auto">
        <Renderer artifactId={artifact.id} data={data} />
      </div>
    </div>
  );
}