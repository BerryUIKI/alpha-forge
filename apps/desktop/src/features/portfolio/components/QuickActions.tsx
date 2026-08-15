/**
 * QuickActions Component
 *
 * Displays quick action buttons for the portfolio dashboard:
 * create snapshot, refresh data.
 *
 * @module features/portfolio/components/QuickActions
 */

import { useState } from "react";
import { useLocale } from "@/lib/i18n/useLocale";
import { useCreateSnapshot } from "@/features/portfolio/hooks/useFinancialData";
import { Camera, RefreshCw, CheckCircle2, AlertCircle } from "lucide-react";

interface QuickActionsProps {
  accountId: string;
  onRefresh: () => void;
  asOfDate: string;
}

export function QuickActions({ accountId, onRefresh, asOfDate }: QuickActionsProps) {
  const { t } = useLocale();
  const createSnapshot = useCreateSnapshot();
  const [notice, setNotice] = useState<"success" | "error" | null>(null);

  const handleCreateSnapshot = async () => {
    if (!accountId) return;
    setNotice(null);
    try {
      await createSnapshot.mutateAsync({
        accountId,
        snapshotDate: asOfDate,
      });
      setNotice("success");
      setTimeout(() => setNotice(null), 3000);
    } catch {
      setNotice("error");
      setTimeout(() => setNotice(null), 3000);
    }
  };

  return (
    <div className="space-y-2">
      <h3 className="text-sm font-medium text-muted-foreground">
        {t("quickActions" as any) || "Quick Actions"}
      </h3>

      <div className="flex flex-col gap-2">
        <button
          onClick={handleCreateSnapshot}
          disabled={!accountId || createSnapshot.isPending}
          className="flex items-center justify-center gap-2 rounded-md border bg-card px-3 py-2 text-sm font-medium transition-colors hover:bg-muted/50 disabled:cursor-not-allowed disabled:opacity-50"
        >
          <Camera className="h-4 w-4" />
          {createSnapshot.isPending
            ? (t("creatingSnapshot" as any) || "Creating…")
            : (t("createSnapshot" as any) || "Create Snapshot")}
        </button>

        <button
          onClick={onRefresh}
          className="flex items-center justify-center gap-2 rounded-md border bg-card px-3 py-2 text-sm font-medium transition-colors hover:bg-muted/50"
        >
          <RefreshCw className="h-4 w-4" />
          {t("refresh" as any) || "Refresh"}
        </button>
      </div>

      {notice === "success" && (
        <div className="flex items-center gap-1.5 text-xs text-green-600 dark:text-green-400">
          <CheckCircle2 className="h-3.5 w-3.5" />
          {t("snapshotCreated" as any) || "Snapshot created"}
        </div>
      )}
      {notice === "error" && (
        <div className="flex items-center gap-1.5 text-xs text-red-600 dark:text-red-400">
          <AlertCircle className="h-3.5 w-3.5" />
          {t("failedToCreateSnapshot" as any) || "Failed to create snapshot"}
        </div>
      )}
    </div>
  );
}