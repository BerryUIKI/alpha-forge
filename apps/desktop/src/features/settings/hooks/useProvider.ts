// Hooks for AI provider settings.

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";

export const PROVIDER_KEYS = {
  status: ["provider", "status"] as const,
};

/**
 * Hook to read provider configuration status.
 */
export function useProviderStatus() {
  return useQuery({
    queryKey: PROVIDER_KEYS.status,
    queryFn: () => desktopApi.provider.getProviderStatus(),
  });
}

/**
 * Hook to save an API key in the OS keychain.
 */
export function useSetProviderApiKey() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (apiKey: string) => desktopApi.provider.setProviderApiKey(apiKey),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: PROVIDER_KEYS.status });
    },
  });
}

/**
 * Hook to remove the stored API key.
 */
export function useClearProviderApiKey() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: () => desktopApi.provider.clearProviderApiKey(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: PROVIDER_KEYS.status });
    },
  });
}
