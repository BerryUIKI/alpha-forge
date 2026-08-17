/**
 * Strategy Builder Component
 *
 * Allows users to build multi-leg option strategies and calculate payoffs.
 *
 * @module features/options/components/StrategyBuilder
 */

import { useState } from "react";
import type { OptionType, PositionType } from "@/types/option";

interface StrategyLeg {
  id: string;
  optionType: OptionType;
  strike: number;
  expiration: string;
  quantity: number;
  positionType: PositionType;
  premium: number;
}

interface StrategyBuilderProps {
  /** Callback when strategy is built */
  onBuild?: (legs: StrategyLeg[], analysis: StrategyAnalysis) => void;
}

interface StrategyAnalysis {
  netCost: number;
  breakEvenPoints: number[];
  maxProfit: number | null;
  maxLoss: number | null;
}

export function StrategyBuilder({ onBuild }: StrategyBuilderProps) {
  const [legs, setLegs] = useState<StrategyLeg[]>([]);
  const [underlyingPrice, setUnderlyingPrice] = useState("100");
  const [analysis, setAnalysis] = useState<StrategyAnalysis | null>(null);

  const addLeg = () => {
    const newLeg: StrategyLeg = {
      id: `leg-${Date.now()}`,
      optionType: "call",
      strike: 100,
      expiration: "2025-12-31",
      quantity: 1,
      positionType: "long",
      premium: 5,
    };
    setLegs([...legs, newLeg]);
  };

  const removeLeg = (id: string) => {
    setLegs(legs.filter((leg) => leg.id !== id));
  };

  const updateLeg = (id: string, updates: Partial<StrategyLeg>) => {
    setLegs(
      legs.map((leg) => (leg.id === id ? { ...leg, ...updates } : leg))
    );
  };

  const calculateAnalysis = () => {
    const netCost = legs.reduce((sum, leg) => {
      const cost = leg.premium * Math.abs(leg.quantity);
      return leg.positionType === "long" ? sum + cost : sum - cost;
    }, 0);

    // Simplified analysis - in production would use strategy_service
    setAnalysis({
      netCost,
      breakEvenPoints: [100], // Placeholder
      maxProfit: null,
      maxLoss: Math.abs(netCost),
    });

    onBuild?.(legs, {
      netCost,
      breakEvenPoints: [100],
      maxProfit: null,
      maxLoss: Math.abs(netCost),
    });
  };

  return (
    <div className="p-4 border rounded-lg">
      <h3 className="text-lg font-semibold mb-4">Strategy Builder</h3>
      <p className="text-sm text-muted-foreground mb-4">
        Build multi-leg option strategies and analyze risk/reward
      </p>

      {/* Underlying Price */}
      <div className="mb-4">
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

      {/* Legs */}
      <div className="mb-4">
        <div className="flex items-center justify-between mb-2">
          <span className="text-sm font-medium">Strategy Legs</span>
          <button
            onClick={addLeg}
            className="text-sm px-3 py-1 bg-primary text-primary-foreground rounded hover:opacity-90"
          >
            Add Leg
          </button>
        </div>

        {legs.length === 0 ? (
          <p className="text-sm text-muted-foreground">No legs added yet.</p>
        ) : (
          <div className="space-y-2">
            {legs.map((leg, index) => (
              <div key={leg.id} className="p-3 border rounded">
                <div className="flex items-center justify-between mb-2">
                  <span className="font-medium">Leg {index + 1}</span>
                  <button
                    onClick={() => removeLeg(leg.id)}
                    className="text-sm text-destructive hover:underline"
                  >
                    Remove
                  </button>
                </div>
                <div className="grid grid-cols-3 gap-2">
                  <div>
                    <label className="text-xs text-muted-foreground">Type</label>
                    <select
                      className="w-full p-1 border rounded text-sm"
                      value={leg.optionType}
                      onChange={(e) =>
                        updateLeg(leg.id, { optionType: e.target.value as OptionType })
                      }
                    >
                      <option value="call">Call</option>
                      <option value="put">Put</option>
                    </select>
                  </div>
                  <div>
                    <label className="text-xs text-muted-foreground">Strike</label>
                    <input
                      type="number"
                      className="w-full p-1 border rounded text-sm"
                      value={leg.strike}
                      onChange={(e) =>
                        updateLeg(leg.id, { strike: parseFloat(e.target.value) })
                      }
                    />
                  </div>
                  <div>
                    <label className="text-xs text-muted-foreground">Quantity</label>
                    <input
                      type="number"
                      className="w-full p-1 border rounded text-sm"
                      value={leg.quantity}
                      onChange={(e) =>
                        updateLeg(leg.id, { quantity: parseInt(e.target.value) })
                      }
                    />
                  </div>
                  <div>
                    <label className="text-xs text-muted-foreground">Position</label>
                    <select
                      className="w-full p-1 border rounded text-sm"
                      value={leg.positionType}
                      onChange={(e) =>
                        updateLeg(leg.id, {
                          positionType: e.target.value as PositionType,
                        })
                      }
                    >
                      <option value="long">Long</option>
                      <option value="short">Short</option>
                    </select>
                  </div>
                  <div>
                    <label className="text-xs text-muted-foreground">Premium</label>
                    <input
                      type="number"
                      className="w-full p-1 border rounded text-sm"
                      value={leg.premium}
                      onChange={(e) =>
                        updateLeg(leg.id, { premium: parseFloat(e.target.value) })
                      }
                    />
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Analyze Button */}
      <button
        onClick={calculateAnalysis}
        disabled={legs.length === 0}
        className="w-full p-2 bg-primary text-primary-foreground rounded hover:opacity-90 disabled:opacity-50"
      >
        Analyze Strategy
      </button>

      {/* Analysis Results */}
      {analysis && (
        <div className="mt-4 p-3 bg-muted rounded">
          <h4 className="font-medium mb-2">Strategy Analysis</h4>
          <div className="grid grid-cols-2 gap-2 text-sm">
            <div>
              <span className="text-muted-foreground">Net Cost: </span>
              <span className="font-mono">${analysis.netCost.toFixed(2)}</span>
            </div>
            <div>
              <span className="text-muted-foreground">Max Loss: </span>
              <span className="font-mono">
                {analysis.maxLoss !== null ? `$${analysis.maxLoss.toFixed(2)}` : "Unlimited"}
              </span>
            </div>
            <div>
              <span className="text-muted-foreground">Max Profit: </span>
              <span className="font-mono">
                {analysis.maxProfit !== null ? `$${analysis.maxProfit.toFixed(2)}` : "Unlimited"}
              </span>
            </div>
            <div>
              <span className="text-muted-foreground">Break-even: </span>
              <span className="font-mono">
                {analysis.breakEvenPoints.map((p) => `$${p}`).join(", ")}
              </span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}