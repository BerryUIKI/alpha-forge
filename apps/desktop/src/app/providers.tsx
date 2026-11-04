import { QueryClientProvider } from "@/lib/query-client/QueryClientProvider";
import { ErrorBoundary } from "@/components/common/ErrorBoundary";
import { ToastProvider } from "@/components/common/ToastProvider";
import { ThemeProvider } from "@/app/theme/ThemeProvider";
import type { ReactNode } from "react";

export function Providers({ children }: { children: ReactNode }) {
  return (
    <ThemeProvider>
      <ToastProvider>
        <ErrorBoundary>
          <QueryClientProvider>{children}</QueryClientProvider>
        </ErrorBoundary>
      </ToastProvider>
    </ThemeProvider>
  );
}
