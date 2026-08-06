/**
 * Tools Configuration
 *
 * Maps functional views to their available tools.
 * Each tool has a unique ID, i18n key, icon, and optional route.
 *
 * @version GUI-M2
 */

import type { FunctionalView, Tool } from "@/components/layout/types";

/**
 * Tool definitions organized by functional view
 */
export const toolsConfig: Record<FunctionalView, Tool[]> = {
  analyze: [
    {
      id: "research-projects",
      nameKey: "toolResearchProjects",
      icon: "Search",
      route: "/research",
      descriptionKey: "toolResearchProjectsDescription",
    },
    {
      id: "documents",
      nameKey: "toolDocuments",
      icon: "FileText",
      route: "/research",
      descriptionKey: "toolDocumentsDescription",
    },
    {
      id: "thesis",
      nameKey: "toolThesis",
      icon: "Lightbulb",
      route: "/journal",
      descriptionKey: "toolThesisDescription",
    },
  ],

  quantification: [
    {
      id: "backtesting",
      nameKey: "toolBacktesting",
      icon: "LineChart",
      route: "/research",
      descriptionKey: "toolBacktestingDescription",
    },
    {
      id: "risk-controls",
      nameKey: "toolRiskControls",
      icon: "Shield",
      route: "/portfolio",
      descriptionKey: "toolRiskControlsDescription",
    },
    {
      id: "signals",
      nameKey: "toolSignals",
      icon: "Zap",
      route: "/research",
      descriptionKey: "toolSignalsDescription",
    },
  ],

  "comprehensive-market": [
    {
      id: "market-overview",
      nameKey: "toolMarketOverview",
      icon: "TrendingUp",
      route: "/research",
      descriptionKey: "toolMarketOverviewDescription",
    },
    {
      id: "sector-analysis",
      nameKey: "toolSectorAnalysis",
      icon: "PieChart",
      route: "/research",
      descriptionKey: "toolSectorAnalysisDescription",
    },
    {
      id: "knowledge-graph",
      nameKey: "knowledgeGraph",
      icon: "Network",
      route: "/research",
      descriptionKey: "knowledgeGraphDescription",
    },
  ],

  options: [
    {
      id: "greeks-calculator",
      nameKey: "toolGreeksCalculator",
      icon: "Calculator",
      route: "/options",
      descriptionKey: "toolGreeksCalculatorDescription",
    },
    {
      id: "strategy-builder",
      nameKey: "toolStrategyBuilder",
      icon: "Layers",
      route: "/options",
      descriptionKey: "toolStrategyBuilderDescription",
    },
    {
      id: "option-chain",
      nameKey: "toolOptionChain",
      icon: "List",
      route: "/options",
      descriptionKey: "toolOptionChainDescription",
    },
  ],

  futures: [
    {
      id: "futures-chain",
      nameKey: "toolFuturesChain",
      icon: "GitBranch",
      route: "/research",
      descriptionKey: "toolFuturesChainDescription",
    },
    {
      id: "term-structure",
      nameKey: "toolTermStructure",
      icon: "BarChart3",
      route: "/research",
      descriptionKey: "toolTermStructureDescription",
    },
  ],

  "other-derivatives": [
    {
      id: "derivatives-overview",
      nameKey: "toolDerivativesOverview",
      icon: "Package",
      route: "/research",
      descriptionKey: "toolDerivativesOverviewDescription",
    },
  ],
};

/**
 * Get tools for a specific functional view
 */
export function getToolsForView(view: FunctionalView): Tool[] {
  return toolsConfig[view] || [];
}

/**
 * Get all unique tools across all views
 */
export function getAllTools(): Tool[] {
  const toolMap = new Map<string, Tool>();
  Object.values(toolsConfig).forEach((tools) => {
    tools.forEach((tool) => {
      toolMap.set(tool.id, tool);
    });
  });
  return Array.from(toolMap.values());
}

/**
 * Default functional view
 */
export const DEFAULT_FUNCTIONAL_VIEW: FunctionalView = "analyze";

/**
 * Storage key for persisting active view
 */
export const ACTIVE_VIEW_STORAGE_KEY = "left-sidebar:active-view";