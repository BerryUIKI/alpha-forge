/**
 * OverviewTab Component
 *
 * Dashboard overview: stat cards row + holdings/activity two-column layout.
 *
 * @version GUI-M3
 */

import { DashboardCard, StatCard } from "@/components/ui";
import { HoldingsList, type Holding } from "@/components/portfolio/HoldingsList";
import { ActivityFeed, type ActivityItem } from "@/components/activity/ActivityFeed";

// Placeholder data — wired to real desktopApi queries in a later phase.
const SAMPLE_HOLDINGS: Holding[] = [
  { id: "1", ticker: "AAPL", name: "Apple Inc.", sector: "Technology", allocation: "15.2%", value: "$128,400", change: "+2.3%", changePositive: true },
  { id: "2", ticker: "MSFT", name: "Microsoft Corp.", sector: "Technology", allocation: "12.8%", value: "$108,200", change: "+1.8%", changePositive: true },
  { id: "3", ticker: "NVDA", name: "NVIDIA Corp.", sector: "Semiconductors", allocation: "10.1%", value: "$85,600", change: "-0.7%", changePositive: false },
  { id: "4", ticker: "AMZN", name: "Amazon.com Inc.", sector: "Consumer Cyclical", allocation: "8.5%", value: "$72,100", change: "+0.9%", changePositive: true },
];

const SAMPLE_ACTIVITY: ActivityItem[] = [
  { id: "1", type: "research", title: "Research", description: "Q2 Earnings analysis for NVDA completed", timestamp: "12m ago" },
  { id: "2", type: "thesis", title: "Thesis", description: "Updated bullish thesis on renewable energy", timestamp: "45m ago" },
  { id: "3", type: "portfolio", title: "Portfolio", description: "Rebalanced: reduced energy exposure by 3%", timestamp: "2h ago" },
  { id: "4", type: "research", title: "Research", description: `New article: "AI Infrastructure Spending Outlook"`, timestamp: "3h ago" },
];

export function OverviewTab() {
  return (
    <div className="flex flex-col gap-6">
      {/* Stats Row */}
      <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
        <StatCard
          label="Total Portfolio Value"
          value="$847,350"
          change="+$12,430 (+1.49%)"
          isPositive
        />
        <StatCard
          label="Active Theses"
          value="12"
          change="+3 this week"
          isPositive
        />
        <StatCard
          label="Unrealized P&L"
          value="+$43,200"
          change="+5.4%"
          isPositive
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
            <HoldingsList holdings={SAMPLE_HOLDINGS} />
          </div>
        </DashboardCard>

        <DashboardCard
          title="Recent Activity"
          meta="View all"
          padded={false}
        >
          <div className="px-4">
            <ActivityFeed items={SAMPLE_ACTIVITY} />
          </div>
        </DashboardCard>
      </div>
    </div>
  );
}