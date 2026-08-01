/**
 * useGreeksCalculator Hook
 * TanStack Query mutation for Greeks calculation
 */

import { useMutation } from '@tanstack/react-query';
import { optionsApi } from '@/lib/desktop-api/options';
import type { CalculateGreeksParams, GreeksResponse } from '@/types/option';

export function useGreeksCalculator() {
  return useMutation<GreeksResponse, Error, CalculateGreeksParams>({
    mutationFn: (params) => optionsApi.calculateGreeks(params),
    retry: 1,
  });
}

export function useOptionPricing() {
  return useMutation<number, Error, CalculateGreeksParams>({
    mutationFn: (params) => optionsApi.calculateOptionPrice(params),
    retry: 1,
  });
}