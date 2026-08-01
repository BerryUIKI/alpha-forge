/**
 * Strategy Preview Component
 * Shows real-time payoff diagram and risk metrics
 */

export interface StrategyPreviewProps {
  legs: Array<{
    id: string;
    optionType: 'call' | 'put';
    strike: number;
    expiration: string;
    quantity: number;
    premium: number;
  }>;
  symbol: string;
}

export function StrategyPreview({ legs, symbol }: StrategyPreviewProps) {
  return (
    <div className="strategy-preview">
      <p>Strategy Preview - Phase 4 Component</p>
      <p>Symbol: {symbol}, Legs: {legs.length}</p>
    </div>
  );
}