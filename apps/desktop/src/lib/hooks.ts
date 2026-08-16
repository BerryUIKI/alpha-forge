/**
 * Shared hooks — Stub module
 *
 * TODO: Implement focus trap and escape key hooks.
 * These stubs prevent typecheck failures while the feature is incomplete.
 */

import { useRef, useEffect, useCallback } from "react";

/**
 * Focus trap hook — stub implementation
 * Prevents focus from leaving a modal/container.
 */
export function useFocusTrap<T extends HTMLElement>({
  enabled = true,
  returnFocus,
}: {
  enabled?: boolean;
  returnFocus?: HTMLElement | null;
}): React.RefObject<T | null> {
  const ref = useRef<T>(null);

  useEffect(() => {
    if (!enabled) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key !== "Tab") return;
      const container = ref.current;
      if (!container) return;

      const focusable = container.querySelectorAll<HTMLElement>(
        'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])',
      );
      if (focusable.length === 0) return;

      const first = focusable[0];
      const last = focusable[focusable.length - 1];

      if (e.shiftKey) {
        if (document.activeElement === first) {
          last?.focus();
          e.preventDefault();
        }
      } else {
        if (document.activeElement === last) {
          first?.focus();
          e.preventDefault();
        }
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    // Focus first element
    const timer = setTimeout(() => {
      const container = ref.current;
      if (container) {
        const first = container.querySelector<HTMLElement>(
          'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])',
        );
        first?.focus();
      }
    }, 0);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
      clearTimeout(timer);
      if (returnFocus) {
        returnFocus.focus();
      }
    };
  }, [enabled, returnFocus]);

  return ref;
}

/**
 * Escape key hook — stub implementation
 * Calls a callback when Escape is pressed.
 */
export function useEscapeKey(callback: () => void, enabled = true): void {
  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        callback();
      }
    },
    [callback],
  );

  useEffect(() => {
    if (!enabled) return;
    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [handleKeyDown, enabled]);
}