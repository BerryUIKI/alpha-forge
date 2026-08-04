/**
 * Shadow Analysis Component
 *
 * Provides UI for running Goose shadow-mode analysis.
 * Displays structured results including claims, evidence, and risks.
 *
 * @module features/goose/components/ShadowAnalysis
 */

import { useState } from "react";
import { useGooseShadowAnalysis } from "@/hooks/useGooseAnalysis";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { ErrorState } from "@/components/common/ErrorState";
import { EmptyState } from "@/components/common/EmptyState";
import type { StructuredResponse, RiskSeverity } from "@/lib/desktop-api/goose";

interface ShadowAnalysisProps {
  workspaceId: string;
  thesisId?: string;
  onComplete?: (result: StructuredResponse) => void;
}

/**
 * Shadow Analysis Component
 *
 * @example
 * <ShadowAnalysis
 *   workspaceId={workspace.id}
 *   thesisId={thesis?.id}
 *   onComplete={(result) => console.log("Analysis complete:", result)}
 * />
 */
export function ShadowAnalysis({
  workspaceId,
  thesisId,
  onComplete,
}: ShadowAnalysisProps) {
  const [instructions, setInstructions] = useState("");
  const {
    isReady,
    health,
    startAnalysis,
    isRunning,
    result,
    startError,
    cancelAnalysis,
  } = useGooseShadowAnalysis(workspaceId);

  const handleStart = () => {
    startAnalysis({
      thesis_id: thesisId,
      instructions: instructions || undefined,
    });
  };

  const handleCancel = () => {
    if (result?.run_id) {
      cancelAnalysis(result.run_id);
    }
  };

  // Health check loading
  if (!health) {
    return (
      <div className="flex items-center justify-center p-8">
        <LoadingSpinner size="lg" ariaLabel="Checking Goose service status" />
      </div>
    );
  }

  // Goose not available
  if (!isReady) {
    return (
      <EmptyState
        title="Shadow Analysis Unavailable"
        description={
          !health.binary_available
            ? "Goose binary not found."
            : "Shadow mode is disabled."
        }
      />
    );
  }

  // Running state
  if (isRunning) {
    return (
      <div className="flex flex-col items-center justify-center p-8 space-y-4">
        <LoadingSpinner size="lg" ariaLabel="Running shadow analysis" />
        <p className="text-sm text-muted-foreground text-center">
          Running shadow analysis. This may take a few minutes. No changes will be made to your data.
        </p>
        <button
          className="px-4 py-2 bg-red-500 text-white rounded-md hover:bg-red-600"
          onClick={handleCancel}
        >
          Cancel Analysis
        </button>
      </div>
    );
  }

  // Error state
  if (startError) {
    // startError is the result of processErrorResponse which returns an object
    const errorMessage = typeof startError === 'string' 
      ? startError 
      : (startError as any)?.description || 'Analysis failed';
    return (
      <ErrorState
        title="Analysis Failed"
        message={errorMessage}
        onRetry={handleStart}
      />
    );
  }

  // Success state - show results
  if (result) {
    return (
      <AnalysisResult
        response={result.response}
        durationMs={result.duration_ms}
        onReset={() => {
          setInstructions("");
          onComplete?.(result.response);
        }}
      />
    );
  }

  // Initial state
  return (
    <div className="space-y-6 p-6 border rounded-lg">
      <div className="space-y-2">
        <h2 className="text-xl font-semibold">Shadow Analysis</h2>
        <p className="text-sm text-muted-foreground">
          Run a read-only analysis of your workspace research data using Goose.
          Results will show claims, evidence, and risks without making any changes.
        </p>
      </div>

      {thesisId && (
        <p className="text-sm text-muted-foreground">
          Focusing on thesis: {thesisId}
        </p>
      )}

      <div className="space-y-2">
        <label className="text-sm font-medium">Custom Instructions (Optional)</label>
        <textarea
          className="w-full min-h-[80px] rounded-md border border-input bg-transparent px-3 py-2 text-sm"
          placeholder="E.g., 'Focus on financial metrics' or 'Ignore news older than 2024'"
          value={instructions}
          onChange={(e) => setInstructions(e.target.value)}
        />
      </div>

      <button
        className="w-full px-4 py-2 bg-primary text-white rounded-md hover:bg-primary/90"
        onClick={handleStart}
      >
        Start Shadow Analysis
      </button>

      <p className="text-xs text-muted-foreground text-center">
        This analysis is read-only. No data will be modified.
      </p>
    </div>
  );
}

/**
 * Analysis Result Component
 */
function AnalysisResult({
  response,
  durationMs,
  onReset,
}: {
  response: StructuredResponse;
  durationMs: number;
  onReset: () => void;
}) {
  const severityColors: Record<RiskSeverity, string> = {
    low: "bg-blue-100 text-blue-800",
    medium: "bg-yellow-100 text-yellow-800",
    high: "bg-orange-100 text-orange-800",
    critical: "bg-red-100 text-red-800",
  };

  return (
    <div className="space-y-6">
      {/* Summary */}
      <div className="p-6 border rounded-lg space-y-4">
        <div className="flex items-center justify-between">
          <h2 className="text-xl font-semibold">Analysis Complete</h2>
          <span className="px-2 py-1 text-xs bg-secondary rounded-md">
            {durationMs / 1000}s • {response.confidence}% confidence
          </span>
        </div>
        <p className="text-sm">{response.summary}</p>
      </div>

      {/* Claims */}
      {response.claims.length > 0 && (
        <div className="p-6 border rounded-lg space-y-4">
          <h3 className="text-lg font-semibold">Claims</h3>
          <div className="space-y-3">
            {response.claims.map((claim) => (
              <div key={claim.id} className="border-l-2 border-primary pl-3 py-2">
                <p className="text-sm">{claim.claim}</p>
                <div className="flex items-center gap-2 mt-1">
                  <span className="px-2 py-0.5 text-xs bg-secondary rounded">
                    {claim.confidence}% confidence
                  </span>
                  <span className="text-xs text-muted-foreground">
                    {claim.source_ids.length} sources
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Risks */}
      {response.risks.length > 0 && (
        <div className="p-6 border rounded-lg space-y-4">
          <h3 className="text-lg font-semibold">Risks</h3>
          <div className="space-y-3">
            {response.risks.map((risk) => (
              <div key={risk.id} className="flex items-start gap-3">
                <span className={`px-2 py-0.5 text-xs rounded ${severityColors[risk.severity]}`}>
                  {risk.severity}
                </span>
                <div className="flex-1">
                  <p className="text-sm">{risk.risk}</p>
                  {risk.mitigation && (
                    <p className="text-xs text-muted-foreground mt-1">
                      Mitigation: {risk.mitigation}
                    </p>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Unknowns */}
      {response.unknowns.length > 0 && (
        <div className="p-6 border rounded-lg space-y-4">
          <h3 className="text-lg font-semibold">Unknowns</h3>
          <ul className="list-disc list-inside space-y-1 text-sm text-muted-foreground">
            {response.unknowns.map((unknown, i) => (
              <li key={i}>{unknown}</li>
            ))}
          </ul>
        </div>
      )}

      {/* Actions */}
      <button
        className="px-4 py-2 border rounded-md hover:bg-secondary"
        onClick={onReset}
      >
        Run New Analysis
      </button>
    </div>
  );
}

export default ShadowAnalysis;