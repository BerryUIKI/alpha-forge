/**
 * Simplified Chinese common messages catalog.
 */

export const common = {
  // Navigation
  today: "今日",
  research: "研究",
  journal: "投资日志",
  portfolio: "投资组合",
  artifacts: "研究产物",
  settings: "设置",

  // Language selection
  language: "语言",
  languageDescription: "中文为默认语言；你可以随时切换到英语。",
  simplifiedChinese: "简体中文",
  english: "English",

  // Backup
  localBackup: "本地备份",
  localBackupDescription: "导出一致的 SQLite 备份。现有文件绝不会被覆盖。",
  exportLocalBackup: "导出本地备份",
  exporting: "正在导出…",
  backupCreated: "备份已创建：{path}",
  backupCancelled: "已取消备份导出。",
  backupFailed: "备份导出失败。请选择一个新的可写文件名后重试。",

  // Updates
  updates: "更新",
  updatesDescription:
    "仅在你请求时检查 GitHub Releases。更新需要手动下载；应用不会自动安装。",
  checkForUpdates: "检查更新",
  checking: "正在检查…",
  updateAvailable: "版本 {version} 可用。",
  upToDate: "已是最新版本（{version}）。",
  updateCheckFailed: "无法检查 GitHub Releases。请检查网络连接后重试。",

  // About and privacy
  aboutAndPrivacy: "关于与隐私",
  aboutAndPrivacyDescription:
    "AlphaForge 是本地优先的开源 MVP。它不要求账户、不进行自动云备份，且默认关闭遥测。你的本地数据库始终是唯一事实来源。",
  openPrivacyNotice: "打开隐私声明",
  openResearchDisclaimer: "打开研究免责声明",

  // Common states
  loading: "加载中…",
  retry: "重试",
  offline: "你已离线",
  offlineDescription: "请检查你的网络连接后重试。",
  unexpectedError: "出了点问题",
  unexpectedErrorDescription: "发生了意外错误。",
} as const;

export type CommonKey = keyof typeof common;