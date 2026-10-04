/**
 * Application startup and lifecycle coordinator.
 *
 * Provides startup diagnostics, environment verification, and initial services
 * readiness checks.
 *
 * @module app/startup
 */

export interface AppStartupInfo {
  isTauri: boolean;
  platform: string;
  userAgent: string;
  startedAt: string;
}

/**
 * Collects runtime startup diagnostics for the desktop environment.
 */
export function getAppStartupInfo(): AppStartupInfo {
  const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
  return {
    isTauri,
    platform: typeof navigator !== "undefined" ? navigator.platform : "unknown",
    userAgent: typeof navigator !== "undefined" ? navigator.userAgent : "unknown",
    startedAt: new Date().toISOString(),
  };
}

/**
 * Performs startup environment checks (e.g., local storage access, online status).
 */
export function verifyStartupEnvironment(): { ok: boolean; errors: string[] } {
  const errors: string[] = [];

  if (typeof window !== "undefined") {
    try {
      const testKey = "__alpha_forge_startup_test__";
      window.localStorage.setItem(testKey, "1");
      window.localStorage.removeItem(testKey);
    } catch {
      errors.push("localStorage is inaccessible or quota exceeded.");
    }
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}
