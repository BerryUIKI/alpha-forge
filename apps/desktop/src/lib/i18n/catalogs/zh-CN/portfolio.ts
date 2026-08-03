/**
 * Simplified Chinese portfolio messages catalog.
 */

export const portfolio = {
  // Page header
  portfolioTitle: "投资组合",
  portfolioDescription: "跟踪持仓以进行审查和风险分析。此工作区从不执行交易。",
  
  // Workspace
  workspace: "工作区",
  
  // Account form
  newAccount: "新建账户",
  newAccountDescription: "仅添加持仓来源以进行跟踪。",
  accountName: "账户名称",
  accountNamePlaceholder: "主要券商",
  accountType: "账户类型",
  accountTypeBrokerage: "券商",
  accountTypeRetirement: "退休",
  accountTypeCash: "现金",
  accountTypeOther: "其他",
  accountCurrency: "账户货币",
  creating: "创建中…",
  addAccount: "添加账户",
  accountNameRequired: "账户名称为必填项。",
  unableToCreateAccount: "无法创建账户。",
  
  // Account list
  accounts: "账户",
  noAccountsYet: "暂无账户",
  noAccountsDescription: "添加一个账户以开始跟踪持仓。",
  
  // Positions
  symbol: "代码",
  quantity: "数量",
  costBasis: "成本基准",
  addHolding: "添加持仓",
  adding: "添加中…",
  noHoldingsYet: "暂无持仓",
  noHoldingsDescription: "手动记录持仓，或在下方导入交易历史。",
  failedToLoadHoldings: "加载持仓失败。",
  holdingValidationError: "请输入代码、非零数量和可选的数字成本基准。",
  unableToAddHolding: "无法添加持仓。",
  
  // Transactions import
  importTransactionHistory: "导入交易历史",
  importDescription: "粘贴 CSV 格式：symbol, transaction_type (buy/sell), quantity, price, executed_at (RFC 3339)。导入仅记录历史；从不发送交易。",
  transactionCsv: "交易 CSV",
  csvPlaceholder: "symbol,transaction_type,quantity,price,executed_at\nMSFT,buy,2,420,2026-08-01T00:00:00Z",
  importTransactions: "导入交易",
  importing: "导入中…",
  unableToImportTransactions: "无法导入交易。",
  importedTransactions: "已导入交易",
  noTransactionHistory: "尚未导入交易历史。",
  
  // Allocation
  costBasisAllocation: "成本基准配置",
  allocationDescription: "配置使用记录的成本基准，而非实时市场价格。没有成本基准的持仓在定价前贡献为零。",
  addHoldingsPrompt: "添加带有成本基准的持仓以查看配置和集中度。",
  recordedCost: "记录成本",
  
  // Concentration
  concentrationReview: "集中度审查",
  concentrationDescription: "基于规则的信号：占记录成本配置的10%为中等，25%为高。这不是投资建议。",
  noConcentrationRisks: "目前没有持仓超过审查阈值。",
  moderate: "中等",
  high: "高",
  
  // Theme exposure
  themeExposure: "主题敞口",
  themeExposureDescription: "将持有的代码链接到现有的知识实体。敞口仅基于你的显式链接和记录的成本基准。",
  knowledgeEntity: "知识实体…",
  linkTheme: "链接主题",
  noThemeLinks: "尚无主题链接。",
  failedToLoadThemeExposure: "加载主题敞口失败。",
  unableToLinkTheme: "无法链接主题。",
  
  // Alignment review
  thesisAlignmentReview: "论点对齐和审查",
  alignmentDescription: "仅当代码在工作区论点内容中出现时匹配。审查结果仅供参考，并非建议。",
  noThesisAlignment: "目前没有持有的代码匹配工作区论点。",
  generatePortfolioReview: "生成投资组合审查",
  reviewing: "审查中…",
  unableToGenerateReview: "无法生成审查。",
  reviewGenerated: "审查已生成",
  unalignedSymbols: "未对齐代码",
  concentrationSignals: "集中度信号",
  
  // Select account
  selectAccount: "选择账户",
  selectAccountDescription: "选择一个账户以审查和记录其持仓。",
  
  // Errors
  failedToLoadWorkspaces: "加载工作区失败。",
  createWorkspaceFirst: "请先创建工作区",
  createWorkspaceFirstDescription: "投资组合账户在工作区内组织。",
  failedToCalculateAllocation: "计算配置失败。",
  failedToAnalyzeConcentration: "分析集中度失败。",
  failedToCheckThesisAlignment: "检查论点对齐失败。",
  failedToLoadTransactions: "加载交易失败。",
  failedToLoadPortfolioAccounts: "加载投资组合账户失败。",
} as const;

export type PortfolioKey = keyof typeof portfolio;