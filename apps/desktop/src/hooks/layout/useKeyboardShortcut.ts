/**
 * useKeyboardShortcut Hook
 *
 * Handles keyboard shortcuts for layout controls.
 * Provides a simple API for registering and handling keyboard events.
 *
 * @module hooks/layout
 */

import { useEffect, useCallback } from "react";

type KeyModifier = "ctrl" | "alt" | "shift" | "meta";

interface KeyboardShortcutConfig {
  /** Key to listen for (e.g., '1', 'b', 'k') */
  key: string;
  /** Modifier keys required */
  modifiers?: KeyModifier[];
  /** Callback when shortcut is triggered */
  callback: () => void;
  /** Whether the shortcut is enabled */
  enabled?: boolean;
  /** Prevent default browser behavior */
  preventDefault?: boolean;
  /**
   * Ignore the shortcut while focus is in a text-editing control.
   * Use for shortcuts that would otherwise interfere with typing.
   */
  ignoreEditable?: boolean;
}

function isEditableTarget(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) return false;
  if (target.isContentEditable) return true;
  const tag = target.tagName;
  return tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT";
}

/**
 * Hook for handling keyboard shortcuts
 */
export function useKeyboardShortcut({
  key,
  modifiers = [],
  callback,
  enabled = true,
  preventDefault = true,
  ignoreEditable = false,
}: KeyboardShortcutConfig) {
  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      // Check if enabled
      if (!enabled) return;

      // Never run during IME composition (e.g., Chinese input).
      if (event.isComposing) return;

      // Optionally stay out of the way of text editing.
      if (ignoreEditable && isEditableTarget(event.target)) return;

      // Check if key matches
      if (event.key.toLowerCase() !== key.toLowerCase()) return;

      // Check modifiers
      const hasModifier = (modifier: KeyModifier) => {
        switch (modifier) {
          case "ctrl":
            return event.ctrlKey || event.metaKey; // metaKey for macOS
          case "alt":
            return event.altKey;
          case "shift":
            return event.shiftKey;
          case "meta":
            return event.metaKey;
          default:
            return false;
        }
      };

      // Verify all required modifiers are pressed
      const modifiersPressed = modifiers.every(hasModifier);

      // If no modifiers required, ensure no modifiers are pressed
      const noModifiersPressed =
        !event.ctrlKey && !event.altKey && !event.shiftKey && !event.metaKey;

      if (modifiers.length > 0 && !modifiersPressed) return;
      if (modifiers.length === 0 && !noModifiersPressed) return;

      // Prevent default behavior if requested
      if (preventDefault) {
        event.preventDefault();
      }

      // Execute callback
      callback();
    },
    [key, modifiers, callback, enabled, preventDefault, ignoreEditable],
  );

  useEffect(() => {
    if (!enabled) return;

    window.addEventListener("keydown", handleKeyDown);

    return () => {
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [handleKeyDown, enabled]);
}

/**
 * Hook for handling sidebar keyboard shortcuts
 */
export function useSidebarShortcuts(config: {
  onToggleLeft?: () => void;
  onToggleRight?: () => void;
  onToggleBoth?: () => void;
  enabled?: boolean;
}) {
  // Ctrl/Cmd + 1: Toggle left sidebar
  useKeyboardShortcut({
    key: "1",
    modifiers: ["ctrl"],
    callback: config.onToggleLeft || (() => {}),
    enabled: config.enabled,
  });

  // Ctrl/Cmd + 2: Toggle right sidebar
  useKeyboardShortcut({
    key: "2",
    modifiers: ["ctrl"],
    callback: config.onToggleRight || (() => {}),
    enabled: config.enabled,
  });

  // Ctrl/Cmd + J: Toggle right (Agent Copilot) sidebar
  useKeyboardShortcut({
    key: "j",
    modifiers: ["ctrl"],
    callback: config.onToggleRight || (() => {}),
    enabled: config.enabled,
  });

  // Ctrl/Cmd + B: Toggle both sidebars
  useKeyboardShortcut({
    key: "b",
    modifiers: ["ctrl"],
    callback: config.onToggleBoth || (() => {}),
    enabled: config.enabled,
  });
}
