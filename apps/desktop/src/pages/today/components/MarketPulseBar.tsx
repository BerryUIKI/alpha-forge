import { RefreshCw } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { cn } from "@/lib/utils";

interface BenchmarkItem {
  symbol: string;
  name: string;
  value: string;
  change: string;
  isPositive: boolean;
}

/**
 * Fixed benchmark values shown only when no live market feed exists.
 * They are demonstration content: the strip labels them explicitly and never
 * claims live or recently-synchronized data.
 */
const DEMO_BENCHMARKS: BenchmarkItem[] = [
  { symbol: "SPX", name: "S&P 500", value: "5,864.20", change: "+0.68%", isPositive: true },
  { symbol: "NDX", name: "NASDAQ 100", value: "18,342.10", change: "+1.14%", isPositive: true },
  { symbol: "US10Y", name: "US 10Y", value: "4.182%", change: "-2.4bp", isPositive: false },
  { symbol: "VIX", name: "VIX", value: "14.62", change: "-4.10%", isPositive: true },
];

export function MarketPulseBar() {
  const { t } = useLocale();

  return (
    <section
      aria-label={t("marketPulse")}
      data-demo="true"
      className="flex h-10 w-full items-center justify-between gap-4 overflow-x-auto border-b border-border bg-muted/40 px-5 text-xs select-none"
    >
      <div className="flex items-center gap-6">
        {/* Market Pulse label + explicit demo badge */}
        <div className="flex items-center gap-2 text-[11px] font-semibold text-muted-foreground">
          <span className="h-1.5 w-1.5 rounded-full bg-muted-foreground/50" aria-hidden="true" />
          <span>{t("marketPulse")}</span>
          <span className="rounded border border-border bg-background px-1.5 py-0.5 text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
            {t("marketDemoBadge")}
          </span>
        </div>

        {/* Benchmark list */}
        <div className="flex items-center gap-5 font-mono">
          {DEMO_BENCHMARKS.map((item) => (
            <div key={item.symbol} className="flex items-center gap-1.5">
              <span className="text-muted-foreground font-medium text-[11px] font-sans">
                {item.name}
              </span>
              <span className="text-foreground font-semibold text-xs">{item.value}</span>
              <span
                className={cn(
                  "text-[11px] font-medium",
                  item.isPositive ? "text-market-positive" : "text-market-negative",
                )}
              >
                {item.change}
              </span>
            </div>
          ))}
        </div>
      </div>

      {/* Demo notice — no freshness timestamp is claimed without retrieval metadata */}
      <div className="flex items-center gap-2 text-[11px] text-muted-foreground shrink-0">
        <span>{t("marketDemoNotice")}</span>
        <button
          type="button"
          disabled
          aria-label={t("refresh")}
          title={t("marketRefreshUnavailable")}
          className="rounded p-0.5 disabled:opacity-40"
        >
          <RefreshCw className="h-3 w-3" />
        </button>
      </div>
    </section>
  );
}
