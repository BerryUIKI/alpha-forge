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
import { FeatureErrorState } from "./FeatureErrorState";

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
