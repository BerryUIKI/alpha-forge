/**
 * Strategy Selector - Pre-built strategy templates
 */

import { useState } from 'react';
import { cn } from '@/lib/utils';
import type { StrategyType } from '@/types/option';

interface StrategySelectorProps {
  strategyType: StrategyType;
  onChange: (type: StrategyType) => void;
}

const STRATEGY_TEMPLATES: Array<{
  type: StrategyType;
  name: string;
  description: string;
  legs: number;
}> = [
  { 
    type: 'long_call', 
    name: 'Long Call', 
    description: 'Bullish, unlimited profit, limited risk',
    legs: 1 
  },
  { 
    type: 'long_put', 
    name: 'Long Put', 
    description: 'Bearish, profit from decline',
    legs: 1 
  },
  { 
    type: 'covered_call', 
    name: 'Covered Call', 
    description: 'Neutral to bullish, income generation',
    legs: 2 
  },
  { 
    type: 'protective_put', 
    name: 'Protective Put', 
    description: 'Portfolio insurance, limited downside',
    legs: 2 
  },
  { 
    type: 'bull_call_spread', 
    name: 'Bull Call Spread', 
    description: 'Moderate bullish, defined risk/reward',
    legs: 2 
  },
  { 
    type: 'bear_put_spread', 
    name: 'Bear Put Spread', 
    description: 'Moderate bearish, defined risk/reward',
    legs: 2 
  },
  { 
    type: 'straddle', 
    name: 'Straddle', 
    description: 'Volatility play, either direction',
    legs: 2 
  },
  { 
    type: 'strangle', 
    name: 'Strangle', 
    description: 'Lower cost volatilty play',
    legs: 2 
  },
  { 
    type: 'iron_condor', 
    name: 'Iron Condor', 
    description: 'Range-bound, premium collection',
    legs: 4 
  },
  { 
    type: 'butterfly', 
    name: 'Butterfly', 
    description: 'Target price, low cost',
    legs: 3 
  },
  { 
    type: 'custom', 
    name: 'Custom Strategy', 
    description: 'Build your own multi-leg strategy',
    legs: 0 
  },
];

export function StrategySelector({ strategyType, onChange }: StrategySelectorProps) {
  return (
    <div className="space-y-2">
      <h3 className="text-sm font-semibold text-gray-900 mb-3">
        Strategy Templates
      </h3>
      
      <div 
        className="space-y-1"
        role="listbox"
        aria-label="Strategy templates"
      >
        {STRATEGY_TEMPLATES.map((template) => (
          <button
            key={template.type}
            onClick={() => onChange(template.type)}
            className={cn(
              "w-full text-left px-3 py-2 rounded-md transition-colors",
              "hover:bg-gray-100",
              strategyType === template.type 
                ? "bg-blue-50 border-blue-500 border" 
                : "border border-transparent"
            )}
            role="option"
            aria-selected={strategyType === template.type}
          >
            <div className="font-medium text-sm">{template.name}</div>
            <div className="text-xs text-gray-500">{template.description}</div>
            {template.legs > 0 && (
              <div className="text-xs text-gray-400 mt-1">
                {template.legs} leg{template.legs > 1 ? 's' : ''}
              </div>
            )}
          </button>
        ))}
      </div>
    </div>
  );
}

export default StrategySelector;