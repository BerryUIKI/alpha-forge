import { useMemo, useState } from "react";
import { BookOpen, Kanban, ListFilter } from "lucide-react";
import { EmptyState, ErrorState, LoadingSpinner } from "@/components/common";
import type { InvestmentThesis, ThesisStatus } from "@/lib/desktop-api/thesis";
import { useTheses, useActivateThesis, useStartThesisValidation, useCompleteThesisValidation } from "../hooks/useTheses";
import { useLocale } from "@/lib/i18n/useLocale";
import { ThesisCard } from "./ThesisCard";

interface ThesisPipelineBoardProps {
  workspaceId: string;
  selectedId?: string;
  onSelect: (thesis: InvestmentThesis) => void;
}

const STAGES: { status: ThesisStatus; titleKey: "statusDraft" | "statusActive" | "statusValidating" | "statusValidated" | "statusClosed"; dotColor: string }[] = [
  { status: "draft", titleKey: "statusDraft", dotColor: "bg-neutral-400" },
  { status: "active", titleKey: "statusActive", dotColor: "bg-indigo-400" },
  { status: "validating", titleKey: "statusValidating", dotColor: "bg-amber-400" },
  { status: "validated", titleKey: "statusValidated", dotColor: "bg-emerald-400" },
  { status: "closed", titleKey: "statusClosed", dotColor: "bg-zinc-500" },
];

export function ThesisPipelineBoard({
  workspaceId,
  selectedId,
  onSelect,
}: ThesisPipelineBoardProps) {
  const { t } = useLocale();
  const { data, isLoading, error, refetch } = useTheses(workspaceId);
  const [filter, setFilter] = useState<string>("all");

  const activate = useActivateThesis();
  const startValidation = useStartThesisValidation();
  const completeValidation = useCompleteThesisValidation();

  const handleQuickTransition = async (thesis: InvestmentThesis, nextStatus: ThesisStatus) => {
    try {
      if (nextStatus === "active") {
        await activate.mutateAsync(thesis.id);
      } else if (nextStatus === "validating") {
        await startValidation.mutateAsync(thesis.id);
      } else if (nextStatus === "validated") {
        await completeValidation.mutateAsync({
          id: thesis.id,
          outcome: "Thesis validated from board transition",
          validated: true,
        });
      }
    } catch (err) {
      console.error("Failed to transition thesis status:", err);
    }
  };

  const filteredTheses = useMemo(() => {
    if (!data) return [];
    if (filter === "all") return data;
    return data.filter((item) => item.status === filter);
  }, [data, filter]);

  const groupedTheses = useMemo(() => {
    const groups: Record<ThesisStatus, InvestmentThesis[]> = {
      draft: [],
      active: [],
      validating: [],
      validated: [],
      closed: [],
    };
    for (const item of filteredTheses) {
      if (groups[item.status]) {
        groups[item.status].push(item);
      }
    }
    return groups;
  }, [filteredTheses]);

  if (isLoading) return <LoadingSpinner className="p-8" />;
  if (error) return <ErrorState message={t("failedToLoadTheses")} onRetry={() => refetch()} />;
  if (!data?.length) {
    return (
      <EmptyState
        icon={<BookOpen className="h-8 w-8" />}
        title={t("noThesesYet")}
        description={t("noThesesDescription")}
      />
    );
  }

  return (
    <div className="space-y-4">
      {/* Board Controls & Filter Bar */}
      <div className="flex flex-wrap items-center justify-between gap-3 rounded-xl border border-white/10 bg-black/20 px-4 py-2.5">
        <div className="flex items-center gap-2">
          <Kanban className="h-4 w-4 text-primary" />
          <span className="text-xs font-semibold text-white tracking-tight">
            {t("thesesPipelineTitle")}
          </span>
          <span className="rounded-full bg-white/10 px-2 py-0.5 text-[10px] font-mono text-neutral-300">
            {filteredTheses.length}
          </span>
        </div>

        <div className="flex items-center gap-2">
          <ListFilter className="h-3.5 w-3.5 text-neutral-400" />
          <span className="text-xs text-neutral-400">{t("filterStatus")}:</span>
          <select
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
            className="rounded-lg border border-white/10 bg-[#1c1e28] px-2.5 py-1 text-xs text-neutral-200 outline-none focus:border-primary"
          >
            <option value="all">{t("allStatuses")} ({data.length})</option>
            <option value="draft">{t("statusDraft")}</option>
            <option value="active">{t("statusActive")}</option>
            <option value="validating">{t("statusValidating")}</option>
            <option value="validated">{t("statusValidated")}</option>
            <option value="closed">{t("statusClosed")}</option>
          </select>
        </div>
      </div>

      {/* Horizontal Kanban Columns */}
      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-5">
        {STAGES.map(({ status, titleKey, dotColor }) => {
          const items = groupedTheses[status] ?? [];
          return (
            <div
              key={status}
              className="flex flex-col rounded-xl border border-white/5 bg-[#14161f]/80 p-3 shadow-inner"
            >
              {/* Column Header */}
              <div className="flex items-center justify-between border-b border-white/5 pb-2 mb-3">
                <div className="flex items-center gap-1.5">
                  <span className={`h-2 w-2 rounded-full ${dotColor}`} />
                  <span className="text-xs font-semibold text-neutral-200">
                    {t(titleKey)}
                  </span>
                </div>
                <span className="rounded bg-white/5 px-1.5 py-0.2 text-[10px] font-mono text-neutral-400">
                  {items.length}
                </span>
              </div>

              {/* Cards List in Column */}
              <div className="space-y-3 flex-1 overflow-y-auto max-h-[560px] pr-1">
                {items.length === 0 ? (
                  <div className="py-8 text-center text-[11px] text-neutral-400 border border-dashed border-white/5 rounded-lg">
                    {t("noThesesInStage")}
                  </div>
                ) : (
                  items.map((thesis) => (
                    <ThesisCard
                      key={thesis.id}
                      thesis={thesis}
                      isSelected={selectedId === thesis.id}
                      onSelect={onSelect}
                      onQuickTransition={handleQuickTransition}
                    />
                  ))
                )}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
