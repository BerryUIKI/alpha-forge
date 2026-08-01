/**
 * Strategy Payoff View
 * Main view component for strategy payoff analysis
 */

import { PayoffChart } from './PayoffChart';
import { RiskPanel } from './RiskPanel';

export default function StrategyPayoffView() {
  return (
    <div className="strategy-payoff-view">
      <h1>Strategy Payoff Analysis - Phase 4 Plugin</h1>
      <PayoffChart />
      <RiskPanel />
    </div>
  );
}