/**
 * Greeks Results Component
 * Displays calculated Greeks values
 */

import type { GreeksResponse } from '@/types/option';

export interface GreeksResultsProps {
  results: GreeksResponse | null;
}

export function GreeksResults({ results }: GreeksResultsProps) {
  if (!results) {
    return (
      <div className="p-4 text-gray-500 text-center">
        Enter parameters and click Calculate to see Greeks
      </div>
    );
  }

  const greeks = [
    { name: 'Delta', value: results.delta, description: 'Price sensitivity to underlying' },
    { name: 'Gamma', value: results.gamma, description: 'Delta sensitivity to underlying' },
    { name: 'Theta', value: results.theta, description: 'Time decay per day' },
    { name: 'Vega', value: results.vega, description: 'Sensitivity to 1% IV change' },
    { name: 'Rho', value: results.rho, description: 'Sensitivity to 1% rate change' },
  ];

  return (
    <div className="space-y-3">
      <h3 className="text-lg font-semibold">Greeks Results</h3>
      <div className="grid gap-3">
        {greeks.map((greek) => (
          <div key={greek.name} className="border rounded p-3">
            <div className="flex justify-between items-center">
              <span className="font-medium">{greek.name}</span>
              <span className="text-lg font-bold">{greek.value.toFixed(4)}</span>
            </div>
            <p className="text-xs text-gray-500 mt-1">{greek.description}</p>
          </div>
        ))}
      </div>
    </div>
  );
}