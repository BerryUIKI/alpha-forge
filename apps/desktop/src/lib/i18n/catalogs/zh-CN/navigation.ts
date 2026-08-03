/**
 * Simplified Chinese navigation messages catalog.
 */

export const navigation = {
  today: "今日",
  research: "研究",
  journal: "投资日志",
  portfolio: "投资组合",
  artifacts: "研究产物",
  settings: "设置",
} as const;

export type NavigationKey = keyof typeof navigation;