export const LOCALES = ["zh-CN", "en"] as const;

export type Locale = (typeof LOCALES)[number];

export const DEFAULT_LOCALE: Locale = "zh-CN";
export const LOCALE_SETTING_KEY = "app.locale";

const messages = {
  "zh-CN": {
    today: "今日",
    research: "研究",
    journal: "投资日志",
    portfolio: "投资组合",
    artifacts: "研究产物",
    settings: "设置",
    settingsDescription: "本地优先控制、隐私信息和版本更新。",
    language: "语言",
    languageDescription: "中文为默认语言；你可以随时切换到英语。",
    simplifiedChinese: "简体中文",
    english: "English",
    localBackup: "本地备份",
    localBackupDescription: "导出一致的 SQLite 备份。现有文件绝不会被覆盖。",
    exportLocalBackup: "导出本地备份",
    exporting: "正在导出…",
    backupCreated: "备份已创建：{path}",
    backupCancelled: "已取消备份导出。",
    backupFailed: "备份导出失败。请选择一个新的可写文件名后重试。",
    updates: "更新",
    updatesDescription: "仅在你请求时检查 GitHub Releases。更新需要手动下载；应用不会自动安装。",
    checkForUpdates: "检查更新",
    checking: "正在检查…",
    updateAvailable: "版本 {version} 可用。",
    upToDate: "已是最新版本（{version}）。",
    updateCheckFailed: "无法检查 GitHub Releases。请检查网络连接后重试。",
    aboutAndPrivacy: "关于与隐私",
    aboutAndPrivacyDescription:
      "AlphaForge 是本地优先的开源 MVP。它不要求账户、不进行自动云备份，且默认关闭遥测。你的本地数据库始终是唯一事实来源。",
    openPrivacyNotice: "打开隐私声明",
    openResearchDisclaimer: "打开研究免责声明",
    loading: "加载中…",
    retry: "重试",
    offline: "你已离线",
    offlineDescription: "请检查你的网络连接后重试。",
    unexpectedError: "出了点问题",
    unexpectedErrorDescription: "发生了意外错误。",
    // Workspace
    noWorkspaces: "暂无工作区",
    noWorkspacesDescription: "创建你的第一个工作区，开始整理你的研究。",
    createWorkspace: "创建工作区",
    failedToLoadWorkspaces: "加载工作区失败",
    created: "创建于 {date}",
    createWorkspaceTitle: "创建工作区",
    workspaceName: "工作区名称",
    workspaceNamePlaceholder: "我的研究",
    workspaceNameRequired: "工作区名称为必填项",
    workspaceNameTooLong: "工作区名称不能超过 200 个字符",
    failedToCreateWorkspace: "创建工作区失败",
    cancel: "取消",
    creating: "创建中…",
    create: "创建",
    todayDescription: "你的投资研究仪表板",
    selectWorkspace: "选择工作区",
    selectWorkspaceDescription: "选择一个工作区来查看和管理你的研究，或创建一个新的工作区开始使用。",
    workspaceSelected: "已选择工作区。研究功能将在第二阶段推出。",
    changeWorkspace: "切换工作区",
  },
  "en": {
    today: "Today",
    research: "Research",
    journal: "Journal",
    portfolio: "Portfolio",
    artifacts: "Artifacts",
    settings: "Settings",
    settingsDescription: "Local-first controls, privacy information, and release updates.",
    language: "Language",
    languageDescription: "Chinese is the default language; you can switch to English at any time.",
    simplifiedChinese: "Simplified Chinese",
    english: "English",
    localBackup: "Local backup",
    localBackupDescription:
      "Export a consistent SQLite backup. Existing files are never overwritten.",
    exportLocalBackup: "Export local backup",
    exporting: "Exporting…",
    backupCreated: "Backup created at {path}",
    backupCancelled: "Backup export cancelled.",
    backupFailed: "Backup export failed. Choose a new writable filename and try again.",
    updates: "Updates",
    updatesDescription:
      "Checks GitHub Releases only when you request it. Updates are downloaded manually; nothing is installed automatically.",
    checkForUpdates: "Check for updates",
    checking: "Checking…",
    updateAvailable: "Version {version} is available.",
    upToDate: "You are up to date ({version}).",
    updateCheckFailed: "Could not check GitHub Releases. Check your connection and try again.",
    aboutAndPrivacy: "About and privacy",
    aboutAndPrivacyDescription:
      "AlphaForge is a local-first, open-source MVP. It has no account requirement, no automatic cloud backup, and telemetry is disabled by default. Your local database remains the source of truth.",
    openPrivacyNotice: "Open privacy notice",
    openResearchDisclaimer: "Open research disclaimer",
    loading: "Loading…",
    retry: "Try Again",
    offline: "You are offline",
    offlineDescription: "Please check your internet connection and try again.",
    unexpectedError: "Something went wrong",
    unexpectedErrorDescription: "An unexpected error occurred.",
    // Workspace
    noWorkspaces: "No workspaces yet",
    noWorkspacesDescription: "Create your first workspace to start organizing your research.",
    createWorkspace: "Create Workspace",
    failedToLoadWorkspaces: "Failed to load workspaces",
    created: "Created {date}",
    createWorkspaceTitle: "Create Workspace",
    workspaceName: "Workspace Name",
    workspaceNamePlaceholder: "My Research",
    workspaceNameRequired: "Workspace name is required",
    workspaceNameTooLong: "Workspace name must be 200 characters or less",
    failedToCreateWorkspace: "Failed to create workspace",
    cancel: "Cancel",
    creating: "Creating…",
    create: "Create",
    todayDescription: "Your investment research dashboard",
    selectWorkspace: "Select a Workspace",
    selectWorkspaceDescription:
      "Choose a workspace to view and manage your research, or create a new one to get started.",
    workspaceSelected: "Workspace selected. Research features coming in Phase 2+.",
    changeWorkspace: "Change workspace",
  },
} as const;

export type MessageKey = keyof (typeof messages)[Locale];

export function translate(locale: Locale, key: MessageKey): string {
  return messages[locale][key];
}

export function formatMessage(message: string, values: Record<string, string>): string {
  return Object.entries(values).reduce(
    (formatted, [key, value]) => formatted.replaceAll(`{${key}}`, value),
    message,
  );
}

export function parseLocale(value: string | null | undefined): Locale {
  return LOCALES.includes(value as Locale) ? (value as Locale) : DEFAULT_LOCALE;
}
