/**
 * AgentPanel Component
 *
 * Right sidebar content for Agent interaction.
 * Combines task management with a message-based chat interface.
 *
 * @version GUI-M4
 */

import { useState } from "react";
import { Bot, X, Play, Square, Send } from "lucide-react";
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

interface AgentPanelProps {
  status?: string;
  placeholder?: string;
}

// --- Agent Message Types ---

interface AgentMessage {
  id: string;
  type: "research" | "alert" | "thesis" | "info";
  label: string;
  content: string;
  timestamp: string;
}

const MESSAGE_TYPE_STYLES = {
  research: { dot: "bg-indigo-400", label: "text-indigo-400" },
  alert: { dot: "bg-amber-400", label: "text-amber-400" },
  thesis: { dot: "bg-green-500", label: "text-green-500" },
  info: { dot: "bg-sky-400", label: "text-sky-400" },
};

const SAMPLE_MESSAGES: AgentMessage[] = [
  {
    id: "m1",
    type: "research",
    label: "Research Analysis",
    content: "NVDA's Q2 earnings exceeded consensus by 8%. Key drivers: data center revenue up 42% YoY. Maintaining bullish thesis.",
    timestamp: "12m ago",
  },
  {
    id: "m2",
    type: "alert",
    label: "Portfolio Alert",
    content: "Sector concentration warning: Technology sector now represents 42% of portfolio. Consider rebalancing.",
    timestamp: "2h ago",
  },
  {
    id: "m3",
    type: "thesis",
    label: "Thesis Update",
    content: "Your 'Renewable Energy Infrastructure' thesis has 3 new supporting articles. Reviewing now.",
    timestamp: "5h ago",
  },
];

// ─── Sub-components ──────────────────────────────────────────────────────────

function AgentMessageItem({ message }: { message: AgentMessage }) {
  const style = MESSAGE_TYPE_STYLES[message.type];

  return (
    <div className="rounded-lg border border-border/60 bg-card/50 p-3">
      <div className="mb-1 flex items-center gap-2">
        <div className={`h-1.5 w-1.5 rounded-full ${style.dot}`} />
        <span className={`text-xs font-semibold ${style.label}`}>{message.label}</span>
        <span className="ml-auto text-[10px] text-muted-foreground/60">{message.timestamp}</span>
      </div>
      <p className="text-sm leading-relaxed text-muted-foreground">{message.content}</p>
    </div>
  );
}

function AgentInput() {
  const [input, setInput] = useState("");

  const handleSend = () => {
    if (!input.trim()) return;
    // TODO: wire up to actual agent chat
    setInput("");
  };

  return (
    <div className="flex gap-2 border-t border-border p-3">
      <input
        type="text"
        value={input}
        onChange={(e) => setInput(e.target.value)}
        onKeyDown={(e) => {
          if (e.key === "Enter" && !e.shiftKey) {
            e.preventDefault();
            handleSend();
          }
        }}
        placeholder="Ask the agent..."
        className="flex-1 rounded-lg border border-border bg-muted/50 px-3 py-1.5 text-sm outline-none placeholder:text-muted-foreground/40 focus:border-primary focus:ring-1 focus:ring-primary"
      />
      <button
        onClick={handleSend}
        disabled={!input.trim()}
        className="flex h-8 w-8 items-center justify-center rounded-lg bg-primary text-primary-foreground transition-colors hover:bg-primary/90 disabled:opacity-40"
      >
        <Send className="h-4 w-4" />
      </button>
    </div>
  );
}

// --- Main Component ---

export function AgentPanel(_props: AgentPanelProps) {
  const { t } = useLocale();
  const { data: workspaces, isLoading: workspacesLoading, error: workspacesError } = useWorkspaces();
  const [selectedTaskId, setSelectedTaskId] = useState<string | null>(null);
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [showConfigGuide, setShowConfigGuide] = useState(false);
  const [messages] = useState<AgentMessage[]>(SAMPLE_MESSAGES);

  const workspaceId = workspaces?.[0]?.id || "";
  const { status: agentStatus } = useAgentStatus(workspaceId);
  const startTask = useRunAgentTask();
  const cancelTask = useCancelAgentTask();
  const { data: selectedTask } = useAgentTask(selectedTaskId || "");

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
      {/* Messages Area */}
      <div className="flex-1 space-y-2 overflow-y-auto p-3">
        {/* Agent Messages */}
        {messages.map((message) => (
          <AgentMessageItem key={message.id} message={message} />
        ))}

        {/* Create Task Section */}
        <div className="pt-2">
          {showCreateForm ? (
            <div className="relative rounded-lg border border-border/60 bg-card/50 p-3">
              <button
                onClick={() => setShowCreateForm(false)}
                className="absolute right-2 top-2 rounded-md p-1 hover:bg-accent"
              >
                <X className="h-3 w-3" />
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
              className={`w-full rounded-lg border border-dashed border-border p-3 text-center text-xs text-muted-foreground transition-colors ${
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
        <div>
          <h4 className="mb-2 text-[10px] font-semibold uppercase tracking-wider text-muted-foreground/60">
            Recent Tasks
          </h4>
          <AgentTaskList workspaceId={workspaceId} onSelectTask={handleTaskSelect} />
        </div>

        {/* Selected Task Details */}
        {selectedTask && (
          <div className="rounded-lg border border-border bg-card p-3">
            <div className="mb-2 flex items-start justify-between">
              <div className="min-w-0">
                <h4 className="truncate text-sm font-semibold">{selectedTask.title}</h4>
                <TaskStatusBadge status={selectedTask.status} />
              </div>
              <button
                onClick={() => { startTask.reset(); cancelTask.reset(); setSelectedTaskId(null); }}
                className="ml-2 rounded-md p-1 hover:bg-accent"
              >
                <X className="h-3 w-3" />
              </button>
            </div>
            {selectedTask.description && (
              <p className="mb-2 text-xs text-muted-foreground">{selectedTask.description}</p>
            )}
            {startTask.isError && (
              <div className="mb-2 rounded-md border border-destructive/30 bg-destructive/10 p-2 text-xs text-destructive" role="alert">
                <p>
                  {typeof startTask.error === "object" && startTask.error !== null && "queued" in startTask.error && startTask.error.queued === true
                    ? t("taskStartFailed")
                    : t("taskQueueFailed")}
                </p>
              </div>
            )}
            <div className="flex gap-2">
              {(selectedTask.status === "created" || selectedTask.status === "queued") && (
                <button
                  onClick={() => { const s = selectedTask.status; if (s === "created" || s === "queued") startTask.mutate({ taskId: selectedTask.id, status: s }); }}
                  disabled={startTask.isPending}
                  className="flex items-center gap-1 rounded-md bg-primary px-2.5 py-1 text-xs font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
                >
                  <Play className="h-3 w-3" />
                  {startTask.isPending ? t("startingTask") : selectedTask.status === "queued" ? t("retryStartTask") : t("startTask")}
                </button>
              )}
              {selectedTask.status === "running" && (
                <button
                  onClick={() => cancelTask.mutate(selectedTask.id)}
                  disabled={cancelTask.isPending}
                  className="flex items-center gap-1 rounded-md border border-border px-2.5 py-1 text-xs font-medium hover:bg-accent disabled:opacity-50"
                >
                  <Square className="h-3 w-3" />
                  {cancelTask.isPending ? t("cancellingTask") : t("cancelTask")}
                </button>
              )}
            </div>
          </div>
        )}
      </div>

      {/* Input */}
      <AgentInput />

      {/* Agent Config Guide Dialog */}
      <AgentConfigGuide
        isOpen={showConfigGuide}
        onClose={() => setShowConfigGuide(false)}
        status={agentStatus}
      />
    </div>
  );
}