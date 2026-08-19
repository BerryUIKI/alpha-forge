/**
 * Feature Error Boundary Component
 *
 * Catches rendering errors within a specific feature section and displays
 * a user-friendly error state with retry capability, preventing the entire
 * application from crashing due to an isolated feature failure.
 *
 * @module components/common/FeatureErrorBoundary
 */

import { Component, type ErrorInfo, type ReactNode } from "react";
import { useLocale } from "@/lib/i18n/useLocale";

interface FeatureErrorBoundaryProps {
  children: ReactNode;
  feature: string;
  onRetry?: () => void;
}

interface FeatureErrorBoundaryState {
  hasError: boolean;
  error: Error | null;
}

export class FeatureErrorBoundary extends Component<
  FeatureErrorBoundaryProps,
  FeatureErrorBoundaryState
> {
  constructor(props: FeatureErrorBoundaryProps) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error): FeatureErrorBoundaryState {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo): void {
    console.error(
      `[FeatureErrorBoundary] Error in feature "${this.props.feature}":`,
      error,
      errorInfo,
    );
  }

  handleRetry = (): void => {
    this.setState({ hasError: false, error: null });
    this.props.onRetry?.();
  };

  render(): ReactNode {
    if (this.state.hasError) {
      return (
        <FeatureErrorState
          feature={this.props.feature}
          error={this.state.error}
          onRetry={this.handleRetry}
        />
      );
    }

    return this.props.children;
  }
}

/**
 * Displays a user-friendly error message for feature-level errors.
 */
function FeatureErrorState({
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