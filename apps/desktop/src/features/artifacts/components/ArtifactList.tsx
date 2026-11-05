// Artifact list.

import { cn } from "@investment-os/ui";
import type { Artifact } from "@/lib/desktop-api/artifacts";
import { artifactTypeLabel } from "./artifactLabels";

interface ArtifactListProps {
  artifacts: Artifact[];
  selectedArtifactId?: string;
  onSelect: (artifact: Artifact) => void;
}

export function ArtifactList({ artifacts, selectedArtifactId, onSelect }: ArtifactListProps) {
  return (
    <div className="space-y-3">
      {artifacts.map((artifact) => {
        const isSelected = selectedArtifactId === artifact.id;

        return (
          <button
            key={artifact.id}
            type="button"
            onClick={() => onSelect(artifact)}
            aria-pressed={isSelected}
            className={cn(
              "w-full rounded-lg border bg-card p-4 text-left transition-colors hover:border-primary hover:bg-accent",
              isSelected ? "border-primary" : "border-border",
            )}
          >
            <h3 className="truncate font-semibold">{artifact.title}</h3>
            <p className="mt-1 text-xs text-muted-foreground">
              {artifactTypeLabel(artifact.artifact_type)} · Created{" "}
              {new Date(artifact.created_at).toLocaleDateString()}
            </p>
          </button>
        );
      })}
    </div>
  );
}
