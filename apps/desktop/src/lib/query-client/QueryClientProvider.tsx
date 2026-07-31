import { QueryClient, QueryClientProvider as TanStackProvider } from "@tanstack/react-query";
import type { ReactNode } from "react";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 30_000,
      retry: 1,
    },
  },
});

export function QueryClientProvider({ children }: { children: ReactNode }) {
  return <TanStackProvider client={queryClient}>{children}</TanStackProvider>;
}
