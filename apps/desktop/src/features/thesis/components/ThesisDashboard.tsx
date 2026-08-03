import { useEffect, useState } from "react";
import { EmptyState, ErrorState, LoadingSpinner } from "@/components/common";
import { useWorkspaces } from "@/features/workspace/hooks/useWorkspaces";
import { useLocale } from "@/lib/i18n/useLocale";
import type { InvestmentThesis } from "@/lib/desktop-api/thesis";
import { useTheses } from "../hooks/useTheses";
import { CreateThesisForm } from "./CreateThesisForm";
import { ThesisDetail } from "./ThesisDetail";
import { ThesisList } from "./ThesisList";
import { KnowledgeGraphPanel } from "./KnowledgeGraphPanel";

export function ThesisDashboard() {
  const { t } = useLocale();
  const { data: workspaces, isLoading, error, refetch } = useWorkspaces();
  const [workspaceId, setWorkspaceId] = useState("");
  const [selectedId, setSelectedId] = useState<string>();
  const thesesQuery = useTheses(workspaceId);
  const selected = thesesQuery.data?.find((thesis) => thesis.id === selectedId);
  useEffect(() => { if (!workspaceId && workspaces?.[0]) setWorkspaceId(workspaces[0].id); }, [workspaceId, workspaces]);
  if (isLoading) return <LoadingSpinner className="p-8" ariaLabel={t("loading")} />;
  if (error) return <ErrorState message={t("thesisFailedToLoadWorkspaces")} retryLabel={t("retry")} onRetry={() => refetch()} />;
  if (!workspaces?.length) return <EmptyState title={t("createWorkspaceFirst")} description={t("createWorkspaceFirstDescription")} />;
  return <div className="space-y-6"><div><h2 className="text-2xl font-bold">{t("investmentTheses")}</h2><p className="mt-1 text-muted-foreground">{t("thesisDescription")}</p></div><label className="block max-w-sm text-sm font-medium">{t("workspace")}<select value={workspaceId} onChange={(event) => { setWorkspaceId(event.target.value); setSelectedId(undefined); }} className="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm">{workspaces.map((workspace) => <option key={workspace.id} value={workspace.id}>{workspace.name}</option>)}</select></label><div className="grid gap-6 lg:grid-cols-[minmax(0,0.8fr)_minmax(0,1.2fr)]"><div className="space-y-4"><CreateThesisForm workspaceId={workspaceId} onCreated={setSelectedId} /><ThesisList workspaceId={workspaceId} selectedId={selectedId} onSelect={(thesis: InvestmentThesis) => setSelectedId(thesis.id)} /></div><div>{selected ? <ThesisDetail thesis={selected} onDeleted={() => setSelectedId(undefined)} /> : <EmptyState title={t("selectThesis")} description={t("selectThesisDescription")} />}</div></div><KnowledgeGraphPanel workspaceId={workspaceId} /></div>;
}
