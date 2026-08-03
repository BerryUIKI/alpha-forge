// Error state component.

import { AlertCircle, RefreshCw } from "lucide-react";

interface ErrorStateProps {
  title?: string;
  message: string;
  /** Localized retry button text */
  retryLabel?: string;
  onRetry?: () => void;
}

export function ErrorState({
  title,
  message,
  retryLabel = "Try Again",
  onRetry,
}: ErrorStateProps) {
  return (
    <div
      className="flex flex-col items-center justify-center p-8 text-center"
      role="alert"
      aria-live="polite"
    >
      <div className="mb-4 rounded-full bg-destructive/10 p-4">
        <AlertCircle className="h-8 w-8 text-destructive" aria-hidden="true" />
      </div>
      <h3 className="mb-2 text-lg font-semibold">{title || "Something went wrong"}</h3>
      <p className="mb-4 max-w-md text-sm text-muted-foreground">{message}</p>
      {onRetry && (
        <button
          className="inline-flex items-center rounded-md border border-input bg-background px-3 py-2 text-sm font-medium ring-offset-background transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
          onClick={onRetry}
          aria-label={retryLabel}
        >
          <RefreshCw className="mr-2 h-4 w-4" aria-hidden="true" />
          {retryLabel}
        </button>
      )}
    </div>
  );
}