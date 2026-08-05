/**
 * useSystemHealth Hook
 *
 * Polls the backend database health check endpoint.
 * Uses TanStack Query for caching and refetch intervals.
 *
 * @module hooks/useSystemHealth
 */

import { useQuery } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";

interface SystemHealth {
  /** Whether the database is healthy */
  isHealthy: boolean;
  /** Whether there was an error checking health */
  hasError: boolean;
  /** Error message if any */
  errorMessage?: string;
  /** Raw health check result */
  status?: string;
}

const HEALTH_CHECK_INTERVAL_MS = 30000; // 30 seconds

/**
 * Hook for monitoring backend/database health status
 */
export function useSystemHealth(): SystemHealth {
  const { data, isError, error } = useQuery({
    queryKey: ["system-health"],
    queryFn: () => desktopApi.system.checkDatabaseHealth(),
    refetchInterval: HEALTH_CHECK_INTERVAL_MS,
    retry: false,
    staleTime: HEALTH_CHECK_INTERVAL_MS,
  });

  return {
    isHealthy: !isError && data === "healthy",
    hasError: isError,
    errorMessage: error?.message,
    status: data,
  };
}