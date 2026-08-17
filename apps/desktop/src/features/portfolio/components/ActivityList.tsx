/**
 * ActivityList Component
 *
 * Displays recent activities for the selected account. Uses the legacy
 * portfolio transactions command until repository-level activity CRUD
 * commands are wired (see FRONTEND_INTEGRATION.md §3.3).
 *
 * @module features/portfolio/components/ActivityList
 */

import { useLocale } from "@/lib/i18n/useLocale";
import { usePortfolioTransactions } from "@/features/portfolio/hooks/usePortfolio";
import { fmtMoney, fmtNumber } from "./helpers";
import { LoadingSpinner, EmptyState, ErrorState } from "@/components/common";

interface ActivityListProps {
  accountId: string;
  limit?: number;
}

export function ActivityList({ accountId, limit = 6 }: ActivityListProps) {
  const { t } = useLocale();
  const activities = usePortfolioTransactions(accountId);

  if (activities.isLoading) {
    return <LoadingSpinner className="p-4" />;
  }

  if (activities.error) {
    return (
      <ErrorState
        message={t("failedToLoadTransactions")}
        onRetry={() => activities.refetch()}
      />
    );
  }

  const data = activities.data ?? [];
  const recent = data.slice(0, limit);

  if (recent.length === 0) {
    return (
      <EmptyState
        title={t("noTransactionsImported")}
        description="Import transaction history to see recent activity."
      />
    );
  }

  return (
    <div className="space-y-2">
      <h3 className="text-sm font-medium text-muted-foreground">
        {t("recentActivity" as any) || "Recent Activity"}
      </h3>
      <div className="space-y-1">
        {recent.map((tx) => {
          const isBuy = tx.transaction_type === "buy";
          const isSell = tx.transaction_type === "sell";
          const adjective = isBuy ? "BUY" : isSell ? "SELL" : tx.transaction_type;
          const color = isBuy
            ? "text-green-600 dark:text-green-400"
            : isSell
              ? "text-red-600 dark:text-red-400"
              : "text-muted-foreground";

          return (
            <div
              key={tx.id}
              className="flex items-center justify-between rounded-md border px-3 py-1.5 text-xs hover:bg-muted/30"
            >
              <div className="flex items-center gap-2">
                <span className="font-mono text-muted-foreground">
                  {tx.executed_at?.slice(0, 10) ?? ""}
                </span>
                <span className={`font-medium uppercase ${color}`}>
                  {adjective}
                </span>
                <span className="font-mono">{tx.symbol}</span>
                <span className="text-muted-foreground">
                  x{fmtNumber(tx.quantity.toString(), 4)}
                </span>
              </div>
              <span className="font-mono">
                {fmtMoney(tx.price?.toString() ?? "0", "USD")}
              </span>
            </div>
          );
        })}
      </div>
    </div>
  );
}