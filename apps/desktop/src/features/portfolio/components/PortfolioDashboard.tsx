/**
 * Portfolio Dashboard Component
 *
 * Main dashboard for portfolio management and analysis.
 * Orchestrates account management and analysis panels.
 *
 * @module features/portfolio/components/PortfolioDashboard
 */

import { useEffect, useState, useMemo, useCallback } from "react";
import { EmptyState, ErrorState, LoadingSpinner } from "@/components/common";
import { useWorkspaces } from "@/features/workspace/hooks/useWorkspaces";
import { useLocale } from "@/lib/i18n/useLocale";
import { usePortfolioAccounts } from "@/features/portfolio/hooks/usePortfolio";
import type { PortfolioAccount } from "@/lib/desktop-api/portfolio";

// Import refactored components
import { CreateAccountForm, AccountList, PositionPanel } from "./AccountManagement";
import {
  AllocationPanel,
  ConcentrationPanel,
  ThemeExposurePanel,
  AlignmentReviewPanel,
} from "./Analysis";

export function PortfolioDashboard() {
  const { t } = useLocale();
  const workspaces = useWorkspaces();
  const [workspaceId, setWorkspaceId] = useState("");
  const [selectedAccountId, setSelectedAccountId] = useState("");
  const accounts = usePortfolioAccounts(workspaceId);

  // Memoize selected account lookup
  const selectedAccount = useMemo(() => {
    return accounts.data?.find((account) => account.id === selectedAccountId);
  }, [accounts.data, selectedAccountId]);

  // Memoize callbacks to prevent unnecessary re-renders
  const handleWorkspaceChange = useCallback((event: React.ChangeEvent<HTMLSelectElement>) => {
    setWorkspaceId(event.target.value);
    setSelectedAccountId("");
  }, []);

  const handleAccountCreated = useCallback((id: string) => {
    setSelectedAccountId(id);
  }, []);

  const handleAccountSelect = useCallback((account: PortfolioAccount) => {
    setSelectedAccountId(account.id);
  }, []);

  // Auto-select first workspace
  useEffect(() => {
    if (!workspaceId && workspaces.data?.[0]) {
      setWorkspaceId(workspaces.data[0].id);
    }
  }, [workspaceId, workspaces.data]);

  // Auto-select first account
  useEffect(() => {
    if (!selectedAccountId && accounts.data?.[0]) {
      setSelectedAccountId(accounts.data[0].id);
    }
  }, [selectedAccountId, accounts.data]);

  // Loading and error states
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

  return (
    <div className="space-y-6">
      <p className="text-muted-foreground">{t("portfolioDescription")}</p>

      {/* Workspace selector */}
      <label className="block max-w-sm text-sm font-medium">
        {t("workspaceLabel")}
        <select
          value={workspaceId}
          onChange={handleWorkspaceChange}
          className="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
        >
          {workspaces.data.map((workspace) => (
            <option key={workspace.id} value={workspace.id}>
              {workspace.name}
            </option>
          ))}
        </select>
      </label>

      {/* Account management grid */}
      <div className="grid gap-6 lg:grid-cols-[minmax(0,0.8fr)_minmax(0,1.2fr)]">
        <div className="space-y-4">
          <CreateAccountForm
            workspaceId={workspaceId}
            onCreated={handleAccountCreated}
          />
          <AccountList
            workspaceId={workspaceId}
            selectedAccountId={selectedAccountId}
            onSelect={handleAccountSelect}
          />
        </div>
        <div>
          {selectedAccount ? (
            <PositionPanel account={selectedAccount} />
          ) : (
            <EmptyState
              title={t("selectAnAccount")}
              description={t("selectAnAccountDescription")}
            />
          )}
        </div>
      </div>

      {/* Analysis panels */}
      <div className="grid gap-6 lg:grid-cols-2">
        <AllocationPanel workspaceId={workspaceId} />
        <ConcentrationPanel workspaceId={workspaceId} />
      </div>
      <ThemeExposurePanel workspaceId={workspaceId} />
      <AlignmentReviewPanel workspaceId={workspaceId} />
    </div>
  );
}