// Create workspace dialog component.

import { useState } from "react";
import { X } from "lucide-react";
import { useCreateWorkspace } from "@/features/workspace/hooks/useWorkspaces";
import { useLocale } from "@/lib/i18n/useLocale";

interface CreateWorkspaceDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess?: (workspace: { id: string; name: string }) => void;
}

export function CreateWorkspaceDialog({ isOpen, onClose, onSuccess }: CreateWorkspaceDialogProps) {
  const [name, setName] = useState("");
  const [error, setError] = useState("");
  const createMutation = useCreateWorkspace();
  const { t } = useLocale();

  if (!isOpen) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const trimmedName = name.trim();
    if (!trimmedName) {
      setError(t("workspaceNameRequired"));
      return;
    }

    if (trimmedName.length > 200) {
      setError(t("workspaceNameTooLong"));
      return;
    }

    try {
      const workspace = await createMutation.mutateAsync(trimmedName);
      setName("");
      setError("");
      onSuccess?.({ id: workspace.id, name: workspace.name });
      onClose();
    } catch (err) {
      setError(err instanceof Error ? err.message : t("failedToCreateWorkspace"));
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50" role="dialog" aria-modal="true" aria-labelledby="dialog-title">
      <div className="w-full max-w-md rounded-lg bg-card p-6 shadow-xl">
        <div className="mb-4 flex items-center justify-between">
          <h2 id="dialog-title" className="text-lg font-semibold">{t("createWorkspaceTitle")}</h2>
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
            <label htmlFor="workspace-name" className="mb-2 block text-sm font-medium">
              {t("workspaceName")}
            </label>
            <input
              id="workspace-name"
              type="text"
              value={name}
              onChange={(e) => {
                setName(e.target.value);
                setError("");
              }}
              placeholder={t("workspaceNamePlaceholder")}
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
              disabled={createMutation.isPending}
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