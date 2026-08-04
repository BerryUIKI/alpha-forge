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

interface OptionChainListProps {
  /** Workspace ID to fetch chains for */
  workspaceId: string;
  /** Callback when a chain is selected */
  onSelectChain?: (chainId: string) => void;
}

export function OptionChainList({ workspaceId, onSelectChain }: OptionChainListProps) {
  const { data: chains, isLoading, error } = useOptionChains(workspaceId);

  if (isLoading) {
    return (
      <div className="p-4">
        <h3 className="text-lg font-semibold mb-4">Option Chains</h3>
        <div className="flex justify-center">
          <LoadingSpinner />
        </div>
      </div>
    );
  }

  if (error) {
    return <ErrorState message="Failed to load option chains" />;
  }

  if (!chains || chains.length === 0) {
    return (
      <div className="p-4">
        <h3 className="text-lg font-semibold mb-4">Option Chains</h3>
        <EmptyState
          title="No option chains"
          description="Fetch a chain for a symbol to get started"
        />
      </div>
    );
  }

  return (
    <div className="p-4">
      <h3 className="text-lg font-semibold mb-4">Option Chains</h3>
      <div className="space-y-2">
        {chains.map((chain) => (
          <div
            key={chain.id}
            className="p-3 border rounded-lg hover:bg-muted/50 cursor-pointer transition-colors"
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
          </div>
        ))}
      </div>
    </div>
  );
}