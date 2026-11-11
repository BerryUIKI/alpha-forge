import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { FileText, ExternalLink, Loader2, RefreshCw } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { formatMessage } from "@/lib/i18n/locale";
import { desktopApi } from "@/lib/desktop-api";
import { useListActiveAssets } from "@/features/portfolio/hooks/useFinancialData";
import { useTheses } from "@/features/thesis/hooks/useTheses";
import { useActiveWorkspaceId } from "@/features/workspace/hooks/useActiveWorkspace.context";
import type { SecFiling } from "@/lib/desktop-api/research";
import type { InvestmentThesis } from "@/lib/desktop-api/thesis";
import type { Asset } from "@/types/financial";

interface FilingDisplayItem {
  id: string;
  ticker: string;
  formType: string;
  filingDate: string;
  companyName?: string;
  accessionNumber?: string;
  summary: string;
  /** Only populated from a persisted thesis ↔ asset link. */
  linkedThesis: string | null;
  filingUrl?: string;
}

function mapSecFilingToDisplay(
  filing: SecFiling,
  theses: InvestmentThesis[],
  assetMap: Map<string, Asset>,
): FilingDisplayItem {
  const matchingAsset = Array.from(assetMap.values()).find(
    (asset) =>
      asset.display_code?.toUpperCase() === filing.ticker.toUpperCase() ||
      asset.instrument_symbol?.toUpperCase() === filing.ticker.toUpperCase(),
  );

  // Thesis linkage comes from canonical records only; ticker/title heuristics
  // and row-order sentiment are never presented as analysis.
  const matchedThesis = matchingAsset
    ? theses.find((thesis) => thesis.portfolioAssetId === matchingAsset.id)
    : undefined;

  return {
    id: filing.id,
    ticker: filing.ticker,
    formType: `SEC Form ${filing.formType}`,
    filingDate: filing.filingDate,
    companyName: filing.companyName,
    accessionNumber: filing.accessionNumber,
    summary:
      filing.primaryDocDescription ||
      filing.summary ||
      `${filing.formType} submission for period ending ${filing.reportDate ?? filing.filingDate}.`,
    linkedThesis: matchedThesis?.title ?? null,
    filingUrl: filing.filingUrl,
  };
}

export function SecFilingFeed() {
  const { t } = useLocale();
  const workspaceId = useActiveWorkspaceId();
  const { data: assets = [] } = useListActiveAssets();
  const { data: theses = [] } = useTheses(workspaceId);

  // Equity tickers come from the user's own assets; there is no demo fallback.
  const equityAssets = assets.filter((asset) => asset.kind === "investment");
  const availableTickers = Array.from(
    new Set(
      equityAssets
        .map((asset) => asset.display_code || asset.instrument_symbol)
        .filter(
          (symbol): symbol is string =>
            typeof symbol === "string" && symbol.trim().length > 0 && symbol.length <= 5,
        ),
    ),
  );

  const [selectedTicker, setSelectedTicker] = useState<string | null>(null);
  const activeTicker =
    selectedTicker && availableTickers.includes(selectedTicker)
      ? selectedTicker
      : (availableTickers[0] ?? "");

  const {
    data: filings,
    isLoading: loadingFilings,
    isFetching: fetchingFilings,
    dataUpdatedAt,
    isError: filingsError,
    refetch: refetchFilings,
  } = useQuery({
    queryKey: ["sec-filings", activeTicker],
    queryFn: () => desktopApi.research.fetchSecCompanyFilings(activeTicker, 3),
    enabled: Boolean(activeTicker),
    staleTime: 1000 * 60 * 15,
    retry: false,
  });

  const assetMap = new Map<string, Asset>(assets.map((asset) => [asset.id, asset]));
  const items: FilingDisplayItem[] = (filings ?? []).map((filing) =>
    mapSecFilingToDisplay(filing, theses, assetMap),
  );

  return (
    <div className="rounded-xl border border-border bg-card p-5 shadow-sm">
      <div className="flex flex-wrap items-center justify-between gap-3 mb-4">
        <div className="flex items-center gap-3">
          <div className="flex items-center gap-2">
            <FileText className="h-4 w-4 text-primary" aria-hidden="true" />
            <h3 className="text-sm font-semibold tracking-tight text-foreground">
              {t("secFilingFeed")}
            </h3>
          </div>

          {/* Ticker selector, driven by the user's active assets */}
          {availableTickers.length > 1 && (
            <div className="flex items-center gap-1 rounded-lg border border-border bg-muted/50 p-0.5">
              {availableTickers.slice(0, 4).map((symbol) => (
                <button
                  key={symbol}
                  type="button"
                  onClick={() => setSelectedTicker(symbol)}
                  aria-pressed={activeTicker === symbol}
                  className={
                    activeTicker === symbol
                      ? "rounded border border-primary/30 bg-primary/10 px-2 py-0.5 font-mono text-[10px] font-bold text-primary"
                      : "rounded px-2 py-0.5 font-mono text-[10px] font-bold text-muted-foreground transition-colors hover:text-foreground"
                  }
                >
                  {symbol}
                </button>
              ))}
            </div>
          )}
        </div>

        <div className="flex items-center gap-2">
          {loadingFilings && activeTicker && (
            <span className="flex items-center gap-1 text-[10px] text-muted-foreground">
              <Loader2 className="h-3 w-3 animate-spin text-primary" aria-hidden="true" />
              {t("secLoading")}
            </span>
          )}
          {!loadingFilings && !filingsError && activeTicker && (
            <span className="rounded border border-primary/20 bg-primary/10 px-2 py-0.5 text-[10px] font-medium text-primary">
              {t("secLiveEdgar")}
            </span>
          )}
          {!loadingFilings && !filingsError && activeTicker && dataUpdatedAt > 0 && (
            <span className="text-[10px] text-muted-foreground">
              {formatMessage(t("secRetrievedAt"), {
                time: new Date(dataUpdatedAt).toLocaleTimeString(),
              })}
            </span>
          )}
          {fetchingFilings && !loadingFilings && (
            <span className="text-[10px] text-muted-foreground" role="status">
              {t("refreshing")}
            </span>
          )}
          {filingsError && (
            <span className="rounded border border-destructive/30 bg-destructive/10 px-2 py-0.5 text-[10px] font-medium text-destructive">
              {t("secUnavailable")}
            </span>
          )}
        </div>
      </div>

      {/* No assets to follow */}
      {availableTickers.length === 0 ? (
        <p
          className="rounded-lg border border-dashed border-border p-4 text-xs text-muted-foreground"
          role="status"
        >
          {t("secNoAssets")}
        </p>
      ) : filingsError ? (
        <div
          className="flex flex-col items-start gap-2 rounded-lg border border-destructive/30 bg-destructive/5 p-4 text-xs text-destructive"
          role="alert"
        >
          <p className="font-medium">{t("secErrorTitle")}</p>
          <p className="text-destructive/80">{t("secErrorDescription")}</p>
          <button
            type="button"
            onClick={() => void refetchFilings()}
            className="inline-flex items-center gap-1 rounded border border-destructive/30 px-2 py-1 font-medium hover:bg-destructive/10"
          >
            <RefreshCw className="h-3 w-3" aria-hidden="true" />
            {t("retry")}
          </button>
        </div>
      ) : loadingFilings ? (
        <div className="space-y-3" aria-busy="true">
          <div className="h-20 animate-pulse rounded-lg bg-muted" />
          <div className="h-20 animate-pulse rounded-lg bg-muted" />
        </div>
      ) : items.length === 0 ? (
        <p
          className="rounded-lg border border-dashed border-border p-4 text-xs text-muted-foreground"
          role="status"
        >
          {formatMessage(t("secNoFilings"), { ticker: activeTicker })}
        </p>
      ) : (
        <div className="space-y-3">
          {items.map((item) => (
            <article
              key={item.id}
              className="rounded-lg border border-border bg-muted/30 p-3 text-xs space-y-2"
            >
              <div className="flex items-center justify-between gap-2">
                <span className="flex items-center gap-1.5 font-semibold text-foreground">
                  <span className="font-mono font-bold">{item.ticker}</span>
                  <span className="text-muted-foreground">·</span>
                  <span>{item.formType}</span>
                </span>
                <time
                  dateTime={item.filingDate}
                  className="text-[10px] font-mono text-muted-foreground"
                >
                  {item.filingDate}
                </time>
              </div>

              <p className="text-muted-foreground text-[11px] leading-relaxed">{item.summary}</p>

              <div className="flex flex-wrap items-center justify-between gap-2 border-t border-border pt-2 text-[10px]">
                <div className="flex flex-wrap items-center gap-2 text-muted-foreground">
                  <span>
                    {item.linkedThesis
                      ? formatMessage(t("linkedThesis"), { title: item.linkedThesis })
                      : t("secAnalysisUnavailable")}
                  </span>
                  <span className="text-muted-foreground/60">·</span>
                  <span>{t("secSource")}</span>
                  {item.accessionNumber && (
                    <>
                      <span className="text-muted-foreground/60">·</span>
                      <span className="font-mono">{item.accessionNumber}</span>
                    </>
                  )}
                  {item.filingUrl && (
                    <a
                      href={item.filingUrl}
                      target="_blank"
                      rel="noreferrer"
                      className="inline-flex items-center gap-0.5 text-primary hover:underline"
                    >
                      <span>{t("secViewDocument")}</span>
                      <ExternalLink className="h-2.5 w-2.5" aria-hidden="true" />
                    </a>
                  )}
                </div>
              </div>
            </article>
          ))}
        </div>
      )}
    </div>
  );
}
