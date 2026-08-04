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
import { useLocale } from "@/lib/i18n/useLocale";
import { translate, formatMessage } from "@/lib/i18n/locale";
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
  const { locale } = useLocale();
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
        title={translate(locale, "shadowAnalysisUnavailable")}
        description={
          !health.binary_available
            ? translate(locale, "gooseBinaryNotFound")
            : translate(locale, "shadowModeDisabled")
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
          {translate(locale, "runningShadowAnalysis")}
        </p>
        <button
          className="px-4 py-2 bg-red-500 text-white rounded-md hover:bg-red-600"
          onClick={handleCancel}
        >
          {translate(locale, "cancelAnalysis")}
        </button>
      </div>
    );
  }

  // Error state
  if (startError) {
    const errorMessage = typeof startError === 'string'
      ? startError
      : (startError as any)?.description || translate(locale, "analysisFailed");
    return (
      <ErrorState
        title={translate(locale, "analysisFailed")}
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
        locale={locale}
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
        <h2 className="text-xl font-semibold">{translate(locale, "shadowAnalysis")}</h2>
        <p className="text-sm text-muted-foreground">
          {translate(locale, "shadowAnalysisDescription")}
        </p>
      </div>

      {thesisId && (
        <p className="text-sm text-muted-foreground">
          {formatMessage(translate(locale, "focusingOnThesis"), { thesisId })}
        </p>
      )}

      <div className="space-y-2">
        <label className="text-sm font-medium">{translate(locale, "customInstructions")}</label>
        <textarea
          className="w-full min-h-[80px] rounded-md border border-input bg-transparent px-3 py-2 text-sm"
          placeholder={translate(locale, "customInstructionsPlaceholder")}
          value={instructions}
          onChange={(e) => setInstructions(e.target.value)}
        />
      </div>

      <button
        className="w-full px-4 py-2 bg-primary text-white rounded-md hover:bg-primary/90"
        onClick={handleStart}
      >
        {translate(locale, "startShadowAnalysis")}
      </button>

      <p className="text-xs text-muted-foreground text-center">
        {translate(locale, "analysisReadOnly")}
      </p>
    </div>
  );
}

/**
 * Confidence meter component - visual progress bar
 */
function ConfidenceMeter({ confidence }: { confidence: number }) {
  const getColorClass = (conf: number) => {
    if (conf >= 80) return "bg-green-500";
    if (conf >= 60) return "bg-blue-500";
    if (conf >= 40) return "bg-yellow-500";
    return "bg-red-500";
  };

  return (
    <div className="flex items-center gap-2">
      <div className="flex-1 h-2 bg-secondary rounded-full overflow-hidden">
        <div
          className={`h-full ${getColorClass(confidence)} transition-all duration-300`}
          style={{ width: `${confidence}%` }}
        />
      </div>
      <span className="text-sm font-medium">{confidence}%</span>
    </div>
  );
}

/**
 * Analysis Result Component
 */
function AnalysisResult({
  response,
  durationMs,
  locale,
  onReset,
}: {
  response: StructuredResponse;
  durationMs: number;
  locale: "zh-CN" | "en";
  onReset: () => void;
}) {
  const severityColors: Record<RiskSeverity, string> = {
    low: "bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300",
    medium: "bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-300",
    high: "bg-orange-100 text-orange-800 dark:bg-orange-900/30 dark:text-orange-300",
    critical: "bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-300",
  };

  const relationColors = {
    supports: "text-green-600 dark:text-green-400",
    contradicts: "text-red-600 dark:text-red-400",
    neutral: "text-muted-foreground",
  };

  return (
    <div className="space-y-6">
      {/* Summary */}
      <div className="p-6 border rounded-lg space-y-4">
        <div className="flex items-center justify-between">
          <h2 className="text-xl font-semibold">{translate(locale, "analysisComplete")}</h2>
          <span className="px-2 py-1 text-xs bg-secondary rounded-md">
            {(durationMs / 1000).toFixed(1)}{translate(locale, "seconds")}
          </span>
        </div>
        <p className="text-sm">{response.summary}</p>
        
        {/* Overall Confidence */}
        <div className="space-y-1">
          <label className="text-sm font-medium">{translate(locale, "confidence")}</label>
          <ConfidenceMeter confidence={response.confidence} />
        </div>
      </div>

      {/* Claims */}
      {response.claims.length > 0 && (
        <div className="p-6 border rounded-lg space-y-4">
          <h3 className="text-lg font-semibold">{translate(locale, "claims")}</h3>
          <div className="space-y-3">
            {response.claims.map((claim) => (
              <div key={claim.id} className="border-l-2 border-primary pl-3 py-2">
                <p className="text-sm">{claim.claim}</p>
                <div className="flex items-center gap-2 mt-1">
                  <span className="px-2 py-0.5 text-xs bg-secondary rounded">
                    {claim.confidence}% {translate(locale, "confidence")}
                  </span>
                  <span className="text-xs text-muted-foreground">
                    {claim.source_ids.length} {translate(locale, "sources")}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Evidence Section - NEW */}
      {response.evidence.length > 0 && (
        <div className="p-6 border rounded-lg space-y-4">
          <h3 className="text-lg font-semibold">{translate(locale, "evidenceSection")}</h3>
          <div className="space-y-3">
            {response.evidence.map((evidence, i) => (
              <div key={`${evidence.claim_id}-${evidence.source_id}-${i}`} className="border-l-2 border-muted pl-3 py-2">
                <div className="flex items-center gap-2 mb-1">
                  <span className={`text-xs font-medium ${relationColors[evidence.relation]}`}>
                    {evidence.relation === "supports" 
                      ? translate(locale, "supporting")
                      : evidence.relation === "contradicts"
                        ? translate(locale, "contradicting")
                        : translate(locale, "neutral")}
                  </span>
                  {evidence.confidence !== undefined && (
                    <span className="text-xs text-muted-foreground">
                      ({evidence.confidence}%)
                    </span>
                  )}
                </div>
                <p className="text-sm italic text-muted-foreground">
                  "{evidence.excerpt}"
                </p>
                <p className="text-xs text-muted-foreground mt-1">
                  Source: {evidence.source_id}
                </p>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Risks */}
      {response.risks.length > 0 && (
        <div className="p-6 border rounded-lg space-y-4">
          <h3 className="text-lg font-semibold">{translate(locale, "risksSection")}</h3>
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
                      {translate(locale, "mitigation")}: {risk.mitigation}
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
          <h3 className="text-lg font-semibold">{translate(locale, "unknowns")}</h3>
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
        {translate(locale, "runNewAnalysis")}
      </button>
    </div>
  );
}

export default ShadowAnalysis;