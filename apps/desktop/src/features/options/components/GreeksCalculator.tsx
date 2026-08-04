/**
 * Greeks Calculator Component
 *
 * Allows users to calculate option Greeks (Delta, Gamma, Theta, Vega, Rho)
 * using the Black-Scholes model.
 *
 * @module features/options/components/GreeksCalculator
 */

import { useState } from "react";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { useCalculateGreeks } from "@/hooks/useOptions";
import type { OptionType } from "@/types/option";

interface GreeksCalculatorProps {
  /** Default underlying price */
  defaultUnderlyingPrice?: number;
  /** Default strike price */
  defaultStrike?: number;
  /** Default volatility (decimal, e.g., 0.25 for 25%) */
  defaultVolatility?: number;
}

export function GreeksCalculator({
  defaultUnderlyingPrice = 100,
  defaultStrike = 100,
  defaultVolatility = 0.25,
}: GreeksCalculatorProps) {
  const [optionType, setOptionType] = useState<OptionType>("call");
  const [underlyingPrice, setUnderlyingPrice] = useState(defaultUnderlyingPrice.toString());
  const [strike, setStrike] = useState(defaultStrike.toString());
  const [expirationYears, setExpirationYears] = useState("1");
  const [riskFreeRate, setRiskFreeRate] = useState("0.05");
  const [volatility, setVolatility] = useState(defaultVolatility.toString());
  const [dividendYield, setDividendYield] = useState("0");

  const calculateMutation = useCalculateGreeks();

  const handleCalculate = () => {
    const params = {
      optionType,
      underlyingPrice: parseFloat(underlyingPrice),
      strike: parseFloat(strike),
      expirationYears: parseFloat(expirationYears),
      riskFreeRate: parseFloat(riskFreeRate),
      volatility: parseFloat(volatility),
      dividendYield: parseFloat(dividendYield),
    };

    calculateMutation.mutate(params);
  };

  const greeks = calculateMutation.data;

  return (
    <div className="p-4 border rounded-lg">
      <h3 className="text-lg font-semibold mb-4">Greeks Calculator</h3>
      <p className="text-sm text-muted-foreground mb-4">
        Calculate option Greeks using the Black-Scholes model
      </p>

      {/* Option Type */}
      <div className="mb-4">
        <label className="block text-sm font-medium mb-1">Option Type</label>
        <select
          className="w-full p-2 border rounded"
          value={optionType}
          onChange={(e) => setOptionType(e.target.value as OptionType)}
        >
          <option value="call">Call</option>
          <option value="put">Put</option>
        </select>
      </div>

      {/* Price Inputs Grid */}
      <div className="grid grid-cols-2 gap-4 mb-4">
        <div>
          <label className="block text-sm font-medium mb-1">Underlying Price</label>
          <input
            type="number"
            className="w-full p-2 border rounded"
            value={underlyingPrice}
            onChange={(e) => setUnderlyingPrice(e.target.value)}
            min={0}
            step={0.01}
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">Strike Price</label>
          <input
            type="number"
            className="w-full p-2 border rounded"
            value={strike}
            onChange={(e) => setStrike(e.target.value)}
            min={0}
            step={0.01}
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">Time to Expiry (Years)</label>
          <input
            type="number"
            className="w-full p-2 border rounded"
            value={expirationYears}
            onChange={(e) => setExpirationYears(e.target.value)}
            min={0}
            step={0.01}
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">Volatility (σ)</label>
          <input
            type="number"
            className="w-full p-2 border rounded"
            value={volatility}
            onChange={(e) => setVolatility(e.target.value)}
            min={0}
            step={0.01}
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">Risk-Free Rate</label>
          <input
            type="number"
            className="w-full p-2 border rounded"
            value={riskFreeRate}
            onChange={(e) => setRiskFreeRate(e.target.value)}
            step={0.01}
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">Dividend Yield</label>
          <input
            type="number"
            className="w-full p-2 border rounded"
            value={dividendYield}
            onChange={(e) => setDividendYield(e.target.value)}
            min={0}
            step={0.01}
          />
        </div>
      </div>

      {/* Calculate Button */}
      <button
        onClick={handleCalculate}
        disabled={calculateMutation.isPending}
        className="w-full p-2 bg-primary text-primary-foreground rounded hover:opacity-90 disabled:opacity-50"
      >
        {calculateMutation.isPending ? "Calculating..." : "Calculate Greeks"}
      </button>

      {/* Loading State */}
      {calculateMutation.isPending && (
        <div className="mt-4 flex justify-center">
          <LoadingSpinner />
        </div>
      )}

      {/* Results */}
      {greeks && !calculateMutation.isPending && (
        <div className="grid grid-cols-5 gap-2 mt-4">
          <div className="text-center p-2 bg-muted rounded">
            <div className="text-xs text-muted-foreground">Delta</div>
            <div className="font-mono font-semibold">{greeks.delta.toFixed(4)}</div>
          </div>
          <div className="text-center p-2 bg-muted rounded">
            <div className="text-xs text-muted-foreground">Gamma</div>
            <div className="font-mono font-semibold">{greeks.gamma.toFixed(4)}</div>
          </div>
          <div className="text-center p-2 bg-muted rounded">
            <div className="text-xs text-muted-foreground">Theta</div>
            <div className="font-mono font-semibold">{greeks.theta.toFixed(4)}</div>
          </div>
          <div className="text-center p-2 bg-muted rounded">
            <div className="text-xs text-muted-foreground">Vega</div>
            <div className="font-mono font-semibold">{greeks.vega.toFixed(4)}</div>
          </div>
          <div className="text-center p-2 bg-muted rounded">
            <div className="text-xs text-muted-foreground">Rho</div>
            <div className="font-mono font-semibold">{greeks.rho.toFixed(4)}</div>
          </div>
        </div>
      )}

      {/* Error State */}
      {calculateMutation.isError && (
        <div className="mt-4 p-2 bg-destructive/10 text-destructive rounded text-sm">
          Failed to calculate Greeks. Please check your inputs.
        </div>
      )}
    </div>
  );
}