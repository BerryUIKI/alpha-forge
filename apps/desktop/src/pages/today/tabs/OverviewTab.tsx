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
import { ErrorState, EmptyState } from "@/components/common";
import { HoldingsList } from "@/components/portfolio/HoldingsList";
import { ActivityFeed } from "@/components/activity/ActivityFeed";
import { useLocale } from "@/lib/i18n/useLocale";
import { formatMessage, translate } from "@/lib/i18n/locale";

export function OverviewTab() {
  const { t, locale } = useLocale();
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
        message={t("failedToLoadDashboardData")}
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
          <StatCard label={t("totalPortfolioValue")} value="—" />
          <StatCard label={t("activeTheses")} value="0" />
          <StatCard label={t("unrealizedPL")} value="—" />
        </div>
        <EmptyState
          title={t("noDataYet")}
          description={t("noDashboardDataDescription")}
        />
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-6">
      {/* Stats Row */}
      <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
        <StatCard
          label={t("totalPortfolioValue")}
          value={summary?.portfolioValue
            ? `$${summary.portfolioValue.toLocaleString("en-US", { minimumFractionDigits: 0 })}`
            : "—"
          }
          change={summary?.portfolioValue ? t("lastUpdatedFromPortfolio") : undefined}
          isPositive
        />
        <StatCard
          label={t("activeTheses")}
          value={String(summary?.activeTheses ?? 0)}
          change={summary?.activeTheses
            ? formatMessage(
                summary.activeTheses !== 1
                  ? translate(locale, "activeThesesCount")
                  : translate(locale, "activeThesisCount"),
                { count: String(summary.activeTheses) },
              )
            : undefined}
          isPositive
        />
        <StatCard
          label={t("unrealizedPL")}
          value="—"
          change={t("comingWithPositionTracking")}
        />
      </div>

      {/* Holdings + Activity */}
      <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
        <DashboardCard
          title={t("topHoldings")}
          meta={t("viewAll")}
          padded={false}
        >
          <div className="px-4">
            {summary?.holdings && summary.holdings.length > 0 ? (
              <HoldingsList holdings={summary.holdings} />
            ) : (
              <p className="py-4 text-center text-sm text-muted-foreground">
                {t("noHoldingsYet")}
              </p>
            )}
          </div>
        </DashboardCard>

        <DashboardCard
          title={t("recentActivity")}
          meta={t("viewAll")}
          padded={false}
        >
          <div className="px-4">
            {activity && activity.length > 0 ? (
              <ActivityFeed items={activity} />
            ) : (
              <p className="py-4 text-center text-sm text-muted-foreground">
                {t("noRecentActivity")}
              </p>
            )}
          </div>
        </DashboardCard>
      </div>
    </div>
  );
}