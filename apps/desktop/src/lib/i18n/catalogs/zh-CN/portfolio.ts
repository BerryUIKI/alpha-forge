/**
 * Simplified Chinese portfolio messages catalog.
 */

export const portfolio = {
  // Page header
  portfolioTitle: "投资组合",
  portfolioDescription: "跟踪持仓以便审查和风险分析。此工作区从不执行交易。",

  // Workspace selection
  workspaceLabel: "工作区",

  // Create account form
  newAccount: "新建账户",
  newAccountDescription: "添加持仓来源仅用于跟踪。",
  accountNameLabel: "账户名称",
  accountNamePlaceholder: "主要券商账户",
  accountTypeLabel: "账户类型",
  accountCurrencyLabel: "账户货币",
  accountTypeBrokerage: "券商",
  accountTypeRetirement: "退休",
  accountTypeCash: "现金",
  accountTypeOther: "其他",
  creatingAccount: "创建中…",
  addAccount: "添加账户",
  accountNameRequired: "账户名称为必填项。",
  unableToCreateAccount: "无法创建账户。",

  // Account list
  accounts: "账户",
  noAccountsYet: "暂无账户",
  noAccountsDescription: "添加账户以开始跟踪持仓。",
  failedToLoadAccounts: "加载投资组合账户失败。",

  // Position panel
  symbolLabel: "标的代码",
  quantityLabel: "数量",
  costBasisLabel: "成本基础",
  addHolding: "添加持仓",
  addingHolding: "添加中…",
  noHoldingsYet: "暂无持仓",
  noHoldingsDescription: "手动记录持仓，或在下方导入交易历史。",
  failedToLoadHoldings: "加载持仓失败。",
  invalidHoldingInput: "请输入标的代码、非零数量，以及可选的数字成本基础。",
  unableToAddHolding: "无法添加持仓。",

  // Transaction import
  importTransactionHistory: "导入交易历史",
  importDescription: "粘贴CSV格式，必须包含：symbol, transaction_type (buy/sell), quantity, price, executed_at (RFC 3339)。导入仅记录历史，从不发送交易。",
  transactionCsvLabel: "交易CSV",
  transactionCsvPlaceholder: "symbol,transaction_type,quantity,price,executed_at\nMSFT,buy,2,420,2026-08-01T00:00:00Z",
  importingTransactions: "导入中…",
  importTransactions: "导入交易",
  unableToImportTransactions: "无法导入交易。",

  // Imported transactions
  importedTransactions: "已导入交易",
  noTransactionsImported: "尚未导入交易历史。",
  failedToLoadTransactions: "加载交易失败。",

  // Allocation panel
  costBasisAllocation: "成本基础配置",
  allocationDescription: "敞口使用记录的成本基础，而非实时市场价格。没有成本基础的持仓在定价前贡献为零。",
  noAllocationData: "添加带有成本基础的持仓以查看配置和集中度。",
  failedToCalculateAllocation: "计算配置失败。",
  recordedCost: "记录成本",

  // Concentration panel
  concentrationReview: "集中度审查",
  concentrationDescription: "基于规则的信号：在记录成本配置的10%为中等，25%为高。这不是投资建议。",
  noConcentrationRisks: "当前没有持仓超过审查阈值。",
  failedToAnalyzeConcentration: "分析集中度失败。",
  severityModerate: "中等",
  severityHigh: "高",

  // Theme exposure panel
  themeExposure: "主题敞口",
  themeExposureDescription: "将持有的标的代码链接到现有知识实体。敞口仅基于您的显式链接和记录的成本基础。",
  themeSymbolLabel: "主题标的代码",
  knowledgeEntityLabel: "知识实体",
  knowledgeEntityPlaceholder: "知识实体…",
  linkTheme: "链接主题",
  noThemeLinks: "暂无主题链接。",
  failedToLoadThemeExposure: "加载主题敞口失败。",
  unableToLinkTheme: "无法链接主题。",

  // Alignment review panel
  thesisAlignmentAndReview: "论点对齐和审查",
  alignmentDescription: "仅当标的代码在工作区论点内容中出现时才匹配。审查结果仅供参考，不构成建议。",
  noAlignmentMatches: "当前持有的标的代码没有匹配工作区论点。",
  failedToCheckAlignment: "检查论点对齐失败。",
  reviewing: "审查中…",
  generatePortfolioReview: "生成投资组合审查",
  reviewGenerated: "审查生成于",
  unalignedSymbols: "未对齐标的",
  concentrationSignals: "集中度信号",
  none: "无",

  // Common
  selectAnAccount: "选择账户",
  selectAnAccountDescription: "选择账户以审查和记录其持仓。",
  failedToLoadWorkspaces: "加载工作区失败。",
  createWorkspaceFirst: "请先创建工作区",
  createWorkspaceFirstDescription: "投资组合账户在工作区内组织。",

  // Phase 3 dashboard
  netWorth: "净资产",
  totalValue: "总资产",
  totalLiabilities: "负债",
  total: "合计",
  marketValue: "市值",
  gainLoss: "盈亏",
  weight: "权重",
  allocation: "资产配置",
  accountValue: "账户价值趋势",
  recentActivity: "最近活动",
  quickActions: "快速操作",
  createSnapshot: "创建快照",
  creatingSnapshot: "创建中…",
  snapshotCreated: "快照已创建",
  failedToCreateSnapshot: "创建快照失败",
  refresh: "刷新",
  noValuationData: "暂无估值数据",
  noAllocationDescription: "添加带有成本基础的持仓以查看配置。",
  noRecentActivityDescription: "导入交易历史以查看最近活动。",
} as const;

export type PortfolioKey = keyof typeof portfolio;