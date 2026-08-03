/**
 * English artifacts messages catalog.
 */

export const artifacts = {
  artifactsTitle: "Artifacts",

  // Artifact viewer states
  loadingArtifact: "Loading artifact…",
  errorLoadingArtifact: "Failed to load artifact",
  artifactNotFound: "Artifact not found",
  artifactNotFoundDescription: "The requested artifact could not be found. It may have been deleted or the ID is incorrect.",
  noRendererAvailable: "No renderer available",
  noRendererAvailableDescription: "No renderer is available for artifact type: {type}",
  artifactStatus: "Status",
  artifactCreated: "Created",
} as const;

export type ArtifactsKey = keyof typeof artifacts;