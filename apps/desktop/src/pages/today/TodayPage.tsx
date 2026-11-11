/**
 * TodayPage — Dashboard
 *
 * Main dashboard page with tabbed interface:
 * - Overview: stat cards, top holdings, recent activity
 * - Performance: portfolio performance chart
 * - Activity: full activity feed
 *
 * The tab bar and panels share one content gutter so the heading, tabs, and
 * dashboard surfaces align on the same edge. Layout uses container queries so
 * the dashboard adapts to the actual content width (sidebars included).
 *
 * @version GUI-M3
 */

import { useState, useCallback } from "react";
import { useSearchParams } from "react-router-dom";
import { TabBar, getTabId, getTabPanelId } from "@/components/ui";
import { PageContainer } from "@/components/layout/PageContainer";
import { OverviewTab } from "./tabs/OverviewTab";
import { PerformanceTab } from "./tabs/PerformanceTab";
import { ActivityTab } from "./tabs/ActivityTab";
import { useLocale } from "@/lib/i18n/useLocale";
import { LayoutDashboard, TrendingUp, Activity } from "lucide-react";

import { MarketPulseBar } from "./components/MarketPulseBar";

const TAB_STORAGE_KEY = "dashboard-active-tab";
const TAB_ID_PREFIX = "dashboard";

export function TodayPage() {
  const { t } = useLocale();
  const [searchParams, setSearchParams] = useSearchParams();
  const tabFromUrl = searchParams.get("tab");

  const DASHBOARD_TABS = [
    { id: "overview", label: t("todayOverviewTab"), icon: LayoutDashboard },
    { id: "performance", label: t("todayPerformanceTab"), icon: TrendingUp },
    { id: "activity", label: t("todayActivityTab"), icon: Activity },
  ];

  const [activeTab, setActiveTab] = useState(() => {
    // Check URL first, then localStorage, default to "overview"
    if (tabFromUrl && DASHBOARD_TABS.some((t) => t.id === tabFromUrl)) {
      return tabFromUrl;
    }
    const stored = localStorage.getItem(TAB_STORAGE_KEY);
    if (stored && DASHBOARD_TABS.some((t) => t.id === stored)) {
      return stored;
    }
    return "overview";
  });

  const handleTabChange = useCallback(
    (tabId: string) => {
      setActiveTab(tabId);
      localStorage.setItem(TAB_STORAGE_KEY, tabId);
      // Update URL params
      setSearchParams(
        (prev) => {
          const next = new URLSearchParams(prev);
          next.set("tab", tabId);
          return next;
        },
        { replace: true },
      );
    },
    [setSearchParams],
  );

  return (
    <div className="flex flex-col">
      {/* Market Pulse Bar on top of cockpit */}
      <MarketPulseBar />

      <PageContainer>
        {/* Page Header */}
        <div>
          <h1 className="text-2xl font-bold tracking-tight">{t("todayTitle")}</h1>
          <p className="mt-1 text-sm text-muted-foreground">{t("todayDescription")}</p>
        </div>

        {/* Tab Bar */}
        <TabBar
          tabs={DASHBOARD_TABS}
          activeTab={activeTab}
          onTabChange={handleTabChange}
          className="w-fit"
          idPrefix={TAB_ID_PREFIX}
          label={t("todayTitle")}
        />

        {/* Tab Panels — only the active panel mounts its content, but every
            panel element exists so each tab's aria-controls resolves. */}
        {DASHBOARD_TABS.map((tab) => (
          <div
            key={tab.id}
            role="tabpanel"
            id={getTabPanelId(TAB_ID_PREFIX, tab.id)}
            aria-labelledby={getTabId(TAB_ID_PREFIX, tab.id)}
            hidden={activeTab !== tab.id}
            tabIndex={0}
            className="outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background"
          >
            {activeTab === tab.id && (
              <>
                {tab.id === "overview" && <OverviewTab />}
                {tab.id === "performance" && <PerformanceTab />}
                {tab.id === "activity" && <ActivityTab />}
              </>
            )}
          </div>
        ))}
      </PageContainer>
    </div>
  );
}
