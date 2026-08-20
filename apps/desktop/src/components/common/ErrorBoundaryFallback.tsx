import { useContext } from "react";
import { ErrorState } from "@/components/common/ErrorState";
import { LocaleContext } from "@/lib/i18n/locale-context";

export function ErrorBoundaryFallback({ error, onRetry }: { error: Error | null; onRetry: () => void }) {
  const { t } = useContext(LocaleContext);
  return (
    <div className="flex min-h-screen items-center justify-center">
      <ErrorState
        title={t("unexpectedError")}
        message={error?.message || t("unexpectedErrorDescription")}
        retryLabel={t("retry")}
        onRetry={onRetry}
      />
    </div>
  );
}
