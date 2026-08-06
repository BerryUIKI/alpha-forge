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
 * Functional view types (6 views as per GUI_SPECIFICATION)
 */
export type FunctionalView =
  | "analyze"
  | "quantification"
  | "comprehensive-market"
  | "options"
  | "futures"
  | "other-derivatives";

/**
 * Tool definition
 */
export interface Tool {
  /** Unique identifier */
  id: string;
  /** Tool name (i18n key) */
  nameKey: string;
  /** Icon component name from lucide-react */
  icon: string;
  /** Route path */
  route?: string;
  /** Description (i18n key) */
  descriptionKey?: string;
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
 * Default sidebar widths
 */
export const DEFAULT_SIDEBAR_WIDTHS = {
  left: {
    default: 240,
    min: 180,
    max: 400,
  },
  right: {
    default: 280,
    min: 200,
    max: 400,
  },
} as const;