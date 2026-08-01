import { useState } from "react";
import { Trash2 } from "lucide-react";
import { ErrorState, LoadingSpinner } from "@/components/common";
import type { EvidenceDirection, InvestmentThesis } from "@/lib/desktop-api/thesis";
import {
  useActivateThesis, useAddThesisEvidence, useCloseThesis, useCompleteThesisValidation,
  useDeleteThesis, useDeleteThesisEvidence, useStartThesisValidation, useThesisConfidenceHistory,
  useThesisEvidence, useUpdateThesisConfidence,
} from "../hooks/useTheses";
import { useKnowledgeEntities, useLinkThesisKnowledgeEntity, useThesisKnowledgeLinks } from "../hooks/useKnowledgeGraph";

interface ThesisDetailProps { thesis: InvestmentThesis; onDeleted: () => void; }

export function ThesisDetail({ thesis, onDeleted }: ThesisDetailProps) {
  const [confidence, setConfidence] = useState(thesis.confidence);
  const [evidence, setEvidence] = useState("");
  const [direction, setDirection] = useState<EvidenceDirection>("supporting");
  const [sourceId, setSourceId] = useState("");
  const [outcome, setOutcome] = useState("");
  const [validated, setValidated] = useState(true);
  const [error, setError] = useState("");
  const evidenceQuery = useThesisEvidence(thesis.id);
  const confidenceHistory = useThesisConfidenceHistory(thesis.id);
  const knowledgeEntities = useKnowledgeEntities(thesis.workspace_id);
  const knowledgeLinks = useThesisKnowledgeLinks(thesis.id);
  const linkKnowledgeEntity = useLinkThesisKnowledgeEntity();
  const [knowledgeEntityId, setKnowledgeEntityId] = useState("");
  const activate = useActivateThesis();
  const startValidation = useStartThesisValidation();
  const completeValidation = useCompleteThesisValidation();
  const updateConfidence = useUpdateThesisConfidence();
  const close = useCloseThesis();
  const remove = useDeleteThesis();
  const addEvidence = useAddThesisEvidence();
  const deleteEvidence = useDeleteThesisEvidence();
  const pending = activate.isPending || startValidation.isPending || completeValidation.isPending || updateConfidence.isPending || close.isPending || remove.isPending || addEvidence.isPending;

  async function run(action: () => Promise<unknown>) { try { setError(""); await action(); } catch (cause) { setError(cause instanceof Error ? cause.message : "The thesis could not be updated."); } }
  async function saveEvidence(event: React.FormEvent) {
    event.preventDefault();
    if (!evidence.trim()) { setError("Evidence text is required."); return; }
    await run(async () => { await addEvidence.mutateAsync({ thesisId: thesis.id, direction, evidence: evidence.trim(), sourceId: sourceId.trim() || undefined }); setEvidence(""); setSourceId(""); });
  }

  return <section className="space-y-5 rounded-lg border border-border bg-card p-5">
    <div className="flex items-start justify-between gap-3"><div><p className="text-sm text-muted-foreground capitalize">{thesis.status}</p><h2 className="text-xl font-semibold">{thesis.title}</h2></div><button onClick={() => run(async () => { await remove.mutateAsync(thesis.id); onDeleted(); })} aria-label="Delete thesis" disabled={pending} className="rounded-md p-2 text-destructive hover:bg-destructive/10"><Trash2 className="h-4 w-4" /></button></div>
    <p className="whitespace-pre-wrap text-sm leading-6">{thesis.thesis}</p>
    <div><label htmlFor="thesis-confidence" className="mb-1 flex justify-between text-sm font-medium"><span>Confidence</span><span>{confidence}%</span></label><input id="thesis-confidence" type="range" min="0" max="100" value={confidence} onChange={(event) => setConfidence(Number(event.target.value))} className="w-full" /><button onClick={() => run(() => updateConfidence.mutateAsync({ thesisId: thesis.id, confidence }))} disabled={pending || confidence === thesis.confidence} className="mt-2 rounded-md border border-input px-3 py-1.5 text-sm hover:bg-accent disabled:opacity-50">Save confidence</button></div>
    {error && <p className="text-sm text-destructive">{error}</p>}
    <div className="flex flex-wrap gap-2">
      {thesis.status === "draft" && <button onClick={() => run(() => activate.mutateAsync(thesis.id))} disabled={pending} className="rounded-md bg-primary px-3 py-2 text-sm font-medium text-primary-foreground">Activate thesis</button>}
      {thesis.status === "active" && <button onClick={() => run(() => startValidation.mutateAsync(thesis.id))} disabled={pending} className="rounded-md bg-primary px-3 py-2 text-sm font-medium text-primary-foreground">Start validation</button>}
      {["draft", "active", "validating"].includes(thesis.status) && <button onClick={() => run(() => close.mutateAsync(thesis.id))} disabled={pending} className="rounded-md border border-input px-3 py-2 text-sm hover:bg-accent">Close thesis</button>}
    </div>
    <div className="border-t border-border pt-4"><h3 className="font-semibold">Confidence history</h3>{confidenceHistory.isLoading ? <LoadingSpinner className="p-4" /> : confidenceHistory.error ? <ErrorState message="Failed to load confidence history." onRetry={() => confidenceHistory.refetch()} /> : <ol className="mt-3 space-y-2">{confidenceHistory.data?.map((snapshot) => <li key={snapshot.id} className="flex items-center justify-between rounded-md bg-muted px-3 py-2 text-sm"><span className="font-medium">{snapshot.confidence}%</span><time className="text-muted-foreground">{new Date(snapshot.recorded_at).toLocaleString()}</time></li>)}</ol>}</div>
    <div className="border-t border-border pt-4"><h3 className="font-semibold">Knowledge links</h3><div className="mt-2 flex gap-2"><select value={knowledgeEntityId} onChange={(event) => setKnowledgeEntityId(event.target.value)} className="min-w-0 flex-1 rounded-md border border-input bg-background px-2 py-2 text-sm"><option value="">Link an entity…</option>{knowledgeEntities.data?.filter((entity) => !knowledgeLinks.data?.some((link) => link.entity_id === entity.id)).map((entity) => <option key={entity.id} value={entity.id}>{entity.name}</option>)}</select><button onClick={() => run(async () => { if (!knowledgeEntityId) throw new Error("Select an entity to link."); await linkKnowledgeEntity.mutateAsync({ thesisId: thesis.id, entityId: knowledgeEntityId }); setKnowledgeEntityId(""); })} className="rounded-md border border-input px-3 py-2 text-sm hover:bg-accent">Link</button></div><ul className="mt-3 space-y-1 text-sm">{knowledgeLinks.data?.map((link) => <li key={link.entity_id} className="rounded bg-muted px-2 py-1">{knowledgeEntities.data?.find((entity) => entity.id === link.entity_id)?.name ?? link.entity_id}</li>)}</ul></div>
    {thesis.status === "validating" && <form onSubmit={(event) => { event.preventDefault(); if (!outcome.trim()) { setError("Record an outcome before completing validation."); return; } void run(() => completeValidation.mutateAsync({ id: thesis.id, outcome: outcome.trim(), validated })); }} className="rounded-md bg-muted/40 p-3"><label htmlFor="validation-outcome" className="mb-1 block text-sm font-medium">Validation outcome</label><textarea id="validation-outcome" value={outcome} onChange={(event) => setOutcome(event.target.value)} rows={2} className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm" /><label className="mt-2 flex items-center gap-2 text-sm"><input type="checkbox" checked={validated} onChange={(event) => setValidated(event.target.checked)} /> Thesis was validated</label><button type="submit" disabled={pending} className="mt-3 rounded-md bg-primary px-3 py-2 text-sm font-medium text-primary-foreground">Complete validation</button></form>}
    {thesis.outcome && <div className="rounded-md bg-muted p-3 text-sm"><span className="font-medium">Outcome: </span>{thesis.outcome}</div>}
    <div className="border-t border-border pt-4"><h3 className="font-semibold">Evidence</h3><form onSubmit={saveEvidence} className="mt-3 space-y-2"><div className="flex gap-2"><select value={direction} onChange={(event) => setDirection(event.target.value as EvidenceDirection)} className="rounded-md border border-input bg-background px-2 text-sm"><option value="supporting">Supporting</option><option value="contradicting">Contradicting</option></select><input value={sourceId} onChange={(event) => setSourceId(event.target.value)} placeholder="Source ID (optional)" className="min-w-0 flex-1 rounded-md border border-input bg-background px-3 py-2 text-sm" /></div><textarea value={evidence} onChange={(event) => setEvidence(event.target.value)} rows={3} placeholder="Add a fact, data point, or argument…" className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm" /><button type="submit" disabled={pending} className="rounded-md border border-input px-3 py-2 text-sm hover:bg-accent">Add evidence</button></form>
      {evidenceQuery.isLoading ? <LoadingSpinner className="p-4" /> : evidenceQuery.error ? <ErrorState message="Failed to load evidence." onRetry={() => evidenceQuery.refetch()} /> : <ul className="mt-4 space-y-2">{evidenceQuery.data?.map((item) => <li key={item.id} className="flex gap-3 rounded-md border border-border p-3 text-sm"><span className={item.direction === "supporting" ? "text-green-600" : "text-destructive"}>{item.direction}</span><p className="flex-1">{item.evidence}{item.source_id && <span className="block text-xs text-muted-foreground">Source: {item.source_id}</span>}</p><button onClick={() => run(() => deleteEvidence.mutateAsync({ id: item.id, thesisId: thesis.id }))} aria-label="Delete evidence" className="text-muted-foreground hover:text-destructive"><Trash2 className="h-4 w-4" /></button></li>)}</ul>}</div>
  </section>;
}
