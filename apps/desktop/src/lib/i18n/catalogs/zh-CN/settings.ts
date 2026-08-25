/**
 * Simplified Chinese settings messages catalog.
 */

export const settings = {
  title: "设置",
  description: "本地优先控制、隐私信息和版本更新。",
  settingsNavigation: "设置导航",
  backToApplication: "返回主界面",
  settingsGeneral: "通用",
  settingsAppearance: "外观",
  settingsLocalization: "本地化",
  settingsAgents: "Agents",
  settingsData: "数据与备份",
  settingsPlugins: "内部插件",
  settingsAbout: "关于",
  appearanceDescription: "统一设置界面模式、强调色和行情涨跌颜色。",
  themeMode: "界面模式",
  marketColorScheme: "涨跌颜色",
  marketColorsGlobal: "绿涨红跌（国际）",
  marketColorsChina: "红涨绿跌（中国）",
  accentColor: "强调色",
  professionalTerminology: "专业术语本地化",
  professionalTerminologyDescription: "可选择是否翻译金融与研究领域术语，并覆盖默认译法。",
  enableProfessionalTerminology: "启用专业术语本地化翻译",
  editTerminology: "编辑术语译法",
  apiUsage: "API 用量",
  apiUsageNotCollecting: "统一供应商用量账本尚未接入；当前不会估算或伪造 Token 与费用数据。",

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

  // Internal plugins section
  internalPlugins: "内部插件",
  internalPluginsDescription: "查看内置插件并控制它们是否可以创建研究产物。此处不安装远程插件。",
  loadingInternalPlugins: "正在加载内部插件…",
  failedToLoadInternalPlugins: "加载内部插件失败。",
  failedToUpdateInternalPlugin: "更新插件状态失败。请重试。",
  noInternalPlugins: "暂无内部插件",
  noInternalPluginsDescription: "此版本没有注册任何内部插件。",
  internalPluginBadge: "内置",
  pluginPermissions: "权限",
  noPluginPermissions: "无",
  pluginPermissionNetwork: "网络",
  enableInternalPlugin: "启用内部插件",
  disableInternalPlugin: "禁用内部插件",
  pluginEnabled: "已启用",
  pluginDisabled: "已禁用",
  updatingInternalPlugin: "更新中…",
} as const;

export type SettingsKey = keyof typeof settings;
