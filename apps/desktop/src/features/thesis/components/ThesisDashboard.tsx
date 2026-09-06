import { useEffect, useState } from "react";
import { Kanban, List, Plus } from "lucide-react";
import { EmptyState, ErrorState, LoadingSpinner } from "@/components/common";
import { useWorkspaces } from "@/features/workspace/hooks/useWorkspaces";
import { useActiveWorkspaceId } from "@/features/workspace/hooks/useActiveWorkspace.context";
import { useLocale } from "@/lib/i18n/useLocale";
import type { InvestmentThesis } from "@/lib/desktop-api/thesis";
import { useTheses } from "../hooks/useTheses";
import { CreateThesisForm } from "./CreateThesisForm";
import { ThesisDetail } from "./ThesisDetail";
import { ThesisList } from "./ThesisList";
import { ThesisPipelineBoard } from "./ThesisPipelineBoard";
import { KnowledgeGraphPanel } from "./KnowledgeGraphPanel";

export function ThesisDashboard() {
  const { t } = useLocale();
  // Loading/error states come from the workspace list query; the active
  // workspace itself comes from the global context (ADR-0008).
  const { data: workspaces, isLoading, error, refetch } = useWorkspaces();
  const workspaceId = useActiveWorkspaceId();
  const [selectedId, setSelectedId] = useState<string>();
  const [viewMode, setViewMode] = useState<"board" | "list">("board");
  const [showCreateForm, setShowCreateForm] = useState(false);
  const thesesQuery = useTheses(workspaceId);
  const selected = thesesQuery.data?.find((thesis) => thesis.id === selectedId);

  // Reset the selected thesis when the active workspace changes.
  useEffect(() => {
    setSelectedId(undefined);
  }, [workspaceId]);

  if (isLoading) {
    return <LoadingSpinner className="p-8" ariaLabel={t("loading")} />;
  }
  if (error) {
    return (
      <ErrorState
        message={t("thesisFailedToLoadWorkspaces")}
        retryLabel={t("retry")}
        onRetry={() => refetch()}
      />
    );
  }
  if (!workspaces?.length) {
    return (
      <EmptyState
        title={t("createWorkspaceFirst")}
        description={t("createWorkspaceFirstDescription")}
      />
    );
  }

  return (
    <div className="space-y-6">
      {/* Header & Controls Bar */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h2 className="text-xl font-bold tracking-tight text-white flex items-center gap-2">
            <span className="h-2.5 w-2.5 rounded-full bg-purple-500" />
            <span>{t("investmentTheses")}</span>
          </h2>
          <p className="mt-0.5 text-xs text-neutral-400">{t("thesesPipelineSub")}</p>
        </div>

        <div className="flex items-center gap-2.5">
          {/* View Toggle: Board / List */}
          <div className="flex items-center rounded-lg bg-black/20 p-1 border border-white/10 text-xs font-medium text-neutral-400">
            <button
              onClick={() => setViewMode("board")}
              className={`flex items-center gap-1.5 rounded-md px-2.5 py-1 transition-all ${
                viewMode === "board"
                  ? "bg-white/10 text-white shadow-sm"
                  : "hover:text-neutral-200"
              }`}
            >
              <Kanban className="h-3.5 w-3.5" />
              <span>{t("viewBoard")}</span>
            </button>
            <button
              onClick={() => setViewMode("list")}
              className={`flex items-center gap-1.5 rounded-md px-2.5 py-1 transition-all ${
                viewMode === "list"
                  ? "bg-white/10 text-white shadow-sm"
                  : "hover:text-neutral-200"
              }`}
            >
              <List className="h-3.5 w-3.5" />
              <span>{t("viewList")}</span>
            </button>
          </div>

          {/* New Thesis Toggle */}
          <button
            onClick={() => setShowCreateForm(!showCreateForm)}
            className="flex items-center gap-1.5 rounded-lg bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground shadow-sm hover:bg-primary/90 transition-colors"
          >
            <Plus className="h-3.5 w-3.5" />
            <span>{t("newInvestmentThesis")}</span>
          </button>
        </div>
      </div>

      {/* Optional Collapsible Create Form */}
      {showCreateForm && (
        <div className="rounded-xl border border-white/10 bg-[#1c1e28] p-4 shadow-sm animate-in fade-in duration-150">
          <CreateThesisForm
            workspaceId={workspaceId}
            onCreated={(id) => {
              setSelectedId(id);
              setShowCreateForm(false);
            }}
          />
        </div>
      )}

      {/* Main View Area */}
      {viewMode === "board" ? (
        <div className="space-y-6">
          <ThesisPipelineBoard
            workspaceId={workspaceId}
            selectedId={selectedId}
            onSelect={(thesis: InvestmentThesis) => setSelectedId(thesis.id)}
          />

          {/* When a thesis is selected from the board, open its detailed view below */}
          {selected && (
            <div className="mt-6 rounded-xl border border-white/10 bg-[#1c1e28] p-5 shadow-lg">
              <div className="flex items-center justify-between border-b border-white/5 pb-3 mb-4">
                <span className="text-xs font-semibold text-neutral-300 uppercase tracking-wider">
                  {t("cardDetails")}: {selected.title}
                </span>
                <button
                  onClick={() => setSelectedId(undefined)}
                  className="text-xs text-neutral-400 hover:text-white"
                >
                  ✕
                </button>
              </div>
              <ThesisDetail
                thesis={selected}
                onDeleted={() => setSelectedId(undefined)}
              />
            </div>
          )}
        </div>
      ) : (
        <div className="grid gap-6 lg:grid-cols-[minmax(0,0.8fr)_minmax(0,1.2fr)]">
          <div className="space-y-4">
            <ThesisList
              workspaceId={workspaceId}
              selectedId={selectedId}
              onSelect={(thesis: InvestmentThesis) => setSelectedId(thesis.id)}
            />
          </div>
          <div>
            {selected ? (
              <ThesisDetail thesis={selected} onDeleted={() => setSelectedId(undefined)} />
            ) : (
              <EmptyState title={t("selectThesis")} description={t("selectThesisDescription")} />
            )}
          </div>
        </div>
      )}

      <KnowledgeGraphPanel workspaceId={workspaceId} />
    </div>
  );
}
