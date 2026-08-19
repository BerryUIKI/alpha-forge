/**
 * Agent Config Guide Dialog
 *
 * Displayed when user tries to create a task but Agent is not configured.
 * Guides users to configure API key and model settings.
 *
 * @module features/agent/components/AgentConfigGuide
 */

import { X, Settings, AlertCircle } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { useLocale } from "@/lib/i18n/useLocale";
import type { AgentConnectionStatus } from "@/components/layout/types";

interface AgentConfigGuideProps {
  isOpen: boolean;
  onClose: () => void;
  status: AgentConnectionStatus;
}

export function AgentConfigGuide({ isOpen, onClose, status }: AgentConfigGuideProps) {
  const { t } = useLocale();
  const navigate = useNavigate();

  if (!isOpen) return null;

  const handleGoToSettings = () => {
    onClose();
    navigate("/settings#agent");
  };

  const isError = status === "error";

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
      role="dialog"
      aria-modal="true"
      aria-labelledby="dialog-title"
    >
      <div className="w-full max-w-md rounded-lg bg-card p-6 shadow-xl">
        {/* Header */}
        <div className="mb-4 flex items-center justify-between">
          <div className="flex items-center gap-2">
            {isError ? (
              <AlertCircle className="h-5 w-5 text-red-500" />
            ) : (
              <Settings className="h-5 w-5 text-yellow-500" />
            )}
            <h2 id="dialog-title" className="text-lg font-semibold">
              {isError ? t("agentConnectionFailed") : t("agentNeedsConfig")}
            </h2>
          </div>
          <button
            onClick={onClose}
            className="rounded-md p-1 hover:bg-accent"
            aria-label={t("cancel")}
          >
            <X className="h-5 w-5" />
          </button>
        </div>

        {/* Content */}
        <div className="mb-6">
          {isError ? (
            <>
              <p className="mb-3 text-sm text-muted-foreground">
                {t("agentConnectionErrorDesc")}
              </p>
              <ul className="mb-4 space-y-2 text-sm text-muted-foreground">
                <li className="flex items-start gap-2">
                  <span className="text-red-500">•</span>
                  <span>{t("agentErrorInvalidApiKey")}</span>
                </li>
                <li className="flex items-start gap-2">
                  <span className="text-red-500">•</span>
                  <span>{t("agentErrorProxyConfig")}</span>
                </li>
                <li className="flex items-start gap-2">
                  <span className="text-red-500">•</span>
                  <span>{t("agentErrorServiceUnavailable")}</span>
                </li>
              </ul>
              <p className="text-sm text-muted-foreground">
                {t("checkAgentSettings")}
              </p>
            </>
          ) : (
            <>
              <p className="mb-3 text-sm text-muted-foreground">
                {t("agentNotConfiguredDesc")}
              </p>
              <div className="rounded-lg bg-yellow-500/10 p-3">
                <p className="text-sm font-medium text-yellow-900 dark:text-yellow-200">
                  {t("configStepsTitle")}
                </p>
                <ol className="mt-2 space-y-1 text-sm text-yellow-800 dark:text-yellow-300">
                  <li>1. {t("configStep1")}</li>
                  <li>2. {t("configStep2")}</li>
                  <li>3. {t("configStep3")}</li>
                  <li>4. {t("configStep4")}</li>
                </ol>
              </div>
            </>
          )}
        </div>

        {/* Actions */}
        <div className="flex justify-end gap-2">
          <button
            onClick={onClose}
            className="rounded-md border border-input bg-background px-4 py-2 text-sm font-medium hover:bg-accent"
          >
            {t("cancel")}
          </button>
          <button
            onClick={handleGoToSettings}
            className="flex items-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
          >
            <Settings className="h-4 w-4" />
            {t("goToSettings")}
          </button>
        </div>
      </div>
    </div>
  );
}