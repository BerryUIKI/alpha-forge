/**
 * Layout component type definitions for GUI hybrid layout
 *
 * This module defines the core layout structure following the Codex-style paradigm
 * combined with route-based navigation.
 */

import type { LucideIcon } from "lucide-react";

/**
 * Functional view types (left sidebar dropdown)
 * Maps to different feature areas in the application
 */
export type FunctionalView =
  | "analyze"              // 分析 → /research
  | "quantification"       // 量化 → /research
  | "comprehensive-market" // 综合市场 → /research
  | "options"              // 期权 → /options
  | "futures"              // 期货 → /research (future expansion)
  | "other-derivatives";   // 其他衍生品 → /research (future expansion)

/**
 * Legacy alias for backwards compatibility
 * @deprecated Use FunctionalView instead
 */
export type WorkspaceType = FunctionalView;

/**
 * Sidebar collapse state
 */
export type SidebarState = "expanded" | "collapsed";

/**
 * User menu options in the bottom user operations area
 */
export type UserMenuItem =
  | "profile"
  | "theme-toggle"
  | "language"
  | "settings";

/**
 * Application status displayed in status bar
 */
export type AppStatus = "idle" | "running" | "error" | "syncing";

/**
 * Agent connection status with 4 states
 * Used for Agent panel status indicator
 */
export type AgentConnectionStatus =
  | "idle"         // Gray - 空闲待命 - no running tasks
  | "running"      // Blue (blinking) - 任务执行中 - has running/queued tasks
  | "unconfigured" // Yellow - 需要完成助手配置 - API key/model params empty
  | "error";       // Red - 连接失败 - timeout, API error, proxy error

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
 * Left sidebar props
 */
export interface LeftSidebarProps {
  /** Current expanded/collapsed state */
  state?: SidebarState;
  /** Callback when sidebar state changes */
  onStateChange?: (state: SidebarState) => void;
  /** Current selected workspace type */
  selectedWorkspace?: WorkspaceType;
  /** Callback when workspace selection changes */
  onWorkspaceChange?: (workspace: WorkspaceType) => void;
  /** Default width in pixels */
  defaultWidth?: number;
  /** Minimum width when expanded */
  minWidth?: number;
  /** Maximum width when expanded */
  maxWidth?: number;
}

/**
 * Workspace selector dropdown props
 */
export interface WorkspaceSelectorProps {
  /** Currently selected workspace */
  selected?: WorkspaceType;
  /** Callback when selection changes */
  onSelect?: (workspace: WorkspaceType) => void;
  /** Whether the dropdown is open */
  isOpen?: boolean;
  /** Callback when dropdown state changes */
  onOpenChange?: (open: boolean) => void;
}

/**
 * Scrollable list item type
 */
export interface ScrollableListItem {
  id: string;
  label: string;
  icon?: React.ComponentType<{ className?: string }>;
}

/**
 * Scrollable list props (middle section of left sidebar)
 */
export interface ScrollableListProps {
  /** List items to display */
  items?: ScrollableListItem[];
  /** Selected item ID */
  selectedId?: string;
  /** Callback when item is selected */
  onSelect?: (id: string) => void;
  /** Empty state message */
  emptyMessage?: string;
  /** Loading state */
  isLoading?: boolean;
  /** Error state */
  error?: Error | null;
}

/**
 * User operations menu props (bottom section of left sidebar)
 */
export interface UserOperationsProps {
  /** Current username */
  username?: string;
  /** Whether the user menu is open */
  isMenuOpen?: boolean;
  /** Callback when menu state changes */
  onMenuOpenChange?: (open: boolean) => void;
  /** Callback when menu item is clicked */
  onMenuItemClick?: (item: UserMenuItem) => void;
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
 * Operation bar props (top of main content)
 */
export interface OperationBarProps {
  /** Current context title */
  title?: string;
  /** Whether right sidebar is expanded */
  isRightSidebarExpanded?: boolean;
  /** Callback to toggle right sidebar */
  onToggleRightSidebar?: () => void;
}

/**
 * Status bar props (bottom of main content)
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
 * Functional view options for dropdown selector
 */
export const FUNCTIONAL_VIEW_OPTIONS: Array<{ value: FunctionalView; labelKey: string }> = [
  { value: "analyze", labelKey: "workspaceTypeAnalyze" },
  { value: "quantification", labelKey: "workspaceTypeQuantification" },
  { value: "comprehensive-market", labelKey: "workspaceTypeComprehensiveMarket" },
  { value: "options", labelKey: "workspaceTypeOptions" },
  { value: "futures", labelKey: "workspaceTypeFutures" },
  { value: "other-derivatives", labelKey: "workspaceTypeOtherDerivatives" },
];

/**
 * Legacy workspace options (deprecated, use FUNCTIONAL_VIEW_OPTIONS)
 * @deprecated Use FUNCTIONAL_VIEW_OPTIONS instead
 */
export const WORKSPACE_OPTIONS = FUNCTIONAL_VIEW_OPTIONS;

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

/**
 * Functional view to route mapping
 * Determines which page to navigate to when a view is selected
 */
export const VIEW_ROUTE_MAP: Record<FunctionalView, string> = {
  "analyze": "/research",
  "quantification": "/research",
  "comprehensive-market": "/research",
  "options": "/options",
  "futures": "/research",
  "other-derivatives": "/research",
} as const;

/**
 * Tool item definition for left sidebar tool list
 */
export interface ToolItem {
  /** Unique identifier */
  id: string;
  /** Display label (i18n key) */
  label: string;
  /** Icon component */
  icon: LucideIcon;
  /** Optional route to navigate to */
  route?: string;
  /** Optional custom action handler */
  action?: () => void;
  /** Whether the tool is disabled */
  disabled?: boolean;
}

/**
 * Tool item props for ToolsList component
 */
export interface ToolsListProps {
  /** Current functional view */
  view: FunctionalView;
  /** Selected tool ID */
  selectedToolId?: string;
  /** Callback when tool is selected */
  onSelectTool?: (toolId: string) => void;
}

/**
 * Workspace dropdown props for OperationBar
 */
export interface WorkspaceDropdownProps {
  /** Currently selected workspace ID */
  selectedWorkspaceId?: string;
  /** Callback when workspace is selected */
  onSelectWorkspace?: (workspaceId: string) => void;
}