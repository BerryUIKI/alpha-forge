/**
 * Simplified Chinese research messages catalog.
 */

export const research = {
  // Page header
  researchTitle: "研究",
  researchDescription: "捕获项目、来源出处和文档注释。",

  // Workspace selection
  workspace: "工作区",
  selectWorkspace: "选择工作区",

  // Projects section
  projects: "项目",
  projectTitle: "项目标题",
  create: "创建",

  // Documents section
  documents: "文档",
  documentTitle: "文档标题",
  add: "添加",

  // Import
  importPdf: "导入 PDF",
  importingPdf: "正在导入 PDF…",
  importWebPage: "导入网页",
  importing: "正在导入…",
  webPageUrl: "网页 URL",
  webPageUrlPlaceholder: "https://example.com/research",
  importHint: "PDF 使用原生选择器；网页由 Rust 从验证的公共 HTTPS URL 获取。",

  // Reports section
  reports: "报告",
  reportTitle: "报告标题",
  reportContent: "报告内容",
  reportType: "报告类型",
  reportTypeAnalysis: "分析",
  reportTypeSummary: "摘要",
  reportTypeThesis: "论点",
  reportTypeRecommendation: "建议",
  saveReport: "保存报告",

  // Notes section
  notes: "笔记",
  noteContent: "笔记内容",
  addNote: "添加笔记",

  // Sources section
  sources: "来源",
  sourceUrl: "来源 URL",
  sourceUrlPlaceholder: "https://example.com",
  sourceTitle: "来源标题",
  sourceTitlePlaceholder: "来源标题（可选）",
  addSource: "添加来源",
  sourcesHint: "来源必须使用公共 HTTPS 主机名。",
  untitledSource: "无标题来源",

  // Search section
  searchDocument: "搜索此文档",
  searchPlaceholder: "在保存的文档文本中查找术语",
  searchMode: "搜索模式",
  searchModeLexical: "精确术语",
  searchModeSemantic: "相关术语",
  search: "搜索",
  searchHint: "相关术语使用本地的、可解释的投资词汇；它们不是 AI 生成的建议。",
  searchScore: "得分",

  // Errors
  saveError: "无法保存研究项目。请检查必填字段后重试。",
} as const;

export type ResearchKey = keyof typeof research;