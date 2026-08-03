/**
 * English settings messages catalog.
 */

export const settings = {
  title: "Settings",
  description: "Local-first controls, privacy information, and release updates.",

  // Language section
  language: "Language",
  languageDescription:
    "Chinese is the default language; you can switch to English at any time.",
  simplifiedChinese: "Simplified Chinese",
  english: "English",

  // Backup section
  backup: "Local backup",
  backupDescription:
    "Export a consistent SQLite backup. Existing files are never overwritten.",
  backupExport: "Export local backup",
  backupExporting: "Exporting…",
  backupSuccess: "Backup created at {path}",
  backupCancelled: "Backup export cancelled.",
  backupFailed:
    "Backup export failed. Choose a new writable filename and try again.",

  // Updates section
  updates: "Updates",
  updatesDescription:
    "Checks GitHub Releases only when you request it. Updates are downloaded manually; nothing is installed automatically.",
  updatesCheck: "Check for updates",
  updatesChecking: "Checking…",
  updatesAvailable: "Version {version} is available.",
  updatesUptodate: "You are up to date ({version}).",
  updatesFailed:
    "Could not check GitHub Releases. Check your connection and try again.",

  // Privacy section
  privacy: "About and privacy",
  privacyDescription:
    "AlphaForge is a local-first, open-source MVP. It has no account requirement, no automatic cloud backup, and telemetry is disabled by default. Your local database remains the source of truth.",
  privacyNotice: "Open privacy notice",
  privacyDisclaimer: "Open research disclaimer",
} as const;

export type SettingsKey = keyof typeof settings;