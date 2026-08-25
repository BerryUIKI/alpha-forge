/**
 * English settings messages catalog.
 */

export const settings = {
  title: "Settings",
  description: "Local-first controls, privacy information, and release updates.",
  settingsNavigation: "Settings navigation",
  backToApplication: "Back to application",
  settingsGeneral: "General",
  settingsAppearance: "Appearance",
  settingsLocalization: "Localization",
  settingsAgents: "Agents",
  settingsData: "Data and backup",
  settingsPlugins: "Internal plugins",
  settingsAbout: "About",
  appearanceDescription: "Set the interface mode, accent color, and market gain/loss colors.",
  themeMode: "Interface mode",
  marketColorScheme: "Gain and loss colors",
  marketColorsGlobal: "Green gains, red losses (global)",
  marketColorsChina: "Red gains, green losses (China)",
  accentColor: "Accent color",
  professionalTerminology: "Professional terminology localization",
  professionalTerminologyDescription: "Choose whether domain terms are translated and override the default wording.",
  enableProfessionalTerminology: "Enable professional terminology translation",
  editTerminology: "Edit terminology",
  apiUsage: "API usage",
  apiUsageNotCollecting: "The unified provider usage ledger is not connected yet; token and cost data will not be estimated or fabricated.",

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

  // Database health section
  databaseHealth: "Database health",
  databaseHealthDescription: "Check the integrity and connectivity of the local SQLite database.",
  checkDatabaseHealth: "Check database health",
  checking: "Checking…",
  databaseHealthy: "Database health check complete.",
  databaseCheckFailed: "Database health check failed.",
  databaseStatusHealthy: "Healthy",
  databaseStatusError: "Error",

  // Internal plugins section
  internalPlugins: "Internal plugins",
  internalPluginsDescription:
    "Review bundled plugins and control whether they may create Artifacts. Remote plugins cannot be installed here.",
  loadingInternalPlugins: "Loading internal plugins…",
  failedToLoadInternalPlugins: "Failed to load internal plugins.",
  failedToUpdateInternalPlugin: "Failed to update the plugin status. Try again.",
  noInternalPlugins: "No internal plugins",
  noInternalPluginsDescription: "No internal plugins are registered in this build.",
  internalPluginBadge: "Bundled",
  pluginPermissions: "Permissions",
  noPluginPermissions: "None",
  pluginPermissionNetwork: "Network",
  enableInternalPlugin: "Enable internal plugin",
  disableInternalPlugin: "Disable internal plugin",
  pluginEnabled: "Enabled",
  pluginDisabled: "Disabled",
  updatingInternalPlugin: "Updating…",
} as const;

export type SettingsKey = keyof typeof settings;
