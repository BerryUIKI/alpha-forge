/**
 * Leg Builder Component
 * Constructs individual option legs for strategies
 */

export interface StrategyLeg {
  id: string;
  optionType: 'call' | 'put';
  strike: number;
  expiration: string;
  quantity: number;
  premium: number;
}

export interface LegBuilderProps {
  legs: StrategyLeg[];
  onAdd: () => void;
  onRemove: (id: string) => void;
  onUpdate: (id: string, updates: Partial<StrategyLeg>) => void;
}

export function LegBuilder({ legs, onAdd, onRemove, onUpdate }: LegBuilderProps) {
  return (
    <div className="leg-builder">
      <p>Leg Builder - Phase 4 Component</p>
      <p>Legs: {legs.length}</p>
    </div>
  );
}