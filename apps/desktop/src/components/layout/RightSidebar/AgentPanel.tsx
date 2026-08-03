/**
 * AgentPanel Component
 *
 * Content of the right sidebar for Agent interaction (Module C).
 * UI-only skeleton with placeholder sections for future agent capabilities.
 *
 * Future capabilities (placeholders):
 * - Agent conversation interface
 * - Tool execution panel
 * - Task management and orchestration
 * - Agent status indicators
 *
 * TODO: [GUI-M1-3] Implement agent conversation UI
 * TODO: [GUI-M1-3] Implement tool invocation panel
 * TODO: [GUI-M1-3] Implement task orchestration UI
 * TODO: [GUI-M1-4] Add i18n for status messages
 */

import { Bot, MessageSquare, Wrench, ListTodo } from "lucide-react";
import type { AgentPanelProps } from "../types";

export function AgentPanel({
  status = "Ready to assist",
  placeholder = "Agent capabilities coming soon",
}: AgentPanelProps) {
  return (
    <div className="flex h-full flex-col">
      {/* Header */}
      <div className="border-b border-border p-3">
        <div className="flex items-center gap-2">
          <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-primary/10">
            <Bot className="h-5 w-5 text-primary" />
          </div>
          <div>
            <h3 className="text-sm font-semibold">Agent</h3>
            <p className="text-xs text-muted-foreground">{status}</p>
          </div>
        </div>
      </div>

      {/* Placeholder Sections */}
      <div className="flex-1 overflow-y-auto p-3">
        <div className="space-y-3">
          {/* Conversation Section */}
          <div className="rounded-lg border border-border p-4">
            <div className="flex items-center gap-2 mb-2">
              <MessageSquare className="h-4 w-4 text-muted-foreground" />
              <span className="text-sm font-medium">Conversation</span>
            </div>
            <p className="text-xs text-muted-foreground">
              {/* TODO: [GUI-M1-3] Implement conversation interface */}
              Agent chat interface coming soon
            </p>
          </div>

          {/* Tools Section */}
          <div className="rounded-lg border border-border p-4">
            <div className="flex items-center gap-2 mb-2">
              <Wrench className="h-4 w-4 text-muted-foreground" />
              <span className="text-sm font-medium">Tools</span>
            </div>
            <p className="text-xs text-muted-foreground">
              {/* TODO: [GUI-M1-3] Implement tool execution panel */}
              Tool invocation panel coming soon
            </p>
          </div>

          {/* Tasks Section */}
          <div className="rounded-lg border border-border p-4">
            <div className="flex items-center gap-2 mb-2">
              <ListTodo className="h-4 w-4 text-muted-foreground" />
              <span className="text-sm font-medium">Tasks</span>
            </div>
            <p className="text-xs text-muted-foreground">
              {/* TODO: [GUI-M1-3] Implement task orchestration */}
              Task management coming soon
            </p>
          </div>
        </div>

        {/* Empty State */}
        <div className="mt-6 flex items-center justify-center">
          <div className="rounded-lg border border-dashed border-border p-8 text-center">
            <Bot className="h-12 w-12 mx-auto mb-3 text-muted-foreground" />
            <p className="text-sm font-medium">{placeholder}</p>
            <p className="text-xs text-muted-foreground mt-1">
              Full agent capabilities in future release
            </p>
          </div>
        </div>
      </div>

      {/* Status Footer */}
      <div className="border-t border-border p-3">
        <div className="flex items-center justify-between">
          <span className="text-xs text-muted-foreground">Status: Idle</span>
          {/* TODO: [GUI-M1-3] Add actual agent status indicator */}
        </div>
      </div>

      {/* TODO markers for future implementation */}
      {/* TODO: [GUI-M1-3] Connect to agent runtime */}
      {/* TODO: [GUI-M1-3] Implement real-time status updates */}
      {/* TODO: [GUI-M1-3] Add conversation history persistence */}
    </div>
  );
}