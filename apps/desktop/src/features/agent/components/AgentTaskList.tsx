// Agent task list component.

import { Clock } from "lucide-react";
import { cn } from "@investment-os/ui";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { EmptyState } from "@/components/common/EmptyState";
import { ErrorState } from "@/components/common/ErrorState";
import { TaskStatusBadge } from "./TaskStatusBadge";
import { TaskActions } from "./TaskActions";
import type { AgentTask } from "@/lib/desktop-api/agent";
import { useAgentTasks } from "../hooks/useAgentTasks";

interface AgentTaskListProps {
  workspaceId: string;
  selectedTaskId?: string;
  onSelectTask?: (task: AgentTask) => void;
}

export function AgentTaskList({ workspaceId, selectedTaskId, onSelectTask }: AgentTaskListProps) {
  const { isLoading, error, data: tasks, refetch } = useAgentTasks(workspaceId);

  if (isLoading) {
    return <LoadingSpinner className="p-8" />;
  }

  if (error) {
    return <ErrorState message="Failed to load agent tasks" onRetry={() => refetch()} />;
  }

  if (!tasks || tasks.length === 0) {
    return (
      <EmptyState
        icon={<Clock className="h-8 w-8" />}
        title="No tasks yet"
        description="Create your first agent task to start researching."
      />
    );
  }

  return (
    <div className="space-y-3">
      {tasks.map((task: AgentTask) => {
        const isSelected = selectedTaskId === task.id;

        return (
          <div
            key={task.id}
            className={cn(
              "rounded-lg border bg-card p-4 transition-colors",
              isSelected ? "border-primary" : "border-border",
            )}
          >
            <div className="flex items-start justify-between gap-3">
              <button
                type="button"
                onClick={() => onSelectTask?.(task)}
                aria-pressed={isSelected}
                className="min-w-0 flex-1 text-left"
              >
                <h3 className="font-semibold">{task.title}</h3>
                {task.description && (
                  <p className="mt-1 line-clamp-2 text-sm text-muted-foreground">
                    {task.description}
                  </p>
                )}
                <p className="mt-2 text-xs text-muted-foreground">
                  Created {new Date(task.created_at).toLocaleDateString()}
                </p>
                {task.error && <p className="mt-1 text-xs text-destructive">{task.error}</p>}
              </button>
              <div className="flex shrink-0 flex-col items-end gap-2">
                <TaskStatusBadge status={task.status} />
                <TaskActions task={task} />
              </div>
            </div>
          </div>
        );
      })}
    </div>
  );
}
