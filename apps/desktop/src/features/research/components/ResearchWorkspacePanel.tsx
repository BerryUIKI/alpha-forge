// Workspace research documents panel.

import { useState } from "react";
import { FileText, Plus } from "lucide-react";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { EmptyState } from "@/components/common/EmptyState";
import { ErrorState } from "@/components/common/ErrorState";
import type { ResearchDocument } from "@/lib/desktop-api";
import { useResearchDocuments } from "../hooks/useResearchDocuments";
import { ResearchDocumentEditor } from "./ResearchDocumentEditor";
import { ResearchDocumentList } from "./ResearchDocumentList";

interface ResearchWorkspacePanelProps {
  workspaceId: string;
}

export function ResearchWorkspacePanel({ workspaceId }: ResearchWorkspacePanelProps) {
  const { data: documents, isLoading, error, refetch } = useResearchDocuments(workspaceId);
  const [selectedDocumentId, setSelectedDocumentId] = useState<string | null>(null);
  const [isCreating, setIsCreating] = useState(false);

  const selectedDocument =
    documents?.find((document: ResearchDocument) => document.id === selectedDocumentId) ?? null;

  const handleNew = () => {
    setIsCreating(true);
    setSelectedDocumentId(null);
  };

  const handleSaved = (document: ResearchDocument) => {
    setIsCreating(false);
    setSelectedDocumentId(document.id);
  };

  return (
    <section className="space-y-4">
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div>
          <div className="flex items-center gap-2">
            <FileText className="h-5 w-5 text-primary" />
            <h2 className="text-lg font-semibold">Research Documents</h2>
          </div>
          <p className="mt-1 text-sm text-muted-foreground">
            Capture notes, findings, and source excerpts for this workspace.
          </p>
        </div>
        <button
          type="button"
          onClick={handleNew}
          className="inline-flex items-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90"
        >
          <Plus className="mr-2 h-4 w-4" />
          New Document
        </button>
      </div>

      {isLoading ? (
        <LoadingSpinner className="p-8" />
      ) : error ? (
        <ErrorState message="Failed to load research documents" onRetry={() => refetch()} />
      ) : isCreating ? (
        <ResearchDocumentEditor
          key="new"
          mode="create"
          workspaceId={workspaceId}
          onSaved={handleSaved}
          onCancel={() => {
            setIsCreating(false);
          }}
        />
      ) : !documents || documents.length === 0 ? (
        <EmptyState
          icon={<FileText className="h-8 w-8" />}
          title="No documents yet"
          description="Create your first research document to start collecting notes and findings."
          action={
            <button
              type="button"
              onClick={handleNew}
              className="inline-flex items-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90"
            >
              <Plus className="mr-2 h-4 w-4" />
              Create Document
            </button>
          }
        />
      ) : (
        <div
          className={
            selectedDocument
              ? "grid gap-4 xl:grid-cols-[minmax(0,1fr)_minmax(0,1.4fr)]"
              : "max-w-3xl"
          }
        >
          <ResearchDocumentList
            documents={documents}
            selectedDocumentId={selectedDocumentId ?? undefined}
            onSelect={(document) => {
              setIsCreating(false);
              setSelectedDocumentId(document.id);
            }}
          />
          {selectedDocument && (
            <ResearchDocumentEditor
              key={selectedDocument.id}
              mode="edit"
              workspaceId={workspaceId}
              document={selectedDocument}
              onSaved={handleSaved}
              onDeleted={() => {
                setSelectedDocumentId(null);
              }}
              onCancel={() => {
                setSelectedDocumentId(null);
              }}
            />
          )}
        </div>
      )}
    </section>
  );
}
