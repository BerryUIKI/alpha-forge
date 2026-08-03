/**
 * OptionsView Component
 *
 * Workspace view for Options analysis.
 * UI-only placeholder with empty state.
 *
 * TODO: [GUI-M1-4] Implement Options workspace functionality
 * TODO: [GUI-M1-4] Add options pricing and analysis tools
 * TODO: [GUI-M1-4] Connect to options data feeds
 */

import { TrendingUp } from "lucide-react";
import type { WorkspaceViewProps } from "../../types";

export function OptionsView({ isActive }: WorkspaceViewProps) {
  if (!isActive) return null;

  return (
    <div className="flex h-full flex-col">
      {/* Placeholder Content */}
      <div className="flex flex-1 items-center justify-center">
        <div className="text-center">
          <TrendingUp className="h-16 w-16 mx-auto mb-4 text-muted-foreground" />
          <h2 className="text-2xl font-semibold mb-2">Options Workspace</h2>
          <p className="text-muted-foreground mb-4">
            Options analysis and strategy tools coming soon
          </p>
          <div className="rounded-lg border border-dashed border-border p-8 max-w-md mx-auto">
            <p className="text-sm text-muted-foreground">
              {/* TODO: [GUI-M1-4] Implement options workspace */}
              This workspace will provide options-specific tools including:
            </p>
            <ul className="text-sm text-muted-foreground mt-2 space-y-1 text-left">
              <li>• Options pricing calculator</li>
              <li>• Greeks analysis</li>
              <li>• Strategy builders</li>
              <li>• Volatility analysis</li>
            </ul>
          </div>
        </div>
      </div>

      {/* TODO markers for future implementation */}
      {/* TODO: [GUI-M1-4] Add payoff diagram components */}
      {/* TODO: [GUI-M1-4] Add strategy templates */}
      {/* TODO: [GUI-M1-4] Add risk/reward visualizations */}
    </div>
  );
}