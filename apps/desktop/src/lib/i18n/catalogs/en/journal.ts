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
} as const;

export type JournalKey = keyof typeof journal;