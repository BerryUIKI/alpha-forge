/**
 * HoldingsTable Component
 *
 * Displays current holdings for a selected account with quantity,
 * market value, gain/loss, and weight columns.
 *
 * @module features/portfolio/components/HoldingsTable
 */

import { useLocale } from "@/lib/i18n/useLocale";
import { useHoldings } from "@/features/portfolio/hooks/useFinancialData";
import { fmtMoney, fmtNumber, fmtPercent, fmtGainLoss, gainLossClass } from "./helpers";
import { LoadingSpinner, EmptyState, ErrorState } from "@/components/common";
import { TrendingUp, TrendingDown } from "lucide-react";

interface HoldingsTableProps {
  accountId: string;
  asOfDate: string;
}

export function HoldingsTable({ accountId, asOfDate }: HoldingsTableProps) {
  const { t } = useLocale();
  const holdings = useHoldings(accountId, asOfDate);

  if (holdings.isLoading) {
    return <LoadingSpinner className="p-8" />;
  }

  if (holdings.error) {
    return (
      <ErrorState
        message={t("failedToLoadHoldings")}
        onRetry={() => holdings.refetch()}
      />
    );
  }

  const summary = holdings.data;
  if (!summary || summary.holdings.length === 0) {
    return (
      <EmptyState
        title={t("noHoldingsYet")}
        description={t("noHoldingsDescription")}
      />
    );
  }

  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-medium text-muted-foreground">
          {t("accounts" as any) || "Holdings"} ({summary.holdings.length})
        </h3>
        <span className="text-xs text-muted-foreground">
          {t("totalValue" as any) || "Total"}: {fmtMoney(summary.total_market_value, summary.holdings[0]?.currency)}
        </span>
      </div>

      <div className="overflow-x-auto rounded-lg border">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b bg-muted/50">
              <th className="px-3 py-2 text-left font-medium text-muted-foreground">
                {t("symbolLabel" as any) || "Symbol"}
              </th>
              <th className="px-3 py-2 text-right font-medium text-muted-foreground">
                {t("quantityLabel" as any) || "Qty"}
              </th>
              <th className="px-3 py-2 text-right font-medium text-muted-foreground">
                {t("marketValue" as any) || "Mkt Value"}
              </th>
              <th className="px-3 py-2 text-right font-medium text-muted-foreground">
                {t("costBasisLabel" as any) || "Cost Basis"}
              </th>
              <th className="px-3 py-2 text-right font-medium text-muted-foreground">
                {t("gainLoss" as any) || "Gain/Loss"}
              </th>
              <th className="px-3 py-2 text-right font-medium text-muted-foreground">
                {t("weight" as any) || "Weight"}
              </th>
            </tr>
          </thead>
          <tbody>
            {summary.holdings.map((holding) => {
              const gainLoss = holding.unrealized_gain;
              const isPositive = parseFloat(gainLoss) >= 0;
              return (
                <tr
                  key={holding.asset_id}
                  className="border-b last:border-0 hover:bg-muted/30"
                >
                  <td className="px-3 py-2">
                    <div className="flex items-center gap-2">
                      {isPositive ? (
                        <TrendingUp className="h-3.5 w-3.5 text-green-500" />
                      ) : (
                        <TrendingDown className="h-3.5 w-3.5 text-red-500" />
                      )}
                      <div>
                        <span className="font-medium">
                          {holding.asset_symbol || holding.asset_id.slice(0, 8)}
                        </span>
                        {holding.asset_name && (
                          <span className="ml-1.5 text-xs text-muted-foreground">
                            {holding.asset_name}
                          </span>
                        )}
                      </div>
                    </div>
                  </td>
                  <td className="px-3 py-2 text-right font-mono text-xs">
                    {fmtNumber(holding.quantity, 4)}
                  </td>
                  <td className="px-3 py-2 text-right font-mono text-xs">
                    {fmtMoney(holding.market_value, holding.currency)}
                  </td>
                  <td className="px-3 py-2 text-right font-mono text-xs">
                    {fmtMoney(holding.cost_basis, holding.currency)}
                  </td>
                  <td className={`px-3 py-2 text-right font-mono text-xs ${gainLossClass(gainLoss)}`}>
                    {fmtGainLoss(gainLoss, holding.currency)}
                    {holding.unrealized_gain_pct && (
                      <span className="ml-1">
                        ({fmtPercent(holding.unrealized_gain_pct, 1)})
                      </span>
                    )}
                  </td>
                  <td className="px-3 py-2 text-right font-mono text-xs">
                    {fmtPercent(holding.weight_pct, 1)}
                  </td>
                </tr>
              );
            })}
          </tbody>
          <tfoot>
            <tr className="border-t bg-muted/30 font-medium">
              <td className="px-3 py-2 text-xs text-muted-foreground">
                {t("total" as any) || "Total"}
              </td>
              <td className="px-3 py-2 text-right font-mono text-xs">
                {fmtNumber(
                  summary.holdings.reduce((s, h) => s + parseFloat(h.quantity), 0).toString(),
                  0,
                )}
              </td>
              <td className="px-3 py-2 text-right font-mono text-xs">
                {fmtMoney(summary.total_market_value, summary.holdings[0]?.currency)}
              </td>
              <td className="px-3 py-2 text-right font-mono text-xs">
                {fmtMoney(summary.total_cost_basis, summary.holdings[0]?.currency)}
              </td>
              <td className={`px-3 py-2 text-right font-mono text-xs ${gainLossClass(summary.total_unrealized_gain)}`}>
                {fmtGainLoss(summary.total_unrealized_gain, summary.holdings[0]?.currency)}
              </td>
              <td className="px-3 py-2 text-right font-mono text-xs">
                100%
              </td>
            </tr>
          </tfoot>
        </table>
      </div>

      {/* Summary row */}
      <div className="flex gap-4 text-xs text-muted-foreground">
        <span>
          Cash: {fmtMoney(summary.cash_balance, summary.holdings[0]?.currency)}
        </span>
      </div>
    </div>
  );
}