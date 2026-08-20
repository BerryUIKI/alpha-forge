/**
 * PerformanceTab Component — real portfolio performance chart.
 *
 * Fetches workspace-level portfolio value history via usePortfolioPerformance
 * and renders a recharts LineChart. Implements the GUI-E2 requirement R1–R7.
 *
 * @module pages/today/tabs/PerformanceTab
 */

import { useState } from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
} from "recharts";
import { DashboardCard } from "@/components/ui";
import { EmptyState, ErrorState, LoadingSpinner } from "@/components/common";
import { cn } from "@/lib/utils";
import { useLocale } from "@/lib/i18n/useLocale";
import {
  usePortfolioPerformance,
  type PortfolioPerformancePeriod,
} from "@/pages/today/hooks/usePortfolioPerformance";

const TIME_PERIODS: readonly PortfolioPerformancePeriod[] = ["1W", "1M", "3M", "1Y"];

/** Format a value as currency for chart labels and tooltips. */
function fmtUsd(value: number, withSymbol = true): string {
  const formatter = new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
    maximumFractionDigits: 0,
  });
  if (!withSymbol) return formatter.format(value);
  return formatter.format(value);
}

export function PerformanceTab() {
  const { t } = useLocale();
  const [selectedPeriod, setSelectedPeriod] =
    useState<PortfolioPerformancePeriod>("1M");
  const { data, isLoading, isError, refetch } = usePortfolioPerformance(
    selectedPeriod,
  );

  return (
    <div className="flex flex-col gap-6">
      <DashboardCard
        title={t("portfolioPerformance")}
        action={
          <div className="flex items-center gap-1 rounded-lg bg-muted p-0.5">
            {TIME_PERIODS.map((period) => (
              <button
                key={period}
                onClick={() => setSelectedPeriod(period)}
                className={cn(
                  "rounded-md px-2 py-0.5 text-xs font-medium transition-colors",
                  selectedPeriod === period
                    ? "bg-background text-foreground shadow-sm"
                    : "text-muted-foreground hover:text-foreground",
                )}
              >
                {period}
              </button>
            ))}
          </div>
        }
      >
        {/* R3 — loading state */}
        {isLoading && <LoadingSpinner className="p-16" />}

        {/* R3 — error state with retry */}
        {isError && (
          <ErrorState
            message={t("failedToLoadPortfolioPerformance")}
            onRetry={() => refetch()}
          />
        )}

        {/* R3/R7 — empty state (no accounts or no valuations in this period) */}
        {!isLoading && !isError && (!data || data.points.length === 0) && (
          <EmptyState
            title={t("noValuationDataForPeriod")}
            description={t("noValuationDataDescription")}
          />
        )}

        {/* R1/R2 — render merged chart when data exists */}
        {!isLoading && !isError && data && data.points.length > 0 && (
          <div>
            {/* R5 — performance summary chips */}
            <div className="mb-4 flex flex-wrap gap-3">
              <PerformanceChip
                label={t("totalReturn")}
                value={data.totalReturnPct}
                format={(v) => `${(v * 100).toFixed(2)}%`}
              />
              <PerformanceChip
                label={t("xirr")}
                value={data.xirrPct}
                format={(v) => `${(v * 100).toFixed(2)}%`}
              />
              <PerformanceChip
                label={t("twr")}
                value={data.twrPct}
                format={(v) => `${(v * 100).toFixed(2)}%`}
              />
              <PerformanceChip label={t("accounts")} value={data.accountCount} format={(v) => String(v)} />
            </div>

            {/* R4 — base-currency line chart */}
            <div className="h-64">
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={data.points}>
                  <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border))" />
                  <XAxis
                    dataKey="date"
                    tick={{ fontSize: 11 }}
                    stroke="hsl(var(--muted-foreground))"
                    tickFormatter={(label: string) => label.slice(5)}
                  />
                  <YAxis
                    tick={{ fontSize: 11 }}
                    stroke="hsl(var(--muted-foreground))"
                    tickFormatter={(v: number) => fmtUsd(v, false)}
                  />
                  <Tooltip
                    content={({ active, payload, label }) => {
                      if (!active || !payload?.length) return null;
                      const value = payload[0]?.value;
                      return (
                        <div className="rounded-md border bg-card px-3 py-2 text-xs shadow">
                          <div className="text-muted-foreground">{String(label)}</div>
                          <div className="font-medium">{fmtUsd(Number(value))}</div>
                        </div>
                      );
                    }}
                  />
                  <Line
                    type="monotone"
                    dataKey="total_value_base"
                    stroke="hsl(var(--primary))"
                    strokeWidth={2}
                    dot={false}
                    activeDot={{ r: 4 }}
                  />
                </LineChart>
              </ResponsiveContainer>
            </div>
          </div>
        )}
      </DashboardCard>
    </div>
  );
}

/** Small stat chip for the performance summary row. */
function PerformanceChip({
  label,
  value,
  format,
}: {
  label: string;
  value: number | null;
  format: (value: number) => string;
}) {
  if (value == null) {
    return (
      <div className="rounded-lg border bg-muted/30 px-3 py-1.5">
        <span className="text-[10px] uppercase tracking-wide text-muted-foreground">{label}</span>
        <span className="ml-1.5 text-xs text-muted-foreground">—</span>
      </div>
    );
  }
  return (
    <div className="rounded-lg border bg-muted/30 px-3 py-1.5">
      <span className="text-[10px] uppercase tracking-wide text-muted-foreground">{label}</span>
      <span className="ml-1.5 text-xs font-semibold">{format(value)}</span>
    </div>
  );
}