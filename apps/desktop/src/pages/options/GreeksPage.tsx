/**
 * Greeks Page
 * Greeks calculator interface with form and results
 */

import { GreeksCalculator } from '@/features/options/GreeksCalculator';

export function GreeksPage() {
  return (
    <div className="p-6">
      <div className="mb-6">
        <h1 className="text-2xl font-bold">Greeks Calculator</h1>
        <p className="text-gray-600 mt-2">
          Calculate option Greeks (Delta, Gamma, Theta, Vega, Rho) using Black-Scholes model
        </p>
      </div>
      
      <GreeksCalculator />
    </div>
  );
}