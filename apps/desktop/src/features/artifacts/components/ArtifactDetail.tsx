// Artifact detail with a predefined renderer.

import { X } from "lucide-react";
import { Badge } from "@investment-os/ui";
import type { Artifact } from "@/lib/desktop-api/artifacts";
import { ArtifactRenderer } from "./ArtifactRenderer";
import { artifactTypeLabel } from "./artifactLabels";

interface ArtifactDetailProps {
  artifact: Artifact;
  onClose?: () => void;
}

export function ArtifactDetail({ artifact, onClose }: ArtifactDetailProps) {
  return (
    <div className="rounded-lg border border-border bg-card">
      <div className="flex items-start justify-between gap-3 border-b border-border p-4">
        <div className="min-w-0">
          <h3 className="truncate font-semibold">{artifact.title}</h3>
          <div className="mt-1 flex items-center gap-2">
            <Badge variant="outline">{artifactTypeLabel(artifact.artifact_type)}</Badge>
            <span className="text-xs text-muted-foreground">
              Created {new Date(artifact.created_at).toLocaleString()}
            </span>
          </div>
        </div>
        {onClose && (
          <button
            type="button"
            onClick={onClose}
            aria-label="Close artifact detail"
            className="rounded-md p-1 text-muted-foreground hover:bg-accent hover:text-accent-foreground"
          >
            <X className="h-4 w-4" />
          </button>
        )}
      </div>
      <div className="p-4">
        <ArtifactRenderer artifact={artifact} />
      </div>
    </div>
  );
}
