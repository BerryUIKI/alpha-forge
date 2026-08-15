/**
 * Tool definitions for each functional view
 *
 * Each functional view has a set of tools displayed in the left sidebar.
 * Tools can navigate to routes or execute custom actions.
 *
 * @module components/layout/tools-config
 */

import {
  FolderOpen,
  FileText,
  Activity,
  Shield,
  TrendingUp,
  BarChart3,
  Calculator,
  Layers,
  List,
  CandlestickChart,
  Settings,
  Wallet,
} from "lucide-react";
import type { FunctionalView, ToolItem } from "./types";

/**
 * Tools available for each functional view
 */
export const VIEW_TOOLS: Record<FunctionalView, ToolItem[]> = {
  analyze: [
    {
      id: "portfolio-dashboard",
      label: "投资组合",
      icon: Wallet,
      route: "/portfolio",
    },
    {
      id: "research-projects",
      label: "研究项目",
      icon: FolderOpen,
      route: "/research",
    },
    {
      id: "documents",
      label: "文档管理",
      icon: FileText,
      route: "/research",
    },
    {
      id: "thesis",
      label: "投资论点",
      icon: TrendingUp,
      route: "/journal",
    },
  ],

  quantification: [
    {
      id: "backtesting",
      label: "回测引擎",
      icon: Activity,
      route: "/research",
      disabled: true, // Future feature
    },
    {
      id: "risk-controls",
      label: "风控设置",
      icon: Shield,
      route: "/research",
      disabled: true, // Future feature
    },
    {
      id: "signals",
      label: "信号监控",
      icon: BarChart3,
      route: "/research",
      disabled: true, // Future feature
    },
  ],

  "comprehensive-market": [
    {
      id: "market-overview",
      label: "市场概览",
      icon: TrendingUp,
      route: "/research",
      disabled: true, // Future feature
    },
    {
      id: "sector-analysis",
      label: "板块分析",
      icon: BarChart3,
      route: "/research",
      disabled: true, // Future feature
    },
  ],

  options: [
    {
      id: "greeks-calculator",
      label: "Greeks计算器",
      icon: Calculator,
      route: "/options",
    },
    {
      id: "strategy-builder",
      label: "策略构建器",
      icon: Layers,
      route: "/options",
    },
    {
      id: "option-chain",
      label: "期权链列表",
      icon: List,
      route: "/options",
    },
  ],

  futures: [
    {
      id: "futures-chain",
      label: "期货链",
      icon: CandlestickChart,
      route: "/research",
      disabled: true, // Future feature
    },
    {
      id: "term-structure",
      label: "期限结构",
      icon: TrendingUp,
      route: "/research",
      disabled: true, // Future feature
    },
  ],

  "other-derivatives": [
    {
      id: "derivatives-overview",
      label: "衍生品概览",
      icon: BarChart3,
      route: "/research",
      disabled: true, // Future feature
    },
  ],
};

/**
 * Get tools for a specific functional view
 */
export function getToolsForView(view: FunctionalView): ToolItem[] {
  return VIEW_TOOLS[view] || [];
}
