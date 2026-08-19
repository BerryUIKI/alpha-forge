/**
 * Options Page
 *
 * Options analysis page using M9 components.
 * Displays Greeks Calculator, Option Chains, and Strategy Builder.
 * Workspace-scoped analysis tool: the active workspace comes from the global
 * active-workspace context (ADR-0008); the per-page workspace selector is gone.
 *
 * @module pages/options/OptionsPage
 */

import { useEffect, useState } from "react";
import { useWorkspaces } from "@/features/workspace/hooks/useWorkspaces";
import { useActiveWorkspaceId } from "@/features/workspace/hooks/useActiveWorkspace";
import { useLocale } from "@/lib/i18n/useLocale";
import {
  GreeksCalculator,
  OptionChainList,
  OptionContractTable,
  OptionStrategyPanel,
} from "@/features/options";
import { EmptyState } from "@/components/common/EmptyState";
import { ErrorState } from "@/components/common/ErrorState";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { useFetchOptionChain } from "@/hooks/useOptions";
import type { OptionContract } from "@/types/option";

const SYMBOL_PATTERN = /^[A-Z][A-Z0-9.-]{0,9}$/;

function normalizeSymbol(value: string): string {
  return value.trim().toUpperCase();
}

export function OptionsPage() {
  const { locale, t } = useLocale();
  // Loading/error states come from the workspace list query; the active
  // workspace itself comes from the global context (ADR-0008).
  const { data: workspaces, isLoading, error, refetch } = useWorkspaces();
  const workspaceId = useActiveWorkspaceId();
  const [selectedChainId, setSelectedChainId] = useState<string | null>(null);
  const [selectedContracts, setSelectedContracts] = useState<OptionContract[]>([]);
  const [symbol, setSymbol] = useState("");
  const [fetchError, setFetchError] = useState<string | null>(null);
  const fetchMutation = useFetchOptionChain(locale);

  // Reset the analysis state when the active workspace changes.
  useEffect(() => {
    setSelectedChainId(null);
    setSelectedContracts([]);
    setSymbol("");
    setFetchError(null);
  }, [workspaceId]);

  const submitFetch = () => {
    const normalizedSymbol = normalizeSymbol(symbol);
    if (!workspaceId) return;
    if (!SYMBOL_PATTERN.test(normalizedSymbol)) {
      setFetchError(t("invalidOptionSymbol"));
      return;
    }

    setFetchError(null);
    fetchMutation.mutate(
      { workspaceId, symbol: normalizedSymbol, provider: "demo" },
      {
        onSuccess: (chain) => {
          setSelectedChainId(chain.id);
          setSelectedContracts([]);
          setSymbol("");
        },
        onError: () => setFetchError(t("optionChainFetchFailedDescription")),
      },
    );
  };

  if (isLoading) {
    return <LoadingSpinner className="p-8" ariaLabel={t("loading")} />;
  }
  if (error) {
    return (
      <ErrorState
        message={t("failedToLoadWorkspaces")}
        retryLabel={t("retry")}
        onRetry={() => refetch()}
      />
    );
  }
  if (!workspaces?.length) {
    return (
      <div className="flex h-full flex-col items-center justify-center p-6">
        <EmptyState
          title={t("createWorkspaceFirst")}
          description={t("createWorkspaceFirstDescription")}
        />
      </div>
    );
  }

  return (
    <div className="space-y-6 p-6">
      {/* Header */}
      <div>
        <h1 className="text-2xl font-bold">{t("optionsTitle")}</h1>
        <p className="text-sm text-muted-foreground">{t("optionsDescription")}</p>
      </div>

      {/* Calculation tool */}
      <div className="max-w-3xl">
        <GreeksCalculator />
      </div>

      {/* Option Chains */}
      <section
        className="space-y-4 rounded-lg border border-border p-4"
        aria-labelledby="option-chain-heading"
      >
        <div>
          <h2 id="option-chain-heading" className="text-lg font-semibold">
            {t("demoOptionChainTitle")}
          </h2>
          <p className="text-sm text-muted-foreground">{t("demoOptionChainDescription")}</p>
        </div>
        <form
          className="flex flex-col gap-2 sm:flex-row"
          onSubmit={(event) => {
            event.preventDefault();
            submitFetch();
          }}
        >
          <label className="sr-only" htmlFor="option-symbol">
            {t("symbolLabel")}
          </label>
          <input
            id="option-symbol"
            className="min-w-0 flex-1 rounded-lg border border-border bg-background p-2"
            value={symbol}
            onChange={(event) => {
              setSymbol(event.target.value);
              if (fetchError) setFetchError(null);
            }}
            placeholder={t("optionSymbolPlaceholder")}
            autoComplete="off"
          />
          <button
            type="submit"
            disabled={fetchMutation.isPending}
            className="rounded-lg bg-primary px-4 py-2 text-primary-foreground disabled:opacity-50"
          >
            {fetchMutation.isPending ? t("fetchingDemoChain") : t("fetchDemoChain")}
          </button>
        </form>
        {fetchError && (
          <ErrorState
            title={t("optionChainFetchFailed")}
            message={fetchError}
            onRetry={SYMBOL_PATTERN.test(normalizeSymbol(symbol)) ? submitFetch : undefined}
          />
        )}
        <OptionChainList
          workspaceId={workspaceId}
          selectedChainId={selectedChainId}
          onSelectChain={(chainId) => {
            setSelectedChainId(chainId);
            setSelectedContracts([]);
          }}
        />
      </section>

      {selectedChainId ? (
        <section aria-labelledby="option-contract-heading" className="space-y-3">
          <h2 id="option-contract-heading" className="text-lg font-semibold">
            {t("optionContracts")}
          </h2>
          <OptionContractTable
            chainId={selectedChainId}
            selectedContractIds={new Set(selectedContracts.map((contract) => contract.id))}
            onToggleContract={(contract) =>
              setSelectedContracts((current) =>
                current.some((selected) => selected.id === contract.id)
                  ? current.filter((selected) => selected.id !== contract.id)
                  : [...current, contract],
              )
            }
          />
        </section>
      ) : (
        <EmptyState
          title={t("selectOptionChain")}
          description={t("selectOptionChainDescription")}
        />
      )}

      <section aria-labelledby="option-strategy-heading" className="space-y-3">
        <h2 id="option-strategy-heading" className="sr-only">
          {t("optionStrategies")}
        </h2>
        <OptionStrategyPanel
          key={`${workspaceId}:${selectedChainId ?? "none"}`}
          workspaceId={workspaceId}
          selectedContracts={selectedContracts}
          onStrategyCreated={() => setSelectedContracts([])}
        />
      </section>
    </div>
  );
}
