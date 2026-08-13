/**
 * AgentPanel Component
 *
 * Right sidebar content for Agent task management.
 * Connected to backend via TanStack Query hooks.
 *
 * @version GUI-M2
 */

import { useState } from "react";
import { Bot, X, Play, Square, AlertCircle, Settings, Activity } from "lucide-react";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { ErrorState } from "@/components/common/ErrorState";
import { useWorkspaces } from "@/features/workspace/hooks/useWorkspaces";
import { AgentTaskList } from "@/features/agent/components/AgentTaskList";
import { CreateAgentTask } from "@/features/agent/components/CreateAgentTask";
import {
  useAgentTask,
  useRunAgentTask,
  useCancelAgentTask,
} from "@/features/agent/hooks/useAgentTasks";
import { useAgentStatus } from "@/hooks/useAgentStatus";
import { AgentConfigGuide } from "@/features/agent/components/AgentConfigGuide";
import { TaskStatusBadge } from "@/features/agent/components/TaskStatusBadge";
import type { AgentTask } from "@/lib/desktop-api/agent";
import { useLocale } from "@/lib/i18n/useLocale";
import type { AgentConnectionStatus } from "@/components/layout/types";

interface AgentPanelProps {
  status?: string;
  placeholder?: string;
}

/**
 * Status indicator component with 4 states:
 * - idle (gray): 空闲待命
 * - running (blue blinking): 任务执行中
 * - unconfigured (yellow): 需要完成助手配置
 * - error (red): 连接失败
 */
function AgentStatusIndicator({ status }: { status: AgentConnectionStatus }) {
  const { t } = useLocale();

  const statusConfig = {
    idle: {
      color: "bg-gray-400",
      text: t("statusIdle"),
      icon: Bot,
      animate: "",
    },
    running: {
      color: "bg-blue-500",
      text: t("statusRunning"),
      icon: Activity,
      animate: "animate-pulse",
    },
    unconfigured: {
      color: "bg-yellow-500",
      text: t("statusUnconfigured"),
      icon: Settings,
      animate: "",
    },
    error: {
      color: "bg-red-500",
      text: t("statusError"),
      icon: AlertCircle,
      animate: "",
    },
  };

  const config = statusConfig[status];
  const Icon = config.icon;

  return (
    <div className="flex items-center gap-2">
      <div className={`h-2 w-2 rounded-full ${config.color} ${config.animate}`} />
      <Icon className="h-4 w-4 text-muted-foreground" />
      <span className="text-xs font-medium">{config.text}</span>
    </div>
  );
}

export function AgentPanel({ status: _status = "Ready" }: AgentPanelProps) {
  const { t } = useLocale();
  const {
    data: workspaces,
    isLoading: workspacesLoading,
    error: workspacesError,
  } = useWorkspaces();
  const [selectedTaskId, setSelectedTaskId] = useState<string | null>(null);
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [showConfigGuide, setShowConfigGuide] = useState(false);

  // Use first workspace as default
  const workspaceId = workspaces?.[0]?.id || "";

  // Get agent status
  const { status: agentStatus } = useAgentStatus(workspaceId);

  // Task actions
  const startTask = useRunAgentTask();
  const cancelTask = useCancelAgentTask();

  // Selected task details
  const { data: selectedTask } = useAgentTask(selectedTaskId || "");

  // Handle create button click
  const handleCreateClick = () => {
    if (agentStatus === "unconfigured" || agentStatus === "error") {
      setShowConfigGuide(true);
    } else {
      setShowCreateForm(true);
    }
  };

  const handleTaskSelect = (task: AgentTask) => {
    startTask.reset();
    cancelTask.reset();
    setSelectedTaskId(task.id);
  };

  const handleTaskCreated = (taskId: string) => {
    setSelectedTaskId(taskId);
    setShowCreateForm(false);
  };

  // Loading state
  if (workspacesLoading) {
    return <LoadingSpinner className="p-8" />;
  }

  // Error state
  if (workspacesError) {
    return (
      <div className="p-4">
        <ErrorState message="Failed to load workspaces" onRetry={() => window.location.reload()} />
      </div>
    );
  }

  // No workspace state
  if (!workspaceId) {
    return (
      <div className="flex h-full flex-col items-center justify-center p-6 text-center">
        <Bot className="h-12 w-12 text-muted-foreground" />
        <h3 className="mt-4 text-lg font-semibold">No Workspace</h3>
        <p className="mt-2 text-sm text-muted-foreground">
          Create a workspace first to use the Agent
        </p>
      </div>
    );
  }

  return (
    <div className="flex h-full flex-col">
      {/* Header */}
      <div className="border-b border-border p-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <div className="relative">
              <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-primary/10">
                <Bot className="h-5 w-5 text-primary" />
              </div>
            </div>
            <div>
              <h3 className="text-sm font-semibold">{t("agent")}</h3>
              <AgentStatusIndicator status={agentStatus} />
            </div>
          </div>
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto p-3">
        {/* Create Task Section */}
        <div className="mb-4">
          {showCreateForm ? (
            <div className="relative">
              <button
                onClick={() => setShowCreateForm(false)}
                className="absolute right-2 top-2 rounded-md p-1 hover:bg-accent"
              >
                <X className="h-4 w-4" />
              </button>
              <CreateAgentTask
                workspaceId={workspaceId}
                onSuccess={handleTaskCreated}
                onCancel={() => setShowCreateForm(false)}
              />
            </div>
          ) : (
            <button
              onClick={handleCreateClick}
              className={`w-full rounded-md border border-dashed border-border p-4 text-center text-sm text-muted-foreground transition-colors ${
                agentStatus === "unconfigured" || agentStatus === "error"
                  ? "cursor-pointer hover:border-yellow-500 hover:text-yellow-600"
                  : "hover:border-primary hover:text-primary"
              }`}
            >
              + New Research Task
            </button>
          )}
        </div>

        {/* Task List */}
        <div className="mb-4">
          <h4 className="mb-2 text-xs font-semibold uppercase text-muted-foreground">
            Recent Tasks
          </h4>
          <AgentTaskList workspaceId={workspaceId} onSelectTask={handleTaskSelect} />
        </div>

        {/* Selected Task Details */}
        {selectedTask && (
          <div className="rounded-lg border border-border bg-card p-4">
            <div className="mb-3 flex items-start justify-between">
              <div>
                <h4 className="font-semibold">{selectedTask.title}</h4>
                <TaskStatusBadge status={selectedTask.status} />
              </div>
              <button
                onClick={() => {
                  startTask.reset();
                  cancelTask.reset();
                  setSelectedTaskId(null);
                }}
                className="rounded-md p-1 hover:bg-accent"
              >
                <X className="h-4 w-4" />
              </button>
            </div>
            {selectedTask.description && (
              <p className="mb-3 text-sm text-muted-foreground">{selectedTask.description}</p>
            )}
            {startTask.isError && (
              <div
                className="mb-3 rounded-md border border-destructive/30 bg-destructive/10 p-2 text-sm text-destructive"
                role="alert"
              >
                <p>
                  {typeof startTask.error === "object" &&
                  startTask.error !== null &&
                  "queued" in startTask.error &&
                  startTask.error.queued === true
                    ? t("taskStartFailed")
                    : t("taskQueueFailed")}
                </p>
              </div>
            )}
            <div className="flex gap-2">
              {(selectedTask.status === "created" || selectedTask.status === "queued") && (
                <button
                  onClick={() => {
                    const taskStatus = selectedTask.status;
                    if (taskStatus === "created" || taskStatus === "queued") {
                      startTask.mutate({ taskId: selectedTask.id, status: taskStatus });
                    }
                  }}
                  disabled={startTask.isPending}
                  className="flex items-center gap-1 rounded-md bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
                >
                  <Play className="h-3 w-3" />
                  {startTask.isPending
                    ? t("startingTask")
                    : selectedTask.status === "queued"
                      ? t("retryStartTask")
                      : t("startTask")}
                </button>
              )}
              {selectedTask.status === "running" && (
                <button
                  onClick={() => cancelTask.mutate(selectedTask.id)}
                  disabled={cancelTask.isPending}
                  className="flex items-center gap-1 rounded-md border border-border px-3 py-1.5 text-xs font-medium hover:bg-accent disabled:opacity-50"
                >
                  <Square className="h-3 w-3" />
                  {cancelTask.isPending ? t("cancellingTask") : t("cancelTask")}
                </button>
              )}
            </div>
          </div>
        )}
      </div>

      {/* Footer */}
      <div className="border-t border-border p-3">
        <div className="flex items-center justify-between">
          <AgentStatusIndicator status={agentStatus} />
          <span className="text-xs text-muted-foreground">v0.1.0</span>
        </div>
      </div>

      {/* Agent Config Guide Dialog */}
      <AgentConfigGuide
        isOpen={showConfigGuide}
        onClose={() => setShowConfigGuide(false)}
        status={agentStatus}
      />
    </div>
  );
}
