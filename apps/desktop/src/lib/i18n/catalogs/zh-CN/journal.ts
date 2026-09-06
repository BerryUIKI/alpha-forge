/**
 * Simplified Chinese journal/thesis messages catalog.
 */

export const journal = {
  // Page header
  journalTitle: "投资日志",
  
  // Thesis dashboard
  investmentTheses: "投资论点",
  thesisDescription: "明确你的推理，跟踪证据，并验证结果。",
  
  // Empty/error states
  createWorkspaceFirst: "请先创建工作区",
  createWorkspaceFirstDescription: "论点存储在工作区中，以便其证据保持有序。",
  selectThesis: "选择论点",
  selectThesisDescription: "选择一个论点来查看其置信度、生命周期和证据。",
  thesisFailedToLoadWorkspaces: "加载工作区失败。",
  
  // Create thesis form
  newInvestmentThesis: "新建投资论点",
  newThesisDescription: "在开始跟踪证据之前，捕获一个可验证的论断。",
  titleLabel: "标题",
  titlePlaceholder: "AI 基础设施需求保持强劲",
  thesisStatementLabel: "论点陈述",
  thesisStatementPlaceholder: "陈述论断、其可能成立的原因以及可能的反驳证据。",
  initialConfidence: "初始置信度",
  creatingThesis: "创建论点中…",
  createThesis: "创建论点",
  thesisTitleRequired: "标题和论点陈述为必填项。",
  unableToCreateThesis: "无法创建论点。",

  // Thesis Pipeline Board & Cards
  thesesPipelineTitle: "投资论点跟踪池",
  thesesPipelineSub: "从假说到建仓再到定期证伪回顾，全生命周期跟踪",
  viewBoard: "看板视图",
  viewList: "列表视图",
  statusDraft: "草稿",
  statusActive: "活跃",
  statusValidating: "验证中",
  statusValidated: "已验证",
  statusClosed: "已关闭",
  convictionLabel: "置信度",
  evidenceCountPill: "{supporting} 正向 · {contradicting} 反向",
  supportingCount: "{count} 项正向",
  contradictingCount: "{count} 项反向",
  noThesesInStage: "此阶段暂无论点",
  filterStatus: "状态筛选",
  allStatuses: "全部状态",
  cardDetails: "查看详情",
  cardQuickAction: "流转状态",
  confidenceScore: "{score}% 置信度",
} as const;

export type JournalKey = keyof typeof journal;