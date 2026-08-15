/**
 * English artifacts messages catalog.
 */

export const artifacts = {
  artifactsTitle: "Artifacts",
  createCompanyComparison: "Create company comparison",
  createCompanyComparisonDescription:
    "Enter two unique tickers and one recorded metric. AlphaForge stores the comparison without fetching market data or making a recommendation.",
  loadingCompanyComparisonPlugin: "Loading company comparison plugin…",
  failedToLoadCompanyComparisonPlugin: "Failed to load the company comparison plugin.",
  companyComparisonPluginDisabled: "Company comparison is disabled",
  companyComparisonPluginDisabledDescription:
    "Enable the bundled company-comparison plugin in Settings before creating an Artifact.",
  manageInternalPlugins: "Manage internal plugins",
  firstCompanyTicker: "First ticker",
  firstCompanyMetric: "First value",
  secondCompanyTicker: "Second ticker",
  secondCompanyMetric: "Second value",
  comparisonDimension: "Comparison dimension",
  comparisonDimensionRevenue: "Revenue",
  comparisonDimensionMarketCap: "Market capitalization",
  comparisonDimensionPeRatio: "P/E ratio",
  createAndOpenCompanyComparison: "Create and open Artifact",
  creatingCompanyComparison: "Creating…",
  invalidCompanyComparison:
    "Enter two different valid tickers and a finite value for each company.",
  failedToCreateCompanyComparison: "Unable to create the company-comparison Artifact.",
  companyComparisonCreatedOpenFailed:
    "The Artifact was created, but its isolated window could not be opened.",
  retryOpenArtifact: "Retry opening",

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
