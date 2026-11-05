// Artifacts page — validated visual outputs from agent tasks.

import { useState } from "react";
import { Boxes } from "lucide-react";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { EmptyState } from "@/components/common/EmptyState";
import { ErrorState } from "@/components/common/ErrorState";
import { ArtifactDetail, ArtifactList, useArtifacts } from "@/features/artifacts";
import type { Artifact } from "@/lib/desktop-api";

export function ArtifactsPage() {
  const { data: artifacts, isLoading, error, refetch } = useArtifacts();
  const [selectedArtifactId, setSelectedArtifactId] = useState<string | null>(null);

  const selectedArtifact =
    artifacts?.find((artifact: Artifact) => artifact.id === selectedArtifactId) ?? null;

  return (
    <div className="p-6">
      <div className="mb-6">
        <h1 className="text-2xl font-bold">Artifacts</h1>
        <p className="mt-2 text-muted-foreground">
          Validated visual outputs produced by agent tasks.
        </p>
      </div>

      {isLoading ? (
        <LoadingSpinner className="p-8" />
      ) : error ? (
        <ErrorState message="Failed to load artifacts" onRetry={() => refetch()} />
      ) : !artifacts || artifacts.length === 0 ? (
        <EmptyState
          icon={<Boxes className="h-8 w-8" />}
          title="No artifacts yet"
          description="Run an agent task that produces a comparison table or timeline to see it here."
        />
      ) : (
        <div
          className={
            selectedArtifact
              ? "grid gap-4 xl:grid-cols-[minmax(0,1fr)_minmax(0,1.2fr)]"
              : "max-w-3xl"
          }
        >
          <ArtifactList
            artifacts={artifacts}
            selectedArtifactId={selectedArtifactId ?? undefined}
            onSelect={(artifact) => {
              setSelectedArtifactId(artifact.id);
            }}
          />
          {selectedArtifact && (
            <ArtifactDetail
              artifact={selectedArtifact}
              onClose={() => {
                setSelectedArtifactId(null);
              }}
            />
          )}
        </div>
      )}
    </div>
  );
}
