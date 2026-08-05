/**
 * StatusBar Component
 *
 * Bottom section of Main Content area with new structure:
 * [页面名称] [快捷键提示] [Agent状态] [代理状态] [版本号]
 *
 * @version GUI-M3
 */

import { useLocation, useSearchParams } from "react-router-dom";
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
  "/artifacts": "artifacts",
  "/settings": "settings",
  "/options": "options",
};

// Version from package.json (would be injected at build time)
const APP_VERSION = "0.1.0";

export function StatusBar() {
  const { t } = useLocale();
  const location = useLocation();
  const [searchParams] = useSearchParams();
  const { isOnline } = useNetworkStatus();
  const { data: agentStatus = "idle" } = useAgentGlobalStatus();

  // Get current page name
  const currentPath = location.pathname;
  const pageKey = ROUTE_PAGE_NAMES[currentPath] || "today";
  const pageName = t(pageKey as any);

  // Agent status config
  const agentStatusConfig = {
    idle: {
      color: "text-gray-400",
      bgColor: "bg-gray-400",
      label: t("statusIdle" as any) || "空闲",
      icon: Bot,
      animate: "",
    },
    running: {
      color: "text-blue-500",
      bgColor: "bg-blue-500",
      label: t("statusRunning" as any) || "运行中",
      icon: Activity,
      animate: "animate-pulse",
    },
    unconfigured: {
      color: "text-yellow-500",
      bgColor: "bg-yellow-500",
      label: t("statusUnconfigured" as any) || "需要配置",
      icon: Settings,
      animate: "",
    },
    error: {
      color: "text-red-500",
      bgColor: "bg-red-500",
      label: t("statusError" as any) || "错误",
      icon: AlertCircle,
      animate: "",
    },
  };

  const agentConfig = agentStatusConfig[agentStatus];
  const AgentIcon = agentConfig.icon;

  return (
    <div className="flex items-center justify-between border-t border-border bg-card px-4 py-2">
      {/* Left: Page Name */}
      <div className="flex items-center gap-2">
        <span className="text-sm font-medium">{pageName}</span>
      </div>

      {/* Center: Keyboard Shortcut Hint */}
      <div className="flex items-center">
        <span className="text-xs text-muted-foreground">
          Ctrl+K {t("quickActions" as any) || "快捷操作"}
        </span>
      </div>

      {/* Right: Status Indicators + Version */}
      <div className="flex items-center gap-3">
        {/* Agent Status */}
        <div
          className="flex items-center gap-1.5"
          title={`Agent: ${agentConfig.label}`}
        >
          <div className={`h-2 w-2 rounded-full ${agentConfig.bgColor} ${agentConfig.animate}`} />
          <AgentIcon className={`h-3.5 w-3.5 ${agentConfig.color}`} />
          <span className="text-xs text-muted-foreground">{agentConfig.label}</span>
        </div>

        {/* Divider */}
        <div className="h-4 w-px bg-border" />

        {/* Proxy/Network Status */}
        <div
          className="flex items-center gap-1.5"
          title={isOnline ? t("networkOnline" as any) || "网络在线" : t("networkOffline" as any) || "网络离线"}
        >
          <div className={`h-2 w-2 rounded-full ${isOnline ? "bg-green-500" : "bg-red-500"}`} />
          {isOnline ? (
            <Wifi className="h-3.5 w-3.5 text-green-500" />
          ) : (
            <WifiOff className="h-3.5 w-3.5 text-red-500" />
          )}
          <span className="text-xs text-muted-foreground">
            {isOnline ? (t("networkOnline" as any) || "在线") : (t("networkOffline" as any) || "离线")}
          </span>
        </div>

        {/* Divider */}
        <div className="h-4 w-px bg-border" />

        {/* Version */}
        <span className="text-xs text-muted-foreground">v{APP_VERSION}</span>
      </div>
    </div>
  );
}