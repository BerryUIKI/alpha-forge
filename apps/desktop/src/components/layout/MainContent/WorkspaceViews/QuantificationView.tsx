/**
 * QuantificationView Component
 *
 * Workspace view for Quantification functionality.
 * UI-only placeholder with empty state.
 *
 * TODO: [GUI-M1-4] Implement Quantification workspace functionality
 * TODO: [GUI-M1-4] Add quantitative analysis tools
 * TODO: [GUI-M1-4] Connect to backend data sources
 */

import { Calculator } from "lucide-react";
import type { WorkspaceViewProps } from "../../types";

export function QuantificationView({ isActive }: WorkspaceViewProps) {
  if (!isActive) return null;

  return (
    <div className="flex h-full flex-col">
      {/* Placeholder Content */}
      <div className="flex flex-1 items-center justify-center">
        <div className="text-center">
          <Calculator className="h-16 w-16 mx-auto mb-4 text-muted-foreground" />
          <h2 className="text-2xl font-semibold mb-2">Quantification Workspace</h2>
          <p className="text-muted-foreground mb-4">
            Quantitative analysis tools coming soon
          </p>
          <div className="rounded-lg border border-dashed border-border p-8 max-w-md mx-auto">
            <p className="text-sm text-muted-foreground">
              {/* TODO: [GUI-M1-4] Implement quantification workspace */}
              This workspace will provide quantitative analysis capabilities including:
            </p>
            <ul className="text-sm text-muted-foreground mt-2 space-y-1 text-left">
              <li>• Factor analysis</li>
              <li>• Risk modeling</li>
              <li>• Backtesting</li>
              <li>• Performance metrics</li>
            </ul>
          </div>
        </div>
      </div>

      {/* TODO markers for future implementation */}
      {/* TODO: [GUI-M1-4] Add model configuration UI */}
      {/* TODO: [GUI-M1-4] Add backtesting interface */}
      {/* TODO: [GUI-M1-4] Add results visualization */}
    </div>
  );
}