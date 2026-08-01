/**
 * Greeks Chart Component
 * Visualizes Greeks sensitivities
 */

import type { GreeksResponse } from '@/types/option';

export interface GreeksChartProps {
  greeks: GreeksResponse | null;
}

export function GreeksChart({ greeks }: GreeksChartProps) {
  if (!greeks) return null;

  // Simple bar chart visualization
  const maxAbsValue = Math.max(
    Math.abs(greeks.delta),
    Math.abs(greeks.gamma),
    Math.abs(greeks.theta),
    Math.abs(greeks.vega),
    Math.abs(greeks.rho)
  );

  const bars = [
    { name: 'Delta', value: greeks.delta, color: 'bg-blue-500' },
    { name: 'Gamma', value: greeks.gamma, color: 'bg-green-500' },
    { name: 'Theta', value: greeks.theta, color: 'bg-red-500' },
    { name: 'Vega', value: greeks.vega, color: 'bg-purple-500' },
    { name: 'Rho', value: greeks.rho, color: 'bg-yellow-500' },
  ];

  return (
    <div className="p-4 border rounded-lg">
      <h3 className="text-lg font-semibold mb-4">Greeks Visualization</h3>
      <div className="space-y-3">
        {bars.map((bar) => (
          <div key={bar.name} className="flex items-center gap-3">
            <span className="w-16 text-sm font-medium">{bar.name}</span>
            <div className="flex-1 h-6 bg-gray-100 rounded overflow-hidden">
              <div
                className={`h-full ${bar.color} transition-all duration-300`}
                style={{ width: `${(Math.abs(bar.value) / maxAbsValue) * 100}%` }}
              />
            </div>
            <span className="w-20 text-sm text-right">{bar.value.toFixed(4)}</span>
          </div>
        ))}
      </div>
    </div>
  );
}