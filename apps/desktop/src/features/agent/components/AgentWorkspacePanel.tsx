// Workspace agent panel — create, run, and inspect research tasks.

import { useState } from "react";
import { Bot } from "lucide-react";
import { AgentTaskList } from "./AgentTaskList";
import { AgentTaskDetail } from "./AgentTaskDetail";
import { CreateAgentTask } from "./CreateAgentTask";
import { useAgentTaskEvents } from "../hooks/useAgentTasks";

interface AgentWorkspacePanelProps {
  workspaceId: string;
}

export function AgentWorkspacePanel({ workspaceId }: AgentWorkspacePanelProps) {
  const [selectedTaskId, setSelectedTaskId] = useState<string | null>(null);

  // One subscription for the whole panel keeps task rows and detail in sync.
  useAgentTaskEvents();

  return (
    <section className="space-y-4">
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div>
          <div className="flex items-center gap-2">
            <Bot className="h-5 w-5 text-primary" />
            <h2 className="text-lg font-semibold">Research Tasks</h2>
          </div>
          <p className="mt-1 text-sm text-muted-foreground">
            Run research tasks in the background and follow their progress live.
          </p>
        </div>
        <CreateAgentTask
          workspaceId={workspaceId}
          onSuccess={(taskId) => {
            setSelectedTaskId(taskId);
          }}
        />
      </div>

      <div
        className={
          selectedTaskId ? "grid gap-4 xl:grid-cols-[minmax(0,1fr)_minmax(0,1.1fr)]" : undefined
        }
      >
        <AgentTaskList
          workspaceId={workspaceId}
          selectedTaskId={selectedTaskId ?? undefined}
          onSelectTask={(task) => {
            setSelectedTaskId(task.id);
          }}
        />
        {selectedTaskId && (
          <AgentTaskDetail
            taskId={selectedTaskId}
            onClose={() => {
              setSelectedTaskId(null);
            }}
          />
        )}
      </div>
    </section>
  );
}
