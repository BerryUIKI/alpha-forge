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
} as const;

export type NavigationKey = keyof typeof navigation;