/**
 * Option Strategy Panel Component
 *
 * Manages persisted option strategies: create, view, and delete.
 *
 * @module features/options/components/OptionStrategyPanel
 */

import { useState } from "react";
import { Plus, Trash2, ChevronDown, ChevronUp } from "lucide-react";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { ErrorState } from "@/components/common/ErrorState";
import { EmptyState } from "@/components/common/EmptyState";
import {
  useOptionStrategies,
  useCreateOptionStrategy,
  useDeleteOptionStrategy,
} from "@/hooks/useOptions";
import { useLocale } from "@/lib/i18n/useLocale";
import type {
  CreateStrategyParams,
  OptionContract,
  OptionStrategy,
  PositionType,
  StrategyType,
} from "@/types/option";

interface OptionStrategyPanelProps {
  /** Workspace ID to manage strategies for */
  workspaceId: string;
  /** Contracts selected from the active persisted chain */
  selectedContracts: OptionContract[];
  /** Clears the controlled selection after a successful create */
  onStrategyCreated?: () => void;
  /** Callback when a strategy is selected */
  onSelectStrategy?: (strategy: OptionStrategy) => void;
}

type SortField = "name" | "strategyType" | "maxProfit" | "maxLoss";
type SortOrder = "asc" | "desc";

export function OptionStrategyPanel({
  workspaceId,
  selectedContracts,
  onStrategyCreated,
  onSelectStrategy,
}: OptionStrategyPanelProps) {
  const { t, locale } = useLocale();
  const [sortField, setSortField] = useState<SortField>("name");
  const [sortOrder, setSortOrder] = useState<SortOrder>("asc");
  const [formContracts, setFormContracts] = useState<OptionContract[] | null>(null);

  const { data: strategies, isLoading, error, refetch } = useOptionStrategies(workspaceId);
  const createMutation = useCreateOptionStrategy(locale);
  const deleteMutation = useDeleteOptionStrategy(locale);

  const handleDelete = (id: string) => {
    if (window.confirm(t("confirmDeleteOptionStrategy"))) {
      deleteMutation.mutate(id);
    }
  };

  if (isLoading) {
    return (
      <div className="flex justify-center p-8">
        <LoadingSpinner />
      </div>
    );
  }

  if (error) {
    return (
      <ErrorState message={t("failedToLoadOptionStrategies")} onRetry={() => void refetch()} />
    );
  }

  // Sort strategies
  const sortedStrategies = strategies
    ? [...strategies].sort((a, b) => {
        const aVal = a[sortField] ?? 0;
        const bVal = b[sortField] ?? 0;
        if (typeof aVal === "string" && typeof bVal === "string") {
          return sortOrder === "asc" ? aVal.localeCompare(bVal) : bVal.localeCompare(aVal);
        }
        return sortOrder === "asc"
          ? (aVal as number) - (bVal as number)
          : (bVal as number) - (aVal as number);
      })
    : [];

  const toggleSort = (field: SortField) => {
    if (sortField === field) {
      setSortOrder(sortOrder === "asc" ? "desc" : "asc");
    } else {
      setSortField(field);
      setSortOrder("asc");
    }
  };

  const formatCurrency = (val: number | null | undefined) =>
    val != null ? `$${val.toFixed(2)}` : "-";

  const formatPercent = (val: number | null | undefined) =>
    val != null ? `${(val * 100).toFixed(2)}%` : "-";

  return (
    <div className="space-y-4">
      {/* Header */}
      <div className="flex items-center justify-between">
        <h3 className="text-lg font-semibold">{t("optionStrategies")}</h3>
        <button
          type="button"
          onClick={() => {
            createMutation.reset();
            setFormContracts(selectedContracts);
          }}
          disabled={selectedContracts.length === 0}
          className="flex items-center gap-2 px-3 py-1.5 bg-primary text-primary-foreground rounded-md text-sm hover:bg-primary/90 transition-colors disabled:opacity-50"
        >
          <Plus className="h-4 w-4" />
          {t("createOptionStrategy")}
        </button>
      </div>

      {selectedContracts.length === 0 && !formContracts && (
        <p className="text-sm text-muted-foreground">{t("selectContractsForStrategy")}</p>
      )}

      {/* Controlled create form. Persisted legs are immutable in this slice. */}
      {formContracts && (
        <StrategyForm
          workspaceId={workspaceId}
          selectedContracts={formContracts}
          onSave={(data) => {
            createMutation.mutate(data, {
              onSuccess: () => {
                setFormContracts(null);
                onStrategyCreated?.();
              },
            });
          }}
          onCancel={() => {
            createMutation.reset();
            setFormContracts(null);
          }}
          isLoading={createMutation.isPending}
        />
      )}

      {createMutation.isError && <ErrorState message={t("failedToCreateOptionStrategy")} />}
      {deleteMutation.isError && <ErrorState message={t("failedToDeleteOptionStrategy")} />}

      {/* Strategy List */}
      {!strategies || strategies.length === 0 ? (
        <EmptyState
          title={t("noOptionStrategies")}
          description={t("noOptionStrategiesDescription")}
        />
      ) : (
        <div className="overflow-hidden rounded-lg border border-border">
          <table className="w-full text-sm">
            <thead className="bg-muted/50 border-b border-border">
              <tr>
                <th className="px-3 py-2 text-left font-medium">
                  <button
                    onClick={() => toggleSort("name")}
                    className="flex items-center gap-1 hover:text-foreground"
                  >
                    {t("optionStrategyName")}
                    {sortField === "name" &&
                      (sortOrder === "asc" ? (
                        <ChevronUp className="h-3 w-3" />
                      ) : (
                        <ChevronDown className="h-3 w-3" />
                      ))}
                  </button>
                </th>
                <th className="px-3 py-2 text-left font-medium">Type</th>
                <th className="px-3 py-2 text-right font-medium">Legs</th>
                <th className="px-3 py-2 text-right font-medium">
                  <button
                    onClick={() => toggleSort("maxProfit")}
                    className="flex items-center gap-1 hover:text-foreground ml-auto"
                  >
                    Max Profit
                    {sortField === "maxProfit" &&
                      (sortOrder === "asc" ? (
                        <ChevronUp className="h-3 w-3" />
                      ) : (
                        <ChevronDown className="h-3 w-3" />
                      ))}
                  </button>
                </th>
                <th className="px-3 py-2 text-right font-medium">
                  <button
                    onClick={() => toggleSort("maxLoss")}
                    className="flex items-center gap-1 hover:text-foreground ml-auto"
                  >
                    Max Loss
                    {sortField === "maxLoss" &&
                      (sortOrder === "asc" ? (
                        <ChevronUp className="h-3 w-3" />
                      ) : (
                        <ChevronDown className="h-3 w-3" />
                      ))}
                  </button>
                </th>
                <th className="px-3 py-2 text-right font-medium">Δ</th>
                <th className="px-3 py-2 text-right font-medium">γ</th>
                <th className="px-3 py-2 text-right font-medium">θ</th>
                <th className="px-3 py-2 text-right font-medium">ν</th>
                <th className="px-3 py-2"></th>
              </tr>
            </thead>
            <tbody className="divide-y divide-border">
              {sortedStrategies.map((strategy) => (
                <tr
                  key={strategy.id}
                  className="hover:bg-muted/30 cursor-pointer"
                  onClick={() => onSelectStrategy?.(strategy)}
                >
                  <td className="px-3 py-2 font-medium">{strategy.name}</td>
                  <td className="px-3 py-2 text-muted-foreground">{strategy.strategyType}</td>
                  <td className="px-3 py-2 text-right font-mono">{strategy.legs?.length || 0}</td>
                  <td className="px-3 py-2 text-right font-mono text-green-600">
                    {formatCurrency(strategy.maxProfit)}
                  </td>
                  <td className="px-3 py-2 text-right font-mono text-red-600">
                    {formatCurrency(strategy.maxLoss)}
                  </td>
                  <td className="px-3 py-2 text-right font-mono">
                    {formatPercent(strategy.totalDelta)}
                  </td>
                  <td className="px-3 py-2 text-right font-mono">
                    {formatPercent(strategy.totalGamma)}
                  </td>
                  <td className="px-3 py-2 text-right font-mono">
                    {formatPercent(strategy.totalTheta)}
                  </td>
                  <td className="px-3 py-2 text-right font-mono">
                    {formatPercent(strategy.totalVega)}
                  </td>
                  <td className="px-3 py-2">
                    <div className="flex items-center gap-1">
                      <button
                        type="button"
                        aria-label={`${t("deleteOptionStrategy")}: ${strategy.name}`}
                        onClick={(e) => {
                          e.stopPropagation();
                          handleDelete(strategy.id);
                        }}
                        disabled={deleteMutation.isPending}
                        className="p-1 text-muted-foreground hover:text-destructive transition-colors"
                      >
                        <Trash2 className="h-4 w-4" />
                      </button>
                    </div>
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

interface StrategyFormProps {
  workspaceId: string;
  selectedContracts: OptionContract[];
  onSave: (data: CreateStrategyParams) => void;
  onCancel: () => void;
  isLoading: boolean;
}

interface LegDraft {
  contractId: string;
  quantity: number;
  positionType: PositionType;
}

function StrategyForm({
  workspaceId,
  selectedContracts,
  onSave,
  onCancel,
  isLoading,
}: StrategyFormProps) {
  const { t } = useLocale();
  const [name, setName] = useState("");
  const [strategyType, setStrategyType] = useState<StrategyType>("custom");
  const [legs, setLegs] = useState<LegDraft[]>(() =>
    selectedContracts.map((contract) => ({
      contractId: contract.id,
      quantity: 1,
      positionType: "long",
    })),
  );

  const updateLeg = (contractId: string, update: Partial<LegDraft>) => {
    setLegs((current) =>
      current.map((leg) => (leg.contractId === contractId ? { ...leg, ...update } : leg)),
    );
  };

  return (
    <form
      onSubmit={(event) => {
        event.preventDefault();
        onSave({ workspaceId, name: name.trim(), strategyType, legs });
      }}
      className="space-y-4 rounded-lg border bg-muted/30 p-4"
    >
      <h4 className="font-medium">{t("createOptionStrategy")}</h4>
      <div>
        <label className="mb-1 block text-sm font-medium" htmlFor="option-strategy-name">
          {t("optionStrategyName")}
        </label>
        <input
          id="option-strategy-name"
          type="text"
          value={name}
          onChange={(event) => setName(event.target.value)}
          className="w-full rounded-md border px-3 py-2"
          placeholder={t("optionStrategyNamePlaceholder")}
          maxLength={100}
          required
        />
      </div>
      <div>
        <label className="mb-1 block text-sm font-medium" htmlFor="option-strategy-type">
          {t("optionStrategyType")}
        </label>
        <select
          id="option-strategy-type"
          value={strategyType}
          onChange={(event) => setStrategyType(event.target.value as StrategyType)}
          className="w-full rounded-md border px-3 py-2"
        >
          <option value="custom">{t("customOptionStrategy")}</option>
          <option value="bull_call_spread">{t("bullCallSpread")}</option>
          <option value="bear_put_spread">{t("bearPutSpread")}</option>
          <option value="iron_condor">{t("ironCondor")}</option>
          <option value="straddle">{t("straddle")}</option>
          <option value="strangle">{t("strangle")}</option>
        </select>
      </div>
      <fieldset className="space-y-3">
        <legend className="text-sm font-medium">{t("selectedStrategyContracts")}</legend>
        {selectedContracts.map((contract) => {
          const leg = legs.find((candidate) => candidate.contractId === contract.id);
          if (!leg) return null;
          return (
            <div key={contract.id} className="grid gap-2 rounded-md border p-3 sm:grid-cols-3">
              <div className="text-sm">
                <div className="font-medium">
                  {contract.symbol} {contract.strike.toFixed(2)} {contract.optionType}
                </div>
                <div className="text-muted-foreground">
                  {new Date(contract.expiration).toLocaleDateString()}
                </div>
              </div>
              <label className="text-sm">
                <span className="mb-1 block">{t("strategyLegQuantity")}</span>
                <input
                  aria-label={`${t("strategyLegQuantity")} ${contract.strike.toFixed(2)}`}
                  type="number"
                  min={1}
                  step={1}
                  value={leg.quantity}
                  onChange={(event) =>
                    updateLeg(contract.id, {
                      quantity: Number.parseInt(event.target.value, 10) || 0,
                    })
                  }
                  className="w-full rounded-md border px-2 py-1"
                  required
                />
              </label>
              <label className="text-sm">
                <span className="mb-1 block">{t("strategyLegDirection")}</span>
                <select
                  aria-label={`${t("strategyLegDirection")} ${contract.strike.toFixed(2)}`}
                  value={leg.positionType}
                  onChange={(event) =>
                    updateLeg(contract.id, {
                      positionType: event.target.value as PositionType,
                    })
                  }
                  className="w-full rounded-md border px-2 py-1"
                >
                  <option value="long">{t("longPosition")}</option>
                  <option value="short">{t("shortPosition")}</option>
                </select>
              </label>
            </div>
          );
        })}
      </fieldset>
      <div className="flex items-center gap-2">
        <button
          type="submit"
          disabled={isLoading || !name.trim() || legs.some((leg) => leg.quantity <= 0)}
          className="rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground disabled:opacity-50"
        >
          {isLoading ? t("saving") : t("saveOptionStrategy")}
        </button>
        <button type="button" onClick={onCancel} className="rounded-md border px-4 py-2 text-sm">
          {t("cancel")}
        </button>
      </div>
    </form>
  );
}
