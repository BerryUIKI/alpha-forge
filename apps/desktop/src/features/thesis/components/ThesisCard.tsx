import { useMemo } from "react";
import type { InvestmentThesis, ThesisStatus } from "@/lib/desktop-api/thesis";
import { useLocale } from "@/lib/i18n/useLocale";
import { useThesisEvidence } from "../hooks/useTheses";

interface ThesisCardProps {
  thesis: InvestmentThesis;
  isSelected?: boolean;
  onSelect: (thesis: InvestmentThesis) => void;
  onQuickTransition?: (thesis: InvestmentThesis, nextStatus: ThesisStatus) => void;
}

export function ThesisCard({
  thesis,
  isSelected,
  onSelect,
  onQuickTransition,
}: ThesisCardProps) {
  const { t } = useLocale();
  const evidenceQuery = useThesisEvidence(thesis.id);

  const evidenceCounts = useMemo(() => {
    const list = evidenceQuery.data ?? [];
    let supporting = 0;
    let contradicting = 0;
    for (const item of list) {
      if (item.direction === "supporting") supporting += 1;
      else if (item.direction === "contradicting") contradicting += 1;
    }
    return { supporting, contradicting };
  }, [evidenceQuery.data]);

  // Color gradient and label based on conviction score
  const convictionColor = useMemo(() => {
    if (thesis.confidence >= 80) return "from-indigo-500 to-purple-500 text-purple-300";
    if (thesis.confidence >= 50) return "from-amber-500 to-orange-500 text-amber-300";
    return "from-slate-500 to-zinc-500 text-neutral-400";
  }, [thesis.confidence]);

  // Next logical status in pipeline
  const nextStatus = useMemo<ThesisStatus | null>(() => {
    switch (thesis.status) {
      case "draft":
        return "active";
      case "active":
        return "validating";
      case "validating":
        return "validated";
      default:
        return null;
    }
  }, [thesis.status]);

  const statusLabel = useMemo(() => {
    switch (thesis.status) {
      case "draft":
        return t("statusDraft");
      case "active":
        return t("statusActive");
      case "validating":
        return t("statusValidating");
      case "validated":
        return t("statusValidated");
      case "closed":
        return t("statusClosed");
      default:
        return thesis.status;
    }
  }, [thesis.status, t]);

  return (
    <div
      onClick={() => onSelect(thesis)}
      role="button"
      tabIndex={0}
      onKeyDown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          onSelect(thesis);
        }
      }}
      className={`group relative flex flex-col justify-between rounded-xl border p-4 text-left transition-all cursor-pointer shadow-sm ${
        isSelected
          ? "border-primary/80 bg-primary/5 ring-1 ring-primary/40"
          : "border-white/10 bg-black/20 hover:border-white/20 hover:bg-black/30"
      }`}
    >
      <div>
        {/* Card Header: Title and Status Badge */}
        <div className="flex items-start justify-between gap-2">
          <h3 className="text-sm font-semibold text-white tracking-tight line-clamp-2 group-hover:text-primary transition-colors">
            {thesis.title}
          </h3>
          <span className="shrink-0 rounded-full bg-white/10 px-2 py-0.5 text-[10px] font-medium text-neutral-300 capitalize border border-white/5">
            {statusLabel}
          </span>
        </div>

        {/* Thesis Statement Snippet */}
        <p className="mt-1.5 text-xs text-neutral-400 line-clamp-2 leading-relaxed">
          {thesis.thesis}
        </p>
      </div>

      {/* Conviction Bar */}
      <div className="mt-3.5 space-y-1">
        <div className="flex items-center justify-between text-[11px]">
          <span className="text-neutral-400">{t("convictionLabel")}</span>
          <span className={`font-mono font-semibold ${convictionColor}`}>
            {thesis.confidence}%
          </span>
        </div>
        <div className="h-1.5 w-full overflow-hidden rounded-full bg-white/10">
          <div
            className={`h-full rounded-full bg-gradient-to-r ${convictionColor}`}
            style={{ width: `${thesis.confidence}%` }}
          />
        </div>
      </div>

      {/* Evidence Counters & Footer */}
      <div className="mt-3 pt-3 border-t border-white/5 flex items-center justify-between text-[11px] text-neutral-400">
        <div className="flex items-center gap-1.5 text-[10px]">
          <span className="rounded bg-emerald-500/10 px-1.5 py-0.5 text-emerald-400 font-medium">
            {evidenceCounts.supporting} pro
          </span>
          <span className="rounded bg-rose-500/10 px-1.5 py-0.5 text-rose-400 font-medium">
            {evidenceCounts.contradicting} contra
          </span>
        </div>

        {nextStatus && onQuickTransition && (
          <button
            type="button"
            onClick={(e) => {
              e.stopPropagation();
              onQuickTransition(thesis, nextStatus);
            }}
            className="flex items-center gap-1 rounded px-1.5 py-0.5 text-[10px] text-primary hover:bg-primary/10 transition-colors"
            title={t("cardQuickAction")}
          >
            <span>→ {nextStatus}</span>
          </button>
        )}
      </div>
    </div>
  );
}
