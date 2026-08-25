// Task status badge component.

import { Badge } from "@alpha-forge/ui";
import { useLocale } from "@/lib/i18n/useLocale";
import type { TaskStatus } from "@/lib/desktop-api/agent";

const statusKeys: Record<TaskStatus, { labelKey: "taskStatusCreated" | "taskStatusQueued" | "taskStatusRunning" | "taskStatusWaiting" | "taskStatusCompleted" | "taskStatusFailed" | "taskStatusCancelled"; variant: "default" | "secondary" | "destructive" | "outline" }> = {
  created: { labelKey: "taskStatusCreated", variant: "outline" },
  queued: { labelKey: "taskStatusQueued", variant: "secondary" },
  running: { labelKey: "taskStatusRunning", variant: "default" },
  waiting_for_input: { labelKey: "taskStatusWaiting", variant: "outline" },
  completed: { labelKey: "taskStatusCompleted", variant: "default" },
  failed: { labelKey: "taskStatusFailed", variant: "destructive" },
  cancelled: { labelKey: "taskStatusCancelled", variant: "outline" },
};

interface TaskStatusBadgeProps {
  status: TaskStatus;
}

export function TaskStatusBadge({ status }: TaskStatusBadgeProps) {
  const { t } = useLocale();
  const config = statusKeys[status];

  return (
    <Badge variant={config.variant}>
      {t(config.labelKey)}
    </Badge>
  );
}