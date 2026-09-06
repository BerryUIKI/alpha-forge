import { useState } from "react";
import { Sparkles, FileText, AlertTriangle, Layers, ExternalLink } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { desktopApi } from "@/lib/desktop-api";
import { parseResearchCompletion } from "../types/ResearchCompletion";

interface ResearchResultCardProps {
  payload: string | null;
  workspaceId?: string;
  taskId?: string;
}

export function ResearchResultCard({
  payload,
  workspaceId,
  taskId,
}: ResearchResultCardProps) {
  const { t } = useLocale();
  const [isLaunchingArtifact, setIsLaunchingArtifact] = useState(false);
  const result = parseResearchCompletion(payload);

  if (!result) {
    return (
      <div className="rounded-lg border border-dashed border-border p-4 text-center text-sm text-muted-foreground">
        {t("noResultsAvailable")}
      </div>
    );
  }

  const getConfidenceColor = (score: number) => {
    if (score <= 33) return "bg-destructive/10 text-destructive border-destructive/20";
    if (score <= 66) return "bg-amber-500/10 text-amber-600 border-amber-500/20";
    return "bg-emerald-500/10 text-emerald-600 border-emerald-500/20";
  };

  const handleLaunchArtifact = async () => {
    if (!workspaceId) return;
    setIsLaunchingArtifact(true);
    try {
      const artifact = await desktopApi.artifacts.createArtifact({
        workspaceId,
        taskId,
        artifactType: "comparison_table",
        input: {
          title: result.summary.slice(0, 40) || "Sensitivity Model",
          claims: result.claims,
          risks: result.risks,
          confidence: result.confidence,
        },
      });
      await desktopApi.artifacts.startViewingArtifact(artifact.id);
    } catch (err) {
      console.error("Failed to launch artifact window:", err);
    } finally {
      setIsLaunchingArtifact(false);
    }
  };

  return (
    <div className="flex flex-col gap-3 rounded-xl border border-indigo-500/30 bg-black/30 p-3.5 shadow-md">
      {/* Header with Sparkles & Confidence Pill */}
      <div className="flex items-center justify-between border-b border-white/5 pb-2">
        <div className="flex items-center gap-1.5 text-xs font-semibold text-indigo-400">
          <Sparkles className="h-3.5 w-3.5" />
          <span>{t("structuredInferenceTitle")}</span>
        </div>
        <div
          className={`rounded-full border px-2 py-0.5 text-[10px] font-mono font-semibold ${getConfidenceColor(
            result.confidence,
          )}`}
        >
          {t("researchConfidence")} {result.confidence}%
        </div>
      </div>

      {/* Summary / Thesis Impact */}
      <div>
        <h4 className="text-[11px] font-semibold text-white">
          {t("thesisImpactHeader")}:
        </h4>
        <p className="mt-1 text-xs leading-relaxed text-neutral-300">
          {result.summary}
        </p>
      </div>

      {/* Key Claims */}
      {result.claims.length > 0 && (
        <div className="space-y-1">
          <h4 className="text-[10px] font-semibold uppercase tracking-wider text-neutral-400">
            {t("researchClaims")}
          </h4>
          <ul className="list-inside list-disc space-y-0.5 text-xs text-neutral-300">
            {result.claims.map((claim, idx) => (
              <li key={idx}>{claim}</li>
            ))}
          </ul>
        </div>
      )}

      {/* Verifiable Evidence */}
      {result.evidence.length > 0 && (
        <div className="space-y-1 pt-1">
          <h4 className="text-[10px] font-semibold uppercase tracking-wider text-neutral-400 flex items-center gap-1">
            <FileText className="h-3 w-3 text-cyan-400" />
            <span>{t("verifiableEvidenceHeader")}</span>
          </h4>
          <div className="space-y-1.5">
            {result.evidence.map((item, idx) => (
              <div
                key={idx}
                className="rounded-lg border border-white/5 bg-white/5 p-2 text-xs text-neutral-300 leading-relaxed"
              >
                {item}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Falsification Triggers / Risks */}
      {result.risks.length > 0 && (
        <div className="rounded-lg border border-rose-500/20 bg-rose-500/10 p-2 text-xs text-rose-300 space-y-1">
          <div className="flex items-center gap-1 font-semibold text-[11px]">
            <AlertTriangle className="h-3 w-3" />
            <span>{t("falsificationTriggersHeader")}:</span>
          </div>
          <ul className="list-inside list-disc space-y-0.5 text-[11px] text-neutral-300">
            {result.risks.map((risk, idx) => (
              <li key={idx}>{risk}</li>
            ))}
          </ul>
        </div>
      )}

      {/* Interactive Artifact Launcher Button */}
      {workspaceId && (
        <button
          type="button"
          onClick={handleLaunchArtifact}
          disabled={isLaunchingArtifact}
          className="mt-1 flex w-full items-center justify-center gap-1.5 rounded-lg border border-indigo-500/40 bg-indigo-500/20 px-3 py-1.5 text-xs font-medium text-indigo-300 transition-colors hover:bg-indigo-500/30 disabled:opacity-50"
        >
          <Layers className="h-3.5 w-3.5" />
          <span>
            {isLaunchingArtifact
              ? t("generatingArtifact")
              : t("generateInteractiveArtifact")}
          </span>
          <ExternalLink className="h-3 w-3 opacity-60" />
        </button>
      )}
    </div>
  );
}
