// Agent task detail — status, live execution timeline, and result.

import { useEffect, useRef } from "react";
import {
  AlertCircle,
  Ban,
  CheckCircle2,
  Circle,
  Loader2,
  MessageSquare,
  PlayCircle,
  X,
} from "lucide-react";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { ErrorState } from "@/components/common/ErrorState";
import { Badge } from "@investment-os/ui";
import type { AgentTaskEvent, TaskEventType } from "@/lib/desktop-api/agent";
import { useAgentTask, useTaskEvents } from "../hooks/useAgentTasks";
import { parseCitations } from "./citations";
import { TaskStatusBadge } from "./TaskStatusBadge";

const eventLabels: Record<TaskEventType, string> = {
  task_created: "Task created",
  task_queued: "Queued for execution",
  task_started: "Execution started",
  task_progress: "Progress",
  task_waiting_for_input: "Waiting for input",
  task_completed: "Completed",
  task_failed: "Failed",
  task_cancelled: "Cancelled",
};

function EventIcon({ type }: { type: TaskEventType }) {
  switch (type) {
    case "task_completed":
      return <CheckCircle2 className="h-4 w-4 text-green-600" />;
    case "task_failed":
      return <AlertCircle className="h-4 w-4 text-destructive" />;
    case "task_cancelled":
      return <Ban className="h-4 w-4 text-muted-foreground" />;
    case "task_started":
      return <PlayCircle className="h-4 w-4 text-primary" />;
    case "task_progress":
      return <Loader2 className="h-4 w-4 text-primary" />;
    case "task_waiting_for_input":
      return <MessageSquare className="h-4 w-4 text-amber-500" />;
    default:
      return <Circle className="h-4 w-4 text-muted-foreground" />;
  }
}

function formatPayload(event: AgentTaskEvent): string | null {
  if (event.event_type === "task_progress" || event.event_type === "task_failed") {
    return event.payload;
  }
  return null;
}

function formatTimestamp(value: string): string {
  return new Date(value).toLocaleTimeString([], {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });
}

function formatOutput(output: string): string {
  try {
    return JSON.stringify(JSON.parse(output), null, 2);
  } catch {
    return output;
  }
}

interface AgentTaskDetailProps {
  taskId: string;
  onClose?: () => void;
}

export function AgentTaskDetail({ taskId, onClose }: AgentTaskDetailProps) {
  const taskQuery = useAgentTask(taskId);
  const eventsQuery = useTaskEvents(taskId);
  const timelineEndRef = useRef<HTMLDivElement>(null);

  const events = eventsQuery.data;

  useEffect(() => {
    timelineEndRef.current?.scrollIntoView?.({ block: "nearest" });
  }, [events?.length]);

  if (taskQuery.isLoading) {
    return <LoadingSpinner className="p-8" />;
  }

  if (taskQuery.error) {
    return <ErrorState message="Failed to load task" onRetry={() => taskQuery.refetch()} />;
  }

  const task = taskQuery.data;

  if (!task) {
    return (
      <div className="rounded-lg border border-border bg-card p-6 text-sm text-muted-foreground">
        Task not found.
      </div>
    );
  }

  const citations = parseCitations(task.output);

  return (
    <div className="rounded-lg border border-border bg-card">
      <div className="flex items-start justify-between gap-3 border-b border-border p-4">
        <div className="min-w-0">
          <h3 className="truncate font-semibold">{task.title}</h3>
          <p className="mt-1 text-xs text-muted-foreground">
            Created {new Date(task.created_at).toLocaleString()}
          </p>
        </div>
        <div className="flex shrink-0 items-center gap-2">
          <TaskStatusBadge status={task.status} />
          {onClose && (
            <button
              type="button"
              onClick={onClose}
              aria-label="Close task detail"
              className="rounded-md p-1 text-muted-foreground hover:bg-accent hover:text-accent-foreground"
            >
              <X className="h-4 w-4" />
            </button>
          )}
        </div>
      </div>

      <div className="space-y-5 p-4">
        {task.description && <p className="text-sm text-muted-foreground">{task.description}</p>}

        <section>
          <h4 className="mb-3 text-sm font-semibold">Execution timeline</h4>
          {eventsQuery.isLoading && <LoadingSpinner size="sm" className="p-2" />}
          {eventsQuery.error && (
            <div className="flex items-center gap-2 text-sm text-muted-foreground">
              <span>Failed to load task events.</span>
              <button
                type="button"
                onClick={() => eventsQuery.refetch()}
                className="text-primary hover:underline"
              >
                Retry
              </button>
            </div>
          )}
          {!eventsQuery.isLoading && !eventsQuery.error && (!events || events.length === 0) && (
            <p className="text-sm text-muted-foreground">No events recorded yet.</p>
          )}
          {events && events.length > 0 && (
            <ol className="max-h-72 space-y-3 overflow-y-auto pr-1">
              {events.map((event) => {
                const detail = formatPayload(event);
                return (
                  <li key={event.id} className="flex items-start gap-2">
                    <div className="mt-0.5 shrink-0">
                      <EventIcon type={event.event_type} />
                    </div>
                    <div className="min-w-0 flex-1">
                      <div className="flex items-baseline justify-between gap-2">
                        <span className="text-sm">{eventLabels[event.event_type]}</span>
                        <span className="shrink-0 text-xs text-muted-foreground">
                          {formatTimestamp(event.created_at)}
                        </span>
                      </div>
                      {detail && (
                        <p className="mt-0.5 break-words text-xs text-muted-foreground">{detail}</p>
                      )}
                    </div>
                  </li>
                );
              })}
            </ol>
          )}
          <div ref={timelineEndRef} />
        </section>

        {task.output && (
          <section>
            <h4 className="mb-2 text-sm font-semibold">Output</h4>
            <pre className="max-h-80 overflow-auto rounded-md bg-muted p-3 text-xs">
              {formatOutput(task.output)}
            </pre>
          </section>
        )}

        {citations.length > 0 && (
          <section>
            <h4 className="mb-2 text-sm font-semibold">Citations</h4>
            <ul className="space-y-1">
              {citations.map((citation) => (
                <li
                  key={`${citation.kind}-${citation.id}`}
                  className="flex flex-wrap items-center gap-2 text-sm"
                >
                  <Badge variant="outline">
                    {citation.kind === "document" ? "Document" : "Task"}
                  </Badge>
                  <span>{citation.title}</span>
                  <code className="text-xs text-muted-foreground">{citation.id}</code>
                </li>
              ))}
            </ul>
          </section>
        )}

        {task.error && (
          <section>
            <h4 className="mb-2 text-sm font-semibold text-destructive">Error</h4>
            <p className="rounded-md border border-destructive/40 bg-destructive/10 p-3 text-sm break-words text-destructive">
              {task.error}
            </p>
          </section>
        )}
      </div>
    </div>
  );
}
