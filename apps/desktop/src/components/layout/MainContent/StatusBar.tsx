/**
 * StatusBar Component
 *
 * Bottom section of Main Content area.
 * Shows database health, network connectivity, agent status, and version.
 *
 * Each indicator maps to its own signal:
 * - Database readiness comes from `check_database_health`.
 * - Connectivity comes from `navigator.onLine` and is labelled as such.
 * - Agent readiness comes from the secure keychain probe.
 * Unknown/loading and failed states are shown explicitly; none default to a
 * green success state.
 */

import {
  Wifi,
  WifiOff,
  Bot,
  Activity,
  Settings,
  AlertCircle,
  Database,
  Loader2,
} from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { useNetworkStatus } from "@/hooks/useNetworkStatus";
import { useAgentGlobalStatusDisplay } from "@/hooks/useAgentStatus";
import { useSystemHealth } from "@/hooks/useSystemHealth";

// Version from package.json (would be injected at build time)
const APP_VERSION = "0.1.0";

export function StatusBar() {
  const { t } = useLocale();
  const { isOnline } = useNetworkStatus();
  const { status: agentStatus } = useAgentGlobalStatusDisplay();
  const {
    isHealthy: isDbHealthy,
    isLoading: isHealthLoading,
    hasError: healthError,
  } = useSystemHealth();

  // Agent status config — `checking` and `unavailable` are first-class states.
  const agentStatusConfig = {
    idle: {
      color: "text-muted-foreground",
      bgColor: "bg-muted-foreground/50",
      label: t("statusIdle"),
      icon: Bot,
      animate: "",
    },
    running: {
      color: "text-blue-500",
      bgColor: "bg-blue-500",
      label: t("statusRunning"),
      icon: Activity,
      animate: "animate-pulse",
    },
    unconfigured: {
      color: "text-amber-500",
      bgColor: "bg-amber-500",
      label: t("statusUnconfigured"),
      icon: Settings,
      animate: "",
    },
    error: {
      color: "text-destructive",
      bgColor: "bg-destructive",
      label: t("statusError"),
      icon: AlertCircle,
      animate: "",
    },
    checking: {
      color: "text-muted-foreground",
      bgColor: "bg-muted-foreground/50",
      label: t("statusChecking"),
      icon: Loader2,
      animate: "",
    },
    unavailable: {
      color: "text-destructive",
      bgColor: "bg-destructive",
      label: t("statusUnavailable"),
      icon: AlertCircle,
      animate: "",
    },
  };

  const agentConfig = agentStatusConfig[agentStatus];
  const AgentIcon = agentConfig.icon;

  const systemState = !isOnline
    ? "offline"
    : isHealthLoading
      ? "checking"
      : healthError || !isDbHealthy
        ? "degraded"
        : "operational";

  const systemLabel = {
    offline: t("systemOffline"),
    checking: t("systemChecking"),
    degraded: t("systemDegraded"),
    operational: t("systemOperational"),
  }[systemState];

  const systemDotClass = {
    offline: "bg-amber-500",
    checking: "bg-muted-foreground/50",
    degraded: "bg-destructive",
    operational: "bg-emerald-500",
  }[systemState];

  const dbLabel = isHealthLoading
    ? t("dbChecking")
    : isDbHealthy
      ? t("dbHealthy")
      : t("dbUnhealthy");

  return (
    <div className="flex h-7 shrink-0 items-center justify-between gap-4 border-t border-border bg-background px-4 md:px-6">
      {/* Left: backend status */}
      <div className="flex min-w-0 items-center gap-3" aria-live="polite">
        <div className={`h-1.5 w-1.5 shrink-0 rounded-full ${systemDotClass}`} />
        <span className="truncate text-xs text-muted-foreground">{systemLabel}</span>
        <span className="text-xs text-muted-foreground/50">·</span>
        <div
          className="flex min-w-0 items-center gap-1 text-xs text-muted-foreground"
          title={dbLabel}
        >
          <Database
            className={`h-3 w-3 shrink-0 ${
              isHealthLoading
                ? "text-muted-foreground"
                : isDbHealthy
                  ? "text-emerald-500"
                  : "text-destructive"
            }`}
          />
          <span className="truncate">{dbLabel}</span>
        </div>
      </div>

      {/* Right: Agent status + connectivity + version */}
      <div className="flex shrink-0 items-center gap-3">
        {/* Agent Status */}
        <div className="flex min-w-0 items-center gap-1.5" title={`Agent: ${agentConfig.label}`}>
          <div
            className={`h-1.5 w-1.5 rounded-full ${agentConfig.bgColor} ${agentConfig.animate}`}
          />
          <AgentIcon className={`h-3 w-3 ${agentConfig.color}`} aria-hidden="true" />
          <span className="truncate text-xs text-muted-foreground">{agentConfig.label}</span>
        </div>

        {/* Divider */}
        <div className="h-3 w-px bg-border" />

        {/* Connectivity — labelled as connectivity only */}
        <div
          className="flex items-center gap-1 text-xs text-muted-foreground"
          title={isOnline ? t("networkOnline") : t("networkOffline")}
        >
          {isOnline ? (
            <Wifi className="h-3 w-3 text-emerald-500" aria-hidden="true" />
          ) : (
            <WifiOff className="h-3 w-3 text-amber-500" aria-hidden="true" />
          )}
          <span className="hidden sm:inline">
            {isOnline ? t("networkOnline") : t("networkOffline")}
          </span>
        </div>

        {/* Divider */}
        <div className="h-3 w-px bg-border" />

        {/* Version */}
        <span className="text-xs text-muted-foreground/70">v{APP_VERSION}</span>
      </div>
    </div>
  );
}
