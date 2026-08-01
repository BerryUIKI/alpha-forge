/**
 * Option Chain Viewer - Main container component
 */

import { useState, useMemo } from 'react';
import { useQuery } from '@tanstack/react-query';
import { optionsApi } from '@/lib/desktop-api/options';
import { OptionChainTable } from './OptionChainTable';
import { ChainFilters, type FilterState } from './ChainFilters';
import { ChainSkeleton } from './ChainSkeleton';
import { ChainError } from './ChainError';
import { EmptyChain } from './EmptyChain';
import type { OptionContract } from '@/types/option';

interface OptionChainViewerProps {
  symbol: string;
  workspaceId: string;
  provider?: 'live' | 'demo' | 'file';
}

export function OptionChainViewer({ 
  symbol, 
  workspaceId,
  provider = 'demo' 
}: OptionChainViewerProps) {
  const [filters, setFilters] = useState<FilterState>({
    expirationRange: 'all',
    strikeRange: 'all',
    minVolume: 0,
    minOpenInterest: 0,
  });

  // Fetch option chain
  const { 
    data: chain, 
    isLoading, 
    error,
    refetch 
  } = useQuery({
    queryKey: ['optionChain', symbol, workspaceId, provider],
    queryFn: () => optionsApi.fetchOptionChain({ 
      symbol, 
      workspaceId, 
      provider 
    }),
    staleTime: 5 * 60 * 1000, // 5 minutes
    retry: 2,
  });

  // Fetch contracts for chain
  const { data: contracts = [] } = useQuery({
    queryKey: ['optionContracts', chain?.id],
    queryFn: async () => {
      if (!chain?.id) return [];
      // TODO: Implement getChainContracts API call
      return [] as OptionContract[];
    },
    enabled: !!chain?.id,
  });

  // Apply filters
  const filteredContracts = useMemo(() => {
    return contracts.filter(contract => {
      // Volume filter
      if (contract.volume < filters.minVolume) return false;
      
      // Open interest filter
      if (contract.openInterest < filters.minOpenInterest) return false;
      
      // TODO: Add expiration and strike range filters
      
      return true;
    });
  }, [contracts, filters]);

  // Keyboard navigation
  const handleKeyDown = (event: React.KeyboardEvent) => {
    if (event.key === 'Escape') {
      // Clear filters
      setFilters({
        expirationRange: 'all',
        strikeRange: 'all',
        minVolume: 0,
        minOpenInterest: 0,
      });
    }
  };

  // Render states
  if (isLoading) {
    return <ChainSkeleton />;
  }

  if (error) {
    return (
      <ChainError 
        error={error as Error}
        onRetry={() => refetch()}
      />
    );
  }

  if (!chain || filteredContracts.length === 0) {
    return <EmptyChain symbol={symbol} />;
  }

  return (
    <div 
      className="flex flex-col h-full"
      onKeyDown={handleKeyDown}
      tabIndex={0}
      role="region"
      aria-label="Option chain viewer"
    >
      {/* Filters */}
      <div className="mb-4">
        <ChainFilters 
          filters={filters}
          onChange={setFilters}
        />
      </div>

      {/* Summary */}
      <div className="mb-4 text-sm text-gray-600">
        <span className="font-medium">{symbol}</span>
        {' • '}
        <span>{filteredContracts.length} contracts</span>
        {' • '}
        <span>Underlying: ${chain.underlyingPrice.toFixed(2)}</span>
      </div>

      {/* Chain table */}
      <div className="flex-1 overflow-auto">
        <OptionChainTable 
          contracts={filteredContracts}
          underlyingPrice={chain.underlyingPrice}
        />
      </div>
    </div>
  );
}

export default OptionChainViewer;