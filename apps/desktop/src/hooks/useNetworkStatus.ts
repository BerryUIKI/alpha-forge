/**
 * useNetworkStatus Hook
 *
 * Detects browser online/offline status in real-time.
 * Uses window 'online' and 'offline' events.
 *
 * @module hooks/useNetworkStatus
 */

import { useState, useEffect } from "react";

interface NetworkStatus {
  /** Whether the browser reports online status */
  isOnline: boolean;
  /** Whether the browser reports offline status */
  isOffline: boolean;
}

/**
 * Hook for detecting network connectivity status
 */
export function useNetworkStatus(): NetworkStatus {
  const [isOnline, setIsOnline] = useState<boolean>(
    typeof navigator !== "undefined" ? navigator.onLine : true
  );

  useEffect(() => {
    const handleOnline = () => setIsOnline(true);
    const handleOffline = () => setIsOnline(false);

    window.addEventListener("online", handleOnline);
    window.addEventListener("offline", handleOffline);

    return () => {
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
    };
  }, []);

  return {
    isOnline,
    isOffline: !isOnline,
  };
}