/**
 * useDashboardData Hook
 *
 * Aggregates data from multiple desktopApi sources for the dashboard tabs.
 * Combines portfolio, thesis, agent, and research data into a single view model.
 *
 * @version GUI-E2
 */

import { useMemo } from "react";
import { useQuery } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import { useWorkspaces } from "@/features/workspace/hooks/useWorkspaces";
import type { Holding } from "@/components/portfolio/HoldingsList";
import type { ActivityItem } from "@/components/activity/ActivityFeed";

const DASHBOARD_KEYS = {
  summary: (workspaceId: string) => ["dashboard", "summary", workspaceId] as const,
  activity: (workspaceId: string) => ["dashboard", "activity", workspaceId] as const,
};

/**
 * Returns the first workspace ID from the user's workspaces.
 */
export function useActiveWorkspaceId(): string {
  const { data: workspaces } = useWorkspaces();
  return workspaces?.[0]?.id ?? "";
}

/**
 * Hook to fetch dashboard summary data.
 * Returns portfolio value, active theses count, and top holdings.
 */
export function useDashboardSummary(workspaceId: string) {
  return useQuery({
    queryKey: DASHBOARD_KEYS.summary(workspaceId),
    queryFn: async () => {
      // Fetch theses and portfolio allocation in parallel
      const [theses, allocation] = await Promise.all([
        desktopApi.thesis.listTheses(workspaceId),
        desktopApi.portfolio.getPortfolioAllocation(workspaceId),
      ]);

      // Count active theses (draft + active status)
      const activeTheses = theses.filter(
        (t) => t.status === "active" || t.status === "draft",
      ).length;

      // Build top holdings from allocation data
      const holdings: Holding[] = allocation.slice(0, 5).map((item, idx) => ({
        id: `holding-${idx}`,
        ticker: item.symbol,
        name: item.symbol,
        sector: "—",
        allocation: `${item.weight_percent.toFixed(1)}%`,
        value: `$${(item.allocated_cost).toLocaleString("en-US", { minimumFractionDigits: 0 })}`,
        change: "—",
        changePositive: true,
      }));

      // Compute total portfolio value (sum of allocated costs)
      const totalValue = allocation.reduce((sum, a) => sum + a.allocated_cost, 0);

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