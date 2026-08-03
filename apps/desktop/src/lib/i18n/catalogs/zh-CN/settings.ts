/**
 * Simplified Chinese settings messages catalog.
 */

export const settings = {
  title: "设置",
  description: "本地优先控制、隐私信息和版本更新。",

  // Language section
  language: "语言",
  languageDescription: "中文为默认语言；你可以随时切换到英语。",
  simplifiedChinese: "简体中文",
  english: "English",

  // Backup section
  backup: "本地备份",
  backupDescription: "导出一致的 SQLite 备份。现有文件绝不会被覆盖。",
  backupExport: "导出本地备份",
  backupExporting: "正在导出…",
  backupSuccess: "备份已创建：{path}",
  backupCancelled: "已取消备份导出。",
  backupFailed: "备份导出失败。请选择一个新的可写文件名后重试。",

  // Updates section
  updates: "更新",
  updatesDescription:
    "仅在你请求时检查 GitHub Releases。更新需要手动下载；应用不会自动安装。",
  updatesCheck: "检查更新",
  updatesChecking: "正在检查…",
  updatesAvailable: "版本 {version} 可用。",
  updatesUptodate: "已是最新版本（{version}）。",
  updatesFailed: "无法检查 GitHub Releases。请检查网络连接后重试。",

  // Privacy section
  privacy: "关于与隐私",
  privacyDescription:
    "AlphaForge 是本地优先的开源 MVP。它不要求账户、不进行自动云备份，且默认关闭遥测。你的本地数据库始终是唯一事实来源。",
  privacyNotice: "打开隐私声明",
  privacyDisclaimer: "打开研究免责声明",

  // Database health section
  databaseHealth: "数据库健康",
  databaseHealthDescription: "检查本地 SQLite 数据库的完整性和连接状态。",
  checkDatabaseHealth: "检查数据库健康",
  checking: "正在检查…",
  databaseHealthy: "数据库健康检查完成。",
  databaseCheckFailed: "数据库健康检查失败。",
  databaseStatusHealthy: "健康",
  databaseStatusError: "错误",
} as const;

export type SettingsKey = keyof typeof settings;