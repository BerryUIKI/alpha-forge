/**
 * Greeks Calculator - Main Component
 * Complete Greeks calculation interface
 */

import { useState } from 'react';
import { useMutation } from '@tanstack/react-query';
import { optionsApi } from '@/lib/desktop-api/options';
import { GreeksForm } from './GreeksForm';
import { GreeksResults } from './GreeksResults';
import { GreeksChart } from './GreeksChart';
import type { GreeksResponse, CalculateGreeksParams } from '@/types/option';

export function GreeksCalculator() {
  const [results, setResults] = useState<GreeksResponse | null>(null);

  const calculateMutation = useMutation({
    mutationFn: (params: CalculateGreeksParams) => optionsApi.calculateGreeks(params),
    onSuccess: (data) => {
      setResults(data);
    },
  });

  return (
    <div className="grid grid-cols-2 gap-6">
      <div>
        <GreeksForm
          onSubmit={(params) => calculateMutation.mutate(params)}
          isLoading={calculateMutation.isPending}
        />
      </div>
      
      <div className="space-y-4">
        <GreeksResults results={results} />
        <GreeksChart greeks={results} />
      </div>
      
      {calculateMutation.isError && (
        <div className="col-span-2 p-4 bg-red-50 border border-red-200 rounded text-red-700">
          Error calculating Greeks: {calculateMutation.error?.message}
        </div>
      )}
    </div>
  );
}