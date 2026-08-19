/**
 * ActivityTab Component
 *
 * Full activity feed across all research workflow types.
 * Wired to real desktopApi data via useDashboardActivity hook.
 *
 * @version GUI-E2
 */

import { useActiveWorkspaceId, useDashboardActivity } from "../hooks/useDashboardData";
import { DashboardCard } from "@/components/ui";
import { EmptyState } from "@/components/common";
import { ActivityFeed } from "@/components/activity/ActivityFeed";
import { useLocale } from "@/lib/i18n/useLocale";

export function ActivityTab() {
  const { t } = useLocale();
  const workspaceId = useActiveWorkspaceId();
  const { data: activity, isLoading } = useDashboardActivity(workspaceId);

  return (
    <DashboardCard
      title={t("allActivity")}
      padded={false}
    >
      {isLoading ? (
        <div className="space-y-3 px-4 py-2">
          {Array.from({ length: 5 }).map((_, idx) => (
            <div key={idx} className="h-10 animate-pulse rounded-lg bg-muted" />
          ))}
        </div>
      ) : activity && activity.length > 0 ? (
        <div className="px-4 py-2">
          <ActivityFeed items={activity} />
        </div>
      ) : (
        <EmptyState
          title={t("noRecentActivity")}
          description={t("noActivityDescription")}
        />
      )}
    </DashboardCard>
  );
}