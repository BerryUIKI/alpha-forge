/**
 * AgentPanel Component
 *
 * Right sidebar content for Agent interaction.
 * Combines task management with a message-based chat interface. Sending a
 * message in the chat input creates and auto-starts a research task; the
 * created task is shown in the task-detail card below.
 *
 * Design: docs/GLOBAL_SEARCH_AGENT_CHAT.md
 *
 * @version GUI-M5
 */

import { useState } from "react";
import { Bot, X, Play, Square, Send } from "lucide-react";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { ErrorState } from "@/components/common/ErrorState";
import { useWorkspaces } from "@/features/workspace/hooks/useWorkspaces";
import { useActiveWorkspaceId } from "@/features/workspace/hooks/useActiveWorkspace.context";
import { AgentTaskList } from "@/features/agent/components/AgentTaskList";
import { CreateAgentTask } from "@/features/agent/components/CreateAgentTask";
import {
  useAgentTask,
  useRunAgentTask,
  useCancelAgentTask,
  useCreateAgentTask,
} from "@/features/agent/hooks/useAgentTasks";
import { useAgentStatus } from "@/hooks/useAgentStatus";
import { AgentConfigGuide } from "@/features/agent/components/AgentConfigGuide";
import { TaskStatusBadge } from "@/features/agent/components/TaskStatusBadge";
import type { AgentTask } from "@/lib/desktop-api/agent";
import { useLocale } from "@/lib/i18n/useLocale";

interface ConversationMessage {
  id: string;
  role: "user" | "info";
  text: string;
}

let messageSeq = 0;
function nextMessageId(): string {
  messageSeq += 1;
  return `conversation-${messageSeq}`;
}

// ─── Sub-components ──────────────────────────────────────────────────────────

function ConversationItem({ message }: { message: ConversationMessage }) {
  if (message.role === "user") {
    return (
      <div className="flex justify-end">
        <div className="max-w-[85%] rounded-lg rounded-br-sm bg-primary px-3 py-2 text-sm text-primary-foreground">
          {message.text}
        </div>
      </div>
    );
  }
  return (
    <div className="flex items-start gap-2 rounded-lg border border-border/60 bg-card/50 p-3">
      <div className="mt-1 h-1.5 w-1.5 shrink-0 rounded-full bg-sky-400" />
      <p className="text-xs leading-relaxed text-muted-foreground">{message.text}</p>
    </div>
  );
}

function AgentInput({ onSend }: { onSend: (text: string) => void }) {
  const { t } = useLocale();
  const [input, setInput] = useState("");

  const handleSend = () => {
    const text = input.trim();
    if (!text) return;
    onSend(text);
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
        placeholder={t("askTheAgent")}
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

export function AgentPanel() {
  const { t } = useLocale();
  const { isLoading: workspacesLoading, error: workspacesError } = useWorkspaces();
  const [selectedTaskId, setSelectedTaskId] = useState<string | null>(null);
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [showConfigGuide, setShowConfigGuide] = useState(false);
  const [conversation, setConversation] = useState<ConversationMessage[]>([]);

  // Loading/error states come from the workspace list query; the active
  // workspace itself comes from the global context (ADR-0008).
  const workspaceId = useActiveWorkspaceId();
  const { status: agentStatus } = useAgentStatus(workspaceId);
  const startTask = useRunAgentTask();
  const cancelTask = useCancelAgentTask();
  const createTask = useCreateAgentTask();
  const { data: selectedTask } = useAgentTask(selectedTaskId || "");

  const appendMessage = (message: ConversationMessage) => {
    setConversation((prev) => [...prev, message]);
  };

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

  const handleSend = (text: string) => {
    // Unconfigured/error agents cannot run research; guide the user to configure.
    if (agentStatus === "unconfigured" || agentStatus === "error") {
      appendMessage({ id: nextMessageId(), role: "user", text });
      appendMessage({ id: nextMessageId(), role: "info", text: t("agentChatNeedsConfig") });
      setShowConfigGuide(true);
      return;
    }

    appendMessage({ id: nextMessageId(), role: "user", text });

    createTask.mutate(
      { workspaceId, title: text },
      {
        onSuccess: (task) => {
          // Show the task card, then queue and start execution automatically.
          setSelectedTaskId(task.id);
          setShowCreateForm(false);
          startTask.mutate({ taskId: task.id, status: "created" });
        },
        onError: () => {
          appendMessage({ id: nextMessageId(), role: "info", text: t("agentChatSendFailed") });
        },
      },
    );
  };

  // Loading state
  if (workspacesLoading) {
    return <LoadingSpinner className="p-8" />;
  }

  // Error state
  if (workspacesError) {
    return (
      <div className="p-4">
        <ErrorState message={t("failedToLoadWorkspaces")} onRetry={() => window.location.reload()} />
      </div>
    );
  }

  // No workspace state
  if (!workspaceId) {
    return (
      <div className="flex h-full flex-col items-center justify-center p-6 text-center">
        <Bot className="h-12 w-12 text-muted-foreground" />
        <h3 className="mt-4 text-lg font-semibold">{t("noWorkspace")}</h3>
        <p className="mt-2 text-sm text-muted-foreground">
          {t("createWorkspaceFirstUseAgent")}
        </p>
      </div>
    );
  }

  return (
    <div className="flex h-full flex-col">
      {/* Messages Area */}
      <div className="flex-1 space-y-2 overflow-y-auto p-3">
        {/* Welcome message */}
        <ConversationItem message={{ id: "welcome", role: "info", text: t("agentChatWelcome") }} />

        {/* Conversation */}
        {conversation.map((message) => (
          <ConversationItem key={message.id} message={message} />
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
              + {t("newResearchTask")}
            </button>
          )}
        </div>

        {/* Task List */}
        <div>
          <h4 className="mb-2 text-[10px] font-semibold uppercase tracking-wider text-muted-foreground/60">
            {t("recentTasks")}
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
      <AgentInput onSend={handleSend} />

      {/* Agent Config Guide Dialog */}
      <AgentConfigGuide
        isOpen={showConfigGuide}
        onClose={() => setShowConfigGuide(false)}
        status={agentStatus}
      />
    </div>
  );
}
