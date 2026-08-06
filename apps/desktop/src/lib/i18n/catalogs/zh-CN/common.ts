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

  // Feature error boundary
  featureErrorTitle: "{feature}错误",
  featureErrorDescription: "此功能遇到错误。你可以重试或导航到其他部分。",
  errorDetails: "错误详情",
  retryFeature: "重试",

  // Feature names
  portfolioFeature: "投资组合",
  thesisFeature: "论点",
  researchFeature: "研究",
  optionsFeature: "期权",
  gooseFeature: "Goose分析",
  artifactsFeature: "研究产物",

  // Functional Views
  functionalView: "功能视图",
  selectFunctionalView: "选择功能视图",
  functionalViews: "功能视图",
  functionalViewAnalyze: "分析",
  functionalViewQuantification: "量化",
  functionalViewComprehensiveMarket: "综合市场",
  functionalViewOptions: "期权",
  functionalViewFutures: "期货",
  functionalViewOtherDerivatives: "其他衍生品",

  // Tools
  tools: "工具",
  toolsList: "工具列表",
  noToolsAvailable: "暂无可用工具",

  // Tool names
  toolResearchProjects: "研究项目",
  toolDocuments: "文档",
  toolThesis: "投资论点",
  toolBacktesting: "回测",
  toolRiskControls: "风险控制",
  toolSignals: "信号",
  toolMarketOverview: "市场概览",
  toolSectorAnalysis: "行业分析",
  toolGreeksCalculator: "Greeks计算器",
  toolStrategyBuilder: "策略构建器",
  toolOptionChain: "期权链",
  toolFuturesChain: "期货链",
  toolTermStructure: "期限结构",
  toolDerivativesOverview: "衍生品概览",

  // Tool descriptions
  toolResearchProjectsDescription: "管理研究项目和来源",
  toolDocumentsDescription: "导入和管理文档",
  toolThesisDescription: "跟踪投资论点和证据",
  toolBacktestingDescription: "运行历史数据回测",
  toolRiskControlsDescription: "设置风险管理规则",
  toolSignalsDescription: "监控交易信号",
  toolMarketOverviewDescription: "查看市场整体情况",
  toolSectorAnalysisDescription: "分析行业表现",
  toolGreeksCalculatorDescription: "计算期权Greeks",
  toolStrategyBuilderDescription: "构建期权策略",
  toolOptionChainDescription: "查看期权链数据",
  toolFuturesChainDescription: "查看期货合约链",
  toolTermStructureDescription: "分析期限结构曲线",
  toolDerivativesOverviewDescription: "衍生品市场概览",
} as const;

export type CommonKey = keyof typeof common;