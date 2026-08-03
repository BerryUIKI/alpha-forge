// Agent task list component.

import { Clock } from "lucide-react";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { EmptyState } from "@/components/common/EmptyState";
import { ErrorState } from "@/components/common/ErrorState";
import { TaskStatusBadge } from "./TaskStatusBadge";
import type { AgentTask } from "@/lib/desktop-api/agent";
import { useAgentTasks } from "../hooks/useAgentTasks";
import { useLocale } from "@/lib/i18n/useLocale";
import { formatMessage } from "@/lib/i18n/locale";

interface AgentTaskListProps {
  workspaceId: string;
  onSelectTask?: (task: AgentTask) => void;
}

export function AgentTaskList({ workspaceId, onSelectTask }: AgentTaskListProps) {
  const { t } = useLocale();
  const { isLoading, error, data: tasks, refetch } = useAgentTasks(workspaceId);

  if (isLoading) {
    return <LoadingSpinner className="p-8" />;
  }

  if (error) {
    return (
      <ErrorState
        message={t("failedToLoadAgentTasks")}
        onRetry={() => refetch()}
      />
    );
  }

  if (!tasks || tasks.length === 0) {
    return (
      <EmptyState
        icon={<Clock className="h-8 w-8" />}
        title={t("noTasksYet")}
        description={t("noTasksDescription")}
      />
    );
  }

  return (
    <div className="space-y-3">
      {tasks.map((task: AgentTask) => (
        <button
          key={task.id}
          onClick={() => onSelectTask?.(task)}
          className="w-full rounded-lg border border-border bg-card p-4 text-left transition-colors hover:border-primary hover:bg-accent"
        >
          <div className="flex items-start justify-between">
            <div className="flex-1">
              <h3 className="font-semibold">{task.title}</h3>
              {task.description && (
                <p className="mt-1 text-sm text-muted-foreground line-clamp-2">
                  {task.description}
                </p>
              )}
              <p className="mt-2 text-xs text-muted-foreground">
                {formatMessage(t("created"), { date: new Date(task.created_at).toLocaleDateString() })}
              </p>
            </div>
            <TaskStatusBadge status={task.status} />
          </div>
        </button>
      ))}
    </div>
  );
}