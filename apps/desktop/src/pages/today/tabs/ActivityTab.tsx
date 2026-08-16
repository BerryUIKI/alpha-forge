/**
 * ActivityTab Component
 *
 * Full activity feed across all research workflow types.
 *
 * @version GUI-M3
 */

import { DashboardCard } from "@/components/ui";
import { ActivityFeed, type ActivityItem } from "@/components/activity/ActivityFeed";

const SAMPLE_ACTIVITY: ActivityItem[] = [
  { id: "1", type: "research", title: "Research", description: "Q2 Earnings analysis for NVDA completed", timestamp: "12m ago" },
  { id: "2", type: "thesis", title: "Thesis", description: "Updated bullish thesis on renewable energy", timestamp: "45m ago" },
  { id: "3", type: "portfolio", title: "Portfolio", description: "Rebalanced: reduced energy exposure by 3%", timestamp: "2h ago" },
  { id: "4", type: "research", title: "Research", description: "New article: AI Infrastructure Spending Outlook", timestamp: "3h ago" },
  { id: "5", type: "thesis", title: "Thesis", description: "Bearish thesis on consumer discretionary updated", timestamp: "5h ago" },
  { id: "6", type: "options", title: "Options", description: "Iron condor strategy created on TSLA chain", timestamp: "Yesterday" },
];

export function ActivityTab() {
  return (
    <DashboardCard title="All Activity" padded={false}>
      <div className="px-4 py-2">
        <ActivityFeed items={SAMPLE_ACTIVITY} />
      </div>
    </DashboardCard>
  );
}