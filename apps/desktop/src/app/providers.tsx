import { QueryClientProvider } from "@/lib/query-client/QueryClientProvider";
import { ErrorBoundary } from "@/components/common/ErrorBoundary";
import type { ReactNode } from "react";

export function Providers({ children }: { children: ReactNode }) {
  return (
    <ErrorBoundary>
      <QueryClientProvider>{children}</QueryClientProvider>
    </ErrorBoundary>
  );
}
