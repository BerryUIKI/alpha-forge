/**
 * Option Chain Page
 * Complete option chain viewer with symbol/expiration selection
 */

import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { OptionChainViewer } from '@/features/options/OptionChainViewer';
import { optionsApi } from '@/lib/desktop-api/options';
import { ChainSkeleton } from '@/features/options/OptionChainViewer/ChainSkeleton';
import { ChainError } from '@/features/options/OptionChainViewer/ChainError';

export function OptionChainPage() {
  const [symbol, setSymbol] = useState('AAPL');
  const [workspaceId] = useState('demo-workspace'); // In real app, get from context

  const { data, isLoading, error, refetch } = useQuery({
    queryKey: ['optionChain', symbol],
    queryFn: () => optionsApi.fetchOptionChain({
      symbol,
      workspaceId,
      provider: 'demo',
    }),
    enabled: symbol.length > 0,
  });

  return (
    <div className="p-6 space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold">Option Chain</h1>
        
        <div className="flex gap-4 items-center">
          <div>
            <label className="block text-sm font-medium mb-1">Symbol</label>
            <input
              type="text"
              value={symbol}
              onChange={(e) => setSymbol(e.target.value.toUpperCase())}
              className="border rounded px-3 py-2 w-32"
              placeholder="AAPL"
            />
          </div>
          
          <button
            onClick={() => refetch()}
            disabled={isLoading}
            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50"
          >
            Refresh
          </button>
        </div>
      </div>

      {isLoading && <ChainSkeleton />}
      
      {error && (
        <ChainError 
          error={new Error(error instanceof Error ? error.message : 'Failed to load option chain')}
          onRetry={() => refetch()}
        />
      )}
      
      {data && !isLoading && !error && (
        <OptionChainViewer 
          symbol={symbol}
          workspaceId={workspaceId}
          provider="demo"
        />
      )}
    </div>
  );
}