/**
 * Portfolio Dashboard Component
 *
 * Main dashboard for the Portfolio module (Phase 3).
 * Orchestrates:
 * - Workspace selector
 * - Account cards + net worth
 * - Holdings table
 * - Allocation chart
 * - Valuation chart
 * - Recent activity
 * - Quick actions
 *
 * @module features/portfolio/components/PortfolioDashboard
 */

import { useEffect, useState } from "react";
import { useLocale } from "@/lib/i18n/useLocale";
import { EmptyState, ErrorState, LoadingSpinner } from "@/components/common";
import { useWorkspaces } from "@/features/workspace/hooks/useWorkspaces";
import { AccountCards } from "./AccountCards";
import { HoldingsTable } from "./HoldingsTable";
import { AllocationChart } from "./AllocationChart";
import { ValuationChart } from "./ValuationChart";
import { ActivityList } from "./ActivityList";
import { QuickActions } from "./QuickActions";

/** Default view date — "today" as of the running app. */
function todayIso(): string {
  return new Date().toISOString().slice(0, 10);
}

export function PortfolioDashboard() {
  const { t } = useLocale();
  const workspaces = useWorkspaces();
  const [workspaceId, setWorkspaceId] = useState("");
  const [selectedAccountId, setSelectedAccountId] = useState("");
  const [asOfDate, setAsOfDate] = useState(todayIso());
  const [refreshKey, setRefreshKey] = useState(0);

  // Auto-select first workspace
  useEffect(() => {
    if (!workspaceId && workspaces.data?.[0]) {
      setWorkspaceId(workspaces.data[0].id);
    }
  }, [workspaceId, workspaces.data]);

  // ── Loading / error / empty for workspaces ──
  if (workspaces.isLoading) return <LoadingSpinner className="p-8" />;
  if (workspaces.error)
    return (
      <ErrorState
        message={t("failedToLoadWorkspaces")}
        onRetry={() => workspaces.refetch()}
      />
    );
  if (!workspaces.data?.length)
    return (
      <EmptyState
        title={t("createWorkspaceFirst")}
        description={t("createWorkspaceFirstDescription")}
      />
    );

  const handleRefresh = () => {
    setAsOfDate(todayIso());
    setRefreshKey((k) => k + 1);
  };

  return (
    <div className="space-y-6">
      {/* Header row: description + workspace selector */}
      <div className="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
        <p className="max-w-xl text-muted-foreground">
          {t("portfolioDescription")}
        </p>
        <label className="block text-sm font-medium">
          {t("workspaceLabel")}
          <select
            value={workspaceId}
            onChange={(e) => {
              setWorkspaceId(e.target.value);
              setSelectedAccountId("");
            }}
            className="mt-1 w-full min-w-[180px] rounded-md border border-input bg-background px-3 py-2 text-sm"
          >
            {workspaces.data.map((workspace) => (
              <option key={workspace.id} value={workspace.id}>
                {workspace.name}
              </option>
            ))}
          </select>
        </label>
      </div>

      {/* Account cards + net worth */}
      <AccountCards
        workspaceId={workspaceId}
        asOfDate={asOfDate}
        selectedAccountId={selectedAccountId}
        onSelectAccount={setSelectedAccountId}
      />

      {/* Holdings + Allocation */}
      <div className="grid gap-4 lg:grid-cols-[1.4fr_1fr]">
        <div className="rounded-lg border bg-card p-4">
          <HoldingsTable
            accountId={selectedAccountId}
            asOfDate={asOfDate}
          />
        </div>
        <div className="rounded-lg border bg-card p-4">
          <AllocationChart
            scopeType="account"
            scopeId={selectedAccountId}
            asOfDate={asOfDate}
          />
        </div>
      </div>

      {/* Valuation chart */}
      <div className="rounded-lg border bg-card p-4">
        {selectedAccountId ? (
          <ValuationChart key={`${selectedAccountId}-${refreshKey}`} accountId={selectedAccountId} />
        ) : (
          <EmptyState
            title={t("selectAnAccount")}
            description={t("selectAnAccountDescription")}
          />
        )}
      </div>

      {/* Activity + Quick Actions */}
      <div className="grid gap-4 lg:grid-cols-[1.4fr_1fr]">
        <div className="rounded-lg border bg-card p-4">
          {selectedAccountId ? (
            <ActivityList
              key={selectedAccountId}
              accountId={selectedAccountId}
            />
          ) : (
            <EmptyState
              title={t("selectAnAccount")}
              description={t("selectAnAccountDescription")}
            />
          )}
        </div>
        <div className="rounded-lg border bg-card p-4">
          <QuickActions
            accountId={selectedAccountId}
            onRefresh={handleRefresh}
            asOfDate={asOfDate}
          />
        </div>
      </div>
    </div>
  );
}