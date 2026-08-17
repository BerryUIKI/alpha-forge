/**
 * AllocationChart Component
 *
 * Displays a donut chart of asset allocation using recharts.
 * Fetches allocation breakdown from the financial API.
 *
 * @module features/portfolio/components/AllocationChart
 */

import { useLocale } from "@/lib/i18n/useLocale";
import { useAllocation } from "@/features/portfolio/hooks/useFinancialData";
import { fmtMoney, fmtPercent } from "./helpers";
import { LoadingSpinner, EmptyState, ErrorState } from "@/components/common";
import { PieChart, Pie, Cell, Tooltip, ResponsiveContainer } from "recharts";

interface AllocationChartProps {
  scopeType: string;
  scopeId: string | null;
  asOfDate: string;
}

const COLORS = [
  "#2563eb", "#7c3aed", "#db2777", "#dc2626",
  "#ea580c", "#ca8a04", "#16a34a", "#0891b2",
  "#4f46e5", "#9333ea", "#c026d3", "#e11d48",
];

export function AllocationChart({ scopeType, scopeId, asOfDate }: AllocationChartProps) {
  const { t } = useLocale();
  const allocation = useAllocation(scopeType, scopeId, asOfDate);

  if (allocation.isLoading) {
    return <LoadingSpinner className="p-8" />;
  }

  if (allocation.error) {
    return (
      <ErrorState
        message={t("failedToCalculateAllocation")}
        onRetry={() => allocation.refetch()}
      />
    );
  }

  const data = allocation.data;
  if (!data || data.categories.length === 0) {
    return (
      <EmptyState
        title={t("noAllocationData")}
        description={t("allocationDescription")}
      />
    );
  }

  const chartData = data.categories.map((cat) => ({
    name: cat.category_name,
    value: cat.actual_bps / 100, // convert bps to percentage
    marketValue: cat.market_value,
    marketValueBase: cat.market_value_base,
  }));

  return (
    <div className="space-y-2">
      <h3 className="text-sm font-medium text-muted-foreground">
        {t("allocation" as any) || "Allocation"}
      </h3>

      <div className="flex items-center gap-4">
        <div className="h-48 w-48 flex-shrink-0">
          <ResponsiveContainer width="100%" height="100%">
            <PieChart>
              <Pie
                data={chartData}
                cx="50%"
                cy="50%"
                innerRadius={50}
                outerRadius={80}
                dataKey="value"
                nameKey="name"
              >
                {chartData.map((_entry, index) => (
                  <Cell
                    key={`cell-${index}`}
                    fill={COLORS[index % COLORS.length]}
                  />
                ))}
              </Pie>
              <Tooltip
              content={({ active, payload }) => {
                if (!active || !payload?.length) return null;
                const entry = payload[0];
                return (
                  <div className="rounded-md border bg-card px-3 py-2 text-xs shadow">
                    <div className="font-medium">{String(entry?.name)}</div>
                    <div>{Number(entry?.value).toFixed(1)}%</div>
                  </div>
                );
              }}
            />
            </PieChart>
          </ResponsiveContainer>
        </div>

        {/* Legend */}
        <div className="flex-1 space-y-1">
          {chartData.slice(0, 8).map((entry, index) => (
            <div key={entry.name} className="flex items-center justify-between text-xs">
              <div className="flex items-center gap-1.5">
                <div
                  className="h-2.5 w-2.5 rounded-sm"
                  style={{ backgroundColor: COLORS[index % COLORS.length] }}
                />
                <span className="truncate max-w-[120px]">{entry.name}</span>
              </div>
              <span className="font-mono">{entry.value.toFixed(1)}%</span>
            </div>
          ))}
          {chartData.length > 8 && (
            <div className="text-xs text-muted-foreground pt-1">
              +{chartData.length - 8} more
            </div>
          )}
        </div>
      </div>

      {/* Summary */}
      <div className="text-xs text-muted-foreground">
        {t("totalValue" as any) || "Total"}: {fmtMoney(data.total_market_value, "USD")}
      </div>
    </div>
  );
}