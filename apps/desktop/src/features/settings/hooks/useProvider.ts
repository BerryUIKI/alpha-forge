// Hooks for AI provider settings.

import { useState } from "react";
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
 *
 * The key is passed as a direct argument instead of through `useMutation` so it
 * is not retained in TanStack Query's mutation cache.
 */
export function useSetProviderApiKey() {
  const queryClient = useQueryClient();
  const [isSavingKey, setIsSavingKey] = useState(false);

  const saveApiKey = async (apiKey: string) => {
    setIsSavingKey(true);
    try {
      await desktopApi.provider.setProviderApiKey(apiKey);
      await queryClient.invalidateQueries({ queryKey: PROVIDER_KEYS.status });
    } finally {
      setIsSavingKey(false);
    }
  };

  return { saveApiKey, isSavingKey };
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
