/**
 * useSidebarState Hook
 *
 * Manages sidebar state with localStorage persistence.
 * Provides state management for expand/collapse and width.
 *
 * @module hooks/layout
 */

import { useState, useEffect, useCallback } from "react";
import type { SidebarState } from "@/components/layout/types";

interface SidebarStateConfig {
  /** Storage key for persistence */
  storageKey: string;
  /** Default state */
  defaultState?: SidebarState;
  /** Default width in pixels */
  defaultWidth?: number;
  /** Minimum width constraint */
  minWidth?: number;
  /** Maximum width constraint */
  maxWidth?: number;
}

interface SidebarStateReturn {
  /** Current sidebar state */
  state: SidebarState;
  /** Current width in pixels */
  width: number;
  /** Toggle sidebar state */
  toggleState: () => void;
  /** Set specific state */
  setState: (state: SidebarState) => void;
  /** Set width with constraints */
  setWidth: (width: number) => void;
  /** Reset to defaults */
  reset: () => void;
  /** Whether sidebar is expanded */
  isExpanded: boolean;
}

/**
 * Hook for managing sidebar state with persistence
 */
export function useSidebarState({
  storageKey,
  defaultState = "expanded",
  defaultWidth = 240,
  minWidth = 180,
  maxWidth = 400,
}: SidebarStateConfig): SidebarStateReturn {
  // Initialize state from localStorage or defaults
  const [state, setStateInternal] = useState<SidebarState>(() => {
    if (typeof window === "undefined") return defaultState;
    const stored = localStorage.getItem(`${storageKey}:state`);
    return (stored as SidebarState) || defaultState;
  });

  const [width, setWidthInternal] = useState<number>(() => {
    if (typeof window === "undefined") return defaultWidth;
    const stored = localStorage.getItem(`${storageKey}:width`);
    const parsed = stored ? parseInt(stored, 10) : defaultWidth;
    // Ensure width is within constraints
    return Math.max(minWidth, Math.min(maxWidth, parsed));
  });

  // Persist state changes
  useEffect(() => {
    localStorage.setItem(`${storageKey}:state`, state);
  }, [state, storageKey]);

  // Persist width changes
  useEffect(() => {
    localStorage.setItem(`${storageKey}:width`, width.toString());
  }, [width, storageKey]);

  // Toggle state
  const toggleState = useCallback(() => {
    setStateInternal((prev) => (prev === "expanded" ? "collapsed" : "expanded"));
  }, []);

  // Set specific state
  const setState = useCallback((newState: SidebarState) => {
    setStateInternal(newState);
  }, []);

  // Set width with constraints
  const setWidth = useCallback((newWidth: number) => {
    const constrainedWidth = Math.max(minWidth, Math.min(maxWidth, newWidth));
    setWidthInternal(constrainedWidth);
  }, [minWidth, maxWidth]);

  // Reset to defaults
  const reset = useCallback(() => {
    setStateInternal(defaultState);
    setWidthInternal(defaultWidth);
  }, [defaultState, defaultWidth]);

  return {
    state,
    width,
    toggleState,
    setState,
    setWidth,
    reset,
    isExpanded: state === "expanded",
  };
}
