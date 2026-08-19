/**
 * ValuationChart Component
 *
 * Displays a time-series line chart of account value over time using recharts.
 * Fetches valuation series from the financial API.
 *
 * @module features/portfolio/components/ValuationChart
 */

import { useLocale } from "@/lib/i18n/useLocale";
import { useValuationSeries } from "@/features/portfolio/hooks/useFinancialData";
import { fmtMoney } from "./helpers";
import { LoadingSpinner, EmptyState, ErrorState } from "@/components/common";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
  CartesianGrid,
} from "recharts";

interface ValuationChartProps {
  accountId: string;
}

export function ValuationChart({ accountId }: ValuationChartProps) {
  const { t } = useLocale();
  const valuations = useValuationSeries(accountId);

  if (valuations.isLoading) {
    return <LoadingSpinner className="p-8" />;
  }

  if (valuations.error) {
    return (
      <ErrorState
        message={t("failedToLoadTransactions")}
        onRetry={() => valuations.refetch()}
      />
    );
  }

  const data = valuations.data;
  if (!data || data.length === 0) {
    return (
      <EmptyState
        title={t("noValuationData")}
        description={t("noRecentActivityDescription")}
      />
    );
  }

  const chartData = data.map((v) => ({
    date: v.valuation_date.slice(0, 7), // "2026-08" format
    value: parseFloat(v.total_value_base),
  }));

  const currency = data[0]?.currency || "USD";

  return (
    <div className="space-y-2">
      <h3 className="text-sm font-medium text-muted-foreground">
        {t("accountValue")}
      </h3>

      <div className="h-52">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border))" />
            <XAxis
              dataKey="date"
              tick={{ fontSize: 11 }}
              stroke="hsl(var(--muted-foreground))"
            />
            <YAxis
              tick={{ fontSize: 11 }}
              stroke="hsl(var(--muted-foreground))"
              tickFormatter={(v: number) => {
                if (v >= 1000) return `$${(v / 1000).toFixed(0)}k`;
                return `$${v.toFixed(0)}`;
              }}
            />
            <Tooltip
              content={({ active, payload, label }) => {
                if (!active || !payload?.length) return null;
                const value = payload[0]?.value;
                return (
                  <div className="rounded-md border bg-card px-3 py-2 text-xs shadow">
                    <div className="text-muted-foreground">Date: {String(label)}</div>
                    <div className="font-medium">{fmtMoney(String(value), currency)}</div>
                  </div>
                );
              }}
            />
            <Line
              type="monotone"
              dataKey="value"
              stroke="hsl(var(--primary))"
              strokeWidth={2}
              dot={false}
              activeDot={{ r: 4 }}
            />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}