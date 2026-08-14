/**
 * Option Contract Table Component
 *
 * Displays option contracts in a chain with Greeks and pricing data.
 *
 * @module features/options/components/OptionContractTable
 */

import { useState } from "react";
import { ChevronDown, ChevronUp, Trash2 } from "lucide-react";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { ErrorState } from "@/components/common/ErrorState";
import { useOptionContracts, useDeleteOptionContract } from "@/hooks/useOptions";
import { useLocale } from "@/lib/i18n/useLocale";
import type { OptionType } from "@/types/option";

interface OptionContractTableProps {
  /** Chain ID to fetch contracts for */
  chainId: string;
  /** Show delete button (optional) */
  showDelete?: boolean;
  /** Highlight strike prices near this value */
  spotPrice?: number;
}

type SortField = "strike" | "bid" | "ask" | "volume" | "openInterest" | "impliedVolatility";
type SortOrder = "asc" | "desc";

export function OptionContractTable({
  chainId,
  showDelete = false,
  spotPrice,
}: OptionContractTableProps) {
  const { t } = useLocale();
  const [sortField, setSortField] = useState<SortField>("strike");
  const [sortOrder, setSortOrder] = useState<SortOrder>("asc");
  const [filter, setFilter] = useState<OptionType | "all">("all");

  const { data: contracts, isLoading, error, refetch } = useOptionContracts(chainId);
  const deleteMutation = useDeleteOptionContract("en");

  if (isLoading) {
    return (
      <div className="flex justify-center p-8">
        <LoadingSpinner />
      </div>
    );
  }

  if (error) {
    return (
      <ErrorState
        message={t("failedToLoadOptionContracts")}
        onRetry={() => void refetch()}
      />
    );
  }

  if (!contracts || contracts.length === 0) {
    return (
      <div className="p-4 text-center text-muted-foreground">
        {t("noContracts")}
      </div>
    );
  }

  // Filter by type
  const filteredContracts =
    filter === "all" ? contracts : contracts.filter((c) => c.optionType === filter);

  // Sort contracts
  const sortedContracts = [...filteredContracts].sort((a, b) => {
    const aVal = a[sortField];
    const bVal = b[sortField];
    return sortOrder === "asc" ? aVal - bVal : bVal - aVal;
  });

  const toggleSort = (field: SortField) => {
    if (sortField === field) {
      setSortOrder(sortOrder === "asc" ? "desc" : "asc");
    } else {
      setSortField(field);
      setSortOrder("asc");
    }
  };

  const handleDelete = async (id: string) => {
    if (window.confirm(t("confirmDeleteContract" as any) || "Delete this contract?")) {
      deleteMutation.mutate(id);
    }
  };

  const formatPercent = (val: number | null | undefined) =>
    val != null ? `${(val * 100).toFixed(2)}%` : "-";

  const formatNumber = (val: number | null | undefined) =>
    val != null ? val.toFixed(2) : "-";

  const formatVolume = (val: number | null | undefined) =>
    val != null ? val.toLocaleString() : "-";

  const isNearSpot = (strike: number) => {
    if (!spotPrice) return false;
    const threshold = spotPrice * 0.05; // 5% threshold
    return Math.abs(strike - spotPrice) <= threshold;
  };

  return (
    <div className="overflow-hidden rounded-lg border border-border">
      {/* Filter Tabs */}
      <div className="flex items-center gap-2 p-3 bg-muted/30 border-b">
        <button
          onClick={() => setFilter("all")}
          className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
            filter === "all"
              ? "bg-primary text-primary-foreground"
              : "hover:bg-muted"
          }`}
        >
          All
        </button>
        <button
          onClick={() => setFilter("call")}
          className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
            filter === "call"
              ? "bg-green-600 text-white"
              : "hover:bg-muted"
          }`}
        >
          Calls
        </button>
        <button
          onClick={() => setFilter("put")}
          className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
            filter === "put"
              ? "bg-red-600 text-white"
              : "hover:bg-muted"
          }`}
        >
          Puts
        </button>
      </div>

      {/* Table */}
      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead className="bg-muted/50 border-b border-border">
            <tr>
              <th className="px-3 py-2 text-left font-medium">
                <button
                  onClick={() => toggleSort("strike")}
                  className="flex items-center gap-1 hover:text-foreground"
                >
                  Strike
                  {sortField === "strike" && (
                    sortOrder === "asc" ? <ChevronUp className="h-3 w-3" /> : <ChevronDown className="h-3 w-3" />
                  )}
                </button>
              </th>
              <th className="px-3 py-2 text-right font-medium">Bid</th>
              <th className="px-3 py-2 text-right font-medium">Ask</th>
              <th className="px-3 py-2 text-right font-medium">
                <button
                  onClick={() => toggleSort("volume")}
                  className="flex items-center gap-1 hover:text-foreground ml-auto"
                >
                  Vol
                  {sortField === "volume" && (
                    sortOrder === "asc" ? <ChevronUp className="h-3 w-3" /> : <ChevronDown className="h-3 w-3" />
                  )}
                </button>
              </th>
              <th className="px-3 py-2 text-right font-medium">OI</th>
              <th className="px-3 py-2 text-right font-medium">
                <button
                  onClick={() => toggleSort("impliedVolatility")}
                  className="flex items-center gap-1 hover:text-foreground ml-auto"
                >
                  IV
                  {sortField === "impliedVolatility" && (
                    sortOrder === "asc" ? <ChevronUp className="h-3 w-3" /> : <ChevronDown className="h-3 w-3" />
                  )}
                </button>
              </th>
              {showDelete && <th className="px-3 py-2"></th>}
            </tr>
          </thead>
          <tbody className="divide-y divide-border">
            {sortedContracts.map((contract) => (
              <tr
                key={contract.id}
                className={`hover:bg-muted/30 ${
                  isNearSpot(contract.strike) ? "bg-blue-500/5" : ""
                }`}
              >
                <td className="px-3 py-2 font-mono">
                  <span
                    className={`${
                      contract.optionType === "call"
                        ? "text-green-600"
                        : "text-red-600"
                    }`}
                  >
                    {formatNumber(contract.strike)}
                  </span>
                </td>
                <td className="px-3 py-2 text-right font-mono">
                  {formatNumber(contract.bid)}
                </td>
                <td className="px-3 py-2 text-right font-mono">
                  {formatNumber(contract.ask)}
                </td>
                <td className="px-3 py-2 text-right font-mono">
                  {formatVolume(contract.volume)}
                </td>
                <td className="px-3 py-2 text-right font-mono">
                  {formatVolume(contract.openInterest)}
                </td>
                <td className="px-3 py-2 text-right font-mono">
                  {formatPercent(contract.impliedVolatility)}
                </td>
                {showDelete && (
                  <td className="px-3 py-2">
                    <button
                      onClick={() => handleDelete(contract.id)}
                      disabled={deleteMutation.isPending}
                      className="p-1 text-muted-foreground hover:text-destructive transition-colors"
                    >
                      <Trash2 className="h-4 w-4" />
                    </button>
                  </td>
                )}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
