/**
 * English journal/thesis messages catalog.
 */

export const journal = {
  // Page header
  journalTitle: "Journal",
  
  // Thesis dashboard
  investmentTheses: "Investment theses",
  thesisDescription: "Make your reasoning explicit, track evidence, and validate outcomes.",
  
  // Empty/error states
  createWorkspaceFirst: "Create a workspace first",
  createWorkspaceFirstDescription: "Theses are stored in a workspace so their evidence remains organized.",
  selectThesis: "Select a thesis",
  selectThesisDescription: "Choose a thesis to review its confidence, lifecycle, and evidence.",
  thesisFailedToLoadWorkspaces: "Failed to load workspaces.",
  
  // Create thesis form
  newInvestmentThesis: "New investment thesis",
  newThesisDescription: "Capture a testable claim before you start tracking evidence.",
  titleLabel: "Title",
  titlePlaceholder: "AI infrastructure demand remains durable",
  thesisStatementLabel: "Thesis statement",
  thesisStatementPlaceholder: "State the claim, why it may be true, and what could disprove it.",
  initialConfidence: "Initial confidence",
  creatingThesis: "Creating thesis…",
  createThesis: "Create thesis",
  thesisTitleRequired: "A title and thesis statement are required.",
  unableToCreateThesis: "Unable to create the thesis.",

  // Thesis Pipeline Board & Cards
  thesesPipelineTitle: "Thesis Pipeline Tracker",
  thesesPipelineSub: "From hypothesis to sizing to ongoing falsification tracking",
  viewBoard: "Board View",
  viewList: "List View",
  statusDraft: "Draft",
  statusActive: "Active",
  statusValidating: "Validating",
  statusValidated: "Validated",
  statusClosed: "Closed",
  convictionLabel: "Conviction",
  evidenceCountPill: "{supporting} pro · {contradicting} contra",
  supportingCount: "{count} supporting",
  contradictingCount: "{count} contra",
  noThesesInStage: "No theses in this stage",
  filterStatus: "Filter status",
  allStatuses: "All Statuses",
  cardDetails: "View Details",
  cardQuickAction: "Transition Status",
  confidenceScore: "{score}% Conviction",
} as const;

export type JournalKey = keyof typeof journal;