/**
 * English research messages catalog.
 */

export const research = {
  // Page header
  researchTitle: "Research",
  researchDescription: "Capture projects, source provenance, and document annotations.",

  // Workspace selection
  workspace: "Workspace",
  selectWorkspace: "Select a workspace",

  // Projects section
  projects: "Projects",
  projectTitle: "Project title",
  create: "Create",

  // Documents section
  documents: "Documents",
  documentTitle: "Document title",
  add: "Add",

  // Import
  importPdf: "Import PDF",
  importingPdf: "Importing PDF…",
  importWebPage: "Import web page",
  importing: "Importing…",
  webPageUrl: "Web page URL",
  webPageUrlPlaceholder: "https://example.com/research",
  importHint: "PDFs use the native picker; web pages are fetched in Rust from validated public HTTPS URLs.",

  // Reports section
  reports: "Reports",
  reportTitle: "Report title",
  reportContent: "Report findings",
  reportType: "Report type",
  reportTypeAnalysis: "Analysis",
  reportTypeSummary: "Summary",
  reportTypeThesis: "Thesis",
  reportTypeRecommendation: "Recommendation",
  saveReport: "Save report",

  // Notes section
  notes: "Notes",
  noteContent: "Note content",
  addNote: "Add note",

  // Sources section
  sources: "Sources",
  sourceUrl: "Source URL",
  sourceUrlPlaceholder: "https://example.com",
  sourceTitle: "Source title",
  sourceTitlePlaceholder: "Source title (optional)",
  addSource: "Add source",
  sourcesHint: "Sources must use public HTTPS hostnames.",
  untitledSource: "Untitled source",

  // Search section
  searchDocument: "Search this document",
  searchPlaceholder: "Find terms in the saved document text",
  searchMode: "Search mode",
  searchModeLexical: "Exact terms",
  searchModeSemantic: "Related terms",
  search: "Search",
  searchHint: "Related terms use a local, explainable investment vocabulary; they are not AI-generated recommendations.",
  searchScore: "Score",

  // Errors
  saveError: "Unable to save the research item. Check the required fields and try again.",
} as const;

export type ResearchKey = keyof typeof research;