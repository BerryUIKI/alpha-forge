/**
 * Displays a user-friendly error message for feature-level errors.
 */

import { useLocale } from "@/lib/i18n/useLocale";

export function FeatureErrorState({
  feature,
  error,
  onRetry,
}: {
  feature: string;
  error: Error | null;
  onRetry: () => void;
}) {
  const { t } = useLocale();

  const featureNames: Record<string, string> = {
    portfolio: t("portfolioFeature"),
    thesis: t("thesisFeature"),
    research: t("researchFeature"),
    options: t("optionsFeature"),
    goose: t("gooseFeature"),
    artifacts: t("artifactsFeature"),
    today: t("todayFeature"),
    journal: t("journalFeature"),
    knowledge: t("knowledgeFeature"),
    settings: t("settingsFeature"),
  };

  const featureName = featureNames[feature] || feature;
  const errorTitle = t("featureErrorTitle").replace("{feature}", featureName);

  return (
    <div className="rounded-lg border border-destructive/20 bg-destructive/5 p-6">
      <h2 className="text-lg font-semibold text-destructive">
        {errorTitle}
      </h2>
      <p className="mt-2 text-sm text-muted-foreground">
        {t("featureErrorDescription")}
      </p>
      {error && (
        <details className="mt-4">
          <summary className="cursor-pointer text-sm text-muted-foreground hover:text-foreground">
            {t("errorDetails")}
          </summary>
          <pre className="mt-2 overflow-x-auto rounded bg-muted p-2 text-xs">
            {error.message}
          </pre>
        </details>
      )}
      <button
        onClick={onRetry}
        className="mt-4 rounded-md border border-input bg-background px-4 py-2 text-sm hover:bg-accent"
      >
        {t("retryFeature")}
      </button>
    </div>
  );
}
