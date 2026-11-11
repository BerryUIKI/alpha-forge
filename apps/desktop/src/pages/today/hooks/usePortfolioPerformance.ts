/**
 * usePortfolioPerformance Hook — dashboard Performance tab.
 *
 * Delegates consolidation to the Rust `compute_consolidated_performance`
 * command: portfolio-level base-currency valuations and external flows are
 * aggregated together, so returns, XIRR, and TWR are not averages of
 * per-account metrics. The hook only shapes the response for the chart.
 *
 * @module pages/today/hooks/usePortfolioPerformance
 */

import { useMemo } from "react";
import { useQuery } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import { financialKeys } from "@/features/portfolio/hooks/useFinancialData";

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
  coveredAccountCount: number;
  skippedAccounts: number;
  dataQuality: "complete" | "partial" | "insufficient_data";
}

const PERIOD_DAYS: Record<PortfolioPerformancePeriod, number> = {
  "1W": 7,
  "1M": 30,
  "3M": 91,
  "1Y": 365,
};

/**
 * Hook for the dashboard Performance tab.
 *
 * R1: global aggregation — every account is consolidated in Rust.
 * R2: period selection — 1W/1M/3M/1Y filter applied to the valuation range.
 * R4: one explicit base currency, returned by the backend.
 * R7: zero accounts → empty points, not an error.
 */
export function usePortfolioPerformance(period: PortfolioPerformancePeriod = "1M") {
  const today = new Date().toISOString().slice(0, 10);
  const endDate = today;
  const startDate = useMemo(() => {
    const d = new Date();
    d.setDate(d.getDate() - PERIOD_DAYS[period]);
    return d.toISOString().slice(0, 10);
  }, [period]);

  return useQuery({
    queryKey: [...financialKeys.all, "global", "performance", period, startDate, endDate],
    queryFn: async (): Promise<PortfolioPerformance> => {
      const accounts = await desktopApi.financial.listAllFinancialAccounts();

      // R7 — no accounts → empty series, no error
      if (accounts.length === 0) {
        return {
          points: [],
          currency: "",
          totalReturnPct: null,
          xirrPct: null,
          twrPct: null,
          accountCount: 0,
          coveredAccountCount: 0,
          skippedAccounts: 0,
          dataQuality: "insufficient_data",
        };
      }

      const consolidated = await desktopApi.financial.computeConsolidatedPerformance(
        accounts.map((account) => account.id),
        startDate,
        endDate,
      );

      return {
        points: consolidated.points.map((point) => ({
          date: point.date,
          total_value_base: parseFloat(point.total_value_base),
        })),
        currency: consolidated.base_currency,
        totalReturnPct: parseDecimal(consolidated.total_return_pct),
        xirrPct: parseDecimal(consolidated.xirr_pct),
        twrPct: parseDecimal(consolidated.twr_pct),
        accountCount: consolidated.account_count,
        coveredAccountCount: consolidated.covered_account_count,
        skippedAccounts: consolidated.skipped_accounts,
        dataQuality: consolidated.data_quality as PortfolioPerformance["dataQuality"],
      };
    },
    staleTime: 30_000,
  });
}

function parseDecimal(value: string | null | undefined): number | null {
  if (value == null || value === "") return null;
  const parsed = Number.parseFloat(value);
  return Number.isFinite(parsed) ? parsed : null;
}
