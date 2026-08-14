/**
 * English artifacts messages catalog.
 */

export const artifacts = {
  artifactsTitle: "Artifacts",

  // Artifact viewer states
  loadingArtifact: "Loading artifact…",
  errorLoadingArtifact: "Failed to load artifact",
  artifactNotFound: "Artifact not found",
  artifactNotFoundDescription:
    "The requested artifact could not be found. It may have been deleted or the ID is incorrect.",
  artifactWindowInvalidRoute: "Invalid artifact window route",
  artifactWindowInvalidRouteDescription: "The artifact ID or type in this window URL is not valid.",
  artifactWindowMismatch: "Artifact route mismatch",
  artifactWindowMismatchDescription:
    "The requested artifact does not match the type encoded in this window URL.",
  artifactWindowNoData: "Artifact has no renderable data",
  artifactWindowNoDataDescription:
    "This artifact does not have input or output data to render yet.",
  closeArtifactWindow: "Close artifact window",
  artifactWindowCloseFailed: "Unable to close artifact window",
  noRendererAvailable: "No renderer available",
  noRendererAvailableDescription: "No renderer is available for artifact type: {type}",
  artifactStatus: "Status",
  artifactCreated: "Created",
} as const;

export type ArtifactsKey = keyof typeof artifacts;
