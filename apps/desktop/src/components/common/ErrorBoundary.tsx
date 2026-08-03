// Global error boundary for catching React errors.

import { Component, ErrorInfo, ReactNode, useContext } from "react";
import { ErrorState } from "@/components/common/ErrorState";
import { LocaleContext } from "@/lib/i18n/locale-context";

interface Props {
  children: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
    error: null,
  };

  public static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error("Uncaught error:", error, errorInfo);
  }

  private handleRetry = () => {
    this.setState({ hasError: false, error: null });
  };

  public render() {
    if (this.state.hasError) {
      return <ErrorBoundaryFallback error={this.state.error} onRetry={this.handleRetry} />;
    }

    return this.props.children;
  }
}

function ErrorBoundaryFallback({ error, onRetry }: { error: Error | null; onRetry: () => void }) {
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