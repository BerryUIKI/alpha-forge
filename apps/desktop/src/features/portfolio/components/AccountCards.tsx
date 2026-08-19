/**
 * AccountCards Component
 *
 * Displays a horizontal scrollable row of account summary cards plus
 * a net worth overview card. Portfolio is a global dimension (ADR-0008):
 * all accounts are shown regardless of the active workspace, so the data
 * comes from the canonical `accounts` model (listAllFinancialAccounts).
 *
 * @module features/portfolio/components/AccountCards
 */

import { useLocale } from "@/lib/i18n/useLocale";
import { useListAllFinancialAccounts } from "@/features/portfolio/hooks/useFinancialData";
import { useNetWorth } from "@/features/portfolio/hooks/useFinancialData";
import { fmtMoney } from "./helpers";
import { LoadingSpinner, EmptyState, ErrorState } from "@/components/common";
import { Wallet, Banknote, Plus } from "lucide-react";

interface AccountCardsProps {
  asOfDate: string;
  selectedAccountId: string;
  onSelectAccount: (accountId: string) => void;
  onAddAccount?: () => void;
}

export function AccountCards({
  asOfDate,
  selectedAccountId,
  onSelectAccount,
  onAddAccount,
}: AccountCardsProps) {
  const { t } = useLocale();
  const accounts = useListAllFinancialAccounts();
  const netWorth = useNetWorth(asOfDate);

  // ── Loading ──
  if (accounts.isLoading || netWorth.isLoading) {
    return <LoadingSpinner className="p-4" />;
  }

  // ── Error ──
  if (accounts.error) {
    return (
      <ErrorState
        message={t("failedToLoadAccounts")}
        onRetry={() => accounts.refetch()}
      />
    );
  }

  // ── Empty ──
  if (!accounts.data?.length) {
    return (
      <div className="space-y-3">
        <EmptyState
          title={t("noAccountsYet")}
          description={t("noAccountsDescription")}
        />
        {onAddAccount && (
          <div className="flex justify-center">
            <button
              onClick={onAddAccount}
              className="inline-flex items-center gap-2 rounded-md border bg-card px-3 py-2 text-sm font-medium transition-colors hover:bg-muted/50"
            >
              <Plus className="h-4 w-4" />
              {t("createAccountTitle")}
            </button>
          </div>
        )}
      </div>
    );
  }

  const netWorthData = netWorth.data;

  return (
    <div className="space-y-4">
      {/* Net Worth Overview */}
      {netWorthData && (
        <div className="rounded-lg border bg-card p-4 shadow-sm">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Banknote className="h-5 w-5 text-primary" />
              <span className="text-sm font-medium text-muted-foreground">
                {t("netWorth")}
              </span>
            </div>
            <span className="text-2xl font-bold">
              {fmtMoney(netWorthData.net_worth, netWorthData.base_currency)}
            </span>
          </div>
          <div className="mt-2 flex gap-4 text-xs text-muted-foreground">
            <span>
              {t("totalValue")}: {fmtMoney(netWorthData.total_assets, netWorthData.base_currency)}
            </span>
            <span>
              {t("totalLiabilities")}: {fmtMoney(netWorthData.total_liabilities, netWorthData.base_currency)}
            </span>
          </div>
        </div>
      )}

      {/* Account Cards */}
      <div className="flex gap-3 overflow-x-auto pb-2">
        {accounts.data
          .filter((account) => !account.is_archived)
          .map((account) => {
            const isSelected = account.id === selectedAccountId;
            const isCredit = account.account_type === "credit_card";
            const sign = isCredit ? "-" : "";
            const netWorthValue = netWorthData
              ? netWorthData.accounts?.find(
                  (b) => b.account_id === account.id,
                )?.total_value
              : undefined;
            const balance = netWorthValue ?? "0";

            return (
              <button
                key={account.id}
                onClick={() => onSelectAccount(account.id)}
                className={`flex min-w-[180px] flex-col gap-1 rounded-lg border p-4 text-left transition-all hover:shadow-md ${
                  isSelected
                    ? "border-primary bg-primary/5 ring-1 ring-primary"
                    : "bg-card"
                }`}
              >
                <div className="flex items-center gap-2">
                  <Wallet className="h-4 w-4 text-muted-foreground" />
                  <span className="text-sm font-medium truncate">
                    {account.name}
                  </span>
                  {account.is_default && (
                    <span className="rounded-full bg-primary/10 px-1.5 py-0.5 text-[10px] font-medium text-primary">
                      {t("defaultAccount")}
                    </span>
                  )}
                </div>
                <span className="text-lg font-semibold">
                  {fmtMoney(`${sign}${balance}`, account.currency)}
                </span>
                <div className="flex items-center gap-1 text-xs text-muted-foreground">
                  <span className="capitalize">{account.account_type}</span>
                  <span>·</span>
                  <span>{account.currency}</span>
                </div>
              </button>
            );
          })}
      </div>
    </div>
  );
}