/**
 * Simplified Chinese navigation messages catalog.
 */

export const navigation = {
  dashboard: "仪表盘",
  today: "今日",
  research: "研究",
  theses: "论点",
  journal: "投资日志",
  portfolio: "投资组合",
  knowledge: "知识库",
  options: "期权",
  artifacts: "研究产物",
  settings: "设置",
  cockpit: "驾驶舱",
  pipeline: "投研闭环",
  tools: "工具",
  knowledgeTools: "知识与工具",
} as const;

export type NavigationKey = keyof typeof navigation;