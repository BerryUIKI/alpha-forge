/**
 * Layout component type definitions
 *
 * Simplified types for the unified routing-based layout.
 */

/**
 * Sidebar collapse state
 */
export type SidebarState = "expanded" | "collapsed";

/**
 * Application status displayed in status bar
 */
export type AppStatus = "idle" | "running" | "error" | "syncing";

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
  right: {
    default: 280,
    min: 200,
    max: 400,
  },
} as const;