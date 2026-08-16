/**
 * PerformanceTab Component
 *
 * Portfolio performance chart with time period selector.
 *
 * @version GUI-M3
 */

import { useState } from "react";
import { DashboardCard } from "@/components/ui";
import { cn } from "@/lib/utils";

// Placeholder chart data — replaced by real portfolio history in a later phase.
const CHART_BARS = [45, 55, 35, 65, 50, 70, 40, 60, 75, 55, 80, 65];

const TIME_PERIODS = ["1W", "1M", "3M", "1Y"] as const;

export function PerformanceTab() {
  const [selectedPeriod, setSelectedPeriod] = useState<(typeof TIME_PERIODS)[number]>("1M");

  return (
    <div className="flex flex-col gap-6">
      <DashboardCard
        title="Portfolio Performance"
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
        <div className="flex h-48 items-end gap-1.5">
          {CHART_BARS.map((height, index) => (
            <div
              key={index}
              className="flex-1 rounded-t-sm bg-gradient-to-t from-indigo-600/60 to-indigo-400/80 transition-all hover:from-indigo-600 hover:to-indigo-300"
              style={{ height: `${height}%` }}
            />
          ))}
        </div>
      </DashboardCard>
    </div>
  );
}