import { QueryClientProvider } from "@/lib/query-client/QueryClientProvider";
import { ErrorBoundary } from "@/components/common/ErrorBoundary";
import { ThemeProvider } from "@/components/providers/ThemeProvider";
import { LocaleProvider } from "@/lib/i18n/LocaleProvider";
import { Toaster } from "@investment-os/ui";
import type { ReactNode } from "react";

export function Providers({ children }: { children: ReactNode }) {
  return (
    <ErrorBoundary>
      <ThemeProvider>
        <LocaleProvider>
          <QueryClientProvider>{children}</QueryClientProvider>
        </LocaleProvider>
        <Toaster />
      </ThemeProvider>
    </ErrorBoundary>
  );
}
