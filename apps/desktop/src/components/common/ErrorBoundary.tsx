// Global error boundary for catching React errors.

import { Component, ErrorInfo, ReactNode } from "react";
import { ErrorState } from "@/components/common/ErrorState";

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
      return (
        <div className="flex min-h-screen items-center justify-center">
          <ErrorState
            title="Application Error"
            message={this.state.error?.message || "An unexpected error occurred"}
            onRetry={this.handleRetry}
          />
        </div>
      );
    }

    return this.props.children;
  }
}