/**
 * OnboardingWizardModal Component
 *
 * Automatically displays when the application is completely empty (no workspaces),
 * providing new users with a seamless choice:
 * 1. One-click load full institutional-grade demonstration dataset.
 * 2. Create custom initial workspace from scratch.
 *
 * @module components/onboarding/OnboardingWizardModal
 */

import { useState } from "react";
import { Sparkles, FolderPlus, ArrowRight, CheckCircle2, AlertCircle } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { formatMessage } from "@/lib/i18n/locale";
import { seedDemoData } from "@/lib/demoData";
import { desktopApi } from "@/lib/desktop-api";
import { useActiveWorkspace } from "@/features/workspace/hooks/useActiveWorkspace.context";
import { useQueryClient } from "@tanstack/react-query";
import { useFocusTrap } from "@/lib/hooks";

export function OnboardingWizardModal() {
  const { t } = useLocale();
  const queryClient = useQueryClient();
  const { workspaces, isLoading, setActiveWorkspace } = useActiveWorkspace();

  const [mode, setMode] = useState<"demo" | "custom">("demo");
  const [customName, setCustomName] = useState("");
  const [isProcessing, setIsProcessing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [successNotice, setSuccessNotice] = useState<string | null>(null);

  const containerRef = useFocusTrap<HTMLDivElement>({
    enabled: !isLoading && workspaces.length === 0,
  });

  // Only render if workspaces query has finished and the list is completely empty
  if (isLoading || workspaces.length > 0) {
    return null;
  }

  const handleExecute = async () => {
    setIsProcessing(true);
    setError(null);
    try {
      if (mode === "demo") {
        const result = await seedDemoData();
        await queryClient.invalidateQueries();
        setActiveWorkspace(result.workspaceId);
        setSuccessNotice(
          formatMessage(t("demoDataLoadedSuccess"), { name: result.workspaceName }),
        );
      } else {
        const trimmed = customName.trim();
        if (!trimmed) {
          setError(t("workspaceNameRequired"));
          setIsProcessing(false);
          return;
        }
        const created = await desktopApi.workspace.createWorkspace(trimmed);
        await queryClient.invalidateQueries();
        setActiveWorkspace(created.id);
      }
    } catch (err) {
      console.error("Failed to execute onboarding action:", err);
      const message =
        err instanceof Error
          ? err.message
          : typeof err === "string"
            ? err
            : err && typeof err === "object" && "message" in err
              ? String((err as { message: unknown }).message)
              : t("demoDataLoadedError");
      setError(message);
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm animate-in fade-in"
      data-testid="onboarding-wizard-modal"
    >
      <div
        ref={containerRef}
        role="dialog"
        aria-modal="true"
        aria-labelledby="onboarding-title"
        className="w-full max-w-xl rounded-2xl border bg-card p-6 text-card-foreground shadow-2xl transition-all space-y-6"
      >
        {/* Header */}
        <div className="space-y-1.5 text-center">
          <div className="inline-flex items-center justify-center p-2 rounded-full bg-primary/10 text-primary mb-1">
            <Sparkles className="h-6 w-6" />
          </div>
          <h2 id="onboarding-title" className="text-xl font-bold tracking-tight">
            {t("onboardingTitle")}
          </h2>
          <p className="text-xs text-muted-foreground max-w-md mx-auto">
            {t("onboardingDesc")}
          </p>
        </div>

        {/* Options */}
        <div className="grid gap-3 sm:grid-cols-2">
          {/* Option A: Load Demo Dataset */}
          <button
            type="button"
            onClick={() => setMode("demo")}
            className={`flex flex-col text-left p-4 rounded-xl border transition-all ${
              mode === "demo"
                ? "border-primary bg-primary/5 ring-1 ring-primary shadow-sm"
                : "border-border bg-card hover:bg-muted/40"
            }`}
          >
            <div className="flex items-center justify-between mb-2">
              <span className="p-2 rounded-lg bg-primary/10 text-primary">
                <Sparkles className="h-4 w-4" />
              </span>
              <span className="text-[10px] font-semibold text-primary uppercase tracking-wider bg-primary/10 px-2 py-0.5 rounded-full">
                Recommended
              </span>
            </div>
            <h3 className="text-sm font-semibold text-foreground">
              {t("onboardingLoadDemoOption")}
            </h3>
            <p className="text-xs text-muted-foreground mt-1 leading-relaxed">
              {t("onboardingLoadDemoDesc")}
            </p>
          </button>

          {/* Option B: Start from Scratch */}
          <button
            type="button"
            onClick={() => setMode("custom")}
            className={`flex flex-col text-left p-4 rounded-xl border transition-all ${
              mode === "custom"
                ? "border-primary bg-primary/5 ring-1 ring-primary shadow-sm"
                : "border-border bg-card hover:bg-muted/40"
            }`}
          >
            <div className="flex items-center justify-between mb-2">
              <span className="p-2 rounded-lg bg-muted text-muted-foreground">
                <FolderPlus className="h-4 w-4" />
              </span>
            </div>
            <h3 className="text-sm font-semibold text-foreground">
              {t("onboardingCustomOption")}
            </h3>
            <p className="text-xs text-muted-foreground mt-1 leading-relaxed">
              {t("onboardingCustomDesc")}
            </p>
          </button>
        </div>

        {/* Custom Input */}
        {mode === "custom" && (
          <div className="space-y-1.5 animate-in fade-in">
            <label className="text-xs font-medium text-foreground">
              {t("workspaceName")}
            </label>
            <input
              type="text"
              value={customName}
              onChange={(e) => setCustomName(e.target.value)}
              placeholder={t("onboardingWorkspaceNamePlaceholder")}
              className="w-full rounded-lg border border-input bg-background px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-primary"
            />
          </div>
        )}

        {/* Notices */}
        {error && (
          <div className="flex items-center gap-2 p-3 text-xs rounded-lg bg-destructive/10 text-destructive border border-destructive/20">
            <AlertCircle className="h-4 w-4 shrink-0" />
            <span>{error}</span>
          </div>
        )}
        {successNotice && (
          <div className="flex items-center gap-2 p-3 text-xs rounded-lg bg-green-500/10 text-green-600 dark:text-green-400 border border-green-500/20">
            <CheckCircle2 className="h-4 w-4 shrink-0" />
            <span>{successNotice}</span>
          </div>
        )}

        {/* Action Button */}
        <div className="pt-2">
          <button
            type="button"
            onClick={handleExecute}
            disabled={isProcessing}
            className="w-full flex items-center justify-center gap-2 rounded-xl bg-primary px-4 py-3 text-sm font-semibold text-primary-foreground hover:bg-primary/90 transition-all shadow-md disabled:opacity-50"
          >
            <span>
              {isProcessing
                ? t("loadingDemoDataBtn")
                : mode === "demo"
                  ? t("loadDemoDataBtn")
                  : t("onboardingStartFreshBtn")}
            </span>
            <ArrowRight className="h-4 w-4" />
          </button>
        </div>
      </div>
    </div>
  );
}
