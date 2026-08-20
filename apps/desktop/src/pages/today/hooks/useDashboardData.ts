/**
 * useDashboardData Hook
 *
 * Aggregates data from multiple desktopApi sources for the dashboard tabs.
 * Combines portfolio, thesis, agent, and research data into a single view model.
 *
 * @version GUI-E2
 */

import { useQuery } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import { useActiveWorkspaceId as useActiveWorkspaceIdFromContext } from "@/features/workspace/hooks/useActiveWorkspace.context";
import type { Holding } from "@/components/portfolio/HoldingsList";
import type { ActivityItem } from "@/components/activity/ActivityFeed";

const DASHBOARD_KEYS = {
  summary: (workspaceId: string) => ["dashboard", "summary", workspaceId] as const,
  activity: (workspaceId: string) => ["dashboard", "activity", workspaceId] as const,
};

/**
 * Returns the active research workspace from the global context (ADR-0008).
 * Re-exported here so dashboard tabs keep importing from this module.
 */
export function useActiveWorkspaceId(): string {
  return useActiveWorkspaceIdFromContext();
}

/**
 * Hook to fetch dashboard summary data.
 * Returns portfolio value, active theses count, and top holdings.
 *
 * The portfolio overview is the global dimension (ADR-0008): holdings are
 * aggregated from the canonical `accounts` model across every workspace,
 * while the thesis count follows the active workspace.
 */
export function useDashboardSummary(workspaceId: string) {
  return useQuery({
    queryKey: DASHBOARD_KEYS.summary(workspaceId),
    queryFn: async () => {
      const [theses, summaries] = await Promise.all([
        desktopApi.thesis.listTheses(workspaceId),
        desktopApi.financial.getAllHoldings(new Date().toISOString().slice(0, 10)),
      ]);

      // Count active theses (draft + active status)
      const activeTheses = theses.filter(
        (t) => t.status === "active" || t.status === "draft",
      ).length;

      // Build top holdings from global holdings (largest market value first)
      const allHoldings = summaries.flatMap((summary) => summary.holdings);
      allHoldings.sort(
        (a, b) => parseFloat(b.market_value_base) - parseFloat(a.market_value_base),
      );
      const holdings: Holding[] = allHoldings.slice(0, 5).map((holding, idx) => ({
        id: `holding-${idx}`,
        ticker: holding.asset_symbol ?? "—",
        name: holding.asset_name ?? holding.asset_symbol ?? "—",
        sector: "—",
        allocation: `${parseFloat(holding.weight_pct).toFixed(1)}%`,
        value: `$${Number(holding.market_value_base).toLocaleString("en-US", { minimumFractionDigits: 0 })}`,
        change: "—",
        changePositive: true,
      }));

      // Total portfolio value across all accounts (base currency)
      const totalValue = summaries.reduce(
        (sum, summary) => sum + parseFloat(summary.total_market_value_base),
        0,
      );

      return {
        portfolioValue: totalValue,
        activeTheses,
        holdings,
      };
    },
    enabled: !!workspaceId,
    staleTime: 30_000,
  });
}

/**
 * Hook to fetch recent activity feed.
 * Merges agent tasks, research projects, and theses sorted by created_at.
 */
export function useDashboardActivity(workspaceId: string) {
  return useQuery({
    queryKey: DASHBOARD_KEYS.activity(workspaceId),
    queryFn: async () => {
      const [tasks, projects, theses] = await Promise.all([
        desktopApi.agent.listAgentTasks(workspaceId),
        desktopApi.research.listResearchProjects(workspaceId),
        desktopApi.thesis.listTheses(workspaceId),
      ]);

      const items: ActivityItem[] = [
        // Agent tasks as research activity
        ...tasks.map((task) => ({
          id: `task-${task.id}`,
          type: "research" as const,
          title: "Research",
          description: task.title,
          timestamp: formatRelativeTime(task.created_at),
        })),
        // Research projects
        ...projects.map((project) => ({
          id: `project-${project.id}`,
          type: "research" as const,
          title: "Research",
          description: project.title,
          timestamp: formatRelativeTime(project.created_at),
        })),
        // Theses
        ...theses.map((thesis) => ({
          id: `thesis-${thesis.id}`,
          type: "thesis" as const,
          title: "Thesis",
          description: thesis.title,
          timestamp: formatRelativeTime(thesis.created_at),
        })),
      ];

      // Sort by timestamp (most recent first), take top 10
      items.sort((a, b) => {
        const timeA = parseRelativeTime(a.timestamp);
        const timeB = parseRelativeTime(b.timestamp);
        return timeB - timeA;
      });

      return items.slice(0, 10);
    },
    enabled: !!workspaceId,
    staleTime: 30_000,
  });
}

/**
 * Simple relative time formatter.
 * Produces strings like "12m ago", "2h ago", "3d ago".
 */
function formatRelativeTime(isoString: string): string {
  const now = Date.now();
  const then = new Date(isoString).getTime();
  const diffMs = now - then;

  if (diffMs < 0) return "just now";

  const seconds = Math.floor(diffMs / 1000);
  if (seconds < 60) return "just now";

  const minutes = Math.floor(seconds / 60);
  if (minutes < 60) return `${minutes}m ago`;

  const hours = Math.floor(minutes / 60);
  if (hours < 24) return `${hours}h ago`;

  const days = Math.floor(hours / 24);
  if (days < 30) return `${days}d ago`;

  return isoString.slice(0, 10);
}

/**
 * Crude parse of relative time strings back to timestamps for sorting.
 * Handles: "Xm ago", "Xh ago", "Xd ago", "just now".
 */
function parseRelativeTime(relative: string): number {
  const now = Date.now();
  const match = relative.match(/^(\d+)(m|h|d) ago$/);
  if (!match) return now; // "just now" or fallback

  const value = Number.parseInt(match[1] ?? "0", 10);
  const unit = match[2];

  if (unit === "m") return now - value * 60_000;
  if (unit === "h") return now - value * 3_600_000;
  if (unit === "d") return now - value * 86_400_000;

  return now;
}