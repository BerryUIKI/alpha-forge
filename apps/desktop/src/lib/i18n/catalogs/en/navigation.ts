/**
 * English navigation messages catalog.
 */

export const navigation = {
  dashboard: "Dashboard",
  today: "Today",
  research: "Research",
  theses: "Theses",
  journal: "Journal",
  portfolio: "Portfolio",
  knowledge: "Knowledge",
  options: "Options",
  artifacts: "Artifacts",
  settings: "Settings",
  cockpit: "Cockpit",
  pipeline: "Core Pipeline",
  tools: "Tools",
  knowledgeTools: "Knowledge & Tools",
} as const;

export type NavigationKey = keyof typeof navigation;