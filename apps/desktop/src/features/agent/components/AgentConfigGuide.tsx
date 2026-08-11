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
  const isUnconfigured = status === "unconfigured";

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
              {isError
                ? (t("agentConnectionFailed" as any) || "Agent连接失败")
                : (t("agentNeedsConfig" as any) || "完成Agent配置")}
            </h2>
          </div>
          <button
            onClick={onClose}
            className="rounded-md p-1 hover:bg-accent"
            aria-label={t("cancel" as any) || "Close"}
          >
            <X className="h-5 w-5" />
          </button>
        </div>

        {/* Content */}
        <div className="mb-6">
          {isError ? (
            <>
              <p className="mb-3 text-sm text-muted-foreground">
                {t("agentConnectionErrorDesc" as any) ||
                  "Agent无法连接到AI服务。可能的原因："}
              </p>
              <ul className="mb-4 space-y-2 text-sm text-muted-foreground">
                <li className="flex items-start gap-2">
                  <span className="text-red-500">•</span>
                  <span>API密钥无效或已过期</span>
                </li>
                <li className="flex items-start gap-2">
                  <span className="text-red-500">•</span>
                  <span>网络代理配置错误</span>
                </li>
                <li className="flex items-start gap-2">
                  <span className="text-red-500">•</span>
                  <span>AI服务暂时不可用</span>
                </li>
              </ul>
              <p className="text-sm text-muted-foreground">
                {t("checkAgentSettings" as any) || "请检查Agent设置后重试。"}
              </p>
            </>
          ) : (
            <>
              <p className="mb-3 text-sm text-muted-foreground">
                {t("agentNotConfiguredDesc" as any) ||
                  "要使用Agent创建研究任务，您需要先配置API密钥和模型参数。"}
              </p>
              <div className="rounded-lg bg-yellow-500/10 p-3">
                <p className="text-sm font-medium text-yellow-900 dark:text-yellow-200">
                  {t("configStepsTitle" as any) || "配置步骤："}
                </p>
                <ol className="mt-2 space-y-1 text-sm text-yellow-800 dark:text-yellow-300">
                  <li>1. 点击下方按钮进入设置页面</li>
                  <li>2. 输入您的API密钥</li>
                  <li>3. 选择AI模型</li>
                  <li>4. 保存配置</li>
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
            {t("cancel" as any) || "取消"}
          </button>
          <button
            onClick={handleGoToSettings}
            className="flex items-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
          >
            <Settings className="h-4 w-4" />
            {t("goToSettings" as any) || "去设置"}
          </button>
        </div>
      </div>
    </div>
  );
}