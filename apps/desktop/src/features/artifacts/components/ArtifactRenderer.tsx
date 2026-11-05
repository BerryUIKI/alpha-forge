// Artifact renderer registry.
//
// Only predefined React renderers are used; the agent never supplies HTML.

import type { Artifact } from "@/lib/desktop-api/artifacts";
import { parseArtifactData } from "./parseArtifactData";
import { ComparisonTable } from "./renderers/ComparisonTable";
import { Timeline } from "./renderers/Timeline";

export function ArtifactRenderer({ artifact }: { artifact: Artifact }) {
  const parsed = parseArtifactData(artifact.artifact_type, artifact.data);

  switch (parsed.kind) {
    case "comparison-table":
      return <ComparisonTable data={parsed.data} />;
    case "timeline":
      return <Timeline data={parsed.data} />;
    default:
      return (
        <div className="rounded-md border border-dashed border-border p-4 text-sm text-muted-foreground">
          {parsed.reason}
        </div>
      );
  }
}
