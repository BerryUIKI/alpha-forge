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
import { formatMessage } from "@/lib/i18n/locale";
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

  // ── Account list loading ──
  // Only the account list blocks the section; valuation data loads
  // independently so a valuation failure never hides account navigation.
  if (accounts.isLoading) {
    return <LoadingSpinner className="p-4" />;
  }

  // ── Account list error ──
  if (accounts.error) {
    return <ErrorState message={t("failedToLoadAccounts")} onRetry={() => accounts.refetch()} />;
  }

  // ── Empty ──
  if (!accounts.data?.length) {
    return (
      <div className="space-y-3">
        <EmptyState title={t("noAccountsYet")} description={t("noAccountsDescription")} />
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
  const valuationStale = Boolean(netWorth.error) && Boolean(netWorthData);
  const visibleAccounts = accounts.data.filter((account) => !account.is_archived);
  const missingValuations = visibleAccounts.filter(
    (account) => !netWorthData?.accounts?.some((balance) => balance.account_id === account.id),
  ).length;

  return (
    <div className="space-y-4">
      {/* Net Worth Overview */}
      {netWorth.isLoading ? (
        <div className="rounded-lg border bg-card p-4 text-sm text-muted-foreground" role="status">
          {t("netWorth")}: {t("loading")}
        </div>
      ) : netWorthData ? (
        <div className="rounded-lg border bg-card p-4 shadow-sm">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Banknote className="h-5 w-5 text-primary" />
              <span className="text-sm font-medium text-muted-foreground">{t("netWorth")}</span>
              {netWorth.isFetching && (
                <span className="text-[11px] text-muted-foreground" role="status">
                  {t("refreshing")}
                </span>
              )}
            </div>
            <span className="text-2xl font-bold">
              {fmtMoney(netWorthData.net_worth, netWorthData.base_currency)}
            </span>
          </div>
          <div className="mt-2 flex flex-wrap gap-4 text-xs text-muted-foreground">
            <span>
              {t("totalValue")}: {fmtMoney(netWorthData.total_assets, netWorthData.base_currency)}
            </span>
            <span>
              {t("totalLiabilities")}:{" "}
              {fmtMoney(netWorthData.total_liabilities, netWorthData.base_currency)}
            </span>
            <span>{formatMessage(t("netWorthAsOf"), { date: netWorthData.as_of_date })}</span>
          </div>
          {missingValuations > 0 && (
            <p className="mt-2 text-xs text-amber-700 dark:text-amber-400" role="status">
              {formatMessage(t("partialValuations"), { count: String(missingValuations) })}
            </p>
          )}
          {valuationStale && (
            <p className="mt-2 text-xs text-amber-700 dark:text-amber-400" role="status">
              {t("valuationStale")}{" "}
              <button
                type="button"
                onClick={() => void netWorth.refetch()}
                className="rounded border border-amber-500/30 px-1.5 py-0.5 font-medium hover:bg-amber-500/10"
              >
                {t("retry")}
              </button>
            </p>
          )}
        </div>
      ) : (
        <div
          className="flex flex-wrap items-center justify-between gap-2 rounded-lg border border-destructive/30 bg-destructive/5 p-3 text-xs text-destructive"
          role="alert"
        >
          <span>{t("valuationError")}</span>
          <button
            type="button"
            onClick={() => void netWorth.refetch()}
            className="rounded border border-destructive/30 px-2 py-1 font-medium hover:bg-destructive/10"
          >
            {t("retry")}
          </button>
        </div>
      )}

      {/* Account Cards */}
      <div className="flex gap-3 overflow-x-auto pb-2">
        {visibleAccounts.map((account) => {
          const isSelected = account.id === selectedAccountId;
          const isCredit = account.account_type === "credit_card";
          const sign = isCredit ? "-" : "";
          const netWorthValue = netWorthData
            ? netWorthData.accounts?.find((balance) => balance.account_id === account.id)
                ?.total_value
            : undefined;
          const balanceUnavailable = netWorthValue == null;

          return (
            <button
              key={account.id}
              onClick={() => onSelectAccount(account.id)}
              className={`flex min-w-[180px] flex-col gap-1 rounded-lg border p-4 text-left transition-all hover:shadow-md ${
                isSelected ? "border-primary bg-primary/5 ring-1 ring-primary" : "bg-card"
              }`}
            >
              <div className="flex items-center gap-2">
                <Wallet className="h-4 w-4 text-muted-foreground" />
                <span className="text-sm font-medium truncate">{account.name}</span>
                {account.is_default && (
                  <span className="rounded-full bg-primary/10 px-1.5 py-0.5 text-[10px] font-medium text-primary">
                    {t("defaultAccount")}
                  </span>
                )}
              </div>
              {balanceUnavailable ? (
                <span className="text-sm text-muted-foreground" title={t("valuationUnavailable")}>
                  {t("valuationUnavailable")}
                </span>
              ) : (
                <span className="text-lg font-semibold">
                  {fmtMoney(`${sign}${netWorthValue}`, account.currency)}
                </span>
              )}
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
