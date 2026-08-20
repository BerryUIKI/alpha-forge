import { useLocale } from "@/lib/i18n/useLocale";
import { parseResearchCompletion } from "../types/ResearchCompletion";

interface ResearchResultCardProps {
  payload: string | null;
}

export function ResearchResultCard({ payload }: ResearchResultCardProps) {
  const { t } = useLocale();
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

  return (
    <div className="flex flex-col gap-4 rounded-lg border border-border bg-card p-4">
      {/* Summary Section */}
      <div>
        <h4 className="mb-1 text-xs font-semibold uppercase tracking-wider text-muted-foreground/80">
          {t("researchSummary")}
        </h4>
        <p className="text-sm leading-relaxed">{result.summary}</p>
      </div>

      {/* Key Claims */}
      {result.claims.length > 0 && (
        <div>
          <h4 className="mb-1 text-xs font-semibold uppercase tracking-wider text-muted-foreground/80">
            {t("researchClaims")}
          </h4>
          <ul className="list-inside list-disc space-y-1 text-sm">
            {result.claims.map((claim, idx) => (
              <li key={idx} className="text-muted-foreground">
                {claim}
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Evidence */}
      {result.evidence.length > 0 && (
        <div>
          <h4 className="mb-1 text-xs font-semibold uppercase tracking-wider text-muted-foreground/80">
            {t("researchEvidence")}
          </h4>
          <ul className="list-inside list-disc space-y-1 text-sm">
            {result.evidence.map((item, idx) => (
              <li key={idx} className="text-muted-foreground">
                {item}
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Risks */}
      {result.risks.length > 0 && (
        <div>
          <h4 className="mb-1 text-xs font-semibold uppercase tracking-wider text-amber-600/80">
            {t("researchRisks")}
          </h4>
          <ul className="list-inside list-disc space-y-1 text-sm">
            {result.risks.map((risk, idx) => (
              <li key={idx} className="text-amber-600/90">
                {risk}
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Confidence */}
      <div className="flex items-center justify-between border-t border-border/50 pt-2">
        <span className="text-xs font-semibold uppercase tracking-wider text-muted-foreground/80">
          {t("researchConfidence")}
        </span>
        <div
          className={`rounded-full border px-2.5 py-0.5 text-xs font-semibold ${getConfidenceColor(
            result.confidence,
          )}`}
        >
          {result.confidence}%
        </div>
      </div>
    </div>
  );
}
