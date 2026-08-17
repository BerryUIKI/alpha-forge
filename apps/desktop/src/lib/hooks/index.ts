/**
 * Shared UI hooks — focus trap and escape-key handling.
 */

import { useEffect, useRef, type RefObject } from "react";

interface UseFocusTrapOptions {
  enabled: boolean;
  returnFocus?: HTMLElement | null;
}

/**
 * Trap keyboard focus inside the referenced container while enabled.
 * Returns a ref to attach to the container element.
 */
export function useFocusTrap<T extends HTMLElement>({
  enabled,
  returnFocus,
}: UseFocusTrapOptions): RefObject<T | null> {
  const containerRef = useRef<T | null>(null);
  const previousFocus = useRef<HTMLElement | null>(null);

  useEffect(() => {
    if (!enabled) return;

    previousFocus.current = returnFocus ?? (document.activeElement as HTMLElement);

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "Tab") return;

      const container = containerRef.current;
      if (!container) return;

      const focusables = container.querySelectorAll<HTMLElement>(
        'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])',
      );
      if (focusables.length === 0) return;

      const first = focusables[0];
      const last = focusables[focusables.length - 1];

      if (event.shiftKey && document.activeElement === first) {
        event.preventDefault();
        last?.focus();
      } else if (!event.shiftKey && document.activeElement === last) {
        event.preventDefault();
        first?.focus();
      }
    };

    document.addEventListener("keydown", handleKeyDown);

    // Move focus into the container
    const container = containerRef.current;
    const focusable = container?.querySelector<HTMLElement>(
      'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])',
    );
    focusable?.focus();

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
      // Restore focus to the element that triggered the dialog
      if (previousFocus.current) {
        previousFocus.current.focus();
      }
    };
  }, [enabled, returnFocus]);

  return containerRef;
}

/**
 * Call `handler` when the Escape key is pressed while `enabled`.
 */
export function useEscapeKey(
  handler: () => void,
  enabled = true,
): void {
  useEffect(() => {
    if (!enabled) return;

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        handler();
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [handler, enabled]);
}