// Offline state component.

import { WifiOff } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";

interface OfflineStateProps {
  /** Custom description override */
  description?: string;
  /** Optional retry action */
  onRetry?: () => void;
}

export function OfflineState({ description, onRetry }: OfflineStateProps) {
  const { t } = useLocale();

  return (
    <div
      className="flex flex-col items-center justify-center p-8 text-center"
      role="alert"
      aria-live="polite"
      aria-label={t("offline")}
    >
      <div className="mb-4 rounded-full bg-muted p-4">
        <WifiOff className="h-8 w-8 text-muted-foreground" aria-hidden="true" />
      </div>
      <h3 className="mb-2 text-lg font-semibold">{t("offline")}</h3>
      <p className="mb-4 max-w-md text-sm text-muted-foreground">
        {description || t("offlineDescription")}
      </p>
      {onRetry && (
        <button
          className="inline-flex items-center rounded-md border border-input bg-background px-3 py-2 text-sm font-medium ring-offset-background transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
          onClick={onRetry}
          aria-label={t("retry")}
        >
          {t("retry")}
        </button>
      )}
    </div>
  );
}
