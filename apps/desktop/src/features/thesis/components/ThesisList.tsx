import { BookOpen } from "lucide-react";
import { EmptyState, ErrorState, LoadingSpinner } from "@/components/common";
import type { InvestmentThesis } from "@/lib/desktop-api/thesis";
import { useTheses } from "../hooks/useTheses";

interface ThesisListProps {
  workspaceId: string;
  selectedId?: string;
  onSelect: (thesis: InvestmentThesis) => void;
}

export function ThesisList({ workspaceId, selectedId, onSelect }: ThesisListProps) {
  const { data, isLoading, error, refetch } = useTheses(workspaceId);
  if (isLoading) return <LoadingSpinner className="p-8" />;
  if (error) return <ErrorState message="Failed to load theses." onRetry={() => refetch()} />;
  if (!data?.length) return <EmptyState icon={<BookOpen className="h-8 w-8" />} title="No theses yet" description="Create a thesis to begin preserving your investment reasoning." />;

  return <div className="space-y-2">{data.map((thesis) => <button key={thesis.id} onClick={() => onSelect(thesis)} className={`w-full rounded-lg border p-3 text-left transition-colors ${selectedId === thesis.id ? "border-primary bg-accent" : "border-border bg-card hover:bg-accent"}`}>
    <div className="flex items-start justify-between gap-3"><span className="font-medium">{thesis.title}</span><span className="rounded-full bg-muted px-2 py-0.5 text-xs capitalize">{thesis.status}</span></div>
    <div className="mt-2 h-2 overflow-hidden rounded bg-muted"><div className="h-full bg-primary" style={{ width: `${thesis.confidence}%` }} /></div>
    <p className="mt-1 text-xs text-muted-foreground">{thesis.confidence}% confidence</p>
  </button>)}</div>;
}
