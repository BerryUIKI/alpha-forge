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
import { CreateAccountDialog } from "./CreateAccountDialog";
import { AddAssetDialog } from "./AddAssetDialog";
import { AddActivityDialog } from "./AddActivityDialog";
import { usePortfolioAccounts } from "../hooks/usePortfolio";
import { Plus, ArrowUpRight, PencilLine } from "lucide-react";

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

  const [isCreateAccountOpen, setIsCreateAccountOpen] = useState(false);
  const [isAddAssetOpen, setIsAddAssetOpen] = useState(false);
  const [isAddActivityOpen, setIsAddActivityOpen] = useState(false);

  const portfolioAccounts = usePortfolioAccounts(workspaceId);

  const selectedAccount = portfolioAccounts.data?.find(
    (acc) => acc.id === selectedAccountId,
  );
  const baseCurrency = selectedAccount?.currency ?? "USD";

  // Auto-select first workspace
  useEffect(() => {
    if (!workspaceId && workspaces.data?.[0]) {
      setWorkspaceId(workspaces.data[0].id);
    }
  }, [workspaceId, workspaces.data]);

  const handleCreateAccount = (accountId: string) => {
    setSelectedAccountId(accountId);
  };

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

      {/* Quick-create section */}
      <div className="rounded-lg border bg-card p-4">
        <h3 className="mb-2 text-sm font-medium text-muted-foreground">
          {(t as any)("newAccount") || "Create / Record"}
        </h3>
        <div className="flex flex-wrap gap-2">
          <button
            onClick={() => setIsCreateAccountOpen(true)}
            className="inline-flex items-center gap-2 rounded-md border bg-card px-3 py-2 text-sm font-medium transition-colors hover:bg-muted/50"
          >
            <Plus className="h-4 w-4" />
            {t("newAccount")}
          </button>
          <button
            onClick={() => setIsAddAssetOpen(true)}
            className="inline-flex items-center gap-2 rounded-md border bg-card px-3 py-2 text-sm font-medium transition-colors hover:bg-muted/50"
          >
            <PencilLine className="h-4 w-4" />
            {t("addAssetTitle")}
          </button>
          <button
            onClick={() => {
              if (selectedAccountId) {
                setIsAddActivityOpen(true);
              } else {
                setSelectedAccountId("");
                // Nudge the user to select an account first.
                window.alert(
                  (t as any)("selectAnAccount") ||
                    "Select an account first",
                );
              }
            }}
            disabled={!selectedAccountId}
            className="inline-flex items-center gap-2 rounded-md border bg-card px-3 py-2 text-sm font-medium transition-colors hover:bg-muted/50 disabled:cursor-not-allowed disabled:opacity-50"
          >
            <ArrowUpRight className="h-4 w-4" />
            {t("addActivityTitle")}
          </button>
        </div>
      </div>

      {/* Dialogs */}
      <CreateAccountDialog
        isOpen={isCreateAccountOpen}
        onClose={() => setIsCreateAccountOpen(false)}
        workspaceId={workspaceId}
        defaultCurrency={baseCurrency}
        onSuccess={handleCreateAccount}
      />
      <AddAssetDialog
        isOpen={isAddAssetOpen}
        onClose={() => setIsAddAssetOpen(false)}
        onSuccess={(assetId) => {
          void assetId;
          setIsAddAssetOpen(false);
        }}
      />
      <AddActivityDialog
        isOpen={isAddActivityOpen}
        onClose={() => setIsAddActivityOpen(false)}
        accountId={selectedAccountId}
        accountCurrency={baseCurrency}
        onSuccess={() => setIsAddActivityOpen(false)}
      />
    </div>
  );
}