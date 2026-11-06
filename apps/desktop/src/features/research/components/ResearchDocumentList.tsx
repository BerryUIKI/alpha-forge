// Research document list.

import { cn } from "@investment-os/ui";
import type { ResearchDocument } from "@/lib/desktop-api";

interface ResearchDocumentListProps {
  documents: ResearchDocument[];
  selectedDocumentId?: string;
  onSelect: (document: ResearchDocument) => void;
}

export function ResearchDocumentList({
  documents,
  selectedDocumentId,
  onSelect,
}: ResearchDocumentListProps) {
  return (
    <div className="space-y-3">
      {documents.map((document) => {
        const isSelected = selectedDocumentId === document.id;

        return (
          <button
            key={document.id}
            type="button"
            onClick={() => onSelect(document)}
            aria-pressed={isSelected}
            className={cn(
              "w-full rounded-lg border bg-card p-4 text-left transition-colors hover:border-primary hover:bg-accent",
              isSelected ? "border-primary" : "border-border",
            )}
          >
            <h3 className="truncate font-semibold">{document.title}</h3>
            <p className="mt-1 text-xs text-muted-foreground">
              Updated {new Date(document.updated_at).toLocaleDateString()}
            </p>
          </button>
        );
      })}
    </div>
  );
}
