/**
 * Option Strategy Panel Component
 *
 * Manages option strategies: create, view, update, delete.
 *
 * @module features/options/components/OptionStrategyPanel
 */

import { useState } from "react";
import { Plus, Edit2, Trash2, ChevronDown, ChevronUp } from "lucide-react";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { ErrorState } from "@/components/common/ErrorState";
import { EmptyState } from "@/components/common/EmptyState";
import {
  useOptionStrategies,
  useCreateOptionStrategy,
  useUpdateOptionStrategy,
  useDeleteOptionStrategy,
} from "@/hooks/useOptions";
import { useLocale } from "@/lib/i18n/useLocale";
import type { OptionStrategy, StrategyType } from "@/types/option";

interface OptionStrategyPanelProps {
  /** Workspace ID to manage strategies for */
  workspaceId: string;
  /** Callback when a strategy is selected */
  onSelectStrategy?: (strategy: OptionStrategy) => void;
}

type SortField = "name" | "strategyType" | "maxProfit" | "maxLoss";
type SortOrder = "asc" | "desc";

export function OptionStrategyPanel({
  workspaceId,
  onSelectStrategy,
}: OptionStrategyPanelProps) {
  const { t, locale } = useLocale();
  const [sortField, setSortField] = useState<SortField>("name");
  const [sortOrder, setSortOrder] = useState<SortOrder>("asc");
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [editingStrategy, setEditingStrategy] = useState<OptionStrategy | null>(
    null
  );

  const { data: strategies, isLoading, error } = useOptionStrategies(workspaceId);
  const createMutation = useCreateOptionStrategy(locale);
  const updateMutation = useUpdateOptionStrategy(locale);
  const deleteMutation = useDeleteOptionStrategy(locale);

  const handleDelete = async (id: string) => {
    if (window.confirm(t("confirmDeleteStrategy" as any) || "Delete this strategy?")) {
      deleteMutation.mutate(id);
    }
  };

  const handleEdit = (strategy: OptionStrategy) => {
    setEditingStrategy(strategy);
    setShowCreateForm(false);
  };

  const handleCancelEdit = () => {
    setEditingStrategy(null);
    setShowCreateForm(false);
  };

  if (isLoading) {
    return (
      <div className="flex justify-center p-8">
        <LoadingSpinner />
      </div>
    );
  }

  if (error) {
    return <ErrorState message="Failed to load option strategies" />;
  }

  // Sort strategies
  const sortedStrategies = strategies
    ? [...strategies].sort((a, b) => {
        const aVal = a[sortField] ?? 0;
        const bVal = b[sortField] ?? 0;
        if (typeof aVal === "string" && typeof bVal === "string") {
          return sortOrder === "asc"
            ? aVal.localeCompare(bVal)
            : bVal.localeCompare(aVal);
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
        <h3 className="text-lg font-semibold">
          {t("optionStrategies" as any) || "Option Strategies"}
        </h3>
        <button
          onClick={() => {
            setShowCreateForm(true);
            setEditingStrategy(null);
          }}
          className="flex items-center gap-2 px-3 py-1.5 bg-primary text-primary-foreground rounded-md text-sm hover:bg-primary/90 transition-colors"
        >
          <Plus className="h-4 w-4" />
          {t("createStrategy" as any) || "Create Strategy"}
        </button>
      </div>

      {/* Create/Edit Form */}
      {(showCreateForm || editingStrategy) && (
        <StrategyForm
          strategy={editingStrategy}
          workspaceId={workspaceId}
          onSave={(data) => {
            if (editingStrategy) {
              updateMutation.mutate({ ...data, id: editingStrategy.id });
            } else {
              createMutation.mutate(data);
            }
            handleCancelEdit();
          }}
          onCancel={handleCancelEdit}
          isLoading={createMutation.isPending || updateMutation.isPending}
        />
      )}

      {/* Strategy List */}
      {!strategies || strategies.length === 0 ? (
        <EmptyState
          title={t("noStrategies" as any) || "No strategies created"}
          description={
            t("noStrategiesDescription" as any) ||
            "Create multi-leg strategies to manage option positions"
          }
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
                    {t("name" as any) || "Name"}
                    {sortField === "name" && (
                      sortOrder === "asc" ? (
                        <ChevronUp className="h-3 w-3" />
                      ) : (
                        <ChevronDown className="h-3 w-3" />
                      )
                    )}
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
                    {sortField === "maxProfit" && (
                      sortOrder === "asc" ? (
                        <ChevronUp className="h-3 w-3" />
                      ) : (
                        <ChevronDown className="h-3 w-3" />
                      )
                    )}
                  </button>
                </th>
                <th className="px-3 py-2 text-right font-medium">
                  <button
                    onClick={() => toggleSort("maxLoss")}
                    className="flex items-center gap-1 hover:text-foreground ml-auto"
                  >
                    Max Loss
                    {sortField === "maxLoss" && (
                      sortOrder === "asc" ? (
                        <ChevronUp className="h-3 w-3" />
                      ) : (
                        <ChevronDown className="h-3 w-3" />
                      )
                    )}
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
                  <td className="px-3 py-2 text-muted-foreground">
                    {strategy.strategyType}
                  </td>
                  <td className="px-3 py-2 text-right font-mono">
                    {strategy.legs?.length || 0}
                  </td>
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
                        onClick={(e) => {
                          e.stopPropagation();
                          handleEdit(strategy);
                        }}
                        className="p-1 text-muted-foreground hover:text-foreground transition-colors"
                      >
                        <Edit2 className="h-4 w-4" />
                      </button>
                      <button
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

/**
 * Strategy Create/Edit Form
 */
interface StrategyFormProps {
  strategy: OptionStrategy | null;
  workspaceId: string;
  onSave: (data: any) => void;
  onCancel: () => void;
  isLoading: boolean;
}

function StrategyForm({
  strategy,
  workspaceId,
  onSave,
  onCancel,
  isLoading,
}: StrategyFormProps) {
  const { t } = useLocale();
  const [name, setName] = useState(strategy?.name || "");
  const [strategyType, setStrategyType] = useState<StrategyType>(
    strategy?.strategyType || "custom"
  );
  const [description, setDescription] = useState(strategy?.description || "");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSave({
      name,
      strategyType,
      description,
      workspaceId,
      legs: [], // TODO: Add leg management UI
    });
  };

  return (
    <form
      onSubmit={handleSubmit}
      className="p-4 border rounded-lg bg-muted/30 space-y-4"
    >
      <h4 className="font-medium">
        {strategy
          ? t("editStrategy" as any) || "Edit Strategy"
          : t("createStrategy" as any) || "Create Strategy"}
      </h4>

      <div>
        <label className="block text-sm font-medium mb-1">
          {t("strategyName" as any) || "Strategy Name"}
        </label>
        <input
          type="text"
          value={name}
          onChange={(e) => setName(e.target.value)}
          className="w-full px-3 py-2 border rounded-md"
          placeholder="e.g., Bull Call Spread"
          required
        />
      </div>

      <div>
        <label className="block text-sm font-medium mb-1">
          {t("strategyType" as any) || "Strategy Type"}
        </label>
        <select
          value={strategyType}
          onChange={(e) => setStrategyType(e.target.value as StrategyType)}
          className="w-full px-3 py-2 border rounded-md"
        >
          <option value="custom">Custom</option>
          <option value="bull_call_spread">Bull Call Spread</option>
          <option value="bear_put_spread">Bear Put Spread</option>
          <option value="iron_condor">Iron Condor</option>
          <option value="straddle">Straddle</option>
          <option value="strangle">Strangle</option>
        </select>
      </div>

      <div>
        <label className="block text-sm font-medium mb-1">
          {t("description" as any) || "Description"}
        </label>
        <textarea
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          className="w-full px-3 py-2 border rounded-md"
          rows={3}
          placeholder="Strategy description..."
        />
      </div>

      <div className="flex items-center gap-2">
        <button
          type="submit"
          disabled={isLoading || !name.trim()}
          className="px-4 py-2 bg-primary text-primary-foreground rounded-md text-sm hover:bg-primary/90 disabled:opacity-50"
        >
          {isLoading ? t("saving" as any) || "Saving..." : t("save" as any) || "Save"}
        </button>
        <button
          type="button"
          onClick={onCancel}
          className="px-4 py-2 border rounded-md text-sm hover:bg-muted"
        >
          {t("cancel" as any) || "Cancel"}
        </button>
      </div>
    </form>
  );
}