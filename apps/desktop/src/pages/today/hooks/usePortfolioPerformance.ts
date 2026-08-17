/**
 * usePortfolioPerformance Hook — dashboard Performance tab.
 *
 * Fetches the whole-workspace portfolio value history by:
 * 1. listPortfolioAccounts(workspaceId)  — legacy dev command
 * 2. Promise.all(accounts → getPerformanceTimeSeries(accountId))
 * 3. merging per-account series by date (sum total_value_base)
 * 4. filtering by the selected time period
 *
 * Implements GUI-E2 requirement R1–R7.
 *
 * @module pages/today/hooks/usePortfolioPerformance
 */

import { useMemo } from "react";
import { useQuery } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import { financialKeys } from "@/features/portfolio/hooks/useFinancialData";
import type { PerformancePoint } from "@/types/financial";

export type PortfolioPerformancePeriod = "1W" | "1M" | "3M" | "1Y";

export interface PortfolioPerformancePoint {
  date: string; // YYYY-MM-DD
  total_value_base: number;
}

export interface PortfolioPerformance {
  points: PortfolioPerformancePoint[];
  currency: string;
  totalReturnPct: number | null;
  xirrPct: number | null;
  twrPct: number | null;
  accountCount: number;
}

const PERIOD_DAYS: Record<PortfolioPerformancePeriod, number> = {
  "1W": 7,
  "1M": 30,
  "3M": 91,
  "1Y": 365,
};

/**
 * Merge per-account PerformancePoints into a single workspace series.
 * Sums total_value_base per date (all values are already in base currency).
 */
export function mergePerformanceSeries(
  seriesList: PerformancePoint[][],
  startDate: string,
  endDate: string,
): PortfolioPerformancePoint[] {
  const byDate = new Map<string, number>();

  for (const series of seriesList) {
    for (const point of series) {
      if (point.date < startDate || point.date > endDate) continue;
      const current = byDate.get(point.date) ?? 0;
      byDate.set(point.date, current + parseFloat(point.total_value_base));
    }
  }

  return [...byDate.entries()]
    .map(([date, total_value_base]) => ({ date, total_value_base }))
    .sort((a, b) => a.date.localeCompare(b.date));
}

/**
 * Hook for the dashboard Performance tab.
 *
 * R1: workspace aggregation — fetch per-account series, merge by date.
 * R2: period selection — 1W/1M/3M/1Y filter.
 * R4: base-currency consolidation via total_value_base.
 * R7: zero accounts → empty points, not an error.
 */
export function usePortfolioPerformance(
  workspaceId: string,
  period: PortfolioPerformancePeriod = "1M",
) {
  const today = new Date().toISOString().slice(0, 10);
  const endDate = today;
  const startDate = useMemo(() => {
    const d = new Date();
    d.setDate(d.getDate() - PERIOD_DAYS[period]);
    return d.toISOString().slice(0, 10);
  }, [period]);

  return useQuery({
    queryKey: [...financialKeys.all, "workspace", "performance", workspaceId, period, startDate, endDate],
    queryFn: async (): Promise<PortfolioPerformance> => {
      const accounts = await desktopApi.portfolio.listPortfolioAccounts(workspaceId);

      // R7 — no accounts → empty series, no error
      if (accounts.length === 0) {
        return {
          points: [],
          currency: "USD",
          totalReturnPct: null,
          xirrPct: null,
          twrPct: null,
          accountCount: 0,
        };
      }

      // R1 — fetch per-account series in parallel
      const seriesList = await Promise.all(
        accounts.map((account) =>
          desktopApi.financial.getPerformanceTimeSeries(account.id),
        ),
      );

      // R1/R4 — merge by date, sum total_value_base
      const points = mergePerformanceSeries(seriesList, startDate, endDate);

      // R5 — per-account summary chips aggregated by value-weighted average
      const summaries = await Promise.all(
        accounts.map((account) =>
          desktopApi.financial.computePerformanceSummary(account.id, startDate, endDate),
        ),
      );

      const withValue = summaries.map((summary) => ({
        totalReturnPct: parseFloatMaybe(summary.total_return_pct),
        xirrPct: parseFloatMaybe(summary.xirr_pct),
        twrPct: parseFloatMaybe(summary.twr_pct),
        weight: parseFloat(summary.end_value || "0"),
      }));

      const totalWeight = withValue.reduce((sum, s) => sum + s.weight, 0);
      const weighted = (picker: (s: (typeof withValue)[number]) => number | null) =>
        totalWeight === 0
          ? null
          : withValue
              .map((s) => ({ value: picker(s), weight: s.weight }))
              .reduce((sum, s) => (s.value == null ? sum : sum + s.value * s.weight), 0) /
            totalWeight;

      return {
        points,
        currency: "USD",
        totalReturnPct: weighted((s) => s.totalReturnPct),
        xirrPct: weighted((s) => s.xirrPct),
        twrPct: weighted((s) => s.twrPct),
        accountCount: accounts.length,
      };
    },
    enabled: !!workspaceId,
    staleTime: 30_000,
  });
}

function parseFloatMaybe(value: string | null | undefined): number | null {
  if (value == null || value === "") return null;
  const n = parseFloat(value);
  return Number.isFinite(n) ? n : null;
}