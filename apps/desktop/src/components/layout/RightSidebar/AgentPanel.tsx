/**
 * AgentPanel Component
 *
 * Content of the right sidebar for Agent interaction (Module C).
 * Enhanced placeholder with collapsible sections and better UI.
 *
 * Features:
 * - Collapsible sections for Conversation, Tools, and Tasks
 * - Status indicator with real-time updates placeholder
 * - Empty state with actionable placeholder
 * - Prepared for future Agent runtime integration
 *
 * @version GUI-M1-3
 */

import { useState } from "react";
import { Bot, MessageSquare, Wrench, ListTodo, ChevronDown, ChevronRight, Send, Zap } from "lucide-react";
import type { AgentPanelProps } from "../types";

interface CollapsibleSectionProps {
  title: string;
  icon: typeof MessageSquare;
  children: React.ReactNode;
  defaultOpen?: boolean;
}

function CollapsibleSection({ title, icon: Icon, children, defaultOpen = false }: CollapsibleSectionProps) {
  const [isOpen, setIsOpen] = useState(defaultOpen);

  return (
    <div className="rounded-lg border border-border">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="flex w-full items-center justify-between p-3 hover:bg-accent transition-colors"
        aria-expanded={isOpen}
      >
        <div className="flex items-center gap-2">
          <Icon className="h-4 w-4 text-muted-foreground" />
          <span className="text-sm font-medium">{title}</span>
        </div>
        {isOpen ? (
          <ChevronDown className="h-4 w-4 text-muted-foreground" />
        ) : (
          <ChevronRight className="h-4 w-4 text-muted-foreground" />
        )}
      </button>
      {isOpen && <div className="border-t border-border p-3">{children}</div>}
    </div>
  );
}

export function AgentPanel({
  status = "Ready to assist",
  placeholder = "Agent capabilities coming soon",
}: AgentPanelProps) {
  const [inputValue, setInputValue] = useState("");

  const handleSendMessage = () => {
    // TODO: [GUI-M1-3] Implement message sending to agent
    console.log("Send message:", inputValue);
    setInputValue("");
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

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
              {/* Status indicator */}
              <div className="absolute -right-0.5 -bottom-0.5 h-3 w-3 rounded-full border-2 border-card bg-green-500" />
            </div>
            <div>
              <h3 className="text-sm font-semibold">Agent</h3>
              <p className="text-xs text-muted-foreground">{status}</p>
            </div>
          </div>
          <button
            className="flex h-7 w-7 items-center justify-center rounded-lg transition-colors hover:bg-accent"
            aria-label="Agent settings"
            title="Agent settings (coming soon)"
          >
            <Zap className="h-4 w-4 text-muted-foreground" />
          </button>
        </div>
      </div>

      {/* Collapsible Sections */}
      <div className="flex-1 overflow-y-auto p-3">
        <div className="space-y-2">
          {/* Conversation Section */}
          <CollapsibleSection title="Conversation" icon={MessageSquare} defaultOpen>
            <div className="space-y-2">
              {/* Message input */}
              <div className="flex gap-2">
                <input
                  type="text"
                  value={inputValue}
                  onChange={(e) => setInputValue(e.target.value)}
                  onKeyPress={handleKeyPress}
                  placeholder="Type a message..."
                  className="flex-1 rounded-lg border border-border bg-background px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-primary"
                  disabled
                />
                <button
                  onClick={handleSendMessage}
                  className="flex h-9 w-9 items-center justify-center rounded-lg bg-primary text-primary-foreground transition-colors hover:bg-primary/90 disabled:opacity-50"
                  aria-label="Send message"
                  disabled
                  title="Send message (coming soon)"
                >
                  <Send className="h-4 w-4" />
                </button>
              </div>
              <p className="text-xs text-muted-foreground text-center">
                Chat interface coming soon
              </p>
            </div>
          </CollapsibleSection>

          {/* Tools Section */}
          <CollapsibleSection title="Tools" icon={Wrench}>
            <div className="grid grid-cols-2 gap-2">
              {["Search", "Analyze", "Report", "Query"].map((tool) => (
                <button
                  key={tool}
                  className="flex items-center justify-center rounded-lg border border-border px-3 py-2 text-xs font-medium transition-colors hover:bg-accent disabled:opacity-50"
                  disabled
                  title={`${tool} tool (coming soon)`}
                >
                  {tool}
                </button>
              ))}
            </div>
            <p className="text-xs text-muted-foreground text-center mt-2">
              Tool invocation coming soon
            </p>
          </CollapsibleSection>

          {/* Tasks Section */}
          <CollapsibleSection title="Tasks" icon={ListTodo}>
            <div className="space-y-2">
              {["Active tasks", "Recent tasks", "Task history"].map((item) => (
                <button
                  key={item}
                  className="flex w-full items-center justify-between rounded-lg border border-border px-3 py-2 text-xs transition-colors hover:bg-accent disabled:opacity-50"
                  disabled
                  title={`${item} (coming soon)`}
                >
                  <span>{item}</span>
                  <span className="text-muted-foreground">0</span>
                </button>
              ))}
            </div>
            <p className="text-xs text-muted-foreground text-center mt-2">
              Task management coming soon
            </p>
          </CollapsibleSection>
        </div>

        {/* Empty State */}
        <div className="mt-4 flex items-center justify-center">
          <div className="rounded-lg border border-dashed border-border p-6 text-center max-w-xs">
            <Bot className="h-10 w-10 mx-auto mb-2 text-muted-foreground" />
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
          <div className="flex items-center gap-2">
            <div className="h-2 w-2 rounded-full bg-green-500" />
            <span className="text-xs text-muted-foreground">Status: Idle</span>
          </div>
          <span className="text-xs text-muted-foreground">v0.1.0</span>
        </div>
      </div>

      {/* TODO markers for future implementation */}
      {/* TODO: [GUI-M1-3] Connect to agent runtime via Tauri commands */}
      {/* TODO: [GUI-M1-3] Implement real-time status updates */}
      {/* TODO: [GUI-M1-3] Add conversation history persistence */}
      {/* TODO: [GUI-M1-3] Implement tool execution feedback */}
      {/* TODO: [GUI-M1-3] Add task progress indicators */}
    </div>
  );
}