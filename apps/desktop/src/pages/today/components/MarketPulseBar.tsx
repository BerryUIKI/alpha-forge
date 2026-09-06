import { RefreshCw } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { formatMessage } from "@/lib/i18n/locale";

interface BenchmarkItem {
  symbol: string;
  name: string;
  value: string;
  change: string;
  isPositive: boolean;
}

const DEFAULT_BENCHMARKS: BenchmarkItem[] = [
  { symbol: "SPX", name: "S&P 500", value: "5,864.20", change: "+0.68%", isPositive: true },
  { symbol: "NDX", name: "NASDAQ 100", value: "18,342.10", change: "+1.14%", isPositive: true },
  { symbol: "US10Y", name: "US 10Y", value: "4.182%", change: "-2.4bp", isPositive: false },
  { symbol: "VIX", name: "VIX", value: "14.62", change: "-4.10%", isPositive: true },
];

export function MarketPulseBar() {
  const { t } = useLocale();

  return (
    <section className="flex h-10 w-full items-center justify-between overflow-x-auto border-b border-white/5 bg-black/10 px-5 text-xs select-none">
      <div className="flex items-center gap-6">
        {/* Market Pulse Badge */}
        <div className="flex items-center gap-1.5 text-[11px] font-semibold text-neutral-400">
          <span className="relative flex h-1.5 w-1.5">
            <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75" />
            <span className="relative inline-flex rounded-full h-1.5 w-1.5 bg-emerald-500" />
          </span>
          <span>{t("marketPulse")}</span>
        </div>

        {/* Benchmarks List */}
        <div className="flex items-center gap-5 font-mono">
          {DEFAULT_BENCHMARKS.map((item) => (
            <div key={item.symbol} className="flex items-center gap-1.5">
              <span className="text-neutral-400 font-medium text-[11px] font-sans">
                {item.name}
              </span>
              <span className="text-neutral-200 font-semibold text-xs">
                {item.value}
              </span>
              <span
                className={`text-[11px] font-medium ${
                  item.isPositive ? "text-emerald-400" : "text-rose-400"
                }`}
              >
                {item.change}
              </span>
            </div>
          ))}
        </div>
      </div>

      {/* Synced Info */}
      <div className="flex items-center gap-2 text-[11px] text-neutral-400 shrink-0">
        <span>{formatMessage(t("dataSynced"), { time: "10s" })}</span>
        <button
          type="button"
          aria-label="Refresh data"
          className="rounded p-0.5 hover:text-neutral-200 transition-colors"
        >
          <RefreshCw className="h-3 w-3" />
        </button>
      </div>
    </section>
  );
}
