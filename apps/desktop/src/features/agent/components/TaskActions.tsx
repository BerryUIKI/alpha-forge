// Task action buttons — start and cancel.

import { Play, Square } from "lucide-react";
import { useToast } from "@/components/common/useToast";
import { getErrorMessage } from "@/lib/errors";
import type { AgentTask } from "@/lib/desktop-api/agent";
import { useCancelAgentTask, useRunAgentTask } from "../hooks/useAgentTasks";

interface TaskActionsProps {
  task: AgentTask;
}

export function TaskActions({ task }: TaskActionsProps) {
  const { toast } = useToast();
  const runTask = useRunAgentTask();
  const cancelTask = useCancelAgentTask();

  const canStart = task.status === "created" || task.status === "queued";
  const canCancel =
    task.status === "queued" || task.status === "running" || task.status === "waiting_for_input";
  const isPending = runTask.isPending || cancelTask.isPending;

  if (!canStart && !canCancel) {
    return null;
  }

  const handleStart = () => {
    runTask.mutate(task, {
      onSuccess: () => {
        toast({ title: "Task started", variant: "success" });
      },
      onError: (error) => {
        toast({
          title: "Failed to start task",
          description: getErrorMessage(error),
          variant: "destructive",
        });
      },
    });
  };

  const handleCancel = () => {
    cancelTask.mutate(task.id, {
      onSuccess: (updated) => {
        toast({
          title: updated.status === "cancelled" ? "Task cancelled" : "Cancelling task",
          variant: "success",
        });
      },
      onError: (error) => {
        toast({
          title: "Failed to cancel task",
          description: getErrorMessage(error),
          variant: "destructive",
        });
      },
    });
  };

  return (
    <div className="flex shrink-0 gap-2">
      {canStart && (
        <button
          type="button"
          onClick={handleStart}
          disabled={isPending}
          className="inline-flex items-center gap-1 rounded-md bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground transition-colors hover:bg-primary/90 disabled:opacity-50"
        >
          <Play className="h-3 w-3" />
          Start
        </button>
      )}
      {canCancel && (
        <button
          type="button"
          onClick={handleCancel}
          disabled={isPending}
          className="inline-flex items-center gap-1 rounded-md border border-input bg-background px-3 py-1.5 text-xs font-medium transition-colors hover:bg-accent disabled:opacity-50"
        >
          <Square className="h-3 w-3" />
          Cancel
        </button>
      )}
    </div>
  );
}
