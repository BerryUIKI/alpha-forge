/**
 * Option Chain List Component
 *
 * Displays a list of option chains for a workspace.
 *
 * @module features/options/components/OptionChainList
 */

import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { ErrorState } from "@/components/common/ErrorState";
import { EmptyState } from "@/components/common/EmptyState";
import { useOptionChains } from "@/hooks/useOptions";
import { useLocale } from "@/lib/i18n/useLocale";

interface OptionChainListProps {
  /** Workspace ID to fetch chains for */
  workspaceId: string;
  /** Callback when a chain is selected */
  onSelectChain?: (chainId: string) => void;
  /** Chain currently shown in the contract table */
  selectedChainId?: string | null;
}

export function OptionChainList({
  workspaceId,
  onSelectChain,
  selectedChainId,
}: OptionChainListProps) {
  const { t } = useLocale();
  const { data: chains, isLoading, error, refetch } = useOptionChains(workspaceId);

  if (isLoading) {
    return (
      <div className="p-4">
        <h3 className="mb-4 text-lg font-semibold">{t("toolOptionChain")}</h3>
        <div className="flex justify-center">
          <LoadingSpinner />
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-4">
        <h3 className="mb-4 text-lg font-semibold">{t("toolOptionChain")}</h3>
        <ErrorState
          message={t("failedToLoadOptionChains")}
          onRetry={() => void refetch()}
        />
      </div>
    );
  }

  if (!chains || chains.length === 0) {
    return (
      <div className="p-4">
        <h3 className="mb-4 text-lg font-semibold">{t("toolOptionChain")}</h3>
        <EmptyState
          title={t("noOptionChains")}
          description={t("noOptionChainsDescription")}
        />
      </div>
    );
  }

  return (
    <div className="p-4">
      <h3 className="mb-4 text-lg font-semibold">{t("toolOptionChain")}</h3>
      <div className="space-y-2">
        {chains.map((chain) => (
          <button
            key={chain.id}
            type="button"
            aria-pressed={selectedChainId === chain.id}
            aria-label={`${t("selectOptionChain")} ${chain.symbol}`}
            className={`block w-full rounded-lg border p-3 text-left transition-colors hover:bg-muted/50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring ${
              selectedChainId === chain.id
                ? "border-primary bg-primary/5 ring-1 ring-primary"
                : "border-border"
            }`}
            onClick={() => onSelectChain?.(chain.id)}
          >
            <div className="flex items-center justify-between">
              <div className="font-semibold">{chain.symbol}</div>
              <div className="text-xs text-muted-foreground">
                {chain.dataSource}
              </div>
            </div>
            <div className="flex items-center gap-4 mt-1 text-sm text-muted-foreground">
              <span>Underlying: ${chain.underlyingPrice.toFixed(2)}</span>
              <span>•</span>
              <span>
                As of: {new Date(chain.asOf).toLocaleString()}
              </span>
            </div>
          </button>
        ))}
      </div>
    </div>
  );
}
