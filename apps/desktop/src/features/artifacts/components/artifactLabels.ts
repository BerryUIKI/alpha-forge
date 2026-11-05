// Artifact type labels.

export const artifactTypeLabels: Record<string, string> = {
  "comparison-table": "Comparison table",
  timeline: "Timeline",
};

export function artifactTypeLabel(artifactType: string): string {
  return artifactTypeLabels[artifactType] ?? artifactType;
}
