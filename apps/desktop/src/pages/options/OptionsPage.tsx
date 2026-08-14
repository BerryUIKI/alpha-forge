/**
 * Options Page
 *
 * Options analysis page using M9 components.
 * Displays Greeks Calculator, Option Chains, and Strategy Builder.
 *
 * @module pages/options/OptionsPage
 */

import { useState } from "react";
import { useSearchParams } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import { useLocale } from "@/lib/i18n/useLocale";
import {
  GreeksCalculator,
  OptionChainList,
  OptionContractTable,
  StrategyBuilder,
} from "@/features/options";
import { EmptyState } from "@/components/common/EmptyState";
import { ErrorState } from "@/components/common/ErrorState";
import { useFetchOptionChain } from "@/hooks/useOptions";

const SYMBOL_PATTERN = /^[A-Z][A-Z0-9.-]{0,9}$/;

function normalizeSymbol(value: string): string {
  return value.trim().toUpperCase();
}

export function OptionsPage() {
  const { locale, t } = useLocale();
  const [searchParams, setSearchParams] = useSearchParams();
  const workspaceIdFromUrl = searchParams.get("workspace") || "";
  const [selectedWorkspaceId, setSelectedWorkspaceId] = useState(workspaceIdFromUrl);
  const [selectedChainId, setSelectedChainId] = useState<string | null>(null);
  const [symbol, setSymbol] = useState("");
  const [fetchError, setFetchError] = useState<string | null>(null);
  const fetchMutation = useFetchOptionChain(locale);

  // Fetch workspaces for selection
  const { data: workspaces } = useQuery({
    queryKey: ["workspaces"],
    queryFn: desktopApi.workspace.listWorkspaces,
  });

  // Handle workspace selection
  const handleWorkspaceChange = (id: string) => {
    setSelectedWorkspaceId(id);
    setSelectedChainId(null);
    setSymbol("");
    setFetchError(null);
    setSearchParams({ workspace: id });
  };

  const submitFetch = () => {
    const normalizedSymbol = normalizeSymbol(symbol);
    if (!SYMBOL_PATTERN.test(normalizedSymbol)) {
      setFetchError(t("invalidOptionSymbol"));
      return;
    }

    setFetchError(null);
    fetchMutation.mutate(
      { workspaceId: selectedWorkspaceId, symbol: normalizedSymbol, provider: "demo" },
      {
        onSuccess: (chain) => {
          setSelectedChainId(chain.id);
          setSymbol("");
        },
        onError: () => setFetchError(t("optionChainFetchFailedDescription")),
      },
    );
  };

  // No workspace selected state
  if (!selectedWorkspaceId) {
    return (
      <div className="flex h-full flex-col items-center justify-center p-6">
        <EmptyState
          title={t("selectWorkspace")}
          description={t("selectWorkspaceDescription")}
        />
        {workspaces && workspaces.length > 0 && (
          <div className="mt-4 w-full max-w-xs">
            <select
              className="w-full rounded-lg border border-border bg-background p-2"
              value={selectedWorkspaceId}
              onChange={(e) => handleWorkspaceChange(e.target.value)}
            >
              <option value="">{t("selectWorkspace")}</option>
              {workspaces.map((workspace) => (
                <option key={workspace.id} value={workspace.id}>
                  {workspace.name}
                </option>
              ))}
            </select>
          </div>
        )}
      </div>
    );
  }

  return (
    <div className="space-y-6 p-6">
      {/* Header */}
      <div>
        <h1 className="text-2xl font-bold">{t("optionsTitle")}</h1>
        <p className="text-sm text-muted-foreground">
          {t("optionsDescription")}
        </p>
      </div>

      {/* Workspace Selector */}
      <div className="max-w-xs">
        <label className="block text-sm font-medium">
          {t("workspace")}
        </label>
        <select
          className="mt-1 w-full rounded-lg border border-border bg-background p-2"
          value={selectedWorkspaceId}
          onChange={(e) => handleWorkspaceChange(e.target.value)}
        >
          <option value="">{t("selectWorkspace")}</option>
          {workspaces?.map((workspace) => (
            <option key={workspace.id} value={workspace.id}>
              {workspace.name}
            </option>
          ))}
        </select>
      </div>

      {/* Tools Grid */}
      <div className="grid gap-6 lg:grid-cols-2">
        {/* Greeks Calculator */}
        <GreeksCalculator />

        {/* Strategy Builder */}
        <StrategyBuilder />
      </div>

      {/* Option Chains */}
      <section className="space-y-4 rounded-lg border border-border p-4" aria-labelledby="option-chain-heading">
        <div>
          <h2 id="option-chain-heading" className="text-lg font-semibold">{t("demoOptionChainTitle")}</h2>
          <p className="text-sm text-muted-foreground">
            {t("demoOptionChainDescription")}
          </p>
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
          workspaceId={selectedWorkspaceId}
          selectedChainId={selectedChainId}
          onSelectChain={setSelectedChainId}
        />
      </section>

      {selectedChainId ? (
        <section aria-labelledby="option-contract-heading" className="space-y-3">
          <h2 id="option-contract-heading" className="text-lg font-semibold">{t("optionContracts")}</h2>
          <OptionContractTable chainId={selectedChainId} />
        </section>
      ) : (
        <EmptyState
          title={t("selectOptionChain")}
          description={t("selectOptionChainDescription")}
        />
      )}
    </div>
  );
}
