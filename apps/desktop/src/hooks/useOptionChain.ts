/**
 * useOptionChain Hook
 * TanStack Query hook for fetching option chain data
 */

import { useQuery, useQueryClient } from '@tanstack/react-query';
import { optionsApi } from '@/lib/desktop-api/options';
import type { FetchChainParams } from '@/types/option';

export function useOptionChain(symbol: string, workspaceId: string, provider: 'live' | 'demo' | 'file' = 'demo') {
  const params: FetchChainParams = { symbol, workspaceId, provider };
  
  return useQuery({
    queryKey: ['optionChain', symbol, workspaceId, provider],
    queryFn: () => optionsApi.fetchOptionChain(params),
    staleTime: 5 * 60 * 1000, // 5 minutes
    gcTime: 30 * 60 * 1000, // 30 minutes
    retry: 2,
    retryDelay: (attemptIndex) => Math.min(1000 * 2 ** attemptIndex, 30000),
    enabled: symbol.length > 0 && workspaceId.length > 0,
  });
}

export function useOptionChainRefresh() {
  const queryClient = useQueryClient();
  
  return (symbol: string) => {
    queryClient.invalidateQueries({ queryKey: ['optionChain', symbol] });
  };
}