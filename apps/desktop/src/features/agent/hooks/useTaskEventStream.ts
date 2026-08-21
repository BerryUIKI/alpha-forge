import { useEffect, useState, useCallback } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { AGENT_KEYS } from "./useAgentTasks";
import type { AgentTaskEvent } from "@/lib/desktop-api/agent";

export interface ProgressMessage {
  id: string;
  taskId: string;
  message: string;
  timestamp: string;
}

export function useTaskEventStream() {
  const queryClient = useQueryClient();
  const [progressMessages, setProgressMessages] = useState<ProgressMessage[]>([]);
  const [latestEvent, setLatestEvent] = useState<AgentTaskEvent | null>(null);

  useEffect(() => {
    let unlisteners: UnlistenFn[] = [];
    let isMounted = true;

    const handleEvent = (eventName: string) => (event: { payload: AgentTaskEvent }) => {
      if (!isMounted) return;
      const data = event.payload;
      
      setLatestEvent(data);
      queryClient.invalidateQueries({ queryKey: AGENT_KEYS.all });

      setProgressMessages((prev) => {
        let message = "";
        if (eventName === "task:progress") {
          message = data.payload || "";
        } else if (eventName === "task:completed") {
          message = "Task completed.";
        } else if (eventName === "task:failed") {
          message = `Task failed: ${data.payload || "Unknown error"}`;
        } else if (eventName === "task:cancelled") {
          message = "Task cancelled.";
        }

        const newMsg: ProgressMessage = {
          id: data.id,
          taskId: data.taskId || (data as unknown as { task_id?: string }).task_id || "",
          message,
          timestamp: data.createdAt || (data as unknown as { created_at?: string }).created_at || new Date().toISOString(),
        };

        const updated = [...prev, newMsg];
        return updated.slice(-20);
      });
    };

    const setupListeners = async () => {
      try {
        const events = ["task:progress", "task:completed", "task:failed", "task:cancelled"];
        const promises = events.map((eventName) => 
          listen<AgentTaskEvent>(eventName, handleEvent(eventName))
        );
        const resolvedUnlisteners = await Promise.all(promises);
        
        if (!isMounted) {
          resolvedUnlisteners.forEach((unlisten) => unlisten());
        } else {
          unlisteners = resolvedUnlisteners;
        }
      } catch (error) {
        console.warn("Failed to subscribe to task events:", error);
      }
    };

    setupListeners();

    return () => {
      isMounted = false;
      unlisteners.forEach((unlisten) => unlisten());
    };
  }, [queryClient]);

  const clearProgress = useCallback((taskId: string) => {
    setProgressMessages((prev) => prev.filter((m) => m.taskId !== taskId));
  }, []);

  return { progressMessages, latestEvent, clearProgress };
}
