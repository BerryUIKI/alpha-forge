// Research document editor.

import { useState } from "react";
import { Badge } from "@investment-os/ui";
import { useToast } from "@/components/common/useToast";
import { getErrorMessage } from "@/lib/errors";
import type { ResearchDocument } from "@/lib/desktop-api";
import {
  useCreateResearchDocument,
  useDeleteResearchDocument,
  useUpdateResearchDocument,
} from "../hooks/useResearchDocuments";
import { MarkdownPreview } from "./MarkdownPreview";
import { researchKindLabel } from "./researchLabels";

interface ResearchDocumentEditorProps {
  mode: "create" | "edit";
  workspaceId: string;
  document?: ResearchDocument | null;
  onSaved: (document: ResearchDocument) => void;
  onDeleted?: () => void;
  onCancel?: () => void;
}

export function ResearchDocumentEditor({
  mode,
  workspaceId,
  document,
  onSaved,
  onDeleted,
  onCancel,
}: ResearchDocumentEditorProps) {
  const { toast } = useToast();
  const createDocument = useCreateResearchDocument();
  const updateDocument = useUpdateResearchDocument();
  const deleteDocument = useDeleteResearchDocument();

  const [title, setTitle] = useState(document?.title ?? "");
  const [content, setContent] = useState(document?.content ?? "");
  const [error, setError] = useState("");
  const [confirmingDelete, setConfirmingDelete] = useState(false);
  const [view, setView] = useState<"edit" | "preview">("edit");

  const isSaving = createDocument.isPending || updateDocument.isPending;

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();

    const trimmedTitle = title.trim();
    if (!trimmedTitle) {
      setError("Document title is required");
      return;
    }

    try {
      const saved =
        mode === "create"
          ? await createDocument.mutateAsync({ workspaceId, title: trimmedTitle, content })
          : await updateDocument.mutateAsync({
              id: document?.id ?? "",
              title: trimmedTitle,
              content,
            });

      setError("");
      toast({
        title: mode === "create" ? "Document created" : "Document saved",
        variant: "success",
      });
      onSaved(saved);
    } catch (err) {
      toast({
        title: "Failed to save document",
        description: getErrorMessage(err),
        variant: "destructive",
      });
    }
  };

  const handleDelete = async () => {
    if (!document) {
      return;
    }

    if (!confirmingDelete) {
      setConfirmingDelete(true);
      return;
    }

    try {
      await deleteDocument.mutateAsync(document.id);
      toast({ title: "Document deleted", variant: "success" });
      onDeleted?.();
    } catch (err) {
      toast({
        title: "Failed to delete document",
        description: getErrorMessage(err),
        variant: "destructive",
      });
    }
  };

  return (
    <div className="rounded-lg border border-border bg-card p-4">
      <div className="mb-4 flex items-center justify-between">
        <div className="flex items-center gap-2">
          <h3 className="text-lg font-semibold">
            {mode === "create" ? "New document" : "Edit document"}
          </h3>
          <Badge variant="outline">{researchKindLabel(document?.kind ?? "note")}</Badge>
        </div>
        {onCancel && (
          <button
            type="button"
            onClick={onCancel}
            className="text-sm text-muted-foreground hover:text-foreground"
          >
            Cancel
          </button>
        )}
      </div>

      <form onSubmit={handleSubmit}>
        <div className="mb-4">
          <label htmlFor="document-title" className="mb-2 block text-sm font-medium">
            Title
          </label>
          <input
            id="document-title"
            type="text"
            value={title}
            onChange={(event) => {
              setTitle(event.target.value);
              setError("");
            }}
            placeholder="Tesla Q4 earnings"
            className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
          />
        </div>

        <div className="mb-4">
          <div className="mb-2 flex items-center justify-between">
            <label htmlFor="document-content" className="block text-sm font-medium">
              Content
            </label>
            <div className="flex gap-1">
              <button
                type="button"
                onClick={() => setView("edit")}
                aria-pressed={view === "edit"}
                className={`rounded-md px-2 py-1 text-xs font-medium ${
                  view === "edit" ? "bg-accent" : "text-muted-foreground hover:bg-accent"
                }`}
              >
                Edit
              </button>
              <button
                type="button"
                onClick={() => setView("preview")}
                aria-pressed={view === "preview"}
                className={`rounded-md px-2 py-1 text-xs font-medium ${
                  view === "preview" ? "bg-accent" : "text-muted-foreground hover:bg-accent"
                }`}
              >
                Preview
              </button>
            </div>
          </div>
          {view === "edit" ? (
            <textarea
              id="document-content"
              value={content}
              onChange={(event) => {
                setContent(event.target.value);
              }}
              placeholder="Notes, findings, and source excerpts..."
              rows={12}
              className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
            />
          ) : (document?.format ?? "markdown") === "text" ? (
            <pre className="whitespace-pre-wrap rounded-md border border-border bg-background p-3 text-sm">
              {content}
            </pre>
          ) : (
            <MarkdownPreview content={content} />
          )}
        </div>

        {error && <p className="mb-4 text-sm text-destructive">{error}</p>}

        <div className="flex flex-wrap items-center justify-between gap-2">
          <div>
            {mode === "edit" && (
              <button
                type="button"
                onClick={handleDelete}
                disabled={deleteDocument.isPending}
                className="rounded-md border border-destructive/50 px-3 py-1.5 text-xs font-medium text-destructive transition-colors hover:bg-destructive/10 disabled:opacity-50"
              >
                {deleteDocument.isPending
                  ? "Deleting..."
                  : confirmingDelete
                    ? "Confirm delete"
                    : "Delete"}
              </button>
            )}
          </div>
          <button
            type="submit"
            disabled={isSaving}
            className="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90 disabled:opacity-50"
          >
            {isSaving ? "Saving..." : mode === "create" ? "Create document" : "Save changes"}
          </button>
        </div>
      </form>
    </div>
  );
}
