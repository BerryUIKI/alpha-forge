/**
 * Layout component type definitions for the redesigned GUI.
 *
 * This module defines the core layout types for the three-zone layout:
 * LeftSidebar (navigation) | MainContent (TopBar + Outlet + StatusBar) | RightSidebar (Agent)
 */

import type { LucideIcon } from "lucide-react";

/**
 * Sidebar collapse state
 */
export type SidebarState = "expanded" | "collapsed";

/**
 * Application status displayed in status bar
 */
export type AppStatus = "idle" | "running" | "error" | "syncing";

/**
 * Agent connection status with 4 states
 * Used for Agent panel status indicator
 */
export type AgentConnectionStatus =
  | "idle"         // Gray - no running tasks
  | "running"      // Blue (blinking) - has running/queued tasks
  | "unconfigured" // Yellow - API key/model params empty
  | "error";       // Red - timeout, API error, proxy error

/**
 * Agent status display configuration
 */
export interface AgentStatusConfig {
  status: AgentConnectionStatus;
  label: string;
  labelKey: string;
  colorClass: string;
  animate?: boolean;
}

/**
 * Navigation item definition
 */
export interface NavItem {
  /** Unique identifier */
  id: string;
  /** Display label (i18n key) */
  label: string;
  /** Icon component */
  icon: LucideIcon;
  /** Route path */
  route: string;
  /** Search keywords for command palette */
  keywords?: string[];
}

/**
 * Navigation group with section label and items
 */
export interface NavGroup {
  /** Unique identifier */
  id: string;
  /** Section label (i18n key) */
  label: string;
  /** Navigation items in this group */
  items: NavItem[];
}

/**
 * Left sidebar props
 */
export interface LeftSidebarProps {
  /** Current expanded/collapsed state */
  state?: SidebarState;
  /** Callback when sidebar state changes */
  onStateChange?: (state: SidebarState) => void;
  /** Default width in pixels */
  defaultWidth?: number;
  /** Minimum width when expanded */
  minWidth?: number;
  /** Maximum width when expanded */
  maxWidth?: number;
}

/**
 * Top bar props
 */
export interface TopBarProps {
  /** Whether right sidebar is expanded */
  isRightSidebarExpanded?: boolean;
  /** Callback to toggle right sidebar */
  onToggleRightSidebar?: () => void;
}

/**
 * Main content area props
 */
export interface MainContentProps {
  /** Children to render (Outlet) */
  children?: React.ReactNode;
  /** Whether right sidebar is visible */
  isRightSidebarExpanded?: boolean;
  /** Callback to toggle right sidebar */
  onToggleRightSidebar?: () => void;
}

/**
 * Status bar props
 */
export interface StatusBarProps {
  /** Currently active workspace name */
  workspaceName?: string;
  /** Application running status */
  status?: AppStatus;
  /** Hint text to display */
  hint?: string;
}

/**
 * Right sidebar (Agent panel) props
 */
export interface RightSidebarProps {
  /** Current expanded/collapsed state */
  state?: SidebarState;
  /** Callback when sidebar state changes */
  onStateChange?: (state: SidebarState) => void;
  /** Default width in pixels */
  defaultWidth?: number;
  /** Minimum width when expanded */
  minWidth?: number;
  /** Maximum width when expanded */
  maxWidth?: number;
}

/**
 * Agent panel props (content of right sidebar)
 */
export interface AgentPanelProps {
  /** Agent status message */
  status?: string;
  /** Placeholder content */
  placeholder?: string;
}

/**
 * Default sidebar widths
 */
export const DEFAULT_SIDEBAR_WIDTHS = {
  left: {
    default: 220,
    min: 180,
    max: 320,
  },
  right: {
    default: 320,
    min: 260,
    max: 400,
  },
} as const;