/**
 * TodayPage — Dashboard
 *
 * Main dashboard page with tabbed interface:
 * - Overview: stat cards, top holdings, recent activity
 * - Performance: portfolio performance chart
 * - Activity: full activity feed
 *
 * @version GUI-M3
 */

import { useState, useCallback } from "react";
import { useSearchParams } from "react-router-dom";
import { TabBar } from "@/components/ui";
import { OverviewTab } from "./tabs/OverviewTab";
import { PerformanceTab } from "./tabs/PerformanceTab";
import { ActivityTab } from "./tabs/ActivityTab";
import { useLocale } from "@/lib/i18n/useLocale";
import { LayoutDashboard, TrendingUp, Activity } from "lucide-react";

const TAB_STORAGE_KEY = "dashboard-active-tab";

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
      setSearchParams((prev) => {
        const next = new URLSearchParams(prev);
        next.set("tab", tabId);
        return next;
      }, { replace: true });
    },
    [setSearchParams],
  );

  return (
    <div className="flex flex-col gap-6 p-6">
      {/* Page Header */}
      <div>
        <h1 className="text-2xl font-bold tracking-tight">{t("todayTitle")}</h1>
        <p className="mt-1 text-sm text-muted-foreground">
          {t("todayDescription")}
        </p>
      </div>

      {/* Tab Bar */}
      <TabBar
        tabs={DASHBOARD_TABS}
        activeTab={activeTab}
        onTabChange={handleTabChange}
        className="w-fit"
      />

      {/* Tab Content */}
      {activeTab === "overview" && <OverviewTab />}
      {activeTab === "performance" && <PerformanceTab />}
      {activeTab === "activity" && <ActivityTab />}
    </div>
  );
}