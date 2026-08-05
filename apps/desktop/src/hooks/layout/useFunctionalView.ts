/**
 * Functional View State Hook
 *
 * Manages the current functional view selection and persists it to localStorage.
 * Provides tools for the selected view and route information.
 *
 * @module hooks/layout/useFunctionalView
 */

import { useState, useEffect, useCallback } from "react";
import { useNavigate } from "react-router-dom";
import type { FunctionalView, ToolItem } from "@/components/layout/types";
import { VIEW_ROUTE_MAP, FUNCTIONAL_VIEW_OPTIONS } from "@/components/layout/types";
import { getToolsForView } from "@/components/layout/tools-config";

const FUNCTIONAL_VIEW_STORAGE_KEY = "app.functional-view";

/**
 * Hook to manage functional view state
 *
 * @returns {Object} Functional view state and actions
 * - view: Current selected functional view
 * - setView: Update the functional view (navigates to corresponding route)
 * - route: Current route for the selected view
 * - tools: Tools available for the selected view
 * - options: Available view options for dropdown
 */
export function useFunctionalView() {
  const navigate = useNavigate();

  // Initialize from localStorage or default to "analyze"
  const [view, setViewInternal] = useState<FunctionalView>(() => {
    const stored = localStorage.getItem(FUNCTIONAL_VIEW_STORAGE_KEY);
    if (stored && FUNCTIONAL_VIEW_OPTIONS.some(opt => opt.value === stored)) {
      return stored as FunctionalView;
    }
    return "analyze";
  });

  // Get route and tools for current view
  const route = VIEW_ROUTE_MAP[view];
  const tools: ToolItem[] = getToolsForView(view);

  // Set view and navigate to corresponding route
  const setView = useCallback((newView: FunctionalView) => {
    setViewInternal(newView);
    localStorage.setItem(FUNCTIONAL_VIEW_STORAGE_KEY, newView);

    // Navigate to the corresponding route
    const targetRoute = VIEW_ROUTE_MAP[newView];
    navigate(targetRoute);
  }, [navigate]);

  // Sync with localStorage on mount
  useEffect(() => {
    const stored = localStorage.getItem(FUNCTIONAL_VIEW_STORAGE_KEY);
    if (stored && stored !== view && FUNCTIONAL_VIEW_OPTIONS.some(opt => opt.value === stored)) {
      setViewInternal(stored as FunctionalView);
    }
  }, [view]);

  return {
    view,
    setView,
    route,
    tools,
    options: FUNCTIONAL_VIEW_OPTIONS,
  };
}

/**
 * Hook to get tools for a specific view (without navigation)
 *
 * @param view - Functional view to get tools for
 * @returns Array of tools for the view
 */
export function useViewTools(view: FunctionalView): ToolItem[] {
  return getToolsForView(view);
}