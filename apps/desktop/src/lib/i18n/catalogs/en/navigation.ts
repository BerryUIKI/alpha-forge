/**
 * English navigation messages catalog.
 */

export const navigation = {
  today: "Today",
  research: "Research",
  journal: "Journal",
  portfolio: "Portfolio",
  artifacts: "Artifacts",
  settings: "Settings",
} as const;

export type NavigationKey = keyof typeof navigation;