/**
 * ActivityList Component
 *
 * Displays recent activities for the selected account using the
 * financial API (listActivitiesByAccount).
 */

import { useLocale } from "@/lib/i18n/useLocale";
import { useListActivitiesByAccount } from "@/features/portfolio/hooks/useFinancialData";
import { fmtMoney, fmtNumber } from "./helpers";
import { LoadingSpinner, EmptyState, ErrorState } from "@/components/common";

interface ActivityListProps {
  accountId: string;
  limit?: number;
}

export function ActivityList({ accountId, limit = 6 }: ActivityListProps) {
  const { t } = useLocale();
  const activities = useListActivitiesByAccount(accountId);

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
        description="Record activities or import transaction history to see recent activity."
      />
    );
  }

  const activityColor = (type: string) => {
    switch (type) {
      case "buy": return "text-green-600 dark:text-green-400";
      case "sell": return "text-red-600 dark:text-red-400";
      case "dividend": return "text-blue-600 dark:text-blue-400";
      case "deposit": return "text-emerald-600 dark:text-emerald-400";
      case "withdrawal": return "text-orange-600 dark:text-orange-400";
      default: return "text-muted-foreground";
    }
  };

  return (
    <div className="space-y-2">
      <h3 className="text-sm font-medium text-muted-foreground">
        {(t as any)("recentActivity") || "Recent Activity"}
      </h3>
      <div className="space-y-1">
        {recent.map((tx) => {
          const adjective = tx.activity_type;
          const color = activityColor(tx.activity_type);

          return (
            <div
              key={tx.id}
              className="flex items-center justify-between rounded-md border px-3 py-1.5 text-xs hover:bg-muted/30"
            >
              <div className="flex items-center gap-2">
                <span className="font-mono text-muted-foreground">
                  {tx.activity_date?.slice(0, 10) ?? ""}
                </span>
                <span className={`font-medium uppercase ${color}`}>
                  {adjective}
                </span>
                {tx.quantity && (
                  <>
                    <span className="text-muted-foreground">x</span>
                    <span className="font-mono">
                      {fmtNumber(tx.quantity, 4)}
                    </span>
                  </>
                )}
              </div>
              {tx.amount && (
                <span className="font-mono">
                  {fmtMoney(tx.amount, tx.currency)}
                </span>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}