/**
 * Element id helpers shared by TabBar and its page-level panels.
 *
 * Kept in their own module so the component file only exports components
 * (React Fast Refresh requirement).
 */

export function getTabId(prefix: string, tabId: string): string {
  return `${prefix}-tab-${tabId}`;
}

export function getTabPanelId(prefix: string, tabId: string): string {
  return `${prefix}-panel-${tabId}`;
}
