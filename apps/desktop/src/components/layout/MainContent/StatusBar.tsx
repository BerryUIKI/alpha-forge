/**
 * StatusBar Component
 *
 * Bottom section of Main Content area (Module D - bottom).
 * Displays workspace name, application status, and hints.
 * Always visible, fixed at bottom.
 *
 * TODO: [GUI-M1-4] Add real-time status updates
 * TODO: [GUI-M1-4] Implement status indicators with animations
 * TODO: [GUI-M1-4] Add i18n for status messages
 */

import { Circle, CheckCircle2, AlertCircle, RefreshCw } from "lucide-react";
import type { StatusBarProps, AppStatus } from "../types";

const STATUS_CONFIG: Record<AppStatus, { icon: typeof Circle; color: string; label: string }> = {
  idle: { icon: Circle, color: "text-muted-foreground", label: "Idle" },
  running: { icon: RefreshCw, color: "text-primary", label: "Running" },
  error: { icon: AlertCircle, color: "text-destructive", label: "Error" },
  syncing: { icon: RefreshCw, color: "text-primary", label: "Syncing" },
};

export function StatusBar({
  workspaceName = "Analyze",
  status = "idle",
  hint = "Press ⌘K for quick actions",
}: StatusBarProps) {
  const statusConfig = STATUS_CONFIG[status];
  const StatusIcon = statusConfig.icon;

  return (
    <div className="flex items-center justify-between border-t border-border bg-card px-4 py-2">
      {/* Left: Workspace Name */}
      <div className="flex items-center gap-2">
        <span className="text-sm font-medium">{workspaceName}</span>
      </div>

      {/* Right: Status & Hint */}
      <div className="flex items-center gap-4">
        {/* Status Indicator */}
        <div className="flex items-center gap-1.5">
          <StatusIcon
            className={`h-3.5 w-3.5 ${statusConfig.color} ${
              status === "running" || status === "syncing" ? "animate-spin" : ""
            }`}
          />
          <span className={`text-xs ${statusConfig.color}`}>
            {statusConfig.label}
          </span>
        </div>

        {/* Divider */}
        <div className="h-4 w-px bg-border" />

        {/* Hint Text */}
        <span className="text-xs text-muted-foreground">{hint}</span>
      </div>

      {/* TODO markers for future implementation */}
      {/* TODO: [GUI-M1-4] Add click handlers for status indicators */}
      {/* TODO: [GUI-M1-4] Implement real-time status polling from backend */}
      {/* TODO: [GUI-M1-4] Add tooltips with detailed status information */}
    </div>
  );
}