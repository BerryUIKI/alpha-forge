/**
 * Strategy Builder - Multi-leg strategy construction
 */

import { useState } from 'react';
import { StrategySelector } from './StrategySelector';
import { LegBuilder } from './LegBuilder';
import { StrategyPreview } from './StrategyPreview';
import type { StrategyLeg, StrategyType } from '@/types/option';

interface StrategyBuilderProps {
  symbol: string;
  workspaceId: string;
}

export function StrategyBuilder({ symbol, workspaceId }: StrategyBuilderProps) {
  const [strategyType, setStrategyType] = useState<StrategyType>('custom');
  const [legs, setLegs] = useState<StrategyLeg[]>([]);

  const addLeg = () => {
    const newLeg: StrategyLeg = {
      id: `leg-${Date.now()}`,
      strategyId: '',
      optionContractId: '',
      quantity: 1,
      positionType: 'long',
      premium: 0,
      strike: 0,
      expiration: new Date().toISOString(),
      optionType: 'call',
    };
    setLegs([...legs, newLeg]);
  };

  const removeLeg = (id: string) => {
    setLegs(legs.filter(leg => leg.id !== id));
  };

  const updateLeg = (id: string, updates: Partial<StrategyLeg>) => {
    setLegs(legs.map(leg => 
      leg.id === id ? { ...leg, ...updates } : leg
    ));
  };

  return (
    <div className="grid grid-cols-3 gap-6 h-full">
      {/* Left: Strategy Selector */}
      <div className="col-span-1">
        <StrategySelector 
          strategyType={strategyType}
          onChange={setStrategyType}
        />
      </div>

      {/* Middle: Leg Builder */}
      <div className="col-span-1 overflow-auto">
        <LegBuilder 
          legs={legs}
          onAdd={addLeg}
          onRemove={removeLeg}
          onUpdate={updateLeg}
        />
      </div>

      {/* Right: Preview */}
      <div className="col-span-1">
        <StrategyPreview 
          legs={legs}
          symbol={symbol}
        />
      </div>
    </div>
  );
}

export default StrategyBuilder;