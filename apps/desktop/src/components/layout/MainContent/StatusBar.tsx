/**
 * StatusBar Component
 *
 * Bottom section of Main Content area.
 * Shows system status, sync time, agent status, and version.
 *
 * @version GUI-M0
 */

import { useLocation } from "react-router-dom";
import { Wifi, WifiOff, Bot, Activity, Settings, AlertCircle } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { useNetworkStatus } from "@/hooks/useNetworkStatus";
import { useAgentGlobalStatus } from "@/hooks/useAgentStatus";

// Route to page name mapping
const ROUTE_PAGE_NAMES: Record<string, string> = {
  "/": "today",
  "/today": "today",
  "/research": "research",
  "/journal": "journal",
  "/portfolio": "portfolio",
  "/knowledge": "knowledge",
  "/artifacts": "artifacts",
  "/settings": "settings",
  "/options": "options",
};

// Version from package.json (would be injected at build time)
const APP_VERSION = "0.1.0";

export function StatusBar() {
  const { t } = useLocale();
  const location = useLocation();
  const { isOnline } = useNetworkStatus();
  const { data: agentStatus = "idle" } = useAgentGlobalStatus();

  // Get current page name
  const currentPath = location.pathname;
  const pageKey = ROUTE_PAGE_NAMES[currentPath] || "today";
  const pageName = t(pageKey as any);

  // Agent status config
  const agentStatusConfig = {
    idle: {
      color: "text-gray-500",
      bgColor: "bg-gray-500",
      label: t("statusIdle" as any) || "Idle",
      icon: Bot,
      animate: "",
    },
    running: {
      color: "text-blue-500",
      bgColor: "bg-blue-500",
      label: t("statusRunning" as any) || "Running",
      icon: Activity,
      animate: "animate-pulse",
    },
    unconfigured: {
      color: "text-yellow-500",
      bgColor: "bg-yellow-500",
      label: t("statusUnconfigured" as any) || "Unconfigured",
      icon: Settings,
      animate: "",
    },
    error: {
      color: "text-red-500",
      bgColor: "bg-red-500",
      label: t("statusError" as any) || "Error",
      icon: AlertCircle,
      animate: "",
    },
  };

  const agentConfig = agentStatusConfig[agentStatus];
  const AgentIcon = agentConfig.icon;

  return (
    <div className="flex h-7 items-center justify-between border-t border-border bg-background px-6">
      {/* Left: System status */}
      <div className="flex items-center gap-3">
        <div className={`h-1.5 w-1.5 rounded-full ${isOnline ? "bg-green-500" : "bg-red-500"}`} />
        <span className="text-[11px] text-muted-foreground/60">
          {isOnline ? "All systems operational" : "Offline"}
        </span>
        <span className="text-[11px] text-muted-foreground/30">·</span>
        <span className="text-[11px] text-muted-foreground/60">Last sync: 2m ago</span>
      </div>

      {/* Right: Agent status + Version */}
      <div className="flex items-center gap-3">
        {/* Agent Status */}
        <div
          className="flex items-center gap-1.5"
          title={`Agent: ${agentConfig.label}`}
        >
          <div className={`h-1.5 w-1.5 rounded-full ${agentConfig.bgColor} ${agentConfig.animate}`} />
          <AgentIcon className={`h-3 w-3 ${agentConfig.color}`} />
          <span className="text-[11px] text-muted-foreground/60">{agentConfig.label}</span>
        </div>

        {/* Divider */}
        <div className="h-3 w-px bg-border/60" />

        {/* Network Status */}
        <div
          className="flex items-center gap-1"
          title={isOnline ? "Online" : "Offline"}
        >
          {isOnline ? (
            <Wifi className="h-3 w-3 text-green-500" />
          ) : (
            <WifiOff className="h-3 w-3 text-red-500" />
          )}
        </div>

        {/* Divider */}
        <div className="h-3 w-px bg-border/60" />

        {/* Version */}
        <span className="text-[11px] text-muted-foreground/40">v{APP_VERSION}</span>
      </div>
    </div>
  );
}