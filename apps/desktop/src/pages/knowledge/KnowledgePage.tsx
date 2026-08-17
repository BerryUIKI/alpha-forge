/**
 * KnowledgePage
 *
 * Shows knowledge graph entities with create/list functionality.
 * Uses the existing knowledgeGraph API from desktopApi.
 *
 * @version GUI-M1+
 */

import { useState } from "react";
import { useLocale } from "@/lib/i18n/useLocale";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import { EmptyState, ErrorState, LoadingSpinner } from "@/components/common";
import { BookOpen, Plus, X } from "lucide-react";
import { useWorkspaces } from "@/features/workspace/hooks/useWorkspaces";
import type { KnowledgeEntityType } from "@/lib/desktop-api/knowledge-graph";

const ENTITY_TYPES: KnowledgeEntityType[] = ["company", "industry", "technology", "macro_theme"];

const ENTITY_TYPE_LABELS: Record<KnowledgeEntityType, string> = {
  company: "Company",
  industry: "Industry",
  technology: "Technology",
  macro_theme: "Macro Theme",
};

const ENTITY_TYPE_COLORS: Record<KnowledgeEntityType, string> = {
  company: "border-blue-500/30 bg-blue-500/10 text-blue-400",
  industry: "border-green-500/30 bg-green-500/10 text-green-400",
  technology: "border-purple-500/30 bg-purple-500/10 text-purple-400",
  macro_theme: "border-amber-500/30 bg-amber-500/10 text-amber-400",
};

export function KnowledgePage() {
  const { t } = useLocale();
  const queryClient = useQueryClient();
  const { data: workspaces } = useWorkspaces();
  const workspaceId = workspaces?.[0]?.id ?? "";
  const [showCreate, setShowCreate] = useState(false);
  const [newName, setNewName] = useState("");
  const [newType, setNewType] = useState<KnowledgeEntityType>("company");
  const [newDesc, setNewDesc] = useState("");

  const entities = useQuery({
    queryKey: ["knowledgeEntities", workspaceId],
    queryFn: () => desktopApi.knowledgeGraph.listKnowledgeEntities(workspaceId),
    enabled: !!workspaceId,
  });

  const createEntity = useMutation({
    mutationFn: () =>
      desktopApi.knowledgeGraph.createKnowledgeEntity(workspaceId, newType, newName, newDesc || undefined),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["knowledgeEntities", workspaceId] });
      setShowCreate(false);
      setNewName("");
      setNewType("company");
      setNewDesc("");
    },
  });

  if (!workspaceId) {
    return (
      <div className="p-6">
        <h1 className="text-2xl font-bold">{t("knowledgeGraph")}</h1>
        <EmptyState icon={<BookOpen />} title={t("knowledgeGraph")} description="Create a workspace first." />
      </div>
    );
  }

  return (
    <div className="p-6">
      {/* Header */}
      <div className="mb-6 flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">{t("knowledgeGraph")}</h1>
          <p className="mt-1 text-sm text-muted-foreground">{t("knowledgeGraphDescription")}</p>
        </div>
        <button
          onClick={() => setShowCreate(true)}
          className="inline-flex items-center gap-1.5 rounded-lg bg-primary px-3 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
        >
          <Plus className="h-4 w-4" />
          Add Entity
        </button>
      </div>

      {/* Create form */}
      {showCreate && (
        <div className="mb-6 rounded-lg border bg-card p-4">
          <div className="mb-3 flex items-center justify-between">
            <h3 className="text-sm font-semibold">New Knowledge Entity</h3>
            <button onClick={() => setShowCreate(false)} className="rounded p-1 hover:bg-accent">
              <X className="h-4 w-4" />
            </button>
          </div>
          <div className="space-y-3">
            <div>
              <label className="mb-1 block text-xs font-medium text-muted-foreground">Name</label>
              <input
                value={newName}
                onChange={(e) => setNewName(e.target.value)}
                placeholder="e.g. NVIDIA"
                className="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm outline-none focus:border-primary"
              />
            </div>
            <div>
              <label className="mb-1 block text-xs font-medium text-muted-foreground">Type</label>
              <select
                value={newType}
                onChange={(e) => setNewType(e.target.value as KnowledgeEntityType)}
                className="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm"
              >
                {ENTITY_TYPES.map((t) => (
                  <option key={t} value={t}>{ENTITY_TYPE_LABELS[t]}</option>
                ))}
              </select>
            </div>
            <div>
              <label className="mb-1 block text-xs font-medium text-muted-foreground">Description</label>
              <textarea
                value={newDesc}
                onChange={(e) => setNewDesc(e.target.value)}
                placeholder="Optional description"
                rows={2}
                className="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm outline-none focus:border-primary"
              />
            </div>
            <button
              onClick={() => createEntity.mutate()}
              disabled={!newName.trim() || createEntity.isPending}
              className="rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
            >
              {createEntity.isPending ? "Creating..." : "Create"}
            </button>
          </div>
        </div>
      )}

      {/* Loading */}
      {entities.isLoading && <LoadingSpinner className="p-12" />}

      {/* Error */}
      {entities.error && !entities.isLoading && (
        <ErrorState message="Failed to load knowledge entities" onRetry={() => entities.refetch()} />
      )}

      {/* Empty */}
      {entities.data && entities.data.length === 0 && !entities.isLoading && (
        <EmptyState
          icon={<BookOpen className="h-8 w-8 text-muted-foreground" />}
          title={t("knowledgeGraph")}
          description="Start building your knowledge network by adding companies, industries, technologies, and macro themes."
        />
      )}

      {/* Entity list */}
      {entities.data && entities.data.length > 0 && (
        <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
          {entities.data.map((entity) => (
            <div
              key={entity.id}
              className="rounded-lg border bg-card p-4 transition-colors hover:bg-accent/50"
            >
              <div className="mb-2 flex items-center gap-2">
                <span className={`rounded-md border px-2 py-0.5 text-[10px] font-medium uppercase ${ENTITY_TYPE_COLORS[entity.entity_type]}`}>
                  {ENTITY_TYPE_LABELS[entity.entity_type]}
                </span>
              </div>
              <h3 className="font-semibold">{entity.name}</h3>
              {entity.description && (
                <p className="mt-1 text-sm text-muted-foreground line-clamp-2">{entity.description}</p>
              )}
              <p className="mt-2 text-[10px] text-muted-foreground/60">
                Added {entity.created_at.slice(0, 10)}
              </p>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}