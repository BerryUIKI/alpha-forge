/**
 * Create Research Project Dialog
 *
 * Modal dialog for creating a new research project within a workspace.
 *
 * @module features/research/components/CreateResearchProjectDialog
 */

import { useState } from "react";
import { X } from "lucide-react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import { useLocale } from "@/lib/i18n/useLocale";

interface CreateResearchProjectDialogProps {
  isOpen: boolean;
  onClose: () => void;
  workspaceId: string;
  onSuccess?: (project: { id: string; title: string }) => void;
}

export function CreateResearchProjectDialog({
  isOpen,
  onClose,
  workspaceId,
  onSuccess,
}: CreateResearchProjectDialogProps) {
  const [title, setTitle] = useState("");
  const [error, setError] = useState("");
  const { t } = useLocale();
  const queryClient = useQueryClient();

  const createMutation = useMutation({
    mutationFn: () => desktopApi.research.createResearchProject(workspaceId, title.trim()),
    onSuccess: (project) => {
      setTitle("");
      setError("");
      queryClient.invalidateQueries({ queryKey: ["research", "projects", workspaceId] });
      onSuccess?.({ id: project.id, title: project.title });
      onClose();
    },
    onError: (err) => {
      setError(err instanceof Error && err.message ? err.message : t("failedToCreateProject"));
    },
  });

  if (!isOpen) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const trimmedTitle = title.trim();
    if (!trimmedTitle) {
      setError(t("projectTitleRequired"));
      return;
    }

    if (trimmedTitle.length > 200) {
      setError(t("projectTitleTooLong"));
      return;
    }

    await createMutation.mutateAsync();
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
      role="dialog"
      aria-modal="true"
      aria-labelledby="dialog-title"
    >
      <div className="w-full max-w-md rounded-lg bg-card p-6 shadow-xl">
        <div className="mb-4 flex items-center justify-between">
          <h2 id="dialog-title" className="text-lg font-semibold">
            {t("createNewProject")}
          </h2>
          <button
            onClick={onClose}
            className="rounded-md p-1 hover:bg-accent"
            aria-label={t("cancel")}
          >
            <X className="h-5 w-5" />
          </button>
        </div>

        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label htmlFor="project-title" className="mb-2 block text-sm font-medium">
              {t("projectTitle")}
            </label>
            <input
              id="project-title"
              type="text"
              value={title}
              onChange={(e) => {
                setTitle(e.target.value);
                setError("");
              }}
              placeholder={t("projectTitle")}
              className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
              autoFocus
            />
            {error && <p className="mt-1 text-sm text-destructive" role="alert">{error}</p>}
          </div>

          <div className="flex justify-end gap-2">
            <button
              type="button"
              onClick={onClose}
              className="rounded-md border border-input bg-background px-4 py-2 text-sm font-medium hover:bg-accent"
            >
              {t("cancel")}
            </button>
            <button
              type="submit"
              disabled={createMutation.isPending || !title.trim()}
              className="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
            >
              {createMutation.isPending ? t("creating") : t("create")}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
