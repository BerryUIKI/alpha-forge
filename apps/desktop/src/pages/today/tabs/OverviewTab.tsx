/**
 * OverviewTab Component
 *
 * Dashboard overview: stat cards row + holdings/activity two-column layout.
 * Wired to real desktopApi data via useDashboardData hook.
 *
 * @version GUI-E2
 */

import { useDashboardSummary, useActiveWorkspaceId, useDashboardActivity } from "../hooks/useDashboardData";
import { DashboardCard, StatCard } from "@/components/ui";
import { LoadingSpinner, ErrorState, EmptyState } from "@/components/common";
import { HoldingsList } from "@/components/portfolio/HoldingsList";
import { ActivityFeed } from "@/components/activity/ActivityFeed";

export function OverviewTab() {
  const workspaceId = useActiveWorkspaceId();
  const { data: summary, isLoading: summaryLoading, error: summaryError } = useDashboardSummary(workspaceId);
  const { data: activity, isLoading: activityLoading, error: activityError } = useDashboardActivity(workspaceId);

  // Loading state
  if (summaryLoading || activityLoading) {
    return (
      <div className="flex flex-col gap-6">
        <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
          <div className="h-28 animate-pulse rounded-xl bg-muted" />
          <div className="h-28 animate-pulse rounded-xl bg-muted" />
          <div className="h-28 animate-pulse rounded-xl bg-muted" />
        </div>
        <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
          <div className="h-64 animate-pulse rounded-xl bg-muted" />
          <div className="h-64 animate-pulse rounded-xl bg-muted" />
        </div>
      </div>
    );
  }

  // Error state
  if (summaryError || activityError) {
    return (
      <ErrorState
        message="Failed to load dashboard data"
        onRetry={() => window.location.reload()}
      />
    );
  }

  // Empty state
  const hasPortfolioAccounts = (summary?.holdings?.length ?? 0) > 0;

  if (!hasPortfolioAccounts && (summary?.activeTheses ?? 0) === 0) {
    return (
      <div className="flex flex-col gap-6">
        <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
          <StatCard label="Total Portfolio Value" value="—" />
          <StatCard label="Active Theses" value="0" />
          <StatCard label="Unrealized P&L" value="—" />
        </div>
        <EmptyState
          title="No data yet"
          description="Create a workspace and start researching to see your dashboard."
        />
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-6">
      {/* Stats Row */}
      <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
        <StatCard
          label="Total Portfolio Value"
          value={summary?.portfolioValue
            ? `$${summary.portfolioValue.toLocaleString("en-US", { minimumFractionDigits: 0 })}`
            : "—"
          }
          change={summary?.portfolioValue ? "Last updated from portfolio" : undefined}
          isPositive
        />
        <StatCard
          label="Active Theses"
          value={String(summary?.activeTheses ?? 0)}
          change={summary?.activeTheses ? `${summary.activeTheses} active thesis${summary.activeTheses !== 1 ? "es" : ""}` : undefined}
          isPositive
        />
        <StatCard
          label="Unrealized P&L"
          value="—"
          change="Coming with position tracking"
        />
      </div>

      {/* Holdings + Activity */}
      <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
        <DashboardCard
          title="Top Holdings"
          meta="View all"
          padded={false}
        >
          <div className="px-4">
            {summary?.holdings && summary.holdings.length > 0 ? (
              <HoldingsList holdings={summary.holdings} />
            ) : (
              <p className="py-4 text-center text-sm text-muted-foreground">
                No holdings yet
              </p>
            )}
          </div>
        </DashboardCard>

        <DashboardCard
          title="Recent Activity"
          meta="View all"
          padded={false}
        >
          <div className="px-4">
            {activity && activity.length > 0 ? (
              <ActivityFeed items={activity} />
            ) : (
              <p className="py-4 text-center text-sm text-muted-foreground">
                No recent activity
              </p>
            )}
          </div>
        </DashboardCard>
      </div>
    </div>
  );
}