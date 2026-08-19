/**
 * Portfolio Dashboard Component
 *
 * Main dashboard for the Portfolio module (Phase 3).
 * Portfolio is a global dimension (ADR-0008): all accounts are shown
 * regardless of the active workspace. Orchestrates:
 * - Account cards + net worth
 * - Holdings table
 * - Allocation chart
 * - Valuation chart
 * - Recent activity
 * - Quick actions
 *
 * @module features/portfolio/components/PortfolioDashboard
 */

import { useState } from "react";
import { useLocale } from "@/lib/i18n/useLocale";
import { EmptyState } from "@/components/common";
import { AccountCards } from "./AccountCards";
import { HoldingsTable } from "./HoldingsTable";
import { AllocationChart } from "./AllocationChart";
import { ValuationChart } from "./ValuationChart";
import { ActivityList } from "./ActivityList";
import { QuickActions } from "./QuickActions";
import { CreateAccountDialog } from "./CreateAccountDialog";
import { AddAssetDialog } from "./AddAssetDialog";
import { AddActivityDialog } from "./AddActivityDialog";
import { useListAllFinancialAccounts } from "../hooks/useFinancialData";
import { Plus, ArrowUpRight, PencilLine } from "lucide-react";

/** Default view date — "today" as of the running app. */
function todayIso(): string {
  return new Date().toISOString().slice(0, 10);
}

export function PortfolioDashboard() {
  const { t } = useLocale();
  const [selectedAccountId, setSelectedAccountId] = useState("");
  const [asOfDate, setAsOfDate] = useState(todayIso());
  const [refreshKey, setRefreshKey] = useState(0);

  const [isCreateAccountOpen, setIsCreateAccountOpen] = useState(false);
  const [isAddAssetOpen, setIsAddAssetOpen] = useState(false);
  const [isAddActivityOpen, setIsAddActivityOpen] = useState(false);

  // Portfolio is a global dimension (ADR-0008): every account is listed
  // regardless of the active workspace.
  const portfolioAccounts = useListAllFinancialAccounts();

  const selectedAccount = portfolioAccounts.data?.find(
    (acc) => acc.id === selectedAccountId,
  );
  const baseCurrency = selectedAccount?.currency ?? "USD";

  const handleCreateAccount = (accountId: string) => {
    setSelectedAccountId(accountId);
  };

  const handleRefresh = () => {
    setAsOfDate(todayIso());
    setRefreshKey((k) => k + 1);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <p className="max-w-xl text-muted-foreground">
        {t("portfolioDescription")}
      </p>

      {/* Account cards + net worth */}
      <AccountCards
        asOfDate={asOfDate}
        selectedAccountId={selectedAccountId}
        onSelectAccount={setSelectedAccountId}
        onAddAccount={() => setIsCreateAccountOpen(true)}
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
          {t("newAccount")}
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
                window.alert(t("selectAnAccount"));
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