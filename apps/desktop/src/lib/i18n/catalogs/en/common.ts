/**
 * English common messages catalog.
 * Source locale for all translations.
 */

export const common = {
  // Navigation
  today: "Today",
  research: "Research",
  journal: "Journal",
  portfolio: "Portfolio",
  artifacts: "Artifacts",
  settings: "Settings",

  // Language selection
  language: "Language",
  languageDescription:
    "Chinese is the default language; you can switch to English at any time.",
  simplifiedChinese: "Simplified Chinese",
  english: "English",

  // Backup
  localBackup: "Local backup",
  localBackupDescription:
    "Export a consistent SQLite backup. Existing files are never overwritten.",
  exportLocalBackup: "Export local backup",
  exporting: "Exporting…",
  backupCreated: "Backup created at {path}",
  backupCancelled: "Backup export cancelled.",
  backupFailed:
    "Backup export failed. Choose a new writable filename and try again.",

  // Updates
  updates: "Updates",
  updatesDescription:
    "Checks GitHub Releases only when you request it. Updates are downloaded manually; nothing is installed automatically.",
  checkForUpdates: "Check for updates",
  checking: "Checking…",
  updateAvailable: "Version {version} is available.",
  upToDate: "You are up to date ({version}).",
  updateCheckFailed:
    "Could not check GitHub Releases. Check your connection and try again.",

  // About and privacy
  aboutAndPrivacy: "About and privacy",
  aboutAndPrivacyDescription:
    "AlphaForge is a local-first, open-source MVP. It has no account requirement, no automatic cloud backup, and telemetry is disabled by default. Your local database remains the source of truth.",
  openPrivacyNotice: "Open privacy notice",
  openResearchDisclaimer: "Open research disclaimer",

  // Common states
  loading: "Loading…",
  retry: "Try Again",
  offline: "You are offline",
  offlineDescription: "Please check your internet connection and try again.",
  unexpectedError: "Something went wrong",
  unexpectedErrorDescription: "An unexpected error occurred.",
} as const;

export type CommonKey = keyof typeof common;