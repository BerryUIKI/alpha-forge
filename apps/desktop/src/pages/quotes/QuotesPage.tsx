import { useState } from "react";
import { Link } from "react-router-dom";
import {
  RefreshCw,
  Briefcase,
  Layers,
  ArrowUpRight,
  ArrowDownRight,
  Clock,
  BarChart2,
  AlertCircle,
  TrendingUp,
  Table2,
} from "lucide-react";
import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
  CartesianGrid,
} from "recharts";
import {
  useListActiveAssets,
  useListQuotesForAsset,
  useRefreshAllActiveQuotes,
  useRefreshAssetQuote,
} from "@/features/portfolio/hooks/useFinancialData";
import { useTheses } from "@/features/thesis/hooks/useTheses";
import { useActiveWorkspaceId } from "@/features/workspace/hooks/useActiveWorkspace.context";
import { useLocale } from "@/lib/i18n/useLocale";
import { EmptyState, ErrorState, LoadingSpinner } from "@/components/common";
import type { Asset, Quote } from "@/types/financial";
import { cn } from "@/lib/utils";

/**
 * Single Asset Quote Card with Sparkline & on-demand single refresh.
 */
function AssetQuoteCard({
  asset,
  isSelected,
  onSelect,
}: {
  asset: Asset;
  isSelected: boolean;
  onSelect: () => void;
}) {
  const { t } = useLocale();
  const workspaceId = useActiveWorkspaceId();
  const { data: quotes = [], isLoading } = useListQuotesForAsset(asset.id);
  const { data: theses = [] } = useTheses(workspaceId);
  const refreshQuote = useRefreshAssetQuote();

  const linkedTheses = theses.filter(
    (thesis) => thesis.portfolioAssetId === asset.id,
  );

  const latestQuote: Quote | undefined = quotes[0];
  const previousQuote: Quote | undefined = quotes[1];

  let changePercent: number | null = null;
  let isPositive = true;

  if (latestQuote && previousQuote) {
    const curr = parseFloat(latestQuote.close);
    const prev = parseFloat(previousQuote.close);
    if (prev > 0) {
      changePercent = ((curr - prev) / prev) * 100;
      isPositive = changePercent >= 0;
    }
  }

  const handleSingleRefresh = (e: React.MouseEvent) => {
    e.stopPropagation();
    refreshQuote.mutate(asset.id);
  };

  return (
    <div
      onClick={onSelect}
      className={cn(
        "cursor-pointer rounded-xl border p-4 transition-all hover:border-primary/50 hover:shadow-md",
        isSelected
          ? "border-primary bg-primary/5 ring-1 ring-primary/20"
          : "border-border/60 bg-card/60 hover:bg-card/90",
      )}
    >
      <div className="flex items-start justify-between">
        <div>
          <div className="flex items-center gap-2">
            <span className="font-mono text-base font-bold tracking-tight">
              {asset.display_code || asset.instrument_symbol || "—"}
            </span>
            <span className="rounded bg-muted px-1.5 py-0.5 text-[10px] font-medium uppercase text-muted-foreground">
              {asset.kind}
            </span>
          </div>
          <p className="mt-0.5 max-w-[200px] truncate text-xs text-muted-foreground">
            {asset.name || asset.display_code || "Unnamed Asset"}
          </p>
        </div>

        <button
          type="button"
          onClick={handleSingleRefresh}
          disabled={refreshQuote.isPending}
          title={t("refreshSingleQuote")}
          className="rounded-lg p-1.5 text-muted-foreground transition-colors hover:bg-accent hover:text-foreground disabled:opacity-50"
        >
          <RefreshCw
            className={cn("h-3.5 w-3.5", refreshQuote.isPending && "animate-spin")}
          />
        </button>
      </div>

      <div className="mt-4 flex items-baseline justify-between">
        <div>
          {isLoading ? (
            <div className="h-6 w-20 animate-pulse rounded bg-muted" />
          ) : latestQuote ? (
            <div className="flex items-baseline gap-1.5">
              <span className="font-mono text-xl font-bold tracking-tight">
                {parseFloat(latestQuote.close).toFixed(2)}
              </span>
              <span className="text-xs text-muted-foreground">
                {latestQuote.currency || asset.quote_ccy}
              </span>
            </div>
          ) : (
            <span className="text-xs text-muted-foreground">
              {t("noQuotesYet")}
            </span>
          )}
        </div>

        {changePercent !== null && (
          <div
            className={cn(
              "flex items-center gap-0.5 text-xs font-semibold",
              isPositive ? "text-emerald-500" : "text-rose-500",
            )}
          >
            {isPositive ? (
              <ArrowUpRight className="h-3.5 w-3.5" />
            ) : (
              <ArrowDownRight className="h-3.5 w-3.5" />
            )}
            <span>{Math.abs(changePercent).toFixed(2)}%</span>
          </div>
        )}
      </div>

      {/* Footer tags */}
      <div className="mt-3 flex items-center justify-between border-t border-border/40 pt-2.5 text-[11px] text-muted-foreground">
        <span className="flex items-center gap-1">
          <Clock className="h-3 w-3" />
          {latestQuote ? latestQuote.day : "—"}
        </span>

        {linkedTheses.length > 0 && (
          <span className="flex items-center gap-1 text-primary">
            <Layers className="h-3 w-3" />
            <span>
              {linkedTheses.length} {t("thesisFeature")}
            </span>
          </span>
        )}
      </div>
    </div>
  );
}

/**
 * Historical Quotes Viewer with Visual Trend Chart (AreaChart) & Tabular Breakdown.
 */
function AssetQuoteHistory({ asset }: { asset: Asset }) {
  const { t } = useLocale();
  const { data: quotes = [], isLoading, error } = useListQuotesForAsset(asset.id);
  const [activeTab, setActiveTab] = useState<"chart" | "table">("chart");

  if (isLoading) {
    return (
      <div className="flex h-48 items-center justify-center rounded-xl border border-border/60 bg-card p-6">
        <LoadingSpinner />
      </div>
    );
  }

  if (error) {
    return (
      <ErrorState
        title={t("unknownError")}
        message={error.message}
      />
    );
  }

  // Quotes are typically returned latest first. Sort chronologically for charts.
  const chronologicalQuotes = [...quotes].sort(
    (a, b) => new Date(a.day).getTime() - new Date(b.day).getTime(),
  );

  const chartData = chronologicalQuotes.map((q) => ({
    day: q.day,
    close: parseFloat(q.close),
    open: q.open ? parseFloat(q.open) : null,
    high: q.high ? parseFloat(q.high) : null,
    low: q.low ? parseFloat(q.low) : null,
    volume: q.volume ? parseInt(q.volume, 10) : 0,
    currency: q.currency || asset.quote_ccy,
  }));

  const closePrices = chartData.map((d) => d.close).filter((p) => !isNaN(p));
  const periodHigh = closePrices.length ? Math.max(...closePrices) : null;
  const periodLow = closePrices.length ? Math.min(...closePrices) : null;
  const periodAvg = closePrices.length
    ? closePrices.reduce((acc, cur) => acc + cur, 0) / closePrices.length
    : null;

  return (
    <div className="rounded-xl border border-border/60 bg-card/60 p-5">
      {/* Header with Title and Mode Toggle */}
      <div className="mb-4 flex flex-wrap items-center justify-between gap-4">
        <div>
          <h3 className="text-base font-semibold">
            {t("quoteHistory")}: {asset.display_code || asset.instrument_symbol}
          </h3>
          <p className="mt-0.5 text-xs text-muted-foreground">
            {asset.name} · {asset.quote_ccy}
          </p>
        </div>

        <div className="flex items-center gap-3">
          {/* Quick Metrics */}
          {periodHigh !== null && periodLow !== null && (
            <div className="hidden items-center gap-2 sm:flex">
              <span className="rounded bg-muted px-2 py-0.5 text-[11px] text-muted-foreground">
                {t("highestInPeriod")}: <strong className="font-mono text-foreground">{periodHigh.toFixed(2)}</strong>
              </span>
              <span className="rounded bg-muted px-2 py-0.5 text-[11px] text-muted-foreground">
                {t("lowestInPeriod")}: <strong className="font-mono text-foreground">{periodLow.toFixed(2)}</strong>
              </span>
              {periodAvg !== null && (
                <span className="rounded bg-muted px-2 py-0.5 text-[11px] text-muted-foreground">
                  {t("averageClose")}: <strong className="font-mono text-foreground">{periodAvg.toFixed(2)}</strong>
                </span>
              )}
            </div>
          )}

          {/* Chart / Table View Toggle */}
          <div className="flex items-center rounded-lg bg-muted p-0.5">
            <button
              type="button"
              onClick={() => setActiveTab("chart")}
              className={cn(
                "flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs font-medium transition-colors",
                activeTab === "chart"
                  ? "bg-background text-foreground shadow-sm"
                  : "text-muted-foreground hover:text-foreground",
              )}
            >
              <TrendingUp className="h-3.5 w-3.5" />
              <span>{t("quoteChartView")}</span>
            </button>
            <button
              type="button"
              onClick={() => setActiveTab("table")}
              className={cn(
                "flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs font-medium transition-colors",
                activeTab === "table"
                  ? "bg-background text-foreground shadow-sm"
                  : "text-muted-foreground hover:text-foreground",
              )}
            >
              <Table2 className="h-3.5 w-3.5" />
              <span>{t("quoteTableView")}</span>
            </button>
          </div>
        </div>
      </div>

      {quotes.length === 0 ? (
        <p className="py-8 text-center text-xs text-muted-foreground">
          {t("noQuotesYet")}
        </p>
      ) : activeTab === "chart" ? (
        /* Visual Chart View */
        <div className="space-y-4">
          <div className="h-72 w-full pt-2">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart data={chartData} margin={{ top: 10, right: 10, left: 0, bottom: 0 }}>
                <defs>
                  <linearGradient id="quotePriceGradient" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="hsl(var(--primary))" stopOpacity={0.4} />
                    <stop offset="95%" stopColor="hsl(var(--primary))" stopOpacity={0.0} />
                  </linearGradient>
                </defs>
                <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border))" opacity={0.6} />
                <XAxis
                  dataKey="day"
                  tick={{ fontSize: 11 }}
                  stroke="hsl(var(--muted-foreground))"
                  tickFormatter={(val: string) => (val.length > 5 ? val.slice(5) : val)}
                />
                <YAxis
                  domain={["auto", "auto"]}
                  tick={{ fontSize: 11 }}
                  stroke="hsl(var(--muted-foreground))"
                  tickFormatter={(val: number) => val.toFixed(1)}
                />
                <Tooltip
                  content={({ active, payload, label }) => {
                    if (!active || !payload?.length) return null;
                    const item = payload[0]?.payload;
                    if (!item) return null;
                    return (
                      <div className="rounded-lg border border-border bg-card p-2.5 text-xs shadow-md">
                        <div className="font-semibold text-foreground">{String(label)}</div>
                        <div className="mt-1 flex items-center justify-between gap-4">
                          <span className="text-muted-foreground">{t("closePrice")}:</span>
                          <span className="font-mono font-bold text-primary">
                            {item.close.toFixed(2)} {item.currency}
                          </span>
                        </div>
                        {item.high && item.low && (
                          <div className="mt-0.5 flex items-center justify-between gap-4 text-muted-foreground">
                            <span>{t("highLow")}:</span>
                            <span className="font-mono">
                              {item.high.toFixed(2)} / {item.low.toFixed(2)}
                            </span>
                          </div>
                        )}
                        {item.volume > 0 && (
                          <div className="mt-0.5 flex items-center justify-between gap-4 text-muted-foreground">
                            <span>{t("quoteVolume")}:</span>
                            <span className="font-mono">{item.volume.toLocaleString()}</span>
                          </div>
                        )}
                      </div>
                    );
                  }}
                />
                <Area
                  type="monotone"
                  dataKey="close"
                  stroke="hsl(var(--primary))"
                  strokeWidth={2}
                  fillOpacity={1}
                  fill="url(#quotePriceGradient)"
                  activeDot={{ r: 5, strokeWidth: 1 }}
                />
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </div>
      ) : (
        /* Tabular View */
        <div className="overflow-x-auto">
          <table className="w-full text-left text-xs">
            <thead>
              <tr className="border-b border-border/60 text-muted-foreground">
                <th className="pb-2 font-medium">{t("quoteDay")}</th>
                <th className="pb-2 font-medium">{t("closePrice")}</th>
                <th className="pb-2 font-medium">{t("openPrice")}</th>
                <th className="pb-2 font-medium">{t("highLow")}</th>
                <th className="pb-2 font-medium">{t("quoteVolume")}</th>
                <th className="pb-2 font-medium">{t("quoteSource")}</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-border/30">
              {quotes.map((q) => (
                <tr key={q.id} className="hover:bg-muted/30">
                  <td className="py-2.5 font-mono">{q.day}</td>
                  <td className="py-2.5 font-mono font-bold">
                    {parseFloat(q.close).toFixed(2)} {q.currency}
                  </td>
                  <td className="py-2.5 font-mono text-muted-foreground">
                    {q.open ? parseFloat(q.open).toFixed(2) : "—"}
                  </td>
                  <td className="py-2.5 font-mono text-muted-foreground">
                    {q.high && q.low
                      ? `${parseFloat(q.high).toFixed(2)} / ${parseFloat(q.low).toFixed(2)}`
                      : "—"}
                  </td>
                  <td className="py-2.5 font-mono text-muted-foreground">
                    {q.volume ? parseInt(q.volume, 10).toLocaleString() : "—"}
                  </td>
                  <td className="py-2.5">
                    <span className="rounded bg-muted px-1.5 py-0.5 text-[10px] uppercase font-mono text-muted-foreground">
                      {q.source}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}

/**
 * Market Quotes Dashboard Page Component
 */
export function QuotesPage() {
  const { t } = useLocale();
  const { data: assets = [], isLoading, error } = useListActiveAssets();
  const refreshAll = useRefreshAllActiveQuotes();
  const [selectedAssetId, setSelectedAssetId] = useState<string | null>(null);

  const selectedAsset =
    assets.find((a) => a.id === selectedAssetId) || assets[0] || null;

  return (
    <div className="space-y-6 p-6">
      {/* Page Header */}
      <div className="flex flex-wrap items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold tracking-tight">
            {t("quotesTitle")}
          </h1>
          <p className="mt-1 text-sm text-muted-foreground">
            {t("quotesDescription")}
          </p>
        </div>

        <button
          type="button"
          onClick={() => refreshAll.mutate()}
          disabled={refreshAll.isPending || assets.length === 0}
          className="inline-flex items-center gap-2 rounded-lg bg-primary px-3.5 py-2 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90 disabled:opacity-50"
        >
          <RefreshCw
            className={cn("h-4 w-4", refreshAll.isPending && "animate-spin")}
          />
          <span>
            {refreshAll.isPending
              ? t("refreshingAllQuotes")
              : t("refreshAllQuotes")}
          </span>
        </button>
      </div>

      {refreshAll.isError && (
        <div className="flex items-center gap-2 rounded-lg border border-destructive/40 bg-destructive/10 px-3.5 py-2 text-xs text-destructive">
          <AlertCircle className="h-4 w-4" />
          <span>
            {refreshAll.error instanceof Error
              ? refreshAll.error.message
              : t("unknownError")}
          </span>
        </div>
      )}

      {/* Main Content */}
      {isLoading ? (
        <div className="flex h-64 items-center justify-center">
          <LoadingSpinner />
        </div>
      ) : error ? (
        <ErrorState
          title={t("unknownError")}
          message={error.message}
        />
      ) : assets.length === 0 ? (
        <div className="rounded-xl border border-dashed border-border p-8">
          <EmptyState
            icon={<BarChart2 className="h-8 w-8 text-muted-foreground" />}
            title={t("noActiveAssetsForQuotes")}
            description={t("noActiveAssetsForQuotesDesc")}
          />
          <div className="mt-4 flex justify-center">
            <Link
              to="/portfolio"
              className="inline-flex items-center gap-1.5 rounded-lg border bg-card px-3 py-1.5 text-xs font-medium hover:bg-accent"
            >
              <Briefcase className="h-3.5 w-3.5" />
              <span>{t("goToPortfolio")}</span>
            </Link>
          </div>
        </div>
      ) : (
        <div className="space-y-6">
          {/* Active Assets Grid */}
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
            {assets.map((asset) => (
              <AssetQuoteCard
                key={asset.id}
                asset={asset}
                isSelected={selectedAsset?.id === asset.id}
                onSelect={() => setSelectedAssetId(asset.id)}
              />
            ))}
          </div>

          {/* Detailed Quote History Table */}
          {selectedAsset && <AssetQuoteHistory asset={selectedAsset} />}
        </div>
      )}
    </div>
  );
}
