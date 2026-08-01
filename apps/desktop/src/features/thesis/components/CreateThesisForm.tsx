import { useState } from "react";
import { useCreateThesis } from "../hooks/useTheses";

interface CreateThesisFormProps {
  workspaceId: string;
  onCreated: (thesisId: string) => void;
}

export function CreateThesisForm({ workspaceId, onCreated }: CreateThesisFormProps) {
  const [title, setTitle] = useState("");
  const [thesis, setThesis] = useState("");
  const [confidence, setConfidence] = useState(50);
  const [error, setError] = useState("");
  const createThesis = useCreateThesis();

  async function handleSubmit(event: React.FormEvent) {
    event.preventDefault();
    if (!title.trim() || !thesis.trim()) {
      setError("A title and thesis statement are required.");
      return;
    }

    try {
      const created = await createThesis.mutateAsync({
        workspaceId,
        title: title.trim(),
        thesis: thesis.trim(),
        confidence,
      });
      setTitle("");
      setThesis("");
      setConfidence(50);
      setError("");
      onCreated(created.id);
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : "Unable to create the thesis.");
    }
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-4 rounded-lg border border-border bg-card p-4">
      <div>
        <h2 className="text-lg font-semibold">New investment thesis</h2>
        <p className="text-sm text-muted-foreground">Capture a testable claim before you start tracking evidence.</p>
      </div>
      <div>
        <label htmlFor="thesis-title" className="mb-1 block text-sm font-medium">Title</label>
        <input id="thesis-title" value={title} onChange={(event) => setTitle(event.target.value)} placeholder="AI infrastructure demand remains durable" className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
      </div>
      <div>
        <label htmlFor="thesis-statement" className="mb-1 block text-sm font-medium">Thesis statement</label>
        <textarea id="thesis-statement" value={thesis} onChange={(event) => setThesis(event.target.value)} rows={4} placeholder="State the claim, why it may be true, and what could disprove it." className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
      </div>
      <div>
        <label htmlFor="initial-confidence" className="mb-1 flex justify-between text-sm font-medium"><span>Initial confidence</span><span>{confidence}%</span></label>
        <input id="initial-confidence" type="range" min="0" max="100" value={confidence} onChange={(event) => setConfidence(Number(event.target.value))} className="w-full" />
      </div>
      {error && <p className="text-sm text-destructive">{error}</p>}
      <button type="submit" disabled={createThesis.isPending} className="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground disabled:opacity-50">
        {createThesis.isPending ? "Creating…" : "Create thesis"}
      </button>
    </form>
  );
}
