/**
 * OtherDerivativesView Component
 *
 * Workspace view for Other Derivatives analysis.
 * UI-only placeholder with empty state.
 *
 * TODO: [GUI-M1-4] Implement Other Derivatives workspace functionality
 * TODO: [GUI-M1-4] Add derivatives analysis tools
 * TODO: [GUI-M1-4] Connect to derivatives data sources
 */

import { Layers } from "lucide-react";
import type { WorkspaceViewProps } from "../../types";

export function OtherDerivativesView({ isActive }: WorkspaceViewProps) {
  if (!isActive) return null;

  return (
    <div className="flex h-full flex-col">
      {/* Placeholder Content */}
      <div className="flex flex-1 items-center justify-center">
        <div className="text-center">
          <Layers className="h-16 w-16 mx-auto mb-4 text-muted-foreground" />
          <h2 className="text-2xl font-semibold mb-2">Other Derivatives Workspace</h2>
          <p className="text-muted-foreground mb-4">
            Specialized derivatives analysis tools coming soon
          </p>
          <div className="rounded-lg border border-dashed border-border p-8 max-w-md mx-auto">
            <p className="text-sm text-muted-foreground">
              {/* TODO: [GUI-M1-4] Implement other derivatives workspace */}
              This workspace will provide tools for exotic derivatives including:
            </p>
            <ul className="text-sm text-muted-foreground mt-2 space-y-1 text-left">
              <li>• Swaps analysis</li>
              <li>• Structured products</li>
              <li>• Credit derivatives</li>
              <li>• Custom derivative modeling</li>
            </ul>
          </div>
        </div>
      </div>

      {/* TODO markers for future implementation */}
      {/* TODO: [GUI-M1-4] Add derivative pricing models */}
      {/* TODO: [GUI-M1-4] Add payoff analysis tools */}
      {/* TODO: [GUI-M1-4] Add risk metrics calculators */}
    </div>
  );
}