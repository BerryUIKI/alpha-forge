import { useState, useEffect } from "react";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export function useAppReady() {
  const [isReady, setIsReady] = useState(() => {
    // Default to true in non-Tauri environments to avoid blocking
    if (typeof window === "undefined") return true;
    return !("__TAURI_INTERNALS__" in window);
  });
  const [initError, setInitError] = useState<string | null>(null);

  useEffect(() => {
    const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
    if (!isTauri) {
      return;
    }

    let unlistenReady: UnlistenFn | undefined;
    let unlistenError: UnlistenFn | undefined;
    let isMounted = true;

    const setupListeners = async () => {
      try {
        const readyUnlisten = await listen("app:ready", () => {
          if (isMounted) setIsReady(true);
        });
        if (isMounted) {
          unlistenReady = readyUnlisten;
        } else {
          readyUnlisten();
        }

        const errorUnlisten = await listen<string>("app:init-failed", (event) => {
          if (isMounted) setInitError(event.payload);
        });
        if (isMounted) {
          unlistenError = errorUnlisten;
        } else {
          errorUnlisten();
        }
      } catch (err) {
        console.error("Failed to setup app ready listeners", err);
      }
    };

    setupListeners();

    return () => {
      isMounted = false;
      if (unlistenReady) unlistenReady();
      if (unlistenError) unlistenError();
    };
  }, []);

  return { isReady, initError };
}
