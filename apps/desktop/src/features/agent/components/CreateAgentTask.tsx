// Create agent task component.

import { useState } from "react";
import { Plus, X } from "lucide-react";

interface CreateAgentTaskProps {
  workspaceId: string;
  onSuccess?: (taskId: string) => void;
  onCancel?: () => void;
}

export function CreateAgentTask({ workspaceId, onSuccess, onCancel }: CreateAgentTaskProps) {
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [error, setError] = useState("");
  const [isOpen, setIsOpen] = useState(false);

  const { useCreateAgentTask } = require("../hooks/useAgentTasks");
  const createMutation = useCreateAgentTask();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const trimmedTitle = title.trim();
    if (!trimmedTitle) {
      setError("Task title is required");
      return;
    }

    try {
      const task = await createMutation.mutateAsync({
        workspaceId,
        title: trimmedTitle,
        description: description.trim() || undefined,
      });
      
      setTitle("");
      setDescription("");
      setError("");
      setIsOpen(false);
      onSuccess?.(task.id);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to create task");
    }
  };

  if (!isOpen) {
    return (
      <button
        onClick={() => setIsOpen(true)}
        className="inline-flex items-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
      >
        <Plus className="mr-2 h-4 w-4" />
        New Task
      </button>
    );
  }

  return (
    <div className="rounded-lg border border-border bg-card p-4">
      <div className="mb-4 flex items-center justify-between">
        <h3 className="text-lg font-semibold">Create Agent Task</h3>
        <button
          onClick={() => {
            setIsOpen(false);
            onCancel?.();
          }}
          className="rounded-md p-1 hover:bg-accent"
        >
          <X className="h-5 w-5" />
        </button>
      </div>

      <form onSubmit={handleSubmit}>
        <div className="mb-4">
          <label htmlFor="task-title" className="mb-2 block text-sm font-medium">
            Title
          </label>
          <input
            id="task-title"
            type="text"
            value={title}
            onChange={(e) => {
              setTitle(e.target.value);
              setError("");
            }}
            placeholder="Research Tesla's Q4 earnings"
            className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
            autoFocus
          />
        </div>

        <div className="mb-4">
          <label htmlFor="task-description" className="mb-2 block text-sm font-medium">
            Description (optional)
          </label>
          <textarea
            id="task-description"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            placeholder="Provide more details about what you want to research..."
            rows={3}
            className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
          />
        </div>

        {error && <p className="mb-4 text-sm text-destructive">{error}</p>}

        <div className="flex justify-end gap-2">
          <button
            type="button"
            onClick={() => {
              setIsOpen(false);
              onCancel?.();
            }}
            className="rounded-md border border-input bg-background px-4 py-2 text-sm font-medium hover:bg-accent"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={createMutation.isPending}
            className="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
          >
            {createMutation.isPending ? "Creating..." : "Create Task"}
          </button>
        </div>
      </form>
    </div>
  );
}