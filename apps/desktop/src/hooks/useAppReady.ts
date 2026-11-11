/**
 * Application readiness hook.
 *
 * Startup is signalled by `app:ready` / `app:init-failed`, but events can be
 * missed by panels that mount after the event fired. The hook therefore
 * registers its listeners first and then reads the backend readiness snapshot,
 * so an event arriving between registration and the snapshot read cannot be
 * lost. Listener failure and prolonged initialization have explicit states.
 */

import { useState, useEffect } from "react";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { desktopApi } from "@/lib/desktop-api";

/** How long to wait before telling the user that startup is taking unusually long. */
const INITIALIZATION_STALL_MS = 20_000;

export interface AppReadyState {
  isReady: boolean;
  initError: string | null;
  /** True when readiness listeners could not be registered. */
  listenerError: boolean;
  /** True when initialization has not settled within the stall window. */
  isStalled: boolean;
}

function isTauriEnvironment(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export function useAppReady(): AppReadyState {
  const [isReady, setIsReady] = useState(() => !isTauriEnvironment());
  const [initError, setInitError] = useState<string | null>(null);
  const [listenerError, setListenerError] = useState(false);
  const [isStalled, setIsStalled] = useState(false);

  useEffect(() => {
    if (!isTauriEnvironment()) {
      return;
    }

    let isMounted = true;
    let unlistenReady: UnlistenFn | undefined;
    let unlistenError: UnlistenFn | undefined;
    let stallTimer: ReturnType<typeof setTimeout> | undefined;

    const clearStallTimer = () => {
      if (stallTimer !== undefined) {
        clearTimeout(stallTimer);
        stallTimer = undefined;
      }
    };

    const markReady = () => {
      if (!isMounted) return;
      setIsReady(true);
      setIsStalled(false);
      clearStallTimer();
    };

    const markFailed = (code: string) => {
      if (!isMounted) return;
      setInitError(code);
      setIsStalled(false);
      clearStallTimer();
    };

    const probeReadiness = async () => {
      try {
        const status = await desktopApi.system.getAppStatus();
        if (status.status === "ready") {
          markReady();
        } else if (status.status === "failed") {
          markFailed(status.errorCode ?? "APP_INIT_FAILED");
        }
      } catch {
        // The probe is a fallback; events remain authoritative. If both the
        // probe and the listeners fail, the stall state reports the problem.
      }
    };

    const setupListeners = async () => {
      try {
        // Subscribe before reading the snapshot so events and state cannot race.
        const readyUnlisten = await listen("app:ready", markReady);
        if (isMounted) {
          unlistenReady = readyUnlisten;
        } else {
          readyUnlisten();
        }

        const errorUnlisten = await listen<string>("app:init-failed", (event) => {
          markFailed(event.payload);
        });
        if (isMounted) {
          unlistenError = errorUnlisten;
        } else {
          errorUnlisten();
        }
      } catch (error) {
        console.error("Failed to setup app ready listeners", error);
        if (isMounted) setListenerError(true);
      }

      await probeReadiness();
    };

    void setupListeners();

    stallTimer = setTimeout(() => {
      if (isMounted) setIsStalled(true);
    }, INITIALIZATION_STALL_MS);

    return () => {
      isMounted = false;
      clearStallTimer();
      if (unlistenReady) unlistenReady();
      if (unlistenError) unlistenError();
    };
  }, []);

  return { isReady, initError, listenerError, isStalled };
}
