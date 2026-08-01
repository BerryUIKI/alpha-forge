import { QueryClientProvider } from "@/lib/query-client/QueryClientProvider";
import { ErrorBoundary } from "@/components/common/ErrorBoundary";
import { ThemeProvider } from "@/components/providers/ThemeProvider";
import { Toaster } from "@investment-os/ui";
import type { ReactNode } from "react";

export function Providers({ children }: { children: ReactNode }) {
  return (
    <ErrorBoundary>
      <ThemeProvider>
        <QueryClientProvider>{children}</QueryClientProvider>
        <Toaster />
      </ThemeProvider>
    </ErrorBoundary>
  );
}
